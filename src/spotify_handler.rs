pub mod spotify_helper {
    use rspotify::{
        model::{SearchType, Country, Market, SearchResult, PrivateUser, PlaylistId},
        prelude::*,
        scopes, AuthCodeSpotify, Credentials, OAuth,
    };

pub struct SpotifyHandler {
        api: AuthCodeSpotify,
        user: PrivateUser,
        market: Market,
        playlist_id: Option<PlaylistId<'static>>,
        track_ids: Vec<PlayableId<'static>>
    }

    impl SpotifyHandler {
        pub async fn new() -> Self {
            let api = SpotifyHandler::setup_spotify_authcode().await;
            let user = SpotifyHandler::get_authenticated_user(api.clone()).await;
            let market = Market::Country(Country::UnitedStates);
            let playlist_id = None;
            let track_ids = vec![];
            let s = Self {
                api,
                user,
                market,
                playlist_id,
                track_ids
            };

            return s;
        }

        pub async fn search_for_track(&mut self, query: &String) {
            let search_res =
                self.api.search(
                    query, 
                    SearchType::Track, 
                    Some(self.market), 
                    None, 
                    Some(1), 
                    None)
                .await;

            let potential_track = match search_res {
                Ok(t) => match t {
                            SearchResult::Tracks(t) => t,
                            _ => return
                },
                _ => return
            };


            let id = match potential_track.items
                .first() {
                    Some(t) => t,
                    None => return
                }
                .id
                .clone()
                .to_owned();

            // safely add track to playlist
            match id {
                Some(id) =>
                    self.track_ids.push(PlayableId::Track(id)),
                None => return
            };
        }


        //spotify.user_playlist_create(h, name, j, collaborative, description)
        pub async fn create_user_playlist(&mut self) {
            // todo: nice user interaction asking for playlist name /description using clap
            let name: String = "bangers".to_string();
            let description = "created from my commandline using rust and a youtube link".to_string();
            let user_id = &self.user.id;
            let playlist = self.api.user_playlist_create(
                user_id.to_owned(), &name, Some(true), Some(false), Some(&description))
                    .await
                    .unwrap();

            self.set_playlist_id(&playlist.id);
        }

        //spotify.playlist_add_items(playlist_id, items, position)
        pub async fn add_tracks_to_playlist(mut self) {
            // if playlist is none, go ahead and create the playlis
            if self.playlist_id == None {
                self.create_user_playlist().await;
            }

            let playlist_id = self.playlist_id.to_owned().unwrap();
            let mut tracks = self.track_ids.into_iter().peekable();
            // tracks cant be more than 100
            while tracks.peek().is_some() {
                let chunk: Vec<_> = tracks.by_ref().take(100).collect();
                let res = self.api.playlist_add_items(playlist_id.clone(), chunk, None)
                    .await;
                match res {
                    Ok(_) => println!("tracks added to playlist"),
                    Err(rspotify::ClientError::Http(error)) => {
                        let e = *error;
                        match e {
                            rspotify::http::HttpError::Client(c) => println!("client: {}", c),
                            rspotify::http::HttpError::StatusCode(e) => println!("status: {}", e.text().await.unwrap()),
                        };
                    },
                    Err(rspotify::ClientError::ParseJson(error)) => {
                        let error_message = error.to_string();
                        println!("Error parsing JSON: {}", error_message);
                    },
                    Err(rspotify::ClientError::ParseUrl(error)) => {
                        let error_message = error.to_string();
                        println!("Error parsing URL: {}", error_message);
                    },
                    Err(error) => println!("Error: {:?}", error)
                }
            }
        }

        // private methods
        fn set_playlist_id(&mut self, id: &PlaylistId<'static>) {
            self.playlist_id = Some(id.clone());
        }

        async fn setup_spotify_authcode() -> AuthCodeSpotify {
            let creds: Credentials = Credentials::from_env().unwrap();
            let oauth: OAuth = OAuth::from_env(scopes!("playlist-modify-public")).unwrap();
            let spotify: AuthCodeSpotify = AuthCodeSpotify::new(creds, oauth);
            //  obtain access token, and allow spotify to get a token itself
            let url = match spotify.get_authorize_url(false) {
                Ok(t) => t,
                Err(e) => panic!("Error setting up spotify: {}", e)
            };
            
            match spotify.prompt_for_token(&url).await {
                Ok(t) => t,
                Err(e) => panic!("Error setting up spotify: {}", e)
            };
            return spotify;
        }

        async fn get_authenticated_user(api: AuthCodeSpotify) -> PrivateUser {
            let user = match api.me().await {
                Ok(u) => u,
                Err(_) => panic!("could not get user")
            };
            return user;
        }
    }

}

