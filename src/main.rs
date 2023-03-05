pub mod aggregator;
pub mod spotify_handler;

use aggregator::aggregator::Playlist;
use dotenv::dotenv;
use spotify_handler::spotify_helper::SpotifyHandler;

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
    let song_titles = match gather_results().await {
        Ok(t) => t,
        Err(..) => panic!("encountered an error")
    };

    for x in song_titles {
        let t = spotify_handler.get_track(&x).await;
        dbg!(t);
    }
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

pub async fn gather_results() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut collect_data = true;
    let mut next_page_token = "".to_string();
    let mut titles: Vec<String> = vec![];
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

        for i in res.items {
            titles.push(i.snippet.title);
        }
        // if we have another page token, save bind it, and keep gathering results
        next_page_token = res.next_page_token.unwrap_or("".to_string());

        if next_page_token == "".to_string() {
            collect_data = false;
        }
    }
    return Ok(titles);
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

