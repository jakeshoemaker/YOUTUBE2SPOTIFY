pub mod aggregator;

use std::fmt::Formatter;

use aggregator::aggregator::Playlist;
use dotenv::dotenv;
use rspotify::{
    model::{SearchType, Country, Market, SearchResult},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // load env
    dotenv().ok();
    let spotify = setup_spotify().await;
    let market = Market::Country(Country::UnitedStates);
    /*
    TODO: 
      1. grab spotify' user id
      2. build call to make playlist
      3. build call to search for song, 
      4. if found, add to playlist

    // Running the requests
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");
    */
    let song_titles = match gather_results().await {
        Ok(t) => t,
        Err(..) => panic!("encountered an error")
    };

    for t in song_titles {
        // search spotify to see if track exists
        let potential_track = spotify.search(&t, SearchType::Track, Some(market), None, None, None).await.unwrap();
        let tracks = match potential_track {
            SearchResult::Tracks(t) => t.items,
            _ => todo!()
        };
        // todo: do something with search results
        // need to come  up with a way that actually selects the correct song we want
        // any way to validate other than title? 
    };

    Ok(())
}

pub async fn gather_results() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut collect_data = true;
    let mut next_page_token = "".to_string();
    let mut yt_results: Vec<String> = Vec::new(); 
    let client = reqwest::Client::new();
    let mut res;
    
    // as long as we keep getting a next_page, gather results and stuff titles into list
    while collect_data {
        // build the youtube call
        let url = build_youtube_url(
            dotenv::var("PLAYLIST_ID").unwrap(), 
            dotenv::var("YOUTUBE_API_KEY").unwrap(),
            next_page_token);
        res = client.get(&url)
            .send()
            .await?
            .json::<Playlist>()
            .await?;

        for i in res.items {
            yt_results.push(i.snippet.title);
        };
        // if we have another page token, save bind it, and keep gathering results
        next_page_token = match res.next_page_token {
            Some(t) => t.to_string(),
            None => "".to_string()
        };
        if next_page_token == "".to_string() {
            collect_data = false;
        }
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
    let url = match spotify.get_authorize_url(false) {
        Ok(t) => t,
        Err(e) => panic!("error getting auth url: {}", e)
    };
    
    match spotify.prompt_for_token(&url).await {
        Ok(t) => t,
        Err(e) => panic!("error getting token: {}", e)
    };

    return spotify;
}

