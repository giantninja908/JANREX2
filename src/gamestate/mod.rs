mod maps;
mod net;
mod render;
mod update;
use crate::{gui, packet_sender::PacketSender, token_fetch};
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
        match a.get(1) {
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

impl ::std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            self.minutes,
            if self.seconds > 9 {
                format!("{}", self.seconds)
            } else {
                format!("0{}", self.seconds)
            }
        )
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

enum ActiveMenu {
    MainMenu,
    InGame,
}

struct GameMenus {
    main_menu: gui::GuiElement,
    in_game: gui::GuiElement,
    active: ActiveMenu,
}

pub struct Gamestate {
    messages: Vec<Message>,
    players: Vec<Player>,
    time: Time,
    socket: SocketData,
    code: String,
    welc_msg: String,
    menus: GameMenus,
    window_size: Vector2,
    map: maps::Map,
}

impl Gamestate {
    pub async fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
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
        let map_dat = maps::map::from_index(0);
        println!("{}", map_dat);
        let map = maps::Map::from_map_text(map_dat, rl, thread).unwrap();
        println!("{:?}", map);

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
            code: webinfo.gameId,
            welc_msg: String::new(),
            menus: GameMenus {
                main_menu: gui::GuiElement::main_menu(),
                in_game: gui::GuiElement::ingame_menu(),
                active: ActiveMenu::MainMenu,
            },
            window_size: Vector2::new(1280.0, 720.0),
            map,
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
