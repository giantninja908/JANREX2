use super::Gamestate;
use futures_util::StreamExt;
use std::io::Cursor;
use std::io::BufReader;
use messagepack_rs::{deserializable::Deserializable, value::Value};

impl Gamestate {
    pub async fn parse_network(&mut self) {
        if let Some(msg) = self.socket.read_stream.next().await {
            if let Ok(msg) = msg {
                if msg.is_binary() {
                    let msg = Value::deserialize(&mut BufReader::new(Cursor::new(msg.into_data())))
                        .unwrap();

                    match msg {
                        Value::Array(mes) => {
                            println!("{:?}, {:?}", mes, mes[0]);
                            if mes[0] == Value::from("pi") {
                                self.socket
                                    .stream_writer
                                    .send(Value::from(vec![Value::from("po")]))
                                    .await;
                                println!("PONG")
                            } else if mes[0] == Value::from("load") {
                                self.socket
                                    .stream_writer
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
                            } else if mes[0] == Value::from("ready") {
                                self.socket
                                    .stream_writer
                                    .send(Value::from(vec![
                                        Value::from("sb"),
                                        Value::from("welc"),
                                        Value::Nil,
                                    ]))
                                    .await;
                            }
                        }
                        _ => {
                            println!("ERROR! NON ARRAY GIVEN")
                        }
                    }
                } else {
                    println!("EE")
                }
            }
        }
    }
}
