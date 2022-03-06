use std::io::Error as IoError;
use std::os::unix::net::UnixStream as StdUnixStream;
use std::os::unix::prelude::FromRawFd;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::UnixStream;

use super::*;

pub struct UnixConnection {
    connection: UnixStream,
}

pub const DEFAULT_BASE_PATH: &str = "/tmp/.X11-unix/X";

impl UnixConnection {
    pub async fn connect(display: u16) -> Result<Self> {
        Self::connect_path(DEFAULT_BASE_PATH, display).await
    }

    pub async fn connect_path(path: &str, display: u16) -> Result<Self> {
        let path = format!("{}{}", path, display);
        if let Ok(connection) = Self::connect_abstract(&*path).await {
            Ok(connection)
        } else {
            Self::connect_unix(&*path).await
        }
    }

    async fn connect_unix(path: &str) -> Result<Self> {
        let socket = UnixStream::connect(path).await?;
        Ok(UnixConnection { connection: socket })
    }

    async fn connect_abstract(raw_path: &str) -> Result<Self> {
        // let mut abstract_stream = StdUnixStream::
        let socket =
            unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0) };
        //  | libc::SOCK_NONBLOCK
        if socket < 0 {
            return Err(IoError::last_os_error().into());
        }
        let path = format!("\0{}", raw_path);

        tokio::task::spawn_blocking(move || -> Result<()> {
            let (sockaddr, len) = unsafe { sockaddr_un(&*path)? };
            let ret = unsafe {
                libc::connect(
                    socket,
                    &sockaddr as *const libc::sockaddr_un as *const libc::sockaddr,
                    len,
                )
            };
            if ret < 0 {
                return Err(IoError::last_os_error().into());
            }
            Ok(())
        })
        .await??;
        let ret = unsafe { libc::fcntl(socket, libc::F_SETFL, libc::O_NONBLOCK) };
        if ret < 0 {
            return Err(IoError::last_os_error().into());
        }

        let socket = unsafe { StdUnixStream::from_raw_fd(socket) };
        let socket = UnixStream::from_std(socket)?;

        info!("X11 connected at {}", raw_path);

        Ok(UnixConnection { connection: socket })
    }

    pub fn into_split(
        self,
    ) -> (
        impl AsyncRead + Unpin + Send + Sync + 'static,
        impl AsyncWrite + Unpin + Send + Sync + 'static,
    ) {
        self.connection.into_split()
    }
}

// copied from stdlib
unsafe fn sockaddr_un(path: &str) -> std::io::Result<(libc::sockaddr_un, libc::socklen_t)> {
    let mut addr: libc::sockaddr_un = std::mem::zeroed();
    addr.sun_family = libc::AF_UNIX as libc::sa_family_t;

    let bytes = path.as_bytes();

    // we are allowing NUL bytes for abstract socket
    // if bytes.contains(&0) {
    //     return Err(IoError::new(
    //         std::io::ErrorKind::InvalidInput,
    //         anyhow!("paths must not contain interior null bytes"),
    //     ));
    // }

    if bytes.len() >= addr.sun_path.len() {
        return Err(IoError::new(
            std::io::ErrorKind::InvalidInput,
            anyhow!("path must be shorter than SUN_LEN"),
        ));
    }
    for (dst, src) in addr.sun_path.iter_mut().zip(bytes.iter()) {
        *dst = *src as libc::c_char;
    }
    // null byte for pathname addresses is already there because we zeroed the
    // struct

    let mut len = sun_path_offset(&addr) + bytes.len();
    match bytes.get(0) {
        Some(&0) | None => {}
        Some(_) => len += 1,
    }
    Ok((addr, len as libc::socklen_t))
}

fn sun_path_offset(addr: &libc::sockaddr_un) -> usize {
    // Work with an actual instance of the type since using a null pointer is UB
    let base = addr as *const _ as usize;
    let path = &addr.sun_path as *const _ as usize;
    path - base
}
