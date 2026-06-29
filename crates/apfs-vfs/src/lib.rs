#![forbid(unsafe_code)]

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VfsNodeKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VfsMetadata {
    pub inode: u64,
    pub kind: VfsNodeKind,
    pub logical_size: u64,
    pub physical_size: Option<u64>,
    pub readonly: bool,
    pub created_time_unix_nanos: Option<i128>,
    pub modified_time_unix_nanos: Option<i128>,
    pub changed_time_unix_nanos: Option<i128>,
    pub accessed_time_unix_nanos: Option<i128>,
    pub mode: Option<u16>,
    pub owner_uid: Option<u32>,
    pub owner_gid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VfsEntry {
    pub name: String,
    pub metadata: VfsMetadata,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VfsReadResult {
    pub offset: u64,
    pub bytes_read: usize,
    pub eof: bool,
    #[serde(skip_serializing)]
    pub bytes: Vec<u8>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VfsError {
    #[error("path not found: {0}")]
    NotFound(String),
    #[error("operation is read-only: {0}")]
    ReadOnly(String),
    #[error("unsupported APFS VFS operation: {0}")]
    Unsupported(String),
    #[error("invalid path: {0}")]
    InvalidPath(String),
    #[error("backend error: {0}")]
    Backend(String),
}

pub trait ReadOnlyVfs {
    fn root_metadata(&self) -> Result<VfsMetadata, VfsError>;
    fn lookup(&self, path: &str) -> Result<VfsMetadata, VfsError>;
    fn read_dir(&self, path: &str) -> Result<Vec<VfsEntry>, VfsError>;
    fn read_file(&self, path: &str, offset: u64, length: usize) -> Result<VfsReadResult, VfsError>;

    fn write_file(&self, path: &str, _offset: u64, _bytes: &[u8]) -> Result<(), VfsError> {
        Err(VfsError::ReadOnly(path.to_owned()))
    }

    fn create_file(&self, path: &str) -> Result<(), VfsError> {
        Err(VfsError::ReadOnly(path.to_owned()))
    }

    fn remove_file(&self, path: &str) -> Result<(), VfsError> {
        Err(VfsError::ReadOnly(path.to_owned()))
    }

    fn rename(&self, from: &str, _to: &str) -> Result<(), VfsError> {
        Err(VfsError::ReadOnly(from.to_owned()))
    }
}

pub fn reject_path_traversal(path: &str) -> Result<(), VfsError> {
    if path.contains('\0') {
        return Err(VfsError::InvalidPath("NUL byte is not allowed".to_owned()));
    }
    let normalized = path.replace('\\', "/");
    if normalized.starts_with('/') || normalized.split('/').any(|part| part == "..") {
        return Err(VfsError::InvalidPath(format!("path escapes read-only APFS namespace: {path}")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_traversal_guard_rejects_parent_escape() {
        assert!(reject_path_traversal("../secret").is_err());
        assert!(reject_path_traversal("safe/../secret").is_err());
        assert!(reject_path_traversal("/absolute").is_err());
        assert!(reject_path_traversal("safe/path").is_ok());
    }
}
