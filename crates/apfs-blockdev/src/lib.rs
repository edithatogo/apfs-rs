#![forbid(unsafe_code)]

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::Mutex,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockDeviceError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("read range is out of bounds: offset={offset}, len={len}, size={size}")]
    OutOfBounds { offset: u64, len: usize, size: u64 },
    #[error("read range overflow: offset={offset}, len={len}")]
    RangeOverflow { offset: u64, len: usize },
    #[error("internal file lock poisoned")]
    LockPoisoned,
}

pub trait ReadOnlyBlockDevice {
    fn size(&self) -> Result<u64, BlockDeviceError>;
    fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, BlockDeviceError>;

    fn read_prefix(&self, max_len: usize) -> Result<Vec<u8>, BlockDeviceError> {
        let size = self.size()?;
        let len = max_len.min(usize::try_from(size).unwrap_or(usize::MAX));
        self.read_at(0, len)
    }
}

#[derive(Debug)]
pub struct ImageBlockDevice {
    path: PathBuf,
    size: u64,
    file: Mutex<File>,
}

impl ImageBlockDevice {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, BlockDeviceError> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let size = file.metadata()?.len();
        Ok(Self { path, size, file: Mutex::new(file) })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl ReadOnlyBlockDevice for ImageBlockDevice {
    fn size(&self) -> Result<u64, BlockDeviceError> {
        Ok(self.size)
    }

    fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, BlockDeviceError> {
        let len_u64 = u64::try_from(len).map_err(|_| BlockDeviceError::RangeOverflow { offset, len })?;
        let end = offset.checked_add(len_u64).ok_or(BlockDeviceError::RangeOverflow { offset, len })?;
        if end > self.size {
            return Err(BlockDeviceError::OutOfBounds { offset, len, size: self.size });
        }

        let mut file = self.file.lock().map_err(|_| BlockDeviceError::LockPoisoned)?;
        file.seek(SeekFrom::Start(offset))?;
        let mut buf = vec![0_u8; len];
        file.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[derive(Debug, Clone)]
pub struct MemoryBlockDevice {
    bytes: Vec<u8>,
}

impl MemoryBlockDevice {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl ReadOnlyBlockDevice for MemoryBlockDevice {
    fn size(&self) -> Result<u64, BlockDeviceError> {
        u64::try_from(self.bytes.len()).map_err(|_| BlockDeviceError::RangeOverflow { offset: 0, len: self.bytes.len() })
    }

    fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, BlockDeviceError> {
        let len_u64 = u64::try_from(len).map_err(|_| BlockDeviceError::RangeOverflow { offset, len })?;
        let end = offset.checked_add(len_u64).ok_or(BlockDeviceError::RangeOverflow { offset, len })?;
        let size = self.size()?;
        if end > size {
            return Err(BlockDeviceError::OutOfBounds { offset, len, size });
        }
        let start = usize::try_from(offset).map_err(|_| BlockDeviceError::RangeOverflow { offset, len })?;
        let end = start.checked_add(len).ok_or(BlockDeviceError::RangeOverflow { offset, len })?;
        Ok(self.bytes[start..end].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::{MemoryBlockDevice, ReadOnlyBlockDevice};

    #[test]
    fn memory_device_reads_ranges() {
        let dev = MemoryBlockDevice::new(b"0123456789abcdef".to_vec());
        assert_eq!(dev.size().unwrap(), 16);
        assert_eq!(dev.read_at(4, 4).unwrap(), b"4567");
        assert!(dev.read_at(15, 2).is_err());
    }
}
