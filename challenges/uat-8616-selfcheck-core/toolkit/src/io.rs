//! Filesystem abstraction layer.
//!
//! Provides the `Fs` trait so that all filesystem operations can be swapped
//! between a real OS implementation and an in-memory mock for testing.
//! This is Assessment Recommendation #1 — parallel to the `Sys` trait for
//! command execution, enabling deterministic test harnesses without touching
//! disk or requiring root/device access.

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// ─── Filesystem metadata ────────────────────────────────────────────────────

/// Minimal stat-equivalent for forensic purposes.
#[derive(Debug, Clone)]
pub struct FsMeta {
    pub size_bytes: u64,
    pub modified: Option<u64>,
    pub is_file: bool,
    pub is_dir: bool,
}

/// Directory entry returned by `read_dir`.
#[derive(Debug, Clone)]
pub struct FsEntry {
    pub path: PathBuf,
    pub meta: FsMeta,
}

// ─── Fs trait ────────────────────────────────────────────────────────────────

/// Abstraction over filesystem operations.
///
/// Every file read, stat, hash, or directory walk in the forensic pipeline
/// goes through this trait. The live implementation delegates to `std::fs`;
/// the mock implementation operates against an in-memory tree populated
/// from a known-clean image snapshot.
pub trait Fs: Send + Sync {
    /// Check whether the path exists.
    fn exists(&self, path: &Path) -> bool;

    /// Read file contents as bytes.
    fn read(&self, path: &Path) -> Result<Vec<u8>>;

    /// Read file contents as a UTF-8 string.
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// Stat a path (size, modified, type).
    fn metadata(&self, path: &Path) -> Result<FsMeta>;

    /// List entries in a directory (non-recursive).
    fn read_dir(&self, path: &Path) -> Result<Vec<FsEntry>>;

    /// Read exactly `n` bytes from a path (or fewer if file is smaller).
    /// Used for ambient padding reads.
    fn read_bytes(&self, path: &Path, n: usize) -> Result<Vec<u8>> {
        let data = self.read(path)?;
        Ok(data.into_iter().take(n).collect())
    }
}

// ─── Live implementation ─────────────────────────────────────────────────────

/// Real filesystem — delegates to `std::fs`.
pub struct LiveFs;

impl Fs for LiveFs {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        Ok(std::fs::read(path)?)
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    fn metadata(&self, path: &Path) -> Result<FsMeta> {
        let md = std::fs::metadata(path)?;
        Ok(FsMeta {
            size_bytes: md.len(),
            modified: md.modified().ok().and_then(|t| {
                t.duration_since(SystemTime::UNIX_EPOCH).ok().map(|d| d.as_secs())
            }),
            is_file: md.is_file(),
            is_dir: md.is_dir(),
        })
    }

    fn read_dir(&self, path: &Path) -> Result<Vec<FsEntry>> {
        let mut entries = Vec::new();
        for e in std::fs::read_dir(path)? {
            let e = e?;
            let ft = e.file_type()?;
            let md = e.metadata()?;
            entries.push(FsEntry {
                path: e.path(),
                meta: FsMeta {
                    size_bytes: md.len(),
                    modified: md.modified().ok().and_then(|t| {
                        t.duration_since(SystemTime::UNIX_EPOCH).ok().map(|d| d.as_secs())
                    }),
                    is_file: ft.is_file(),
                    is_dir: ft.is_dir(),
                },
            });
        }
        Ok(entries)
    }
}

// ─── Mock implementation ─────────────────────────────────────────────────────

/// In-memory filesystem for deterministic testing.
/// Populate with `insert_file` / `insert_dir` before passing to the executor.
pub struct MockFs {
    files: std::collections::HashMap<PathBuf, Vec<u8>>,
    dirs: std::collections::HashSet<PathBuf>,
}

impl MockFs {
    pub fn new() -> Self {
        Self {
            files: std::collections::HashMap::new(),
            dirs: std::collections::HashSet::new(),
        }
    }

    pub fn insert_file(&mut self, path: impl Into<PathBuf>, content: impl Into<Vec<u8>>) {
        let p: PathBuf = path.into();
        if let Some(parent) = p.parent() {
            self.dirs.insert(parent.to_path_buf());
        }
        self.files.insert(p, content.into());
    }

    pub fn insert_dir(&mut self, path: impl Into<PathBuf>) {
        self.dirs.insert(path.into());
    }
}

impl Fs for MockFs {
    fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path) || self.dirs.contains(path)
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("mock: file not found: {}", path.display()))
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        let bytes = self.read(path)?;
        Ok(String::from_utf8(bytes)?)
    }

    fn metadata(&self, path: &Path) -> Result<FsMeta> {
        if let Some(data) = self.files.get(path) {
            Ok(FsMeta {
                size_bytes: data.len() as u64,
                modified: Some(0),
                is_file: true,
                is_dir: false,
            })
        } else if self.dirs.contains(path) {
            Ok(FsMeta {
                size_bytes: 0,
                modified: Some(0),
                is_file: false,
                is_dir: true,
            })
        } else {
            Err(anyhow::anyhow!("mock: path not found: {}", path.display()))
        }
    }

    fn read_dir(&self, path: &Path) -> Result<Vec<FsEntry>> {
        let mut entries = Vec::new();
        let prefix = path.to_path_buf();
        // Files directly under this directory
        for (p, data) in &self.files {
            if p.parent() == Some(&prefix) {
                entries.push(FsEntry {
                    path: p.clone(),
                    meta: FsMeta {
                        size_bytes: data.len() as u64,
                        modified: Some(0),
                        is_file: true,
                        is_dir: false,
                    },
                });
            }
        }
        // Subdirectories
        for d in &self.dirs {
            if d.parent() == Some(&prefix) && d != &prefix {
                entries.push(FsEntry {
                    path: d.clone(),
                    meta: FsMeta {
                        size_bytes: 0,
                        modified: Some(0),
                        is_file: false,
                        is_dir: true,
                    },
                });
            }
        }
        Ok(entries)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_fs_roundtrip() {
        let mut fs = MockFs::new();
        fs.insert_file("/tmp/test.txt", b"hello world".to_vec());
        assert!(fs.exists(Path::new("/tmp/test.txt")));
        assert_eq!(fs.read_to_string(Path::new("/tmp/test.txt")).unwrap(), "hello world");
        let md = fs.metadata(Path::new("/tmp/test.txt")).unwrap();
        assert_eq!(md.size_bytes, 11);
        assert!(md.is_file);
    }

    #[test]
    fn mock_fs_read_dir() {
        let mut fs = MockFs::new();
        fs.insert_file("/data/a.txt", b"aaa".to_vec());
        fs.insert_file("/data/b.txt", b"bbb".to_vec());
        fs.insert_dir(PathBuf::from("/data/sub"));
        let entries = fs.read_dir(Path::new("/data")).unwrap();
        assert!(entries.len() >= 2);
    }

    #[test]
    fn mock_fs_missing_returns_error() {
        let fs = MockFs::new();
        assert!(!fs.exists(Path::new("/nope")));
        assert!(fs.read(Path::new("/nope")).is_err());
    }
}
