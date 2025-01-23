use tokio::net::TcpStream;

// to ping a port of a host. it is an asynchronous function
pub async fn tcp_ping(host: &str, port: u16) -> bool {
    let address = (host, port);
    match TcpStream::connect(address).await {
        Ok(_) => true,
        Err(_) => false,
    }
}
