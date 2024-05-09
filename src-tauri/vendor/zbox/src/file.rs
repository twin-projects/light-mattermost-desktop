use std::fmt::{self, Debug};
use std::io::{self, Error as IoError, ErrorKind, Read, Seek, SeekFrom, Write};

use super::{Error, Result};
use crate::fs::fnode::{
    Fnode, Metadata, Reader as FnodeReader, Version, Writer as FnodeWriter,
};
use crate::fs::Handle;
use crate::trans::{TxHandle, TxMgr};

/// A reader for a specific vesion of file content.
///
/// This reader can be obtained by [`version_reader`] method, and it
/// implements [`Read`] trait.
///
/// [`version_reader`]: struct.File.html#method.version_reader
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
#[derive(Debug)]
pub struct VersionReader {
    handle: Handle,
    rdr: FnodeReader,
}

impl VersionReader {
    fn new(handle: &Handle, ver: usize) -> Result<Self> {
        let rdr = FnodeReader::new(handle.fnode.clone(), ver, &handle.store)?;
        Ok(VersionReader {
            handle: handle.clone(),
            rdr,
        })
    }

    /// Returns the content version associated with this reader.
    pub fn version(&self) -> Result<Version> {
        let fnode = self.handle.fnode.read().unwrap();
        fnode
            .ver(self.rdr.version_num())
            .cloned()
            .ok_or(Error::NoVersion)
    }
}

impl Read for VersionReader {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.rdr.read(buf)
    }
}

impl Seek for VersionReader {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.rdr.seek(pos)
    }
}

