//! Exclusive creation in a trusted project directory. Roll back ordinary I/O
//! failures after closing the handles, including partial writes on Windows.
use std::{
    fs,
    io::{self, Read},
    path::Path,
};

pub(super) const MANIFEST: &[u8] = b"[package]\nname = \"hello\"\nversion = \"0.1.0\"\n# Manifest is scaffolding only; no package resolver in v0.1.\n";
pub(super) const SOURCE: &[u8] = b"fn main() { print(\"Hello, Mongolia!\"); }\n";

pub(super) fn create(
    manifest: &Path,
    source: &Path,
    mut manifest_data: impl Read,
    mut source_data: impl Read,
) -> io::Result<()> {
    let mut owned = Vec::with_capacity(2);
    let result = (|| {
        for (path, data) in [
            (manifest, &mut manifest_data as &mut dyn Read),
            (source, &mut source_data),
        ] {
            let mut file = fs::File::create_new(path)?;
            owned.push(path);
            io::copy(data, &mut file)?;
            file.sync_all()?;
        }
        Ok(())
    })();
    if let Err(cause) = result {
        let mut cleanup_failures = Vec::new();
        for path in owned.into_iter().rev() {
            if let Err(e) = fs::remove_file(path) {
                cleanup_failures.push(format!("{} ({e})", path.display()));
            }
        }
        if !cleanup_failures.is_empty() {
            return Err(io::Error::other(format!(
                "{cause}. Could not remove incomplete scaffold: {}",
                cleanup_failures.join(", ")
            )));
        }
        return Err(cause);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fail;
    impl Read for Fail {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("injected failure"))
        }
    }

    #[test]
    fn init_rolls_back_only_files_it_created_and_allows_retry() {
        let dir = std::env::temp_dir().join(format!("mn-phase9-rollback-{}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        let m = dir.join("mn.toml");
        let s = dir.join("main.mn");
        assert!(create(&m, &s, Fail, SOURCE).is_err());
        assert!(!m.exists() && !s.exists());
        assert!(create(&m, &s, MANIFEST, (&b"partial"[..]).chain(Fail)).is_err());
        assert!(!m.exists() && !s.exists());
        fs::write(&s, "existing").unwrap();
        assert!(create(&m, &s, MANIFEST, SOURCE).is_err());
        assert!(!m.exists());
        assert_eq!(fs::read(&s).unwrap(), b"existing");
        fs::remove_file(&s).unwrap();
        create(&m, &s, MANIFEST, SOURCE).unwrap();
        assert_eq!(fs::read(&s).unwrap(), SOURCE);
        fs::remove_dir_all(dir).unwrap();
    }
}
