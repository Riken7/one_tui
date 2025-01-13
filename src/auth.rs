use dirs_next;
use oauth2::{basic::BasicClient, reqwest::http_client, RefreshToken, TokenResponse};
use std::{error::Error, fs, io};
use tiny_http::{ Response, Server};

pub fn get_refresh_token_path() -> String {
    let home_dir = dirs_next::home_dir().expect("Unable to get home directory");
    let app_data_dir = home_dir.join(".config").join("onetui");
    fs::create_dir_all(&app_data_dir).expect("Unable to create app data directory");
    app_data_dir
        .join("refresh_token.txt")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn read_refresh_token() -> Result<String, io::Error> {
    let refresh_token_path = get_refresh_token_path();
    fs::read_to_string(refresh_token_path)
}

pub fn write_refresh_token(refresh_token: &str) -> io::Result<()> {
    let refresh_token_path = get_refresh_token_path();
    fs::write(refresh_token_path, refresh_token)?;
    Ok(())
}

pub fn get_access_token_from_rt(client: &BasicClient) -> Result<String, Box<dyn Error>> {
    let refresh_token = match read_refresh_token() {
        Ok(token) => token,
        Err(_) => {
            // If refresh token is not found, initialize the auth process
            initialize_auth(client)?;
            read_refresh_token()?
        }
    };

    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request(http_client)
        .map_err(|e| format!("unable to request token: {}", e))?;

    Ok(token_result.access_token().secret().to_string())
}

pub fn initialize_auth(client: &BasicClient) -> Result<(), Box<dyn Error>> {
    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let (auth_url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("files.read".to_string()))
        .add_scope(oauth2::Scope::new("offline_access".to_string()))
        .add_scope(oauth2::Scope::new("files.readwrite".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Go to the following URL to authorize: {}", auth_url);
    println!("Waiting for authorization...");

    let server =
        Server::http("127.0.0.1:8080").map_err(|e| format!("unable to start server: {}", e))?;
    let  auth_code; 
    loop {
        let request = server
            .recv()
            .map_err(|e| format!("unable to receive request: {}", e))?;
        let query = request
            .url()
            .split("?")
            .nth(1)
            .ok_or("No parameters found")?;

        auth_code = query
            .split('&')
            .find(|x| x.starts_with("code="))
            .ok_or("No auth code found")?
            .split('=')
            .nth(1)
            .ok_or("Failed to extract auth code")?
            .to_string();
        let response =
            Response::from_string("Authorization successful. You can close this tab now.");
        request.respond(response).unwrap();
        break;
    }
    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(auth_code.to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)
        .map_err(|e| format!("unable to exchange authorization code: {}", e))?;

    let refresh_token = token_result
        .refresh_token()
        .ok_or("no refresh token found")?
        .secret()
        .to_string();

    write_refresh_token(&refresh_token)?;

    Ok(())
}
