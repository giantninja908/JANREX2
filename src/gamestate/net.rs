use super::{Gamestate, Message, Time};
use futures_util::StreamExt;
use messagepack_rs::{deserializable::Deserializable, value::Value};
use std::io::BufReader;
use std::io::Cursor;

macro_rules! send {
    ($s:expr, $x:expr) => {
        $s.socket.stream_writer.send($x).await;
    };
}

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
                                send!(self, Value::from(vec![Value::from("po")]));
                                println!("PONG")
                            } else if mes[0] == Value::from("load") {
                                send!(self, Value::from(vec![Value::from("load"), Value::Nil]));
                                println!("LOAD")
                            } else if mes[0] == Value::from("ch") {
                                println!(
                                    "\n\nCHAT MESSAGE\n{}\n\n",
                                    match &mes[3] {
                                        Value::String(msg) => msg,
                                        _ => "undefined",
                                    }
                                );
                                self.messages.push(Message {
                                    content: match &mes[3] {
                                        Value::String(msg) => msg.to_string(),
                                        _ => String::from("undefined"),
                                    },
                                    sender: match &mes[2] {
                                        Value::String(auth) => Some(auth.to_string()),
                                        _ => None,
                                    },
                                });
                            } else if mes[0] == Value::from("ready") {
                                send!(
                                    self,
                                    Value::from(vec![
                                        Value::from("sb"),
                                        Value::from("welc"),
                                        Value::Nil,
                                    ])
                                );
                            } else if mes[0] == Value::from("t") {
                                if let Value::String(s) = &mes[1] {
                                    self.time = Time::from(s.to_string());
                                }
                            } else if mes[0] == Value::from("inst-id") {
                                if let Value::String(s) = &mes[1] {
                                    self.code = s.to_string();
                                }
                            } else if mes[0] == Value::from("sb") {
                                if let Value::String(s) = &mes[1] {
                                    self.welc_msg = s.to_string();
                                }
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
