use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("{}", track.name);
        println!("{}", track.album.name);
        println!(
            "{}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("{}", track.external_urls.spotify);
        println!("--------------------------------");
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: spotify-search <query> <auth_token>");
        return;
    }
    let search_query = &args[1];
    let auth_token = &args[2];
    // https://developer.spotify.com/documentation/web-api/reference/search?q=Muse&type=track&market=US&limit=5&offset=5&include_external=
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=artist,album,track",
        query = search_query
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<APIResponse>().await {
            Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
            Err(e) => println!("the response didn't match the struct Error: {}", e),
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new access token")
        }
        _ => println!("something went wrong"),
    }
}
