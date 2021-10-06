use messagepack_rs::serializable::Serializable;
use regex::Regex;
use reqwest;

/// retrieves the initial key used in anti hack bit checks from krunker social
pub async fn get_init_key() -> Result<u32, Box<dyn std::error::Error + Sync + Send>> {
    let init_key_regex: Regex = Regex::new(r"\['exports'\]=JSON\['parse'\]\('(\d+)'\)").unwrap();

    let body = reqwest::get("https://krunker.io/social.html")
        .await?
        .text()
        .await?;

    let cap = init_key_regex.captures(&body).unwrap();
    Ok(cap.get(1).unwrap().as_str().parse::<u32>().unwrap())
}

/// struct representation of rotating hack bit and hack bit key
pub struct KeyRotator {
    ah_num: u8,
    ah_k: u32,
}

impl KeyRotator {
    /// returns a new KeyRotator
    pub async fn new() -> Self {
        let init_key = get_init_key().await.unwrap();
        KeyRotator {
            ah_num: 0,
            ah_k: init_key,
        }
    }

    /// appends 2 hack bits to an already existing stream of sendable data
    pub fn encode_network_msg(&mut self, msg: Vec<u8>) -> Vec<u8> {
        self.ah_num = KeyRotator::rotate_number(self.ah_num, self.ah_k);
        let mut ret = msg;
        let mut add = KeyRotator::encode_short(self.ah_num);
        ret.append(&mut add);
        ret
    }

    /// shorthand to encode a Value into an encoded network message
    pub fn encode_network_msg_from_val(&mut self, val: messagepack_rs::value::Value) -> Vec<u8> {
        self.encode_network_msg(val.serialize().unwrap())
    }

    /// limit to a 2 bit short that doesn't exceed 256 to append
    fn encode_short(num: u8) -> Vec<u8> {
        vec![(num >> 4) & 0xF, num & 0xF]
    }

    /// rotation algorithm using prime
    fn rotate_number(num: u8, prime: u32) -> u8 {
        ((num as u32 + prime) & 0xFF) as u8
    }
}
