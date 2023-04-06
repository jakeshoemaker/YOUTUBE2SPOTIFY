# YOUTUBE2SPOTIFY
#### YOUTUBE2SPOTIFY -> converts a youtube playlist into a playlist on your spotify account

## Deps
1. Install rust [here](https://rustup.rs) *or* by typing this in a terminal:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2.  Obtain a Youtube API Key. 
    - Log in to Google Developers Console.
    - Create a new project.
    - On the new project dashboard, click Explore & Enable APIs.
    - In the library, navigate to YouTube Data API v3 under YouTube APIs.
    - Enable the API.
    - Create a credential.
    - A screen will appear with the API key. (save this for later)

3. Setup project w/ spotfiy [here](https://developer.spotify.com/dashboard)
    - Gather 3 details: 
        1. redirect_uri
        2. client_id
        3. client_secret
        
 (place those in a dotenv) (update soon will do it for ya)

## Usage

### Installation

1. Make sure you have Rust and Cargo installed on your machine.
2. Clone the repository: 
```
$ git clone https://github.com/jakeshoemaker/YOUTUBE2SPOTIFY.git
```
3. Navigate to project directory
### Running the code
1. Build project: 
```
$ cargo build --release
```
2. Run the executable:
```
$ ./target/release/YOUTUBE2SPOTIFY
```


## thanks, and feel free to open a PR or open a issue
