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
        #[serde(alias = "videoOwnerChannelTitle")]
        pub video_owner_channel_title: String,
    }

    #[derive(Debug)]
    pub struct PlaylistSearchCriteria {
        pub song_titles: Vec<String>,
        pub artists: Vec<String>
    }
}
