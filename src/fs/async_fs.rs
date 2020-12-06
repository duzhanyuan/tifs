use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::SystemTime;
use std::{
    future::Future,
    path::{Path, PathBuf},
};

use async_std::task::{block_on, spawn};
use async_trait::async_trait;
use fuser::*;
use tracing::trace;

use super::error::{FsError, Result};
use super::reply::*;

pub fn spawn_reply<F, R, V>(id: u64, reply: R, f: F)
where
    F: Future<Output = Result<V>> + Send + 'static,
    R: FsReply<V> + Send + 'static,
    V: Debug,
{
    spawn(async move {
        trace!("reply to request({})", id);
        let result = f.await;
        reply.reply(id, result);
    });
}

#[async_trait]
pub trait AsyncFileSystem: Send + Sync {
    /// Initialize filesystem.
    /// Called before any other filesystem method.
    /// The kernel module connection can be configured using the KernelConfig object
    async fn init(&mut self, _config: &mut KernelConfig) -> Result<()> {
        Ok(())
    }

    /// Clean up filesystem.
    /// Called on filesystem exit.
    async fn destroy(&mut self) {}

    /// Look up a directory entry by name and get its attributes.
    async fn lookup(&mut self, _parent: u64, _name: OsString) -> Result<Entry> {
        Err(FsError::unimplemented())
    }

    /// Forget about an inode.
    /// The nlookup parameter indicates the number of lookups previously performed on
    /// this inode. If the filesystem implements inode lifetimes, it is recommended that
    /// inodes acquire a single reference on each lookup, and lose nlookup references on
    /// each forget. The filesystem may ignore forget calls, if the inodes don't need to
    /// have a limited lifetime. On unmount it is not guaranteed, that all referenced
    /// inodes will receive a forget message.
    async fn forget(&mut self, _ino: u64, _nlookup: u64) {}

    /// Get file attributes.
    async fn getattr(&mut self, _ino: u64) -> Result<Attr> {
        Err(FsError::unimplemented())
    }

    /// Set file attributes.
    async fn setattr(
        &mut self,
        _ino: u64,
        _mode: Option<u32>,
        _uid: Option<u32>,
        _gid: Option<u32>,
        _size: Option<u64>,
        _atime: Option<TimeOrNow>,
        _mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<u32>,
    ) -> Result<Attr> {
        Err(FsError::unimplemented())
    }

    /// Read symbolic link.
    async fn readlink(&mut self, _ino: u64) -> Result<Data> {
        Err(FsError::unimplemented())
    }

    /// Create file node.
    /// Create a regular file, character device, block device, fifo or socket node.
    async fn mknod(
        &mut self,
        _parent: u64,
        _name: OsString,
        _mode: u32,
        _umask: u32,
        _rdev: u32,
    ) -> Result<Entry> {
        Err(FsError::unimplemented())
    }

    /// Create a directory.
    async fn mkdir(
        &mut self,
        _parent: u64,
        _name: OsString,
        _mode: u32,
        _umask: u32,
    ) -> Result<Entry> {
        Err(FsError::unimplemented())
    }

    /// Remove a file.
    async fn unlink(&mut self, _parent: u64, _name: OsString) -> Result<()> {
        Err(FsError::unimplemented())
    }

    /// Remove a directory.
    async fn rmdir(&mut self, _parent: u64, _name: OsString) -> Result<()> {
        Err(FsError::unimplemented())
    }

    /// Create a symbolic link.
    async fn symlink(&mut self, _parent: u64, _name: OsString, _link: PathBuf) -> Result<Entry> {
        Err(FsError::unimplemented())
    }

    /// Rename a file.
    async fn rename(
        &mut self,
        _parent: u64,
        _name: OsString,
        _newparent: u64,
        _newname: OsString,
        _flags: u32,
    ) -> Result<()> {
        Err(FsError::unimplemented())
    }

    /// Create a hard link.
    async fn link(&mut self, _ino: u64, _newparent: u64, _newname: OsString) -> Result<Entry> {
        Err(FsError::unimplemented())
    }

    /// Open a file.
    /// Open flags (with the exception of O_CREAT, O_EXCL, O_NOCTTY and O_TRUNC) are
    /// available in flags. Filesystem may store an arbitrary file handle (pointer, index,
    /// etc) in fh, and use this in other all other file operations (read, write, flush,
    /// release, fsync). Filesystem may also implement stateless file I/O and not store
    /// anything in fh. There are also some flags (direct_io, keep_cache) which the
    /// filesystem may set, to change the way the file is opened. See fuse_file_info
    /// structure in <fuse_common.h> for more details.
    async fn open(&mut self, _ino: u64, _flags: i32) -> Result<Open> {
        Ok(Open::new(0, 0))
    }

