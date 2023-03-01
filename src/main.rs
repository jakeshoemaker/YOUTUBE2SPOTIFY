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
      1. build call to search for song, 
      2. if found, add to playlist
    */
    let search_criteria = match gather_results().await {
        Ok(t) => t,
        Err(..) => panic!("encountered an error")
    };

    let titles = search_criteria.first().unwrap();
    let artists = search_criteria.last().unwrap();
    
    // search spotify to see if track exists
    // TODO: need to make a new method that searchs / returns a song ID to add to a playlist
    let potential_track = 
        spotify.search(
            "BoTalks - F*ck It (feat. Caroline Pennell)", 
            SearchType::Track, 
            Some(market), 
            None, 
            Some(10), 
            None)
        .await
        .unwrap();
   
    let search_items = match potential_track {
        SearchResult::Tracks(t) => t.items,
        _ => todo!()
    };

    // get self  ( CONTAINS USER ID )
    let user = spotify
        .me()
        .await
        .unwrap();
    dbg!(user);

    // example of creating a playlist:
    //spotify.user_playlist_create(h, name, j, collaborative, description)
    // example of adding a item to playlist
    //spotify.playlist_add_items(playlist_id, items, position)


    /*
    * OK here we go. 
    * search (limit 5)
    *   -> once we get the search results we could do some string comparison against the
    *   artist/track name
    *   like if the spotify search result (either title | artist) exists inside our title + artist
    *   youtube playlist title -> we accept that as a successful search and add to the playlist
    */
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

