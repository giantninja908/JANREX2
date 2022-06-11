use reqwest;
use serde::{Deserialize, Serialize};
use urlencoding::encode;
use super::consts;
/// fetches client key used for token generation
/// fails in the event it cannot get data
async fn client_key() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let body = reqwest::get(format!("{}{}", consts::UNOFFICIAL_API_BASE_URL, consts::CLIENT_KEY_ENDPOINT))
        .await?
        .text()
        .await?;

    Ok(body)
}

/// a struct to hold a token data
#[derive(Deserialize, Serialize, Debug)]
struct MatchmakerToken {
    token: String,
    cfid: u32,
    sid: u32,
}

/// retrieves the matchmaker token
/// returns it as a struct representation of the json
/// errors on inability to get data
async fn matchmaker_token(
    client_key: &String,
) -> Result<MatchmakerToken, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let body = client
        .get(consts::GENERATE_TOKEN_URL)
        .header("client-key", client_key)
        .send()
        .await?
        .json::<MatchmakerToken>()
        .await?;

    Ok(body)
}

type HashReturn = Vec<u32>;

/// hashes a token
/// krunker requests require a hashed token
/// the alg is not yet publicly known
async fn hask_token(
    token: &MatchmakerToken,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let body = client
        .post(format!("{}{}", consts::UNOFFICIAL_API_BASE_URL, consts::HASH_TOKEN_ENDPOINT))
        .header("Content-Type", "application/json")
        .json(&token)
        .send()
        .await?
        .json::<HashReturn>()
        .await?;

    let b = body
        .iter()
        .map(|e| std::primitive::char::from_u32(*e).unwrap())
        .collect::<String>();

    Ok(b)
}

/// fetches the token argument asyncronously
/// to be used in matchmaking
/// will panic! if an error occurred
pub async fn token_arg() -> String {
    println!("fetching client key...");
    let client_key = client_key().await.unwrap();
    println!("fetching token key...");
    let token = matchmaker_token(&client_key).await.unwrap();
    println!("hashing key...");
    hask_token(&token).await.unwrap()
}

/// representation of data returned from websocket
/// includes potential reason for change
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WebsocketReturnData {
    pub clientId: String,
    pub gameId: String,
    pub host: String,
    pub port: u32,
    pub changeReason: Option<String>,
}

/// gets information about the websocket
/// uses krunker matchmaking to seek a game
/// currently assumes NA East for requirest region
pub async fn get_websocket_info(
    token: &String,
) -> Result<WebsocketReturnData, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let body = client
        .get(format!("{}?hostname=krunker.io&region=us-nj&autoChangeGame=false&validationToken={}", consts::SEEK_GAME_URL, encode(token)))
        .header("Origin", "https://krunker.io")
        // .header("region", "us-nj")
        // .header("autoChangeGame", "false")
        // .header("validationToken", token)
        // .header("dataQuery", "%7B%22v%22%3A%22M9UCk%22%7D")
        .header("accept-language","en-US,en;q=0.6;")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.0 Safari/537.36 Edg/85.0.564.0")
        .send()
        .await?
        // .text()
        .json::<WebsocketReturnData>()
        .await?;
    Ok(body)
}
