#![forbid(unsafe_code)]
use mn_core::{engine, frontend};
use std::{
    env,
    ffi::OsString,
    fs,
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

// Keep one displayed character per source scalar in a diagnostic excerpt.
fn display_text(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\t' => ' ',
            '\u{2028}'
            | '\u{2029}'
            | '\u{061c}'
            | '\u{200e}'
            | '\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2066}'..='\u{2069}' => '�',
            c if c.is_control() => '�',
            c => c,
        })
        .collect()
}

fn diagnostic(source: &str, error: &frontend::Error, lang: &str) -> String {
    let start = error.span.start.min(source.len());
    let start = (0..=start)
        .rev()
        .find(|&i| source.is_char_boundary(i))
        .unwrap_or(0);
    let prefix = &source[..start];
    let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
    let column = prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1;
    let row = source[start..].split('\n').next().unwrap_or("");
    let begin = prefix.rsplit('\n').next().unwrap_or("");
    let title = if lang == "mn" { "Алдаа" } else { "Error" };
    let explanation = if lang == "mn" {
        "Эх кодыг шалгана уу"
    } else {
        "Check the source expression"
    };
    let width = source
        .get(start..error.span.end.min(source.len()))
        .unwrap_or("")
        .chars()
        .take_while(|c| *c != '\n')
        .count()
        .clamp(1, 80);
    // Keep the real source column in the heading, but bound display work
    // and caret indentation for long physical lines.
    let begin_start = if column > 161 {
        begin.char_indices().rev().nth(79).map_or(0, |(i, _)| i)
    } else {
        0
    };
    let left = if begin_start > 0 { "…" } else { "" };
    let begin = &begin[begin_start..];
    let end = row.char_indices().nth(160).map_or(row.len(), |(i, _)| i);
    let right = if end < row.len() { "…" } else { "" };
    let detail = mn_core::diagnostics::render_detail(error, lang);
    let detail_end = detail
        .char_indices()
        .nth(256)
        .map_or(detail.len(), |(i, _)| i);
    let detail_suffix = if detail_end < detail.len() { "…" } else { "" };
    format!(
        "{title}[{}] {line}:{column}: {}{detail_suffix}\n{left}{}{}{right}\n{}{}\n{explanation}",
        error.code,
        display_text(&detail[..detail_end]),
        display_text(begin),
        display_text(&row[..end]),
        " ".repeat(begin.chars().count() + left.chars().count()),
        "^".repeat(width)
    )
}
mod scaffold;

fn execute(args: Vec<OsString>) -> Result<(), (u8, String)> {
    let mut args = args;
    let mut lang = "en".to_owned();
    if let Some(i) = args.iter().position(|a| a == "--lang") {
        if i + 1 >= args.len() {
            return Err((2, "--lang requires en or mn".into()));
        }
        lang = args
            .remove(i + 1)
            .into_string()
            .map_err(|_| (2, "--lang requires en or mn".to_owned()))?;
        args.remove(i);
        if lang != "en" && lang != "mn" {
            return Err((2, "--lang requires en or mn".into()));
        }
    }
    let usage = concat!("Mongolia ", env!("CARGO_PKG_VERSION"), "\nmn run FILE [--lang en|mn]\nmn check FILE [--lang en|mn]\nmn init [DIRECTORY]\nmn --help\nmn --version\nOther commands are not implemented.");
    let command = args
        .first()
        .map_or(Some("--help"), |arg| arg.to_str())
        .ok_or_else(|| (2, usage.to_owned()))?;
    match command {
        "--help" | "--version" if args.len() <= 1 => {
            writeln!(
                std::io::stdout(),
                "{}",
                if command == "--help" {
                    usage
                } else {
                    concat!("mn ", env!("CARGO_PKG_VERSION"))
                }
            )
            .map_err(|e| (2, e.to_string()))?;
            Ok(())
        }
        "init" if args.len() <= 2 => {
            let dir = args.get(1).map_or(Path::new("."), Path::new);
            if dir.as_os_str().as_encoded_bytes().starts_with(b"-") {
                return Err((
                    2,
                    "Unknown init option. Prefix a relative directory beginning with '-' with ./"
                        .into(),
                ));
            }
            fs::create_dir_all(dir).map_err(|e| (2, e.to_string()))?;
            let manifest = dir.join("mn.toml");
            let source = dir.join("main.mn");
            for path in [&manifest, &source] {
                match path.symlink_metadata() {
                    Ok(_) => {
                        return Err((2, "init refuses to overwrite mn.toml or main.mn".into()))
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(e) => return Err((2, e.to_string())),
                }
            }
            scaffold::create(&manifest, &source, scaffold::MANIFEST, scaffold::SOURCE)
                .map_err(|e| (2, e.to_string()))?;
            writeln!(std::io::stdout(), "Created mn.toml and main.mn")
                .map_err(|e| (2, e.to_string()))?;
            Ok(())
        }
        "run" | "check" if args.len() == 2 => {
            let mut bytes = Vec::new();
            fs::File::open(&args[1])
                .map_err(|e| (2, e.to_string()))?
                .take((frontend::MAX_SOURCE_BYTES + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|e| (2, e.to_string()))?;
            if bytes.len() > frontend::MAX_SOURCE_BYTES {
                let detail = if lang == "mn" {
                    "Эх кодын хэмжээ 1 МиБ-аас хэтэрлээ"
                } else {
                    "source exceeds 1 MiB limit"
                };
                return Err((1, format!("E_LIMIT: {detail}")));
            }
            let source = String::from_utf8(bytes).map_err(|e| {
                let offset = e.utf8_error().valid_up_to();
                let message = if lang == "mn" {
                    format!("E_UTF8: {offset}-р байтаас эхлэн UTF-8 кодчилол буруу байна")
                } else {
                    format!("E_UTF8: invalid UTF-8 at byte {offset}")
                };
                (1, message)
            })?;
            let p = frontend::parse(&source).map_err(|e| (1, diagnostic(&source, &e, &lang)))?;
            let checked = engine::validate(&p).map_err(|e| (1, diagnostic(&source, &e, &lang)))?;
            if command == "run" {
                let out = checked
                    .run()
                    .map_err(|e| (1, diagnostic(&source, &e, &lang)))?;
                std::io::stdout()
                    .write_all(out.as_bytes())
                    .map_err(|e| (2, e.to_string()))?;
            }
            Ok(())
        }
        _ => Err((
            2,
            format!("Unsupported command or invalid arguments.\n{usage}"),
        )),
    }
}
fn main() -> ExitCode {
    match execute(env::args_os().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            let _ = writeln!(std::io::stderr(), "{message}");
            ExitCode::from(code)
        }
    }
}
