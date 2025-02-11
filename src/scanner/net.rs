use std::io::Error;

//use tokio::net::TcpStream;
use tokio;

// to ping a port of a host. it is an asynchronous function
pub async fn tcp_ping(host: &str, port: u16) -> Result<bool, Error> {
    let address = (host, port);
    match tokio::net::lookup_host(address).await {
        Ok(boolean) => Ok(boolean),
        Err(e) => Err(e),
    }
}
