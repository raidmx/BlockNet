use std::net::SocketAddr;
use std::str::FromStr;
use bytes::BytesMut;
use tokio::net::UdpSocket;
use binary::{b64, Decode};
use rand::random;
use crate::conn::RakConn;
use crate::packet::{OpenConnectionReply1, OpenConnectionReply2, OpenConnectionRequest1, OpenConnectionRequest2, Packet, PacketId, UnconnectedPing, UnconnectedPong};
use crate::types::Magic;

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

        let guid = b64::new(random());
        let local_addr = self.addr.clone();

        tokio::spawn(async move {
            let mut incm = BytesMut::new();
            let mut outg = BytesMut::new();

            loop {
                incm.resize(1500, 0);
                outg.clear();

                let (len, addr) = match socket.recv_from(&mut incm).await {
                    Ok(v) => v,
                    Err(_) => continue
                };

                let r = &mut &incm[..len];

                println!("Received #1 {:?} bytes from {}", r, addr);

                let id = PacketId::decode(r).unwrap();

                if id == PacketId::UnconnectedPing {
                    let pk = UnconnectedPing::read(r).unwrap();
                    println!("{:?} sent {} bytes => {:?}", addr, len, pk);

                    let pk = UnconnectedPong {
                        pong_time: pk.ping_time,
                        guid: guid.clone(),
                        magic: Magic,
                        data: "MCPE;Dedicated Server;390;1.14.60;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".into(),
                    };

                    pk.write(&mut outg);
                    socket.send_to(&outg[..], addr).await.unwrap();
                }

                if id == PacketId::OpenConnectionRequest1 {
                    let pk = OpenConnectionRequest1::read(r).unwrap();
                    println!("{:?} sent {} bytes => {:?}", addr, len, pk);

                    let pk = OpenConnectionReply1 {
                        magic: Magic,
                        guid: guid.clone(),
                        secure: false,
                        mtu: 1492.into(),
                    };

                    pk.write(&mut outg);
                    socket.send_to(&outg[..], addr).await.unwrap();
                }

                if id == PacketId::OpenConnectionRequest2 {
                    let pk = OpenConnectionRequest2::read(r).unwrap();
                    println!("{:?} sent {} bytes => {:?}", addr, len, pk);

                    let pk = OpenConnectionReply2 {
                        magic: Magic,
                        guid: guid.clone(),
                        addr,
                        mtu: pk.mtu,
                        secure: false,
                    };

                    pk.write(&mut outg);
                    socket.send_to(&outg[..], addr).await.unwrap();

                    let conn = RakConn::new(local_addr, addr);
                    conn.start();
                }
            }
        });
    }
}