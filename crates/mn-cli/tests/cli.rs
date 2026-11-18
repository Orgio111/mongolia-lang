use std::{
    fs,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn invoke(bytes: &[u8], command: &str, lang: &str) -> Output {
    let path = std::env::temp_dir().join(format!(
        "mn-{}-{}.mn",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::write(&path, bytes).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_mn"))
        .args([command, path.to_str().unwrap(), "--lang", lang])
        .output()
        .unwrap();
    fs::remove_file(path).unwrap();
    out
}
#[test]
fn acceptance_a_b_c_e() {
    for (source,expected) in [
        ("fn main(){let A=[[1.0,2.0],[3.0,4.0]];let B=[[5.0,6.0],[7.0,8.0]];print(A@B);}","[[19.0, 22.0], [43.0, 50.0]]\n"),
        ("функц main(){хувьсагч А=[[1.0,2.0],[3.0,4.0]];хувьсагч Б=[[5.0,6.0],[7.0,8.0]];хэвлэх(А@Б);}","[[19.0, 22.0], [43.0, 50.0]]\n"),
        ("функц main(){let a=10;хувьсагч b=20;print(a+b);}","30\n"),
        ("fn add(a:i64,b:i64)->i64{return a+b;} функц main(){хэвлэх(add(12,30));}","42\n")
    ] {let out=invoke(source.as_bytes(),"run","en");assert!(out.status.success(),"{:?}",out);assert_eq!(String::from_utf8(out.stdout).unwrap(),expected);}
}
#[test]
fn shape_error_prevents_output() {
    let s=b"fn main(){print(99);let A=[[1.0,2.0,3.0],[4.0,5.0,6.0]];let B=[[1.0,2.0],[3.0,4.0]];print(A@B);}";
    let source = std::str::from_utf8(s).unwrap();
    let start = source.find("A@B").unwrap();
    let marker = format!("{}^^^", " ".repeat(start));
    for (lang, reason) in [
        ("en", "inner dimensions 3 and 2 are incompatible"),
        ("mn", "дотоод хэмжээсүүд 3 ба 2 хоорондоо тохирохгүй"),
    ] {
        for command in ["check", "run"] {
            let out = invoke(s, command, lang);
            assert_eq!(out.status.code(), Some(1));
            assert!(out.stdout.is_empty());
            let diagnostic = String::from_utf8(out.stderr).unwrap();
            assert!(diagnostic.contains("[2, 3] @ [2, 2]"));
            assert!(diagnostic.contains("E_SHAPE") && diagnostic.contains(reason));
            assert!(diagnostic.lines().any(|line| line == marker));
        }
    }
}
#[test]
fn invalid_utf8() {
    let o = invoke(&[255, 254], "check", "en");
    assert_eq!(o.status.code(), Some(1));
    assert!(String::from_utf8(o.stderr).unwrap().contains("E_UTF8"));
}
#[test]
fn validation_is_not_execution() {
    let o = invoke(b"fn main(){print(1/0);}", "check", "en");
    assert!(o.status.success());
    assert!(o.stdout.is_empty());
    let o = invoke(b"fn main(){print(1/0);}", "run", "en");
    assert_eq!(o.status.code(), Some(1));
}
#[test]
fn unsupported_command() {
    let o = Command::new(env!("CARGO_BIN_EXE_mn"))
        .args(["build", "--target", "cuda"])
        .output()
        .unwrap();
    assert_eq!(o.status.code(), Some(2));
    assert!(String::from_utf8(o.stderr).unwrap().contains("Unsupported"));
}
#[test]
fn init_preserves_existing_files() {
    let dir = std::env::temp_dir().join(format!("mn-init-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let first = Command::new(env!("CARGO_BIN_EXE_mn"))
        .arg("init")
        .arg(&dir)
        .output()
        .unwrap();
    assert!(first.status.success());
    let before = fs::read(dir.join("main.mn")).unwrap();
    let second = Command::new(env!("CARGO_BIN_EXE_mn"))
        .arg("init")
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(second.status.code(), Some(2));
    assert_eq!(fs::read(dir.join("main.mn")).unwrap(), before);
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn negative_programs() {
    for source in [
        "fn main(){print(x);}",
        "fn main(){let x=1;let x=2;}",
        "fn main(){let x=[[1.0],[2.0,3.0]];}",
        "fn main(){let x:Tensor<f64,[1,2]>=[[1.0]];}",
        "fn main(){print(1);",
        "fn f(x:i64){} fn main(){f();}",
        "fn main(){print(9223372036854775807+1);}",
    ] {
        let o = invoke(source.as_bytes(), "run", "en");
        assert_eq!(o.status.code(), Some(1), "{source}");
        assert!(o.stdout.is_empty());
    }
}

#[test]
fn input_errors_report_the_byte_offset_and_limit_in_mongolian() {
    let bad_encoding = invoke(&[b'f', b'n', 0xff], "check", "mn");
    assert_eq!(bad_encoding.status.code(), Some(1));
    assert!(bad_encoding.stdout.is_empty());
    assert!(String::from_utf8(bad_encoding.stderr)
        .unwrap()
        .contains("E_UTF8: 2-р байтаас"));
    let too_large = invoke(&vec![b' '; 1_048_577], "run", "mn");
    assert_eq!(too_large.status.code(), Some(1));
    assert!(too_large.stdout.is_empty());
    assert!(String::from_utf8(too_large.stderr)
        .unwrap()
        .contains("E_LIMIT: Эх кодын хэмжээ"));
}

#[cfg(unix)]
#[test]
fn utf8_program_loads_from_a_non_utf8_path() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};
    let mut name = format!("mn-path-{}-", std::process::id()).into_bytes();
    name.extend_from_slice(b"\xff.mn");
    let path = std::env::temp_dir().join(OsString::from_vec(name));
    fs::write(&path, "функц main(){хэвлэх(42);}").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_mn"))
        .arg("run")
        .arg(&path)
        .output()
        .unwrap();
    fs::remove_file(path).unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, b"42\n");
}

#[cfg(unix)]
#[test]
fn init_refuses_dangling_source_without_creating_manifest() {
    let dir = std::env::temp_dir().join(format!("mn-dangling-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let source = dir.join("main.mn");
    std::os::unix::fs::symlink("absent.mn", &source).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_mn"))
        .arg("init")
        .arg(&dir)
        .output()
        .unwrap();
    let partial_manifest = dir.join("mn.toml").exists();
    let preserved = fs::read_link(&source).unwrap() == std::path::Path::new("absent.mn");
    fs::remove_dir_all(dir).unwrap();
    assert_eq!(result.status.code(), Some(2));
    assert!(result.stdout.is_empty());
    assert!(preserved);
    assert!(!partial_manifest, "failed init left mn.toml behind");
}

#[test]
fn init_rejects_unknown_options_without_creating_a_directory() {
    let dir = std::env::temp_dir().join(format!("mn-init-options-{}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_mn"))
        .args(["init", "--unknown"])
        .current_dir(&dir)
        .output()
        .unwrap();
    let empty = fs::read_dir(&dir).unwrap().next().is_none();
    fs::remove_dir_all(dir).unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(empty);
}

#[test]
fn diagnostic_excerpt_keeps_control_characters_inert() {
    // A CR in a comment and a tab before the error each occupy one source column.
    let source = "fn main(){/*\r*/\tprint(үлмэдэх);}";
    for lang in ["en", "mn"] {
        let out = invoke(source.as_bytes(), "check", lang);
        assert_eq!(out.status.code(), Some(1));
        let message = String::from_utf8(out.stderr).unwrap();
        assert!(!message.contains(['\r', '\t']));
        let column = source.split_once("үлмэдэх").unwrap().0.chars().count();
        assert!(message
            .lines()
            .any(|l| l == format!("{}{}", " ".repeat(column), "^".repeat(7))));
    }
}

#[test]
fn long_unicode_diagnostics_keep_real_column_and_bound_the_excerpt() {
    let prefix = format!("fn main(){{/*{}*/print(", "ө".repeat(4_000));
    let source = format!("{prefix}missing);}}");
    let out = invoke(source.as_bytes(), "check", "mn");
    let msg = String::from_utf8(out.stderr).unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(msg.contains(&format!("1:{}:", prefix.chars().count() + 1)));
    assert!(msg.contains("missing") && msg.len() < 1500);
}

#[test]
fn diagnostic_unicode_line_separator_is_inert() {
    let out = invoke(
        "fn main(){/*\u{2028}\u{2029}*/print(missing);}".as_bytes(),
        "check",
        "en",
    );
    let msg = String::from_utf8(out.stderr).unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(!msg.contains(['\u{2028}', '\u{2029}']));
}

#[test]
fn partial_utf8_sequences_preserve_offset_and_exit_status() {
    for tail in [&[0xc2][..], &[0xe2, 0x82], &[0xf0, 0x9f, 0x98], &[0xff]] {
        let mut bytes = "функц main(){}".as_bytes().to_vec();
        let offset = bytes.len();
        bytes.extend_from_slice(tail);
        for lang in ["en", "mn"] {
            let out = invoke(&bytes, "run", lang);
            assert_eq!(out.status.code(), Some(1));
            assert!(out.stdout.is_empty());
            let msg = String::from_utf8(out.stderr).unwrap();
            assert!(msg.contains("E_UTF8") && msg.contains(&offset.to_string()));
        }
    }
}
