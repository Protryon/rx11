use tokio::{net::TcpStream, io::{AsyncRead, AsyncWrite}};
use super::*;

pub struct TcpConnection {
    connection: TcpStream,
}

pub const X_TCP_PORT: u16 = 6000;

impl TcpConnection {
    pub async fn connect(host: &str, display: u16) -> Result<Self> {
        let socket = TcpStream::connect((host, X_TCP_PORT + display)).await?;
        info!("X11 connected at {}:{}", host, X_TCP_PORT + display);
        Ok(TcpConnection {
            connection: socket,
        })
    }

    pub fn into_split(self) -> (impl AsyncRead + Unpin + Send + Sync + 'static, impl AsyncWrite + Unpin + Send + Sync + 'static) {
        self.connection.into_split()
    }
}