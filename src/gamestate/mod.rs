mod net;
use crate::{packet_sender::PacketSender, token_fetch};
use futures_util::StreamExt;
use raylib::prelude::*;
use tokio_tungstenite::connect_async;

enum Class {
    Triggerman,
    RunAndGun,
}

struct Player {
    pos: Vector3,
    rot: Quaternion,
    class: Class,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Time {
    minutes: u32,
    seconds: u32,
    milliseconds: u8,
}

impl Time {
    pub fn from(s: String) -> Self {
        let mut v = s.split(":").collect::<Vec<_>>();
        let a = v[1].split(".").collect::<Vec<_>>();
        v[1] = a[0];
        match v.get(1) {
            Some(l) => Self {
                minutes: v[0].parse().unwrap(),
                seconds: v[1].parse().unwrap(),
                milliseconds: l.parse().unwrap(),
            },
            None => Self {
                minutes: v[0].parse().unwrap(),
                seconds: v[1].parse().unwrap(),
                milliseconds: 0,
            },
        }
    }
}

struct SocketData {
    stream_writer: PacketSender,
    read_stream: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
}

pub struct Message {
    sender: Option<String>,
    content: String,
}

pub struct Gamestate {
    messages: Vec<Message>,
    players: Vec<Player>,
    time: Time,
    socket: SocketData,
}

impl Gamestate {
    pub async fn new() -> Self {
        let token = token_fetch::token_arg().await;
        println!("{:?}", token);
        let webinfo = token_fetch::get_websocket_info(&token).await.unwrap();
        println!("{:?}", webinfo);

        println!("connecting...");
        let url = format!(
            "wss://{}/ws?gameId={}&clientKey={}&clientUID=H6McRYmC2HiQSb0KUDBr58",
            webinfo.host, webinfo.gameId, webinfo.clientId
        );

        println!("{}", url);

        let url = http::Request::get(url)
            .header("Origin", "https://krunker.io")
            .body(())
            .unwrap();
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        let stream_writer = PacketSender::new(write).await;

        Self {
            messages: Vec::new(),
            players: Vec::new(),
            time: Time {
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            },
            socket: SocketData {
                read_stream: read,
                stream_writer,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Time;
    #[test]
    fn check() {
        let time = Time::from("5:00.09".to_string());
        assert_eq!(
            time,
            Time {
                minutes: 5,
                seconds: 0,
                milliseconds: 9
            }
        );
    }
}
