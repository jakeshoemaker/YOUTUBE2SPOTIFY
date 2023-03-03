pub mod aggregator;
pub mod spotify_handler;

use aggregator::aggregator::Playlist;
use aggregator::aggregator::Item;
use dotenv::dotenv;
use rspotify::{
    model::{SearchType, Country, Market, SearchResult},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};

use crate::spotify_handler::spotify_helper::SpotifyHandler;

// todo: there has to be a better way of authenticating spotify

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // load env
    dotenv().ok();
    let spotify_handler = SpotifyHandler::new().await;
    spotify_handler.test_user_collected();
    /*
    TODO: 
      1. build call to search for song, 
      2. if found, add to playlist
    */
    let search_criteria = match gather_results().await {
        Ok(t) => t,
        Err(..) => panic!("encountered an error")
    };

    let _titles = search_criteria.first().unwrap();
    let _artists = search_criteria.last().unwrap();
    
    // search spotify to see if track exists
    // TODO: need to make a new method that searchs / returns a song ID to add to a playlist

    /*
        let split_title = test_track_name2.split("-").collect::<Vec<_>>();
        dbg!(&split_title);

        let search_items = match potential_track {
            SearchResult::Tracks(t) => t.items,
            _ => todo!()
        };
        dbg!(&search_items);
    */
    // example of creating a playlist:
    //spotify.user_playlist_create(h, name, j, collaborative, description)
    // example of adding a item to playlist
    //spotify.playlist_add_items(playlist_id, items, position)
    Ok(())
}

pub async fn gather_results() -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut collect_data = true;
    let mut next_page_token = "".to_string();
    let mut titles: Vec<String> = vec![];
    let mut artists: Vec<String> = vec![];
    let mut res;

    let client = reqwest::Client::new();
    
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

        //dbg!(&res);
        for i in res.items {
            let mut substr = i.snippet.title.split("-");
            if substr.to_owned().count().eq(&1) {
                // if split size is one, then this is a song
                titles.push(substr
                    .next()
                    .unwrap()
                    .to_string()
                );
                // push the channel title as the artist
                artists.push(i.snippet.video_owner_channel_title);
            } else {
                titles.push(substr
                    .next()
                    .unwrap()
                    .to_string()
                );
                artists.push(substr
                    .next()
                    .unwrap()
                    .to_string()
                );
            }
        }
        // if we have another page token, save bind it, and keep gathering results
        next_page_token = res.next_page_token.unwrap_or("".to_string());

        if next_page_token == "".to_string() {
            collect_data = false;
        }
    }

    // ok now that we have song titles.. when it comes to searching, we are using the title as follows:
    //   1. if the title has a x [-] blah then we treat the minus as the separator: author | song
    //   2. if no - is supplied, we treat this as a song, and use the youtube channel name as the
    //     author. this is because people would most likely be copyrighted to play / take a song
    //     without giving credit
    dbg!(&titles, &artists);
    return Ok(vec![titles, artists]);
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

