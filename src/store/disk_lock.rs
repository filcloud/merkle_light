use std::fs::{Metadata, File};
use std::io::{Initializer, IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write, Result};

use positioned_io::{ReadAt, WriteAt};
use parking_lot::{RwLock, RwLockReadGuard};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DISK_LOCK: RwLock<u32> = RwLock::new(0_u32);
}

pub struct LockedFile {
    pub file: File,
    pub no_lock: bool,
}

impl LockedFile {
    pub fn new(file: File) -> LockedFile {
        LockedFile::new_lock(file, false) // defaults to lock
    }

    pub fn new_lock(file: File, no_lock: bool) -> LockedFile {
        LockedFile {
            file,
            no_lock,
        }
    }
    
    fn lock<'a, 'b>(&'a self) -> Option<RwLockReadGuard<'b, u32>> {
        if self.no_lock {
            None // skip locking
        } else {
            // Some(DISK_LOCK.read())
            None
        }
    }

    pub fn metadata(&self) -> Result<Metadata> {
        self.file.metadata()
    }

    pub fn sync_all(&self) -> Result<()> {
        let _guard = self.lock();
        self.file.sync_all()
    }

    pub fn sync_data(&self) -> Result<()> {
        let _guard = self.lock();
        self.file.sync_data()
    }

    pub fn set_len(&self, size: u64) -> Result<()> {
        let _guard = self.lock();
        self.file.set_len(size)
    }
}

impl Read for LockedFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let _guard = self.lock();
        self.file.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        let _guard = self.lock();
        self.file.read_vectored(bufs)
    }

    #[allow(unsafe_code)]
    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        self.file.initializer()
    }
}

impl Write for LockedFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let _guard = self.lock();
        self.file.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> {
        let _guard = self.lock();
        self.file.write_vectored(bufs)
    }

    fn flush(&mut self) -> Result<()> {
        let _guard = self.lock();
        Write::flush(&mut self.file)
    }
}

impl Seek for LockedFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let _guard = self.lock();
        self.file.seek(pos)
    }
}

impl ReadAt for LockedFile {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        let _guard = self.lock();
        self.file.read_at(pos, buf)
    }
}

impl WriteAt for LockedFile {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        let _guard = self.lock();
        self.file.write_at(pos, buf)
    }
    fn flush(&mut self) -> Result<()> {
        Write::flush(self)
    }
}

impl std::fmt::Debug for LockedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.file.fmt(f)
    }
}
