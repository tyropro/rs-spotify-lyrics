# Rust Spotify Lyrics Finder

This is a simple project that uses the Spotify API to grab your currently playing song and display its lyrics to the console.

## Usage

1. Clone the repository: `git clone https://github.com/tyropro/rs-spotify-lyrics.git`
2. Go to the [Spotify Developer Portal](https://developers.spotify.com/dashboard). If you aren't logged in, log in and click the link again
3. Create a new app and add `http://localhost:8080` as a redirect URI
4. Rename the `.env.example` file to `.env` and copy the Client ID and Client Secret into it
5. Run the app: `cargo run --release`
6. Follow the instructions in the CLI. You will be redirected a Spotify Login Page, login and authorise the app (this is the one you made). You will then be redirected to a web page that should be blank. Copy the URL and paste it into the CLI. You will only have to do this once as your token is cached

## Credits

- [RSpotify](https://github.com/ramsayleung/rspotify): A full-featured Spotify API wrapper for Rust
- [Astridlol's Lyrics](https://github.com/astridlol/lyrics): A really easy and free to use API for getting song lyrics without an API key.

## License

This project is licensed under the [MIT License](LICENSE).