/// A reference to an opened file in the repository.
///
/// An instance of a `File` can be read and/or written depending on what options
/// it was opened with. Files also implement [`Seek`] to alter the logical
/// cursor that the file contains internally.
///
/// Files are automatically closed when they go out of scope.
///
/// As ZboxFS internally cached file content, it is no need to use buffered
/// reader, such as [`BufReader<R>`].
///
/// # Examples
///
/// Create a new file and write bytes to it:
///
/// ```
/// use std::io::prelude::*;
/// # use zbox::{init_env, Result, RepoOpener};
///
/// # fn foo() -> Result<()> {
/// # init_env();
/// # let mut repo = RepoOpener::new().create(true).open("mem://foo", "pwd")?;
/// let mut file = repo.create_file("/foo.txt")?;
/// file.write_all(b"Hello, world!")?;
/// file.finish()?;
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// Read the content of a file into a [`String`]:
///
/// ```
/// # use zbox::{init_env, Result, RepoOpener};
/// use std::io::prelude::*;
/// # use zbox::OpenOptions;
///
/// # fn foo() -> Result<()> {
/// # init_env();
/// # let mut repo = RepoOpener::new().create(true).open("mem://foo", "pwd")?;
/// # {
/// #     let mut file = OpenOptions::new()
/// #         .create(true)
/// #         .open(&mut repo, "/foo.txt")?;
/// #     file.write_all(b"Hello, world!")?;
/// #     file.finish()?;
/// # }
/// let mut file = repo.open_file("/foo.txt")?;
/// let mut content = String::new();
/// file.read_to_string(&mut content)?;
/// assert_eq!(content, "Hello, world!");
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// # Versioning
///
/// `File` contents support up to 255 revision versions. [`Version`] is
/// immutable once it is created.
///
/// By default, the maximum number of versions of a `File` is `1`, which is
/// configurable by [`version_limit`] on both `Repo` and `File` level. File
/// level option takes precedence.
///
/// After reaching this limit, the oldest [`Version`] will be automatically
/// deleted after adding a new one.
///
/// Version number starts from `1` and continuously increases by 1.
///
/// # Writing
///
/// File content is cached internally for deduplication and will be handled
/// automatically, thus calling [`flush`] is **not** recommended.
///
/// `File` can be sent to multiple threads, but only one thread can modify it at
/// a time, which is similar to a `RwLock`.
///
/// `File` is multi-versioned, each time updating its content will create a new
/// permanent [`Version`]. There are two ways of writing data to a file:
///
/// - **Multi-part Write**
///
///   This is done by updating `File` using [`Write`] trait multiple times.
///   After all writing operations, [`finish`] must be called to create a new
///   version. Unless [`finish`] was successfully returned, no data will be
///   written to the file.
///
///   Internally, a transaction is created when writing to the file first time
///   and calling [`finish`] will commit that transaction. If any errors
///   happened during [`write`], that transaction will be aborted. Thus, you
///   should not call [`finish`] after any failed [`write`].
///
///   Because transactions is thread local, multi-part write should be done in
///   one transaction.
///
///   ## Examples
///
///   ```
///   # use zbox::{init_env, Result, RepoOpener};
///   use std::io::prelude::*;
///   use std::io::SeekFrom;
///   # use zbox::OpenOptions;
///
///   # fn foo() -> Result<()> {
///   # init_env();
///   # let mut repo = RepoOpener::new().create(true).open("mem://foo", "pwd")?;
///   let mut file = OpenOptions::new()
///       .create(true)
///       .open(&mut repo, "/foo.txt")?;
///   file.write_all(b"foo ")?;
///   file.write_all(b"bar")?;
///   file.finish()?;
///
///   let mut content = String::new();
///   file.seek(SeekFrom::Start(0))?;
///   file.read_to_string(&mut content)?;
///   assert_eq!(content, "foo bar");
///
///   # Ok(())
///   # }
///   # foo().unwrap();
///   ```
///
/// - **Single-part Write**
///
///   This can be done by calling [`write_once`], which will call [`finish`]
///   internally to create a new version. Unless this method was successfully
///   returned, no data will be written to the file.
///
///   ## Examples
///
///   ```
///   # #![allow(unused_mut, unused_variables)]
///   # use zbox::{init_env, Result, RepoOpener};
///   use std::io::{Read, Seek, SeekFrom};
///   # use zbox::OpenOptions;
///
///   # fn foo() -> Result<()> {
///   # init_env();
///   # let mut repo = RepoOpener::new().create(true).open("mem://foo", "pwd")?;
///   let mut file = OpenOptions::new()
///       .create(true)
///       .open(&mut repo, "/foo.txt")?;
///   file.write_once(b"foo bar")?;
///
///   let mut content = String::new();
///   file.seek(SeekFrom::Start(0))?;
///   file.read_to_string(&mut content)?;
///   assert_eq!(content, "foo bar");
///
///   # Ok(())
///   # }
///   # foo().unwrap();
///   ```
///
/// To gurantee atomicity, ZboxFS uses transaction when updating file so the
/// data either be wholly persisted or nothing has been written.
///
/// - For multi-part write, the transaction begins in the first-time [`write`]
///   and will be committed in [`finish`]. Any failure in [`write`] will abort
///   the transaction, thus [`finish`] should not be called after that. If error
///   happened during [`finish`], the transaction will also be aborted.
/// - For single-part write, [`write_once`] itself is transactional. The
///   transaction begins and will be committed inside this method.
///
/// Keep in mind of those characteristics, especially when writing a large
/// amount of data to file, because any uncomitted transactions will abort
/// and data in those transactions won't be persisted.
///
/// # Reading
///
/// As `File` can contain multiple versions, [`Read`] operation can be
/// associated with different versions. By default, reading on `File` object is
/// always bound to the latest version. To read a specific version, a
/// [`VersionReader`], which supports [`Read`] trait as well, can be used.
///
/// ## Examples
///
/// Read the file content while it is in writing, notice that reading is always
/// bound to latest content version.
///
/// ```
/// use std::io::prelude::*;
/// use std::io::SeekFrom;
/// # use zbox::{init_env, Result, RepoOpener};
/// # use zbox::OpenOptions;
///
/// # fn foo() -> Result<()> {
/// # init_env();
/// # let mut repo = RepoOpener::new().create(true).open("mem://foo", "pwd")?;
/// // create a file and write data to it
/// let mut file = OpenOptions::new().create(true).open(&mut repo, "/foo.txt")?;
/// file.write_once(&[1, 2, 3, 4, 5, 6])?;
///
/// // read the first 2 bytes
/// let mut buf = [0; 2];
/// file.seek(SeekFrom::Start(0))?;
/// file.read_exact(&mut buf)?;
/// assert_eq!(&buf[..], &[1, 2]);
///
/// // create a new version, now the file content is [1, 2, 7, 8, 5, 6]
/// file.write_once(&[7, 8])?;
///
/// // notice that reading is on the latest version
/// file.seek(SeekFrom::Current(-2))?;
/// file.read_exact(&mut buf)?;
/// assert_eq!(&buf[..], &[7, 8]);
///
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// Read multiple versions using [`VersionReader`].
///
/// ```
/// use std::io::prelude::*;
/// # use zbox::{init_env, Result, RepoOpener};
/// # use zbox::OpenOptions;
///
/// # fn foo() -> Result<()> {
/// # init_env();
/// # let mut repo = RepoOpener::new().create(true).open("mem://foo", "pwd")?;
/// // create a file and write 2 versions
/// let mut file = OpenOptions::new()
///     .version_limit(4)
///     .create(true)
///     .open(&mut repo, "/foo.txt")?;
/// file.write_once(b"foo")?;
/// file.write_once(b"bar")?;
///
/// // get latest version number
/// let curr_ver = file.curr_version()?;
///
/// // create a version reader and read latest version of content
/// let mut rdr = file.version_reader(curr_ver)?;
/// let mut content = String::new();
/// rdr.read_to_string(&mut content)?;
/// assert_eq!(content, "foobar");
///
/// // create another version reader and read previous version of content
/// let mut rdr = file.version_reader(curr_ver - 1)?;
/// let mut content = String::new();
/// rdr.read_to_string(&mut content)?;
/// assert_eq!(content, "foo");
///
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// [`Seek`]: https://doc.rust-lang.org/std/io/trait.Seek.html
/// [`BufReader<R>`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
/// [`flush`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [`Version`]: struct.Version.html
/// [`VersionReader`]: struct.VersionReader.html
/// [`version_limit`]: struct.OpenOptions.html#method.version_limit
/// [`finish`]: struct.File.html#method.finish
/// [`write_once`]: struct.File.html#method.write_once
pub struct File {
    handle: Handle,
    pos: SeekFrom, // must always be SeekFrom::Start
    rdr: Option<FnodeReader>,
    wtr: Option<FnodeWriter>,
    tx_handle: Option<TxHandle>,
    can_read: bool,
    can_write: bool,
}

