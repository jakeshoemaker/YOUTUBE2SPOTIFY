pub mod spotify_helper {
    use rspotify::{
        model::{SearchType, Country, Market, SearchResult, PrivateUser, TrackId},
        prelude::*,
        scopes, AuthCodeSpotify, Credentials, OAuth,
    };

    pub struct SpotifyHandler {
        api: AuthCodeSpotify,
        user: PrivateUser,
        market: Market
    }

    impl SpotifyHandler {
        pub async fn new() -> Self {
            let api = SpotifyHandler::setup_spotify_authcode().await;
            let user = SpotifyHandler::get_authenticated_user(api.clone()).await;
            let market = Market::Country(Country::UnitedStates);
            let s = Self {
                api,
                user,
                market
            };

            return s;
        }

        pub fn test_user_collected(&self) {
            dbg!(&self.user);
        } 

        // guessing we need a spotify id here
        pub async fn get_track(&self, title_artist_split: Vec<&str>, called_recursively: bool) /* todo return something */ {
            if called_recursively {
                println!("called recursively, no match found with first search");
            }
            /*
            * In order to have somewhat accurate searching, we will search using a split,
            * if we don't get a match, we will call the function recursively, swapping the 
            * title / artist
            *
            * doing this until we find a better wayt to parse the name. most songs i see are split
            * by "-" so this is how we get data and let spotify find the song. 
            * (until yt gives us more data abt the song playing this will have to suffice) ??
            */

            // these are handy to test with 
            // let test_track_name = &"BoTalks - F*ck It (feat. Caroline Pennell)";
            // let test_track_name2 = &"deadmau5 - My Heart Has Teeth (feat. Skylar Grey)";

            // todo: helper function to build spotify song query
            let potential_track = 
                self.api.search(
                    title_artist_split.first().unwrap(), 
                    SearchType::Track, 
                    Some(self.market), 
                    None, 
                    Some(1), 
                    None)
                .await
                .unwrap();
        }

        // private methods
        async fn setup_spotify_authcode() -> AuthCodeSpotify {
            let creds: Credentials = Credentials::from_env().unwrap();
            let oauth: OAuth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
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

