use std::fmt::Write;
use std::iter::Peekable;
use std::path::Path;

use self::shared::ModifierSet;

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

/// A symbol, either a leaf or with modifiers.
enum Symbol<'a> {
    Single(char),
    Multi(Vec<(ModifierSet<&'a str>, char)>),
    MultiAlias(Vec<(ModifierSet<String>, char)>),
}

/// A single line during parsing.
#[derive(Debug, Copy, Clone)]
enum Line<'a> {
    Blank,
    Deprecated(&'a str),
    ModuleStart(&'a str),
    ModuleEnd,
    Symbol(&'a str, Option<char>),
    Variant(ModifierSet<&'a str>, char),
    Alias(&'a str, &'a str, ModifierSet<&'a str>, bool),
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let mut buf = String::new();
    process(&mut buf, Path::new("src/modules/sym.txt"), "SYM", "Named general symbols.");
    process(&mut buf, Path::new("src/modules/emoji.txt"), "EMOJI", "Named emoji.");

    let out = std::env::var_os("OUT_DIR").unwrap();
    let dest = Path::new(&out).join("out.rs");
    std::fs::write(&dest, buf).unwrap();
}

/// Processes a single file and turns it into a global module.
fn process(buf: &mut String, file: &Path, name: &str, desc: &str) {
    println!("cargo::rerun-if-changed={}", file.display());

    let text = std::fs::read_to_string(file).unwrap();
    let mut line_nr = 0;
    let mut iter = text
        .lines()
        .inspect(|_| line_nr += 1)
        .map(tokenize)
        .filter(|line| !matches!(line, Ok(Line::Blank)))
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
fn tokenize(line: &str) -> StrResult<Line> {
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

    Ok(if head == "@deprecated:" {
        Line::Deprecated(tail.ok_or("missing deprecation message")?.trim())
    } else if tail == Some("{") {
        validate_ident(head)?;
        Line::ModuleStart(head)
    } else if head == "}" && tail.is_none() {
        Line::ModuleEnd
    } else if let Some(rest) = head.strip_prefix('.') {
        for part in rest.split('.') {
            validate_ident(part)?;
        }
        let c = decode_char(tail.ok_or("missing char")?)?;
        Line::Variant(ModifierSet::from_raw_dotted(rest), c)
    } else if let Some(mut value) = tail.and_then(|tail| tail.strip_prefix("@= ")) {
        let alias = head;
        validate_ident(alias)?;
        let mut deep = false;
        if let Some(v) = value.strip_suffix(".*") {
            deep = true;
            value = v;
        }
        let (head, rest) = value.split_once('.').unwrap_or((value, ""));
        validate_ident(head)?;
        if !rest.is_empty() {
            for part in rest.split('.') {
                validate_ident(part)?;
            }
        }

        Line::Alias(alias, head, ModifierSet::from_raw_dotted(rest), deep)
    } else {
        validate_ident(head)?;
        let c = tail.map(decode_char).transpose()?;
        Line::Symbol(head, c)
    })
}

/// Ensures that a string is a valid identifier. In `codex`, we use very strict
/// rules and allow only alphabetic ASCII chars.
fn validate_ident(string: &str) -> StrResult<()> {
    if !string.is_empty() && string.chars().all(|c| c.is_ascii_alphabetic()) {
        return Ok(());
    }
    Err(format!("invalid identifier: {string:?}"))
}

/// Extracts either a single char or parses a U+XXXX escape.
fn decode_char(text: &str) -> StrResult<char> {
    if let Some(hex) = text.strip_prefix("U+") {
        u32::from_str_radix(hex, 16)
            .ok()
            .and_then(|n| char::try_from(n).ok())
            .ok_or_else(|| format!("invalid unicode escape {text:?}"))
    } else {
        let mut chars = text.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(format!("expected exactly one char, found {text:?}")),
        }
    }
}

/// Turns a stream of lines into a list of definitions.
fn parse<'a>(
    p: &mut Peekable<impl Iterator<Item = StrResult<Line<'a>>>>,
) -> StrResult<Vec<(&'a str, Binding<'a>)>> {
    let mut defs = vec![];
    let mut deprecation = None;
    let mut aliases = vec![];
    loop {
        match p.next().transpose()? {
            None | Some(Line::ModuleEnd) => {
                if let Some(message) = deprecation {
                    return Err(format!("dangling `@deprecated: {}`", message));
                }
                break;
            }
            Some(Line::Deprecated(message)) => deprecation = Some(message),
            Some(Line::Alias(alias, name, variant, deep)) => {
                aliases.push((alias, name, variant, deep, deprecation));
                deprecation = None;
            }
            Some(Line::Symbol(name, c)) => {
                let mut variants = vec![];
                while let Some(Line::Variant(name, c)) = p.peek().cloned().transpose()? {
                    variants.push((name, c));
                    p.next();
                }

                let symbol = if !variants.is_empty() {
                    if let Some(c) = c {
                        variants.insert(0, (ModifierSet::default(), c));
                    }
                    Symbol::Multi(variants)
                } else {
                    let c = c.ok_or("symbol needs char or variants")?;
                    Symbol::Single(c)
                };

                defs.push((name, Binding { def: Def::Symbol(symbol), deprecation }));
                deprecation = None;
            }
            Some(Line::ModuleStart(name)) => {
                let module_defs = parse(p)?;
                defs.push((
                    name,
                    Binding {
                        def: Def::Module(Module::new(module_defs)),
                        deprecation,
                    },
                ));
                deprecation = None;
            }
            other => return Err(format!("expected definition, found {other:?}")),
        }
    }
    for (alias, name, variant, deep, deprecation) in aliases {
        let aliased_symbol: &Symbol<'a> = defs
            .iter()
            .filter(|(n, _)| *n == name)
            .find_map(|(_, b)| match &b.def {
                Def::Symbol(s) => Some(s),
                _ => None,
            })
            .ok_or_else(|| format!("alias to nonexistent symbol: {name}"))?;

        match aliased_symbol {
            &Symbol::Single(c) => {
                if !variant.is_empty() {
                    return Err(format!(
                        "alias to nonexistent variant: {name}.{}",
                        variant.as_str()
                    ));
                }
                defs.push((
                    alias,
                    Binding { def: Def::Symbol(Symbol::Single(c)), deprecation },
                ));
            }
            Symbol::MultiAlias(_) => {
                return Err(format!("alias to alias: {name}.{}", variant.as_str()));
            }
            Symbol::Multi(variants) => {
                let variants: Vec<(ModifierSet<String>, char)> = variants
                    .iter()
                    .filter_map(|&(var, c)| {
                        if !variant.is_subset(var) {
                            return None;
                        }

                        let new_var = var.iter().filter(|&v| !variant.contains(v)).fold(
                            ModifierSet::default(),
                            |mut set, m| {
                                set.insert_raw(m);
                                set
                            },
                        );

                        if !(deep || new_var.is_empty()) {
                            return None;
                        }

                        Some((new_var, c))
                    })
                    .collect();
                if variants.is_empty() {
                    return Err(format!(
                        "alias to nonexistent variant: {name}.{}",
                        variant.as_str()
                    ));
                }
                if let [(ref s, c)] = variants[..] {
                    if s.is_empty() {
                        defs.push((
                            alias,
                            Binding { def: Def::Symbol(Symbol::Single(c)), deprecation },
                        ));
                        continue;
                    }
                }
                defs.push((
                    alias,
                    Binding {
                        def: Def::Symbol(Symbol::MultiAlias(variants)),
                        deprecation,
                    },
                ));
            }
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
                    Symbol::Single(c) => write!(buf, "Single({c:?})").unwrap(),
                    Symbol::Multi(list) => write!(buf, "Multi(&{list:?})").unwrap(),
                    Symbol::MultiAlias(list) => write!(buf, "Multi(&{list:?})").unwrap(),
                }
                buf.push(')');
            }
        }
        write!(buf, ", deprecation: {:?} }}),", entry.deprecation).unwrap();
    }
    buf.push_str("])");
}
