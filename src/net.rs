use std::net::{ToSocketAddrs, TcpStream, SocketAddr};
use std::time::Duration;

use openssl::ssl::{SslMethod, SslConnector, HandshakeError, SslStream};

pub fn build_ssl_stream(hostname: &str, ip_addr: SocketAddr) -> Result<SslStream<TcpStream>, HandshakeError<TcpStream>> {
    let ssl_stream = TcpStream::connect_timeout(&ip_addr, Duration::from_millis(5000)).unwrap();
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    match connector.connect(hostname, ssl_stream){
        Ok(stream) => Ok(stream),
        Err(error) => Err(error)
    }
}
