pub mod aggregator;

use std::fmt::Formatter;

use aggregator::aggregator::Playlist;
use dotenv::dotenv;
use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // load env
    dotenv().ok();
    /*
    let spotify = setup_spotify().await;

    // Running the requests
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");
    */
    // set up yt 
    let playlist_id = "PLm323Lc7iSW9oSIDihesMJXmMNfh8U59k";
    let api_key = dotenv::var("YOUTUBE_API_KEY").unwrap();
    let playlist_url = format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}&maxResults=50",
        playlist_id, api_key
    );
    let client = reqwest::Client::new();
    let response = client.get(&playlist_url)
        .send()
        .await?
        .json::<Playlist>()
        .await?;

    for item in response.items {
        println!("Title: {}", item.snippet.title);
        //println!("Description: {}", item.snippet.description);
    }

    let next_page_token = response.next_page_token.unwrap_or("skip".to_string());
    
    Ok(())
}

pub async fn gather_results() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut collect_data = true;
    let mut next_page_token = "";
    let mut yt_results: Vec<String> = Vec::new(); 
    let client = reqwest::Client::new();
    let mut res;
    
    // as long as we keep getting a next_page, gather results and stuff titles into list
    while collect_data {
        // build the youtube call
        let url = build_youtube_url(
            dotenv::var("PLAYLIST_ID").unwrap(), 
            dotenv::var("YOUTUBE_API_KEY").unwrap(),
            next_page_token.to_string());
        res = client.get(&url)
            .send()
            .await?
            .json::<Playlist>()
            .await?;

        for i in res.items {
            yt_results.push(i.snippet.title);
        };
        // whyy it drop valueeeeee
        next_page_token = &res.next_page_token.unwrap_or("skip".to_string());
    }

    return Ok(yt_results);
}

pub fn build_youtube_url(playlist_id: String, api_key: String, next_page_token: String) -> String {
    let mut query = format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}&maxResults=50",
        playlist_id, api_key
    );
    if next_page_token != "skip" || next_page_token != "" {
        query.push_str(&format!("&pageToken={}", next_page_token));
    };
    return query;
}

pub async fn setup_spotify() -> AuthCodeSpotify {

    let creds: Credentials = Credentials::from_env().unwrap();
    let oauth: OAuth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
    let spotify: AuthCodeSpotify = AuthCodeSpotify::new(creds, oauth);
    //  obtain access token, and allow spotify to get a token itself
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    println!("token got");
    return spotify;
}

