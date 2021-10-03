mod gamestate;
mod key_rotate;
mod token_fetch;
use futures_util::{SinkExt, StreamExt};
use http;
use messagepack_rs::{deserializable::Deserializable, serializable::Serializable, value::Value};
use raylib::prelude::*;
use std::io::{BufReader, Cursor};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

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

    let token = token_fetch::token_arg().await;
    println!("{:?}", token);
    let webinfo = token_fetch::get_websocket_info(&token).await.unwrap();
    println!("{:?}", webinfo);

    println!("connecting...");
    let url = format!(
        "wss://{}/ws?gameId={}&clientKey={}&clientUID=H6McRYmC2HiQSb0KUDBr58",
        webinfo.host, webinfo.gameId, webinfo.clientId
    );
    // let url = "https://krunker.io";

    println!("{}", url);

    let url = http::Request::get(url)
        .header("Origin", "https://krunker.io")
        .body(())
        .unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, mut read) = ws_stream.split();

    let mut stream_writer = PacketSender::new(write).await;
    //
    //     let stdin_to_ws = stdin_rx.map(Ok).forward(write);


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("SOON", 10, 10, 50, Color::WHITE);


        if let Some(msg) = read.next().await {
            if let Ok(msg) = msg {
                if msg.is_binary() {
                    let msg = Value::deserialize(&mut BufReader::new(Cursor::new(msg.into_data())))
                        .unwrap();

                    match msg {
                        Value::Array(mes) => {
                            println!("{:?}, {:?}", mes, mes[0]);
                            if mes[0] == Value::from("pi") {
                                stream_writer
                                    .send(Value::from(vec![Value::from("po")]))
                                    .await;
                                println!("PONG")
                            } else if mes[0] == Value::from("load") {
                                stream_writer
                                    .send(Value::from(vec![Value::from("load"), Value::Nil]))
                                    .await;
                                println!("LOAD")
                            } else if mes[0] == Value::from("ch") {
                                println!(
                                    "\n\nCHAT MESSAGE\n{}\n\n",
                                    match &mes[3] {
                                        Value::String(msg) => msg,
                                        _ => "undefined",
                                    }
                                )
                            }
                        }
                        _ => {
                            println!("ERROR! NON ARRAY GIVEN")
                        }
                    }
                } else {
                    println!("EE")
                }
            } else {
                break;
            }
        }
    }

    // pin_mut!(stdin_to_ws, ws_to_stdout);
    // future::select(stdin_to_ws, ws_to_stdout).await;
}

type StreamWriter = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;

struct PacketSender {
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