    /// Read data.
    /// Read should send exactly the number of bytes requested except on EOF or error,
    /// otherwise the rest of the data will be substituted with zeroes. An exception to
    /// this is when the file has been opened in 'direct_io' mode, in which case the
    /// return value of the read system call will reflect the return value of this
    /// operation. fh will contain the value set by the open method, or will be undefined
    /// if the open method didn't set any value.
    ///
    /// flags: these are the file flags, such as O_SYNC. Only supported with ABI >= 7.9
    /// lock_owner: only supported with ABI >= 7.9
    async fn read(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
    ) -> Result<Data> {
        Err(FsError::unimplemented())
    }

    /// Write data.
    /// Write should return exactly the number of bytes requested except on error. An
    /// exception to this is when the file has been opened in 'direct_io' mode, in
    /// which case the return value of the write system call will reflect the return
    /// value of this operation. fh will contain the value set by the open method, or
    /// will be undefined if the open method didn't set any value.
    ///
    /// write_flags: will contain FUSE_WRITE_CACHE, if this write is from the page cache. If set,
    /// the pid, uid, gid, and fh may not match the value that would have been sent if write cachin
    /// is disabled
    /// flags: these are the file flags, such as O_SYNC. Only supported with ABI >= 7.9
    /// lock_owner: only supported with ABI >= 7.9
    async fn write(
        &mut self,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _data: Vec<u8>,
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
    ) -> Result<Write> {
        Err(FsError::unimplemented())
    }

    /// Flush method.
    /// This is called on each close() of the opened file. Since file descriptors can
    /// be duplicated (dup, dup2, fork), for one open call there may be many flush
    /// calls. Filesystems shouldn't assume that flush will always be called after some
    /// writes, or that if will be called at all. fh will contain the value set by the
    /// open method, or will be undefined if the open method didn't set any value.
    /// NOTE: the name of the method is misleading, since (unlike fsync) the filesystem
    /// is not forced to flush pending writes. One reason to flush data, is if the
    /// filesystem wants to return write errors. If the filesystem supports file locking
    /// operations (setlk, getlk) it should remove all locks belonging to 'lock_owner'.
    async fn flush(&mut self, _ino: u64, _fh: u64, _lock_owner: u64) -> Result<()> {
        Err(FsError::unimplemented())
    }

    /// Release an open file.
    /// Release is called when there are no more references to an open file: all file
    /// descriptors are closed and all memory mappings are unmapped. For every open
    /// call there will be exactly one release call. The filesystem may reply with an
    /// error, but error values are not returned to close() or munmap() which triggered
    /// the release. fh will contain the value set by the open method, or will be undefined
    /// if the open method didn't set any value. flags will contain the same flags as for
    /// open.
    async fn release(
        &mut self,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
    ) -> Result<()> {
        Ok(())
    }

    /// Synchronize file contents.
    /// If the datasync parameter is non-zero, then only the user data should be flushed,
    /// not the meta data.
    async fn fsync(&mut self, _ino: u64, _fh: u64, _datasync: bool) -> Result<()> {
        Err(FsError::unimplemented())
    }

    /// Open a directory.
    /// Filesystem may store an arbitrary file handle (pointer, index, etc) in fh, and
    /// use this in other all other directory stream operations (readdir, releasedir,
    /// fsyncdir). Filesystem may also implement stateless directory I/O and not store
    /// anything in fh, though that makes it impossible to implement standard conforming
    /// directory stream operations in case the contents of the directory can change
    /// between opendir and releasedir.
    fn opendir(&mut self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: ReplyOpen) {
        reply.opened(0, 0);
    }

