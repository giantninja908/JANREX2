
// Urls

/// generates a new token which is required for seeking a new game
pub const GENERATE_TOKEN_URL: &'static str = "https://matchmaker.krunker.io/generate-token";
/// seeking a game using the hashed token from generate token
pub const SEEK_GAME_URL:      &'static str = "https://matchmaker.krunker.io/seek-game";

/// [unofficial link] this is the base url required for all the api endpoints below (keep in mind this can at any time stop working since the dev of it retired)
pub const UNOFFICIAL_API_BASE_URL: &'static str = "https://api.sys32.dev";
// pub const UNOFFICIAL_API_BASE_URL: &'static str = "http://127.0.0.1:7300";

/// [unofficial link] hashes the token from "generate-token" to make it usable in the "seek-game" request
pub const HASH_TOKEN_ENDPOINT:          &'static str = "/v3/token";
/// [unofficial link] gets the build, time, md5(game, min) values
pub const GAME_STATS_ENDPOINT:          &'static str = "/v3/stats";
/// [unofficial link] gets the source code of the game
pub const GAME_SOURCE_ENDPOINT:         &'static str = "/v3/source";
/// [unofficial link] gets the source code of the game prettified | this was used to get [./data/krunker_source_code_prettified.js]
pub const GAME_SOURCE_PRETTY_ENDPOINT:  &'static str = "/v3/source.pretty";
/// [unofficial link] gets the client_key even though it changes rarely or never
pub const CLIENT_KEY_ENDPOINT:          &'static str = "/v3/key";