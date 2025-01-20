use std::fmt::Write;
use std::iter::Peekable;
use std::path::Path;

type StrResult<T> = Result<T, String>;

/// A module of definitions.
struct Module<'a>(Vec<(&'a str, Def<'a>)>);

impl<'a> Module<'a> {
    fn new(mut list: Vec<(&'a str, Def<'a>)>) -> Self {
        list.sort_by_key(|&(name, _)| name);
        Self(list)
    }
}

/// A definition in a module.
enum Def<'a> {
    Symbol(Symbol<'a>),
    Module(Module<'a>),
}

/// A symbol, either a leaf or with modifiers.
enum Symbol<'a> {
    Single(char),
    Multi(Vec<(&'a str, char)>),
}

/// A single line during parsing.
#[derive(Debug, Copy, Clone)]
enum Line<'a> {
    Blank,
    ModuleStart(&'a str),
    ModuleEnd,
    Symbol(&'a str, Option<char>),
    Variant(&'a str, char),
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
    buf.push_str(";");
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

    let (head, tail) = match line.split_once(char::is_whitespace) {
        Some((a, b)) => (a, Some(b.trim_start())),
        None => (line, None),
    };

    Ok(if tail == Some("{") {
        validate_ident(head)?;
        Line::ModuleStart(head)
    } else if head == "}" && tail.is_none() {
        Line::ModuleEnd
    } else if let Some(rest) = head.strip_prefix('.') {
        for part in rest.split('.') {
            validate_ident(part)?;
        }
        let c = decode_char(tail.ok_or("missing char")?)?;
        Line::Variant(rest, c)
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
        let (hex, name) = match hex.split_once(char::is_whitespace) {
            Some((hex, name)) => (hex, Some(name.trim_start())),
            None => (hex, None),
        };

        let ch = u32::from_str_radix(hex, 16)
            .ok()
            .and_then(|n| char::from_u32(n))
            .ok_or_else(|| format!("invalid unicode escape {hex:?}"))?;

        #[cfg_attr(not(feature = "unicode_names2"), expect(unused_variables))]
        if let Some(name) = name {
            #[cfg(feature = "unicode_names2")]
            if unicode_names2::character(name) != Some(ch) {
                return Err(format!(
                    "Incorrect name supplied for character U+{hex}: '{name}'{}",
                    unicode_names2::name(ch)
                        .map_or("".to_string(), |name| format!(" (expected {name})"))
                ));
            }
        }

        Ok(ch)
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
) -> StrResult<Vec<(&'a str, Def<'a>)>> {
    let mut defs = vec![];
    loop {
        match p.next().transpose()? {
            None | Some(Line::ModuleEnd) => break,
            Some(Line::Symbol(name, c)) => {
                let mut variants = vec![];
                while let Some(Line::Variant(name, c)) = p.peek().cloned().transpose()? {
                    variants.push((name, c));
                    p.next();
                }

                println!("{c:?}, {variants:?}, {:?}", p.peek());
                let symbol = if variants.len() > 0 {
                    if let Some(c) = c {
                        variants.insert(0, ("", c));
                    }
                    Symbol::Multi(variants)
                } else {
                    let c = c.ok_or("symbol needs char or variants")?;
                    Symbol::Single(c)
                };

                defs.push((name, Def::Symbol(symbol)));
            }
            Some(Line::ModuleStart(name)) => {
                let module_defs = parse(p)?;
                defs.push((name, Def::Module(Module::new(module_defs))));
            }
            other => return Err(format!("expected definition, found {other:?}")),
        }
    }
    Ok(defs)
}

/// Encodes a `Module` into Rust code.
fn encode(buf: &mut String, module: &Module) {
    buf.push_str("Module(&[");
    for (name, def) in &module.0 {
        write!(buf, "({name:?},").unwrap();
        match def {
            Def::Module(module) => {
                buf.push_str("Def::Module(");
                encode(buf, module);
                buf.push_str(")");
            }
            Def::Symbol(symbol) => {
                buf.push_str("Def::Symbol(Symbol::");
                match symbol {
                    Symbol::Single(c) => write!(buf, "Single({c:?})").unwrap(),
                    Symbol::Multi(list) => write!(buf, "Multi(&{list:?})").unwrap(),
                }
                buf.push_str(")");
            }
        }
        buf.push_str("),");
    }
    buf.push_str("])");
}
