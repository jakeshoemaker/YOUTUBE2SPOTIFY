pub mod aggregator;
pub mod spotify_handler;
pub mod helpers;

use aggregator::aggregator::Playlist;
use clap::{Arg, ArgAction, Command};
use dotenv::dotenv;
use helpers::helpers::get_user_input;
use spotify_handler::spotify_helper::SpotifyHandler;
use indicatif::ProgressBar;

// todo: progress bar? setup endpoint to handle callback? allow option for playlist image in sptfy
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Command::new("YOUTUBE2SPOTIFY")
        .version("0.69")
        .author("jake s. <jakeshoe3@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::new("link").short('l').long("link").required(true)
            .help("the link to the youtube playlist you want converted"))
        .arg(Arg::new("playlist_name").short('n').long("name").required(true)
            .help("the title of the playlist"))
        .arg(Arg::new("init").short('i').long("init").required(false)
            .action(ArgAction::SetTrue)
            .help("run the initialization needed to work"))
        .get_matches();

    // handle init if flagged
    if args.get_flag("init") == true {
        let input = get_user_input("gimme something good bro".to_string());
        println!("{}: this is the crap you give me? smh", input);
        panic!("wasn't good enough, bye!");
    }
    
    // IK im panincing rn, dont need to actually build the playlist just wanna short circuit to
    // test
    panic!("bye bish");

    // todo: spotify_handler.init()
    // load env
    // todo: remove for now___
    #[warn(unreachable_code)]
    dotenv().ok();
    let mut spotify_handler = SpotifyHandler::new().await;
    let song_titles = match gather_results().await {
        Ok(t) => t,
        Err(..) => panic!("encountered an error")
    };
 
    println!("gathering tracks from youtube");
    let pg = ProgressBar::new(song_titles.len() as u64);
    for x in song_titles {
        spotify_handler.search_for_track(&x).await; // adds tracks to list
        pg.inc(1);
    }
    pg.finish();

    // create the playlist using track id's from search
    spotify_handler.create_user_playlist().await;

    // once we have titles in handler add them to playlist
    let res = spotify_handler.add_tracks_to_playlist()
        .await;    

    println!("success! playlist created!!");
    dbg!(&res);

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