    /// Read directory.
    /// Send a buffer filled using buffer.fill(), with size not exceeding the
    /// requested size. Send an empty buffer on end of stream. fh will contain the
    /// value set by the opendir method, or will be undefined if the opendir method
    /// didn't set any value.
    fn readdir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        reply: ReplyDirectory,
    ) {
        reply.error(ENOSYS);
    }

    /// Read directory.
    /// Send a buffer filled using buffer.fill(), with size not exceeding the
    /// requested size. Send an empty buffer on end of stream. fh will contain the
    /// value set by the opendir method, or will be undefined if the opendir method
    /// didn't set any value.
    fn readdirplus(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        reply: ReplyDirectoryPlus,
    ) {
        reply.error(ENOSYS);
    }

    /// Release an open directory.
    /// For every opendir call there will be exactly one releasedir call. fh will
    /// contain the value set by the opendir method, or will be undefined if the
    /// opendir method didn't set any value.
    fn releasedir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    /// Synchronize directory contents.
    /// If the datasync parameter is set, then only the directory contents should
    /// be flushed, not the meta data. fh will contain the value set by the opendir
    /// method, or will be undefined if the opendir method didn't set any value.
    fn fsyncdir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        reply.error(ENOSYS);
    }

    /// Get file system statistics.
    fn statfs(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyStatfs) {
        reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
    }

    /// Set an extended attribute.
    fn setxattr(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _name: &OsStr,
        _value: &[u8],
        _flags: i32,
        _position: u32,
        reply: ReplyEmpty,
    ) {
        reply.error(ENOSYS);
    }

    /// Get an extended attribute.
    /// If `size` is 0, the size of the value should be sent with `reply.size()`.
    /// If `size` is not 0, and the value fits, send it with `reply.data()`, or
    /// `reply.error(ERANGE)` if it doesn't.
    fn getxattr(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _name: &OsStr,
        _size: u32,
        reply: ReplyXattr,
    ) {
        reply.error(ENOSYS);
    }

    /// List extended attribute names.
    /// If `size` is 0, the size of the value should be sent with `reply.size()`.
    /// If `size` is not 0, and the value fits, send it with `reply.data()`, or
    /// `reply.error(ERANGE)` if it doesn't.
    fn listxattr(&mut self, _req: &Request<'_>, _ino: u64, _size: u32, reply: ReplyXattr) {
        reply.error(ENOSYS);
    }

    /// Remove an extended attribute.
    fn removexattr(&mut self, _req: &Request<'_>, _ino: u64, _name: &OsStr, reply: ReplyEmpty) {
        reply.error(ENOSYS);
    }

    /// Check file access permissions.
    /// This will be called for the access() system call. If the 'default_permissions'
    /// mount option is given, this method is not called. This method is not called
    /// under Linux kernel versions 2.4.x
    fn access(&mut self, _req: &Request<'_>, _ino: u64, _mask: i32, reply: ReplyEmpty) {
        reply.error(ENOSYS);
    }

    /// Create and open a file.
    /// If the file does not exist, first create it with the specified mode, and then
    /// open it. Open flags (with the exception of O_NOCTTY) are available in flags.
    /// Filesystem may store an arbitrary file handle (pointer, index, etc) in fh,
    /// and use this in other all other file operations (read, write, flush, release,
    /// fsync). There are also some flags (direct_io, keep_cache) which the
    /// filesystem may set, to change the way the file is opened. See fuse_file_info
    /// structure in <fuse_common.h> for more details. If this method is not
    /// implemented or under Linux kernel versions earlier than 2.6.15, the mknod()
    /// and open() methods will be called instead.
    fn create(
        &mut self,
        _req: &Request<'_>,
        _parent: u64,
        _name: &OsStr,
        _mode: u32,
        _umask: u32,
        _flags: i32,
        reply: ReplyCreate,
    ) {
        reply.error(ENOSYS);
    }

    /// Test for a POSIX file lock.
    fn getlk(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: i32,
        _pid: u32,
        reply: ReplyLock,
    ) {
        reply.error(ENOSYS);
    }

    /// Acquire, modify or release a POSIX file lock.
    /// For POSIX threads (NPTL) there's a 1-1 relation between pid and owner, but
    /// otherwise this is not always the case.  For checking lock ownership,
    /// 'fi->owner' must be used. The l_pid field in 'struct flock' should only be
    /// used to fill in this field in getlk(). Note: if the locking methods are not
    /// implemented, the kernel will still allow file locking to work locally.
    /// Hence these are only interesting for network filesystems and similar.
    fn setlk(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        _start: u64,
        _end: u64,
        _typ: i32,
        _pid: u32,
        _sleep: bool,
        reply: ReplyEmpty,
    ) {
        reply.error(ENOSYS);
    }

    /// Map block index within file to block index within device.
    /// Note: This makes sense only for block device backed filesystems mounted
    /// with the 'blkdev' option
    fn bmap(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _blocksize: u32,
        _idx: u64,
        reply: ReplyBmap,
    ) {
        reply.error(ENOSYS);
    }

    /// control device
    fn ioctl(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: u32,
        _cmd: u32,
        _in_data: &[u8],
        _out_size: u32,
        reply: ReplyIoctl,
    ) {
        reply.error(ENOSYS);
    }

    /// Preallocate or deallocate space to a file
    fn fallocate(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _length: i64,
        _mode: i32,
        reply: ReplyEmpty,
    ) {
        reply.error(ENOSYS);
    }

    /// Reposition read/write file offset
    fn lseek(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _whence: i32,
        reply: ReplyLseek,
    ) {
        reply.error(ENOSYS);
    }

    /// Copy the specified range from the source inode to the destination inode
    fn copy_file_range(
        &mut self,
        _req: &Request<'_>,
        _ino_in: u64,
        _fh_in: u64,
        _offset_in: i64,
        _ino_out: u64,
        _fh_out: u64,
        _offset_out: i64,
        _len: u64,
        _flags: u32,
        reply: ReplyWrite,
    ) {
        reply.error(ENOSYS);
    }

    /// macOS only: Rename the volume. Set fuse_init_out.flags during init to
    /// FUSE_VOL_RENAME to enable
    #[cfg(target_os = "macos")]
    fn setvolname(&mut self, _req: &Request<'_>, _name: &OsStr, reply: ReplyEmpty) {
        reply.error(ENOSYS);
    }

    /// macOS only (undocumented)
    #[cfg(target_os = "macos")]
    fn exchange(
        &mut self,
        _req: &Request<'_>,
        _parent: u64,
        _name: &OsStr,
        _newparent: u64,
        _newname: &OsStr,
        _options: u64,
        reply: ReplyEmpty,
    ) {
        reply.error(ENOSYS);
    }

    /// macOS only: Query extended times (bkuptime and crtime). Set fuse_init_out.flags
    /// during init to FUSE_XTIMES to enable
    #[cfg(target_os = "macos")]
    fn getxtimes(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyXTimes) {
        reply.error(ENOSYS);
    }
}

