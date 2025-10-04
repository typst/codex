use self::shared::ModifierSet;
use std::fmt::Write;
use std::iter;
use std::iter::Peekable;
use std::path::Path;

type StrResult<T> = Result<T, String>;

#[path = "src/shared.rs"]
mod shared;

/// A module of definitions.
struct Module<'a>(Vec<(&'a str, Binding<'a>)>);

impl<'a> Module<'a> {
    fn new(mut list: Vec<(&'a str, Binding<'a>)>) -> Self {
        list.sort_by_key(|&(name, _)| name);
        Self(list)
    }
}

/// A definition bound in a module, with metadata.
struct Binding<'a> {
    def: Def<'a>,
    deprecation: Option<&'a str>,
}

/// A definition in a module.
enum Def<'a> {
    Symbol(Symbol<'a>),
    Module(Module<'a>),
}

/// A symbol, either a leaf or with modifiers with optional deprecation.
enum Symbol<'a> {
    Single(String),
    Multi {
        variants: Vec<(ModifierSet<&'a str>, String)>,
        deprecations: Vec<(&'a str, &'a str)>,
    },
}

/// A single line during parsing.
#[derive(Debug, Clone)]
enum Line<'a> {
    Blank,
    Deprecated(Option<&'a str>, &'a str),
    ModuleStart(&'a str),
    ModuleEnd,
    Symbol(&'a str, Option<String>),
    Variant(ModifierSet<&'a str>, String),
    Eof,
}

#[derive(Debug, Clone)]
enum Declaration<'a> {
    ModuleStart(&'a str, Option<&'a str>),
    ModuleEnd,
    Symbol(&'a str, Option<String>, Vec<(Option<&'a str>, &'a str)>),
    Variant(ModifierSet<&'a str>, String),
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let mut buf = String::new();
    process(&mut buf, Path::new("src/modules/sym.txt"), "SYM", "Named general symbols.");
    process(&mut buf, Path::new("src/modules/emoji.txt"), "EMOJI", "Named emoji.");

    let out = std::env::var_os("OUT_DIR").unwrap();
    let dest = Path::new(&out).join("out.rs");
    std::fs::write(&dest, buf).unwrap();

    #[cfg(feature = "_test-unicode-conformance")]
    {
        let emoji_vs_list = Path::new(&out).join("emoji-variation-sequences.txt");
        if !std::fs::read_to_string(&emoji_vs_list)
            .is_ok_and(|text| text.contains("Emoji Version 16.0"))
        {
            let content = ureq::get(
                "https://www.unicode.org/Public/16.0.0/ucd/emoji/emoji-variation-sequences.txt",
            )
                .call()
                .unwrap()
                .body_mut()
                .read_to_string()
                .unwrap();
            std::fs::write(emoji_vs_list, content).unwrap();
        }
    }
}

/// Processes a single file and turns it into a global module.
fn process(buf: &mut String, file: &Path, name: &str, desc: &str) {
    println!("cargo::rerun-if-changed={}", file.display());

    let text = std::fs::read_to_string(file).unwrap();
    let mut line_nr = 0;
    let mut deprecations = Vec::new();
    let mut iter = text
        .lines()
        .inspect(|_| line_nr += 1)
        .map(tokenize)
        .chain(iter::once(Ok(Line::Eof)))
        .filter_map(|line| match line {
            Err(message) => Some(Err(message)),
            Ok(Line::Blank) => None,
            Ok(Line::Deprecated(modifier, message)) => {
                deprecations.push((modifier, message));
                None
            }
            Ok(Line::ModuleStart(name)) => {
                let deprecation = match *std::mem::take(&mut deprecations).as_slice() {
                    [] => None,
                    [(None, deprecation)] => Some(deprecation),
                    _ => return Some(Err("wrong deprecation format for module".into())),
                };
                Some(Ok(Declaration::ModuleStart(name, deprecation)))
            }
            Ok(Line::ModuleEnd) => {
                if !deprecations.is_empty() {
                    return Some(Err("dangling `@deprecated:`".into()));
                }
                Some(Ok(Declaration::ModuleEnd))
            }
            Ok(Line::Symbol(name, value)) => Some(Ok(Declaration::Symbol(
                name,
                value,
                std::mem::take(&mut deprecations),
            ))),
            Ok(Line::Variant(modifiers, value)) => {
                if !deprecations.is_empty() {
                    return Some(Err("dangling `@deprecated:`".into()));
                }
                Some(Ok(Declaration::Variant(modifiers, value)))
            }
            Ok(Line::Eof) => {
                if !deprecations.is_empty() {
                    return Some(Err("dangling `@deprecated:`".into()));
                }
                None
            }
        })
        .peekable();

    let module = match parse(&mut iter) {
        Ok(defs) => Module::new(defs),
        Err(e) => {
            let message = format!("{}:{}: {e}", file.display(), line_nr);
            println!("cargo::warning={message}");
            std::process::exit(1);
        }
    };

    write!(buf, "#[doc = {desc:?}] pub const {name}: Module = ").unwrap();
    encode(buf, &module);
    buf.push(';');
}

/// Tokenizes and classifies a line.
fn tokenize(line: &str) -> StrResult<Line<'_>> {
    // Strip comments.
    let line = line.split_once("//").map_or(line, |(head, _)| head);

    // Ignore empty lines.
    let line = line.trim();
    if line.is_empty() {
        return Ok(Line::Blank);
    }

    let (head, tail) = match line.split_once(' ') {
        Some((a, b)) => (a, Some(b)),
        None => (line, None),
    };

    Ok(
        if let Some(inner) =
            head.strip_prefix("@deprecated").and_then(|s| s.strip_suffix(':'))
        {
            let mut modifier = None;
            if !inner.is_empty() {
                modifier = Some(
                    inner
                        .strip_prefix('(')
                        .and_then(|s| s.strip_suffix(')'))
                        .ok_or("malformed modifier in deprecation")?,
                );
            }
            Line::Deprecated(modifier, tail.ok_or("missing deprecation message")?.trim())
        } else if tail == Some("{") {
            validate_ident(head)?;
            Line::ModuleStart(head)
        } else if head == "}" && tail.is_none() {
            Line::ModuleEnd
        } else if let Some(rest) = head.strip_prefix('.') {
            for part in rest.split('.') {
                validate_ident(part)?;
            }
            let value = decode_value(tail.ok_or("missing char")?)?;
            Line::Variant(ModifierSet::from_raw_dotted(rest), value)
        } else {
            validate_ident(head)?;
            let value = tail.map(decode_value).transpose()?;
            Line::Symbol(head, value)
        },
    )
}

/// Ensures that a string is a valid identifier. In `codex`, we use very strict
/// rules and allow only alphabetic ASCII chars.
fn validate_ident(string: &str) -> StrResult<()> {
    if !string.is_empty() && string.chars().all(|c| c.is_ascii_alphabetic()) {
        return Ok(());
    }
    Err(format!("invalid identifier: {string:?}"))
}

/// Extracts the value of a variant, parsing `\u{XXXX}` and other escapes.
fn decode_value(mut text: &str) -> StrResult<String> {
    let mut result = String::new();
    loop {
        if let Some(rest) = text.strip_prefix("\\u{") {
            let Some((code, tail)) = rest.split_once('}') else {
                return Err(format!(
                    "unclosed Unicode escape: \\u{{{}",
                    rest.escape_debug()
                ));
            };
            result.push(
                u32::from_str_radix(code, 16)
                    .ok()
                    .and_then(|n| char::try_from(n).ok())
                    .ok_or_else(|| format!("invalid Unicode escape \\u{{{code}}}"))?,
            );
            text = tail;
        } else if let Some(rest) = text.strip_prefix("\\vs{") {
            let Some((value, tail)) = rest.split_once('}') else {
                return Err(format!("unclosed VS escape: \\vs{{{}", rest.escape_debug()));
            };
            let vs = match value {
                "1" => '\u{fe00}',
                "2" => '\u{fe01}',
                "3" => '\u{fe02}',
                "4" => '\u{fe03}',
                "5" => '\u{fe04}',
                "6" => '\u{fe05}',
                "7" => '\u{fe06}',
                "8" => '\u{fe07}',
                "9" => '\u{fe08}',
                "10" => '\u{fe09}',
                "11" => '\u{fe0a}',
                "12" => '\u{fe0b}',
                "13" => '\u{fe0c}',
                "14" => '\u{fe0d}',
                "15" | "text" => '\u{fe0e}',
                "16" | "emoji" => '\u{fe0f}',
                code => return Err(format!("invalid VS escape: \\vs{{{code}}}")),
            };
            result.push(vs);
            text = tail;
        } else if let Some((prefix, tail)) = text.find('\\').map(|i| text.split_at(i)) {
            if prefix.is_empty() {
                return Err(format!("invalid escape sequence: {tail}"));
            }
            result.push_str(prefix);
            text = tail;
        } else {
            result.push_str(text);
            return Ok(result);
        }
    }
}

/// Turns a stream of lines into a list of definitions.
fn parse<'a>(
    p: &mut Peekable<impl Iterator<Item = StrResult<Declaration<'a>>>>,
) -> StrResult<Vec<(&'a str, Binding<'a>)>> {
    let mut defs = vec![];
    loop {
        match p.next().transpose()? {
            None | Some(Declaration::ModuleEnd) => {
                break;
            }
            Some(Declaration::Symbol(name, value, deprecations)) => {
                let mut variants = vec![];
                while let Some(Declaration::Variant(name, value)) =
                    p.peek().cloned().transpose()?
                {
                    variants.push((name, value));
                    p.next();
                }

                let deprecation =
                    deprecations.iter().find(|(m, _)| m.is_none()).map(|&(_, d)| d);

                let modifier_deprecations =
                    deprecations.iter().filter_map(|&(m, s)| m.map(|m| (m, s))).collect();

                let symbol = if !variants.is_empty() {
                    if let Some(value) = value {
                        variants.insert(0, (ModifierSet::default(), value));
                    }
                    Symbol::Multi { variants, deprecations: modifier_deprecations }
                } else {
                    let value = value.ok_or("symbol needs char or variants")?;
                    Symbol::Single(value)
                };

                defs.push((name, Binding { def: Def::Symbol(symbol), deprecation }));
            }
            Some(Declaration::ModuleStart(name, deprecation)) => {
                let module_defs = parse(p)?;
                defs.push((
                    name,
                    Binding {
                        def: Def::Module(Module::new(module_defs)),
                        deprecation,
                    },
                ));
            }
            other => return Err(format!("expected definition, found {other:?}")),
        }
    }
    Ok(defs)
}

/// Encodes a `Module` into Rust code.
fn encode(buf: &mut String, module: &Module) {
    buf.push_str("Module(&[");
    for (name, entry) in &module.0 {
        write!(buf, "({name:?}, Binding {{ def: ").unwrap();
        match &entry.def {
            Def::Module(module) => {
                buf.push_str("Def::Module(");
                encode(buf, module);
                buf.push(')');
            }
            Def::Symbol(symbol) => {
                buf.push_str("Def::Symbol(Symbol::");
                match symbol {
                    Symbol::Single(value) => write!(buf, "Single({value:?})").unwrap(),
                    Symbol::Multi { variants, deprecations } => write!(
                        buf,
                        "Multi {{ variants: &{variants:?}, deprecations: &{deprecations:?} }}"
                    )
                    .unwrap(),
                }
                buf.push(')');
            }
        }
        write!(buf, ", deprecation: {:?} }}),", entry.deprecation).unwrap();
    }
    buf.push_str("])");
}
