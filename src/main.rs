// use random_string::generate;
use rspotify::{model::{AdditionalType, PlayableItem}, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, Config};

// const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const LYRICS_API: &str = "https://lyrics.astrid.sh/api/search/";


#[tokio::main]
async fn main() {
    let creds: Credentials = Credentials::from_env().unwrap();

    let scopes: std::collections::HashSet<String> = scopes!(
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-recently-played",
        "user-read-private"
    );
    let oauth: OAuth = OAuth::from_env(scopes).unwrap();
    
    let config: Config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let spotify: AuthCodeSpotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url: String = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let currently_playing: rspotify::model::CurrentlyPlayingContext = spotify
        .current_playing(None, Some(vec![&AdditionalType::Track]))
        .await
        .unwrap()
        .unwrap();

    let content: PlayableItem = currently_playing.item.unwrap();

    let track: rspotify::model::FullTrack = match content {
        PlayableItem::Track(track) => track,
        _ => panic!("Not a track"),
    };

    let track_name: &String = &track.name;
    let artist_name: &String = &track.artists[0].name;
    let query: String = format!("{} {}", track_name, artist_name);

    let lyrics_url: String = url::Url::parse_with_params(LYRICS_API, [("q", query)]).unwrap().to_string();
    let request: ureq::Request = ureq::get(&lyrics_url);
    let response: ureq::Response = request.call().unwrap();

    let json_reponse: serde_json::Value = response.into_json().unwrap();

    let lyrics: &str = json_reponse["lyrics"].as_str().unwrap();

    println!("{}", lyrics);
}