impl File {
    pub(super) fn new(
        handle: Handle,
        pos: SeekFrom,
        can_read: bool,
        can_write: bool,
    ) -> Self {
        File {
            handle,
            pos,
            rdr: None,
            wtr: None,
            tx_handle: None,
            can_read,
            can_write,
        }
    }

    /// Check if file system is closed
    fn check_closed(&self) -> Result<()> {
        let shutter = self.handle.shutter.read().unwrap();
        if shutter.is_closed() {
            return Err(Error::RepoClosed);
        }
        Ok(())
    }

    /// Queries metadata about the file.
    pub fn metadata(&self) -> Result<Metadata> {
        self.check_closed()?;
        let fnode = self.handle.fnode.read().unwrap();
        Ok(fnode.metadata())
    }

    /// Returns a list of all the file content versions.
    pub fn history(&self) -> Result<Vec<Version>> {
        self.check_closed()?;
        let fnode = self.handle.fnode.read().unwrap();
        Ok(fnode.history())
    }

    /// Returns the current content version number.
    pub fn curr_version(&self) -> Result<usize> {
        self.check_closed()?;
        let fnode = self.handle.fnode.read().unwrap();
        Ok(fnode.curr_ver_num())
    }

    /// Returns content byte size of the current version.
    fn curr_len(&self) -> usize {
        let fnode = self.handle.fnode.read().unwrap();
        fnode.curr_len()
    }

