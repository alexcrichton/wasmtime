use super::oshandle::RawOsHandle;
use crate::handle::{HandleRights, Rights, RightsExt};
use crate::sys::osfile::OsFile;
use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::os::unix::prelude::{AsRawFd, FromRawFd, IntoRawFd};

impl TryFrom<File> for OsFile {
    type Error = io::Error;

    fn try_from(file: File) -> io::Result<Self> {
        let ft = file.metadata()?.file_type();
        if !ft.is_file() {
            return Err(io::Error::from_raw_os_error(libc::EINVAL));
        }
        let rights = get_rights(&file)?;
        let handle = unsafe { RawOsHandle::from_raw_fd(file.into_raw_fd()) };
        Ok(Self::new(rights, handle))
    }
}

fn get_rights(file: &File) -> io::Result<HandleRights> {
    use yanix::{fcntl, file::OFlags};
    let mut rights = HandleRights::new(
        Rights::regular_file_base(),
        Rights::regular_file_inheriting(),
    );
    let flags = unsafe { fcntl::get_status_flags(file.as_raw_fd())? };
    let accmode = flags & OFlags::ACCMODE;
    if accmode == OFlags::RDONLY {
        rights.base &= !Rights::FD_WRITE;
    } else if accmode == OFlags::WRONLY {
        rights.base &= !Rights::FD_READ;
    }
    Ok(rights)
}
