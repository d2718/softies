/*!
Machinery for filtering by entry type.

This module is an absolute _mess_ of #[cfg(...)] directives, but I can't
think of a better way to do this. I'd like to thank `rustc` for always
having my back.
*/
use std::{
    convert::TryFrom,
    fs::FileType,
};
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
#[cfg(windows)]
use std::os::windows::fs::FileTypeExt;
#[cfg(wasi)]
use std::os::wasi::fs::FileTypeExt;

use enum_iterator::{all, Sequence};

/// Type of directory entry.
#[derive(Clone, Copy, Debug, Sequence)]
#[non_exhaustive]
pub enum EType {
    /// regular file
    File,
    /// directory
    Dir,
    /// symbolic link
    Link,
    /// FIFO (i.e., a pipe)
    #[cfg(unix)]
    Fifo,
    /// a socket
    #[cfg(any(unix, wasi))]
    Socket,
    /// block device
    #[cfg(any(unix, wasi))]
    Block,
    /// character device
    #[cfg(any(unix, wasi))]
    Char,
}

impl EType {
    pub fn as_str(&self) -> &'static str {
        use EType::*;

        match self {
            File => "file",
            Dir => "dir",
            Link => "link",
            #[cfg(unix)]
            Fifo => "fifo",
            #[cfg(any(unix, wasi))]
            Socket => "socket",
            #[cfg(any(unix, wasi))]
            Block => "block",
            #[cfg(any(unix, wasi))]
            Char => "char",
        }
    }
}

/// The `TryFrom` impl is used in parsing user innput.
impl TryFrom<&str> for EType {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use EType::*;

        match s.to_ascii_lowercase().as_str() {
            "f" | "file" => Ok(File),
            "d" | "dir" | "directory" => Ok(Dir),
            "l" | "link" | "symlink" => Ok(Link),
            #[cfg(unix)]
            "p" | "pipe" | "fifo" => Ok(Fifo),
            #[cfg(any(unix, wasi))]
            "s" | "sock" | "socket" => Ok(Socket),
            #[cfg(any(unix, wasi))]
            "b" | "block" => Ok(Block),
            #[cfg(any(unix, wasi))]
            "c" | "ch" | "char" | "character" => Ok(Char),
            _ => {
                let allowed_types: Vec<&str> = all::<EType>().map(|t| t.as_str()).collect();
                let msg = format!(
                    "directory entry type {} invalid or not supported on this platform\npossible values are: {}",
                    s, &allowed_types.join(", ")
                );
                Err(msg)
            },
        }
    }
}

/// This trait is purely to make checking whether a given file's
/// [`FileType`](std::fs::FileType) is in the collection of filteree-for
/// file types.
pub(crate) trait HasEType {
    /// Return whether the receiver is of the provided `EType`.
    fn is(&self, entry_type: &EType) -> bool;

    /// Return whether the receiver is _any_ of the `EType`s in the
    /// supplied `collection`.
    fn is_one<'a, I>(&'a self, collection: I) -> bool
    where
        I: IntoIterator<Item = &'a EType>
    {
        for t in collection.into_iter() {
            if self.is(t) { return true; }
        }

        false
    }
}

impl HasEType for FileType {
    fn is(&self, t: &EType) -> bool {
        use EType::*;
        match t {
            File => self.is_file(),
            Dir => self.is_dir(),
            Link => self.is_symlink(),
            #[cfg(unix)]
            Fifo => self.is_fifo(),
            #[cfg(unix)]
            Socket => self.is_socket(),
            #[cfg(wasi)]
            Socket => self.is_socket_dgram() || self.is_socket_stream(),
            #[cfg(any(unix, wasi))]
            Block => self.is_block_device(),
            #[cfg(any(unix, wasi))]
            Char => self.is_char_device(),
        }
    }
}