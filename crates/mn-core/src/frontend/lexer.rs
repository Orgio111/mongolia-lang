use super::{error, Error, Span};
use std::borrow::Cow;
use unicode_normalization::{is_nfc_quick, IsNormalized, UnicodeNormalization};
use unicode_script::{Script, UnicodeScript};
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Word(String),
    Number(String),
    Str(String),
    Symbol(char),
    Arrow,
    Eof,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
fn canonical(s: &str) -> &str {
    match s {
        "функц" => "fn",
        "хувьсагч" => "let",
        "өөрчлөгдөх" => "mut",
        "тогтмол" => "const",
        "хэрэв" => "if",
        "үгүйбол" => "else",
        "давтах" => "loop",
        "зуур" => "while",
        "бүр" => "for",
        "дотор" => "in",
        "буцаах" => "return",
        "зогсох" => "break",
        "үргэлжлэх" => "continue",
        "бүтэц" => "struct",
        "тоочилт" => "enum",
        "шинж" => "trait",
        "хэрэгжүүлэх" => "impl",
        "импорт" => "import",
        "нийтийн" => "pub",
        "үнэн" => "true",
        "худал" => "false",
        "хэвлэх" => "print",
        "Тензор" => "Tensor",
        "уламжлал" => "grad",
        "бодит32" => "f32",
        "бодит64" => "f64",
        "бүхэл32" => "i32",
        "бүхэл64" => "i64",
        "логик" => "bool",
        "мөр" => "string",
        _ => s,
    }
}
pub(super) fn reserved(s: &str) -> bool {
    matches!(
        s,
        "fn" | "let"
            | "mut"
            | "const"
            | "if"
            | "else"
            | "loop"
            | "while"
            | "for"
            | "in"
            | "return"
            | "break"
            | "continue"
            | "struct"
            | "enum"
            | "trait"
            | "impl"
            | "import"
            | "pub"
            | "true"
            | "false"
            | "print"
            | "Tensor"
            | "grad"
            | "f32"
            | "f64"
            | "i32"
            | "i64"
            | "bool"
            | "string"
    )
}
fn mixes_scripts(name: &str) -> bool {
    let (mut latin, mut cyrillic) = (false, false);
    for c in name.chars() {
        if c.is_ascii_alphabetic() {
            latin = true;
        } else if ('\u{0400}'..='\u{052f}').contains(&c) {
            cyrillic = true;
        } else {
            match c.script() {
                Script::Latin => latin = true,
                Script::Cyrillic => cyrillic = true,
                _ => {}
            }
        }
        if latin && cyrillic {
            return true;
        }
    }
    false
}