    /// Get a reader of the specified version.
    ///
    /// The returned reader implements [`Read`] trait. To get the version
    /// number, first call [`history`] to get the list of all versions and
    /// then choose the version number from it.
    ///
    /// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
    /// [`history`]: struct.File.html#method.history
    pub fn version_reader(&self, ver_num: usize) -> Result<VersionReader> {
        self.check_closed()?;
        if !self.can_read {
            return Err(Error::CannotRead);
        }
        VersionReader::new(&self.handle, ver_num)
    }

    // calculate the seek position from the start based on file current size
    fn seek_pos(&self, pos: SeekFrom) -> SeekFrom {
        let curr_len = self.curr_len();
        let pos: i64 = match pos {
            SeekFrom::Start(p) => p as i64,
            SeekFrom::End(p) => curr_len as i64 + p,
            SeekFrom::Current(p) => match self.pos {
                SeekFrom::Start(q) => p + q as i64,
                SeekFrom::End(_) => unreachable!(),
                SeekFrom::Current(_) => unreachable!(),
            },
        };
        SeekFrom::Start(pos as u64)
    }

    fn begin_write(&mut self) -> Result<()> {
        if !self.can_write {
            return Err(Error::CannotWrite);
        }

        if self.wtr.is_some() {
            return Err(Error::NotFinish);
        }

        assert!(self.tx_handle.is_none());

        // append zeros if current position is beyond EOF
        let curr_len = self.curr_len();
        match self.pos {
            SeekFrom::Start(pos) => {
                let pos = pos as usize;
                if pos > curr_len {
                    // append zeros by setting file length
                    self.set_len(pos)?;

                    // then seek to new EOF
                    self.pos = self.seek_pos(SeekFrom::End(0));
                }
            }
            _ => unreachable!(),
        }

        // begin write
        let txmgr = self.handle.txmgr.upgrade().ok_or(Error::RepoClosed)?;
        let tx_handle = TxMgr::begin_trans(&txmgr)?;
        tx_handle.run(|| {
            let mut wtr =
                FnodeWriter::new(self.handle.clone(), tx_handle.txid)?;
            wtr.seek(self.seek_pos(self.pos))?;
            self.wtr = Some(wtr);
            Ok(())
        })?;
        self.tx_handle = Some(tx_handle);

        Ok(())
    }

    // re-create reader on latest version
    fn renew_reader(&mut self) -> Result<()> {
        let mut rdr = FnodeReader::new_current(
            self.handle.fnode.clone(),
            &self.handle.store,
        )?;
        rdr.seek(self.pos)?;
        self.rdr = Some(rdr);
        Ok(())
    }

