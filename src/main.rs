mod gamestate;
mod key_rotate;
mod token_fetch;
use futures_util::SinkExt;
use messagepack_rs::value::Value;
pub(crate) use raylib::prelude::*;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let (mut rl, thread) = raylib::init()
        .msaa_4x()
        .size(500, 500)
        .title("JANREX 2")
        .build();

    rl.set_target_fps(120);
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("LOADING", 10, 10, 50, Color::WHITE);
    }

    let mut gamestate = gamestate::Gamestate::new().await;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("SOON", 10, 10, 50, Color::WHITE);
        gamestate.parse_network().await;
    }

    // pin_mut!(stdin_to_ws, ws_to_stdout);
    // future::select(stdin_to_ws, ws_to_stdout).await;
}

type StreamWriter = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;

pub struct PacketSender {
    keyr: key_rotate::KeyRotator,
    writer: StreamWriter,
}

impl PacketSender {
    pub async fn new(writer: StreamWriter) -> Self {
        Self {
            keyr: key_rotate::KeyRotator::new().await,
            writer,
        }
    }
    pub async fn send(&mut self, msg: Value) {
        match self
            .writer
            .send(Message::Binary(self.keyr.encode_network_msg_from_val(msg)))
            .await
        {
            Ok(_) => {}
            Err(res) => {
                println!("!!{:?}", res)
            }
        }
    }
}