pub fn lex(source: &str) -> Result<Vec<Token>, Error> {
    if source.len() > super::MAX_SOURCE_BYTES {
        return Err(error(Span::default(), "E_LIMIT", "source exceeds 1 MiB"));
    }
    let mut out = Vec::new();
    let mut i = 0;
    let mut skeletons = std::collections::HashMap::<String, String>::new();
    while i < source.len() {
        let start = i;
        let c = source[i..].chars().next().ok_or_else(|| {
            error(
                Span { start: i, end: i },
                "E_UTF8",
                "invalid character boundary",
            )
        })?;
        if c.is_whitespace() {
            i += c.len_utf8();
            continue;
        }
        if source[i..].starts_with("//") {
            while i < source.len() && source.as_bytes()[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if source[i..].starts_with("/*") {
            i += 2;
            let mut nesting = 1;
            while i < source.len() && nesting > 0 {
                if source[i..].starts_with("/*") {
                    nesting += 1;
                    i += 2;
                } else if source[i..].starts_with("*/") {
                    nesting -= 1;
                    i += 2;
                } else {
                    i += source[i..].chars().next().map_or(0, char::len_utf8);
                }
            }
            if nesting != 0 {
                return Err(error(
                    Span { start, end: i },
                    "E_LEX",
                    "unterminated block comment",
                ));
            }
            continue;
        }
        let kind = if c == '_' || unicode_ident::is_xid_start(c) {
            i += c.len_utf8();
            while let Some(ch) = source[i..].chars().next() {
                if ch == '_' || unicode_ident::is_xid_continue(ch) {
                    i += ch.len_utf8();
                } else {
                    break;
                }
            }
            let spelling = &source[start..i];
            let normalized: Cow<'_, str> = if is_nfc_quick(spelling.chars()) == IsNormalized::Yes {
                Cow::Borrowed(spelling)
            } else {
                Cow::Owned(spelling.nfc().collect())
            };
            // Reserved aliases are already single-script. Other identifiers must
            // retain the old policy even for marks in the legacy Cyrillic block.
            let word = canonical(&normalized);
            if !reserved(word) && !normalized.is_ascii() && mixes_scripts(&normalized) {
                return Err(error(
                    Span { start, end: i },
                    "E_SCRIPT",
                    "Latin and Cyrillic characters cannot mix within one identifier",
                ));
            }
            if !reserved(word) {
                let skeleton: String = normalized
                    .chars()
                    .map(|c| match c {
                        'А' => 'A',
                        'В' => 'B',
                        'Е' => 'E',
                        'К' => 'K',
                        'М' => 'M',
                        'Н' => 'H',
                        'О' => 'O',
                        'Р' => 'P',
                        'С' => 'C',
                        'Т' => 'T',
                        'Х' => 'X',
                        'а' => 'a',
                        'е' => 'e',
                        'о' => 'o',
                        'р' => 'p',
                        'с' => 'c',
                        'у' => 'y',
                        'х' => 'x',
                        _ => c,
                    })
                    .collect();
                if let Some(previous) = skeletons.get(&skeleton) {
                    if previous != normalized.as_ref() {
                        return Err(error(
                            Span { start, end: i },
                            "E_CONFUSABLE",
                            format!("confusable identifiers {previous} and {normalized}"),
                        ));
                    }
                } else {
                    skeletons.insert(skeleton, normalized.to_string());
                }
            }
            TokenKind::Word(word.to_owned())
        } else if c.is_ascii_digit() {
            i += 1;
            while i < source.len() && source.as_bytes()[i].is_ascii_digit() {
                i += 1;
            }
            if i < source.len() && source.as_bytes()[i] == b'.' {
                i += 1;
                let digits = i;
                while i < source.len() && source.as_bytes()[i].is_ascii_digit() {
                    i += 1;
                }
                if digits == i {
                    return Err(error(
                        Span { start, end: i },
                        "E_NUMBER",
                        "expected digits after decimal point",
                    ));
                }
            }
            TokenKind::Number(source[start..i].to_owned())
        } else if c == '"' {
            i += 1;
            let mut value = String::new();
            let mut closed = false;
            while i < source.len() {
                let ch = source[i..]
                    .chars()
                    .next()
                    .ok_or_else(|| error(Span { start, end: i }, "E_STRING", "invalid string"))?;
                i += ch.len_utf8();
                if ch == '"' {
                    closed = true;
                    break;
                }
                if ch == '\\' {
                    let esc = source[i..].chars().next().ok_or_else(|| {
                        error(Span { start, end: i }, "E_STRING", "unterminated escape")
                    })?;
                    i += esc.len_utf8();
                    value.push(match esc {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        _ => {
                            return Err(error(
                                Span { start, end: i },
                                "E_STRING",
                                "unsupported escape",
                            ))
                        }
                    });
                } else if ch == '\n' || ch == '\r' {
                    return Err(error(
                        Span { start, end: i },
                        "E_STRING",
                        "newline in string",
                    ));
                } else {
                    value.push(ch);
                }
            }
            if !closed {
                return Err(error(
                    Span { start, end: i },
                    "E_STRING",
                    "unterminated string",
                ));
            }
            TokenKind::Str(value)
        } else if source[i..].starts_with("->") {
            i += 2;
            TokenKind::Arrow
        } else if "(){}[],:;=+-*/@<>".contains(c) {
            i += c.len_utf8();
            TokenKind::Symbol(c)
        } else {
            return Err(error(
                Span {
                    start,
                    end: start + c.len_utf8(),
                },
                "E_LEX",
                format!("unexpected character {c:?}"),
            ));
        };
        out.push(Token {
            kind,
            span: Span { start, end: i },
        });
        if out.len() > 100_000 {
            return Err(error(
                Span { start, end: i },
                "E_LIMIT",
                "token limit exceeded",
            ));
        }
    }
    out.push(Token {
        kind: TokenKind::Eof,
        span: Span { start: i, end: i },
    });
    Ok(out)
}