pub struct AsyncFs<T>(Arc<T>);

impl<T: AsyncFileSystem> From<Arc<T>> for AsyncFs<T> {
    fn from(inner: Arc<T>) -> Self {
        Self(inner)
    }
}

impl<T: Debug> Debug for AsyncFs<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: AsyncFileSystem + 'static> Filesystem for AsyncFs<T> {
    fn init(&mut self, _req: &fuse::Request) -> std::result::Result<(), nix::libc::c_int> {
        block_on(self.0.init()).map_err(|err| err.into())
    }

    fn destroy(&mut self, _req: &fuse::Request) {
        block_on(self.0.destroy())
    }

    fn lookup(&mut self, req: &Request, parent: u64, name: &std::ffi::OsStr, reply: ReplyEntry) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.lookup(parent, name).await
        });
    }

    fn forget(&mut self, _req: &Request, ino: u64, nlookup: u64) {
        let async_impl = self.0.clone();

        // TODO: union the spawn function for request without reply
        spawn(async move {
            async_impl.forget(ino, nlookup).await;
        });
    }

    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {
        let async_impl = self.0.clone();
        spawn_reply(
            req.unique(),
            reply,
            async move { async_impl.getattr(ino).await },
        );
    }

    fn setattr(
        &mut self,
        req: &Request,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Timespec>,
        mtime: Option<Timespec>,
        fh: Option<u64>,
        crtime: Option<Timespec>,
        chgtime: Option<Timespec>,
        bkuptime: Option<Timespec>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl
                .setattr(
                    ino, mode, uid, gid, size, atime, mtime, fh, crtime, chgtime, bkuptime, flags,
                )
                .await
        });
    }

    fn readlink(&mut self, req: &Request, ino: u64, reply: ReplyData) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.readlink(ino).await
        });
    }
    fn mknod(
        &mut self,
        req: &Request,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.mknod(parent, name, mode, rdev).await
        });
    }
    fn mkdir(
        &mut self,
        req: &Request,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        reply: ReplyEntry,
    ) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.mkdir(parent, name, mode).await
        });
    }
    fn unlink(&mut self, req: &Request, parent: u64, name: &std::ffi::OsStr, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.unlink(parent, name).await
        });
    }
    fn rmdir(&mut self, req: &Request, parent: u64, name: &std::ffi::OsStr, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.rmdir(parent, name).await
        });
    }
    fn symlink(
        &mut self,
        req: &Request,
        parent: u64,
        name: &std::ffi::OsStr,
        link: &Path,
        reply: ReplyEntry,
    ) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        let link = link.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.symlink(parent, name, link).await
        });
    }
    fn rename(
        &mut self,
        req: &Request,
        parent: u64,
        name: &std::ffi::OsStr,
        newparent: u64,
        newname: &std::ffi::OsStr,
        reply: ReplyEmpty,
    ) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        let newname = newname.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.rename(parent, name, newparent, newname).await
        });
    }
    fn link(
        &mut self,
        req: &Request,
        ino: u64,
        newparent: u64,
        newname: &std::ffi::OsStr,
        reply: ReplyEntry,
    ) {
        let async_impl = self.0.clone();
        let newname = newname.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.link(ino, newparent, newname).await
        });
    }
    fn open(&mut self, req: &Request, ino: u64, flags: u32, reply: ReplyOpen) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.open(ino, flags).await
        });
    }
    fn read(&mut self, req: &Request, ino: u64, fh: u64, offset: i64, size: u32, reply: ReplyData) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.read(ino, fh, offset, size).await
        });
    }
    fn write(
        &mut self,
        req: &Request,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        flags: u32,
        reply: ReplyWrite,
    ) {
        let async_impl = self.0.clone();
        let data = data.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.write(ino, fh, offset, data, flags).await
        });
    }
    fn flush(&mut self, req: &Request, ino: u64, fh: u64, lock_owner: u64, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.flush(ino, fh, lock_owner).await
        });
    }
    fn release(
        &mut self,
        req: &Request,
        ino: u64,
        fh: u64,
        flags: u32,
        lock_owner: u64,
        flush: bool,
        reply: ReplyEmpty,
    ) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.release(ino, fh, flags, lock_owner, flush).await
        });
    }
    fn fsync(&mut self, req: &Request, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.fsync(ino, fh, datasync).await
        });
    }
    fn opendir(&mut self, req: &Request, ino: u64, flags: u32, reply: ReplyOpen) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.opendir(ino, flags).await
        });
    }
    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, reply: ReplyDirectory) {
        let async_impl = self.0.clone();
        spawn(async move {
            async_impl.readdir(ino, fh, offset, reply).await;
        });
    }
    fn releasedir(&mut self, req: &Request, ino: u64, fh: u64, flags: u32, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.releasedir(ino, fh, flags).await
        });
    }
    fn fsyncdir(&mut self, req: &Request, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.fsyncdir(ino, fh, datasync).await
        });
    }
    fn statfs(&mut self, req: &Request, ino: u64, reply: ReplyStatfs) {
        let async_impl = self.0.clone();
        spawn_reply(
            req.unique(),
            reply,
            async move { async_impl.statfs(ino).await },
        );
    }
    fn setxattr(
        &mut self,
        req: &Request,
        ino: u64,
        name: &std::ffi::OsStr,
        value: &[u8],
        flags: u32,
        position: u32,
        reply: ReplyEmpty,
    ) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        let value = value.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.setxattr(ino, name, value, flags, position).await
        });
    }
    fn getxattr(
        &mut self,
        req: &Request,
        ino: u64,
        name: &std::ffi::OsStr,
        size: u32,
        reply: ReplyXattr,
    ) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.getxattr(ino, name, size).await
        });
    }
    fn listxattr(&mut self, req: &Request, ino: u64, size: u32, reply: ReplyXattr) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.listxattr(ino, size).await
        });
    }
    fn removexattr(&mut self, req: &Request, ino: u64, name: &std::ffi::OsStr, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.removexattr(ino, name).await
        });
    }
    fn access(&mut self, req: &Request, ino: u64, mask: u32, reply: ReplyEmpty) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl.access(ino, mask).await
        });
    }
    fn create(
        &mut self,
        req: &Request,
        parent: u64,
        name: &std::ffi::OsStr,
        mode: u32,
        flags: u32,
        reply: ReplyCreate,
    ) {
        let uid = req.uid();
        let gid = req.gid();

        let async_impl = self.0.clone();
        let name = name.to_owned();
        spawn_reply(req.unique(), reply, async move {
            async_impl.create(parent, name, mode, flags, uid, gid).await
        });
    }
    fn getlk(
        &mut self,
        req: &Request,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: u32,
        pid: u32,
        reply: ReplyLock,
    ) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl
                .getlk(ino, fh, lock_owner, start, end, typ, pid)
                .await
        });
    }
    fn setlk(
        &mut self,
        req: &Request,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: u32,
        pid: u32,
        sleep: bool,
        reply: ReplyEmpty,
    ) {
        let async_impl = self.0.clone();
        spawn_reply(req.unique(), reply, async move {
            async_impl
                .setlk(ino, fh, lock_owner, start, end, typ, pid, sleep)
                .await
        });
    }
    fn bmap(&mut self, _req: &Request, ino: u64, blocksize: u32, idx: u64, reply: ReplyBmap) {
        let async_impl = self.0.clone();
        spawn(async move {
            async_impl.bmap(ino, blocksize, idx, reply).await;
        });
    }
}