    /// Complete multi-part write to file and create a new version.
    ///
    /// This method will try to commit the transaction internally, no data will
    /// be persisted if it failed. Do not call this method if any previous
    /// [`write`] failed.
    ///
    /// # Errors
    ///
    /// Calling this method without writing data before will return
    /// [`Error::NotWrite`] error.
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`Error::NotWrite`]: enum.Error.html
    pub fn finish(&mut self) -> Result<()> {
        self.check_closed()?;

        match self.wtr.take() {
            Some(wtr) => {
                let tx_handle = self.tx_handle.take().unwrap();
                let mut end_pos = 0;

                tx_handle.run_all_exclusive(|| {
                    end_pos = wtr.finish()?;
                    Ok(())
                })?;

                // set position
                self.pos = SeekFrom::Start(end_pos as u64);
            }
            None => return Err(Error::NotWrite),
        }

        // re-create reader if there is an existing reader
        if self.rdr.is_some() {
            self.renew_reader()?;
        }

        Ok(())
    }

    /// Single-part write to file and create a new version.
    ///
    /// This method provides a convenient way of combining [`Write`] and
    /// [`finish`].
    ///
    /// This method is atomic.
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`finish`]: struct.File.html#method.finish
    pub fn write_once(&mut self, buf: &[u8]) -> Result<()> {
        self.check_closed()?;
        match self.wtr {
            Some(_) => Err(Error::NotFinish),
            None => {
                self.begin_write()?;
                match self.wtr {
                    Some(ref mut wtr) => match self.tx_handle {
                        Some(ref tx_handle) => {
                            tx_handle.run(|| {
                                wtr.write_all(buf)?;
                                Ok(())
                            })?;
                        }
                        None => unreachable!(),
                    },
                    None => unreachable!(),
                }
                self.finish()
            }
        }
    }

    /// Truncates or extends the underlying file, create a new version of
    /// content which size to become `size`.
    ///
    /// If the size is less than the current content size, then the new
    /// content will be shrunk. If it is greater than the current content size,
    /// then the content will be extended to `size` and have all of the
    /// intermediate data filled in with 0s.
    ///
    /// This method is atomic.
    ///
    /// # Errors
    ///
    /// This method will return an error if the file is not opened for writing
    /// or not finished writing.
    pub fn set_len(&mut self, len: usize) -> Result<()> {
        self.check_closed()?;
        if self.wtr.is_some() {
            return Err(Error::NotFinish);
        }

        if !self.can_write {
            return Err(Error::CannotWrite);
        }

        let txmgr = self.handle.txmgr.upgrade().ok_or(Error::RepoClosed)?;
        let tx_handle = TxMgr::begin_trans(&txmgr)?;
        tx_handle.run_all_exclusive(|| {
            Fnode::set_len(self.handle.clone(), len, tx_handle.txid)
        })?;

        // re-create reader if there is an existing reader
        if self.rdr.is_some() {
            self.renew_reader()?;
        }

        Ok(())
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        map_io_err!(self.check_closed())?;
        if !self.can_read {
            return Err(IoError::new(
                ErrorKind::Other,
                Error::CannotRead.to_string(),
            ));
        }

        // if reader is not created yet, create a new reader and seek to
        // the current file position
        if self.rdr.is_none() {
            map_io_err!(self.renew_reader())?;
        }

        match self.rdr {
            Some(ref mut rdr) => {
                let read = rdr.read(buf)?;
                let new_pos = rdr.seek(SeekFrom::Current(0)).unwrap();
                self.pos = SeekFrom::Start(new_pos);
                Ok(read)
            }
            None => unreachable!(),
        }
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        map_io_err!(self.check_closed())?;
        if self.wtr.is_none() {
            map_io_err!(self.begin_write())?;
        }

        let mut ret = 0;
        map_io_err!(match self.wtr {
            Some(ref mut wtr) => match self.tx_handle {
                Some(ref tx_handle) => tx_handle
                    .run(|| {
                        ret = wtr.write(buf)?;
                        Ok(())
                    })
                    .map(|_| ret),
                None => unreachable!(),
            },
            None => unreachable!(),
        }
        .map_err(|err| {
            // when write failed the tx has been aborted, so we need to clean up
            // writer and tx handle here
            self.wtr.take();
            self.tx_handle.take();
            err
        }))
    }

    fn flush(&mut self) -> io::Result<()> {
        map_io_err!(self.check_closed())?;
        match self.wtr {
            Some(ref mut wtr) => match self.tx_handle {
                Some(ref tx_handle) => {
                    map_io_err!(tx_handle.run(|| {
                        wtr.flush()?;
                        Ok(())
                    }))?;
                    Ok(())
                }
                None => unreachable!(),
            },
            None => Err(IoError::new(
                ErrorKind::PermissionDenied,
                Error::CannotWrite.to_string(),
            )),
        }
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        map_io_err!(self.check_closed())?;
        if self.wtr.is_some() {
            return Err(IoError::new(
                ErrorKind::Other,
                Error::NotFinish.to_string(),
            ));
        }

        self.pos = match self.rdr {
            Some(ref mut rdr) => SeekFrom::Start(rdr.seek(pos)?),
            None => self.seek_pos(pos),
        };

        match self.pos {
            SeekFrom::Start(pos) => Ok(pos),
            _ => unreachable!(),
        }
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("File")
            .field("pos", &self.pos)
            .field("rdr", &self.rdr)
            .field("wtr", &self.wtr)
            .field("can_read", &self.can_read)
            .field("can_write", &self.can_write)
            .finish()
    }
}
