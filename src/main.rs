use rspotify::{model::{AdditionalType, PlayableItem}, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, Config};

const LYRICS_API: &str = "https://lyrics.astrid.sh/api/search/";


#[tokio::main]
async fn main() {
    // get client credentials from `.env` file
    let creds: Credentials = Credentials::from_env().unwrap();

    let scopes: std::collections::HashSet<String> = scopes!( // scopes for application
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-recently-played",
        "user-read-private"
    );
    // get redirect uri from `.env` file and set scopes
    let oauth: OAuth = OAuth::from_env(scopes).unwrap();
    
    // set Config options (for token caching and refreshing)
    let config: Config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    // create Spotify client instance with config
    let spotify: AuthCodeSpotify = AuthCodeSpotify::with_config(creds, oauth, config);

    // get authorize url and prompt for token if not cached
    let url: String = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    // get currently playing track
    let currently_playing: rspotify::model::CurrentlyPlayingContext = spotify
        .current_playing(None, Some(vec![&AdditionalType::Track]))
        .await
        .unwrap()
        .unwrap();

    // get track from currently playing track
    let content: PlayableItem = currently_playing.item.unwrap();

    let track: rspotify::model::FullTrack = match content {
        PlayableItem::Track(track) => track,
        _ => panic!("Not a track"),
    };

    // get track name and artist name
    let track_name: &String = &track.name;
    let artist_name: &String = &track.artists[0].name;
    let query: String = format!("{} {}", track_name, artist_name);

    // get lyrics from lyrics.astrid.sh API
    let lyrics_url: String = url::Url::parse_with_params(LYRICS_API, [("q", query)]).unwrap().to_string(); // parse the ur;
    let request: ureq::Request = ureq::get(&lyrics_url); // create the request
    
    let response: ureq::Response = request.call().unwrap(); // fetch the response
    let json_reponse: serde_json::Value = response.into_json().unwrap(); // parse the response as json

    let lyrics: &str = json_reponse["lyrics"].as_str().unwrap(); // get the lyrics from the json response

    // print lyrics
    println!("{}", lyrics);
}
