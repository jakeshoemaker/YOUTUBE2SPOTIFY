pub mod aggregator;

use aggregator::aggregator::Playlist;
use aggregator::aggregator::Item;
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
    println!("dotenv loaded");
    let spotify = setup_spotify().await;
    println!("spotify setup");
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
    let search_criteria = match gather_results().await {
        Ok(t) => t,
        Err(..) => panic!("encountered an error")
    };

    for vec in search_criteria {
        dbg!(&vec);
        // search spotify to see if track exists
        //let potential_track = spotify.search(&title, SearchType::Track, Some(market), None, None, None).await.unwrap();
        //let tracks = match potential_track {
        //    SearchResult::Tracks(title) => title.items,
        //    _ => todo!()
        //};
        //for i in tracks {
        //    println!("comparing: {} with this spotify result: {}", title, i.name);
        //}
        // todo: do something with search results
        // need to come  up with a way that actually selects the correct song we want
        // any way to validate other than title? 
    };

    Ok(())
}

pub async fn gather_results() -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut collect_data = true;
    let mut next_page_token = "".to_string();
    let mut titles: Vec<String> = vec![];
    let mut artists: Vec<String> = vec![];
    let mut items: Vec<Item> = vec![];
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

        //dbg!(&res);
        for item in res.items {
            items.push(item);    
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
    

    for i in items {
        let mut substr = i.snippet.title.split("-");
        if substr.to_owned().count().eq(&1) {
            // if split size is one, then this is a song
            titles.push(substr
                .next()
                .unwrap()
                .to_string()
            );
            // push the channel title as the artist
            artists.push(i.snippet.channel_title);
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
        dbg!(&substr);
    }            

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

