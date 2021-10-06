use crate::key_rotate;
use futures_util::SinkExt;
use messagepack_rs::value::Value;
use tokio_tungstenite::tungstenite::protocol::Message;

/// simplification type to represent a splitted sync for writing with
type StreamWriter = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;

/// struct that encodes the anti-hack bit properly and appends on send
pub struct PacketSender {
    keyr: key_rotate::KeyRotator,
    writer: StreamWriter,
}

impl PacketSender {
    /// creates a new PacketSender
    pub async fn new(writer: StreamWriter) -> Self {
        Self {
            keyr: key_rotate::KeyRotator::new().await,
            writer,
        }
    }

    /// encode and send a Value to the internal stream
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
