use dotenv::dotenv;
use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};


#[tokio::main]
async fn main() {
    // load env
    dotenv().ok();
    let spotify = setup_spotify().await;
    

    // Running the requests
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");
}

pub async fn setup_spotify() -> AuthCodeSpotify {

    let creds: Credentials = Credentials::from_env().unwrap();
    let oauth: OAuth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();
    let spotify: AuthCodeSpotify = AuthCodeSpotify::new(creds, oauth);
    //  obtain access token, and allow spotify to get a token itself
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    println!("token got");
    return spotify;
}
