use std::net::SocketAddr;
use std::str::FromStr;
use bytes::BytesMut;
use tokio::net::UdpSocket;

pub struct RakListener {
    pub addr: SocketAddr,
}

impl RakListener {
    pub fn new(bind: &str) -> Self {
        let addr = SocketAddr::from_str(bind).expect("Cannot parse SocketAddr from the specified bind address.");

        Self {
            addr,
        }
    }

    pub fn listen(&self) {
        let udp_sock = socket2::Socket::new(
            if self.addr.is_ipv4() {
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
        udp_sock.bind(&socket2::SockAddr::from(self.addr)).unwrap();

        let udp_sock: std::net::UdpSocket = udp_sock.into();
        let socket: UdpSocket = udp_sock.try_into().unwrap();

        tokio::spawn(async move {
            let mut bytes = BytesMut::zeroed(1500);

            loop {
                if let Ok((len, addr)) = socket.recv_from(&mut bytes).await {
                    println!("Received {} bytes from {:?}", len, addr);
                } else {
                    println!("Error received");
                }
            }
        });
    }
}