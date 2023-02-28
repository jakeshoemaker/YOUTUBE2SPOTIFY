pub mod aggregator {
    use serde::Deserialize;
    
    #[derive(Deserialize, Debug)]
    pub struct Playlist {
        #[serde(alias = "prevPageToken")]
        pub prev_page_token: Option<String>,
        #[serde(alias = "nextPageToken")]
        pub next_page_token: Option<String>,
        pub items: Vec<Item>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Item {
        pub snippet: Snippet,
    }

    #[derive(Deserialize, Debug)]
    pub struct Snippet {
        pub title: String,
        pub description: String,
        #[serde(alias = "channelTitle")]
        pub channel_title: String // TODO: this is not correct, we need the title of the video not
        // the title of the playlist
    }
}
