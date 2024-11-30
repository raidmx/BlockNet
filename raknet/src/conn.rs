use std::net::SocketAddr;
use std::sync::Arc;
use bytes::BytesMut;
use tokio::net::UdpSocket;

pub struct RakConn {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub socket: Arc<UdpSocket>
}

impl RakConn {
    pub fn new(local_addr: SocketAddr, remote_addr: SocketAddr) -> Self {
        let udp_sock = socket2::Socket::new(
            if local_addr.is_ipv4() {
                socket2::Domain::IPV4
            } else {
                socket2::Domain::IPV6
            },
            socket2::Type::DGRAM,
            None,
        ).unwrap();

        udp_sock.set_reuse_port(true).unwrap();
        udp_sock.set_cloexec(true).unwrap();
        udp_sock.set_nonblocking(true).unwrap();
        udp_sock.bind(&socket2::SockAddr::from(local_addr)).unwrap();

        let udp_sock: std::net::UdpSocket = udp_sock.into();
        udp_sock.connect(remote_addr).expect("Cannot connect the UdpSocket to remote addr");

        let socket: UdpSocket = udp_sock.try_into().unwrap();

        Self {
            local_addr,
            remote_addr,
            socket: socket.into()
        }
    }

    pub fn start(&self) {
        let remote_addr = self.remote_addr.clone();
        let socket = self.socket.clone();

        tokio::spawn(async move {
            let mut incm = BytesMut::new();
            let mut outg = BytesMut::new();

            loop {
                incm.resize(1500, 0);
                outg.clear();

                let len = match socket.recv(&mut incm).await {
                    Ok(v) => v,
                    Err(_) => continue
                };
                
                println!("Received {} bytes from {}", len, remote_addr)
            }
        });
    }
}