use oauth2::{
    basic::BasicClient, reqwest::http_client, RefreshToken, TokenResponse,
};
use std::{fs, io, error::Error};

pub fn read_refresh_token() -> Result<String, io::Error> {
    // Instead of checking with metadata, directly try reading
    fs::read_to_string("refresh_token.txt")
}

pub fn write_refresh_token(refresh_token: &str) -> io::Result<()> {
    fs::write("refresh_token.txt", refresh_token)?;
    Ok(())
}

pub fn get_access_token_from_rt(client: &BasicClient) -> Result<String, Box<dyn Error>> {
    let refresh_token = match read_refresh_token() {
        Ok(token) => token,
        Err(_) => {
            // If refresh token is not found, initialize the auth process
            initialize_auth(client)?;
            read_refresh_token()? // Retry reading after auth
        }
    };

    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request(http_client)
        .map_err(|e| format!("unable to request token: {}", e))?;

    println!("token expires in {:?}", token_result.expires_in().unwrap());
    Ok(token_result.access_token().secret().to_string())
}

pub fn initialize_auth(client: &BasicClient) -> Result<(), Box<dyn Error>> {
    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let (auth_url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("files.read".to_string()))
        .add_scope(oauth2::Scope::new("offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Go to the following URL to authorize: {}", auth_url);
    println!("Enter the authorization code:");

    let mut auth_code = String::new();
    io::stdin()
        .read_line(&mut auth_code)
        .map_err(|e| format!("failed to read line: {}", e))?;

    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(auth_code.trim().to_string()))
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
