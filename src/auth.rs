use oauth2::{
    basic::BasicClient, reqwest::http_client, RefreshToken , TokenResponse,
};
use std::{fs,io};
pub fn read_refresh_token(client: &BasicClient) -> String {
    if !fs::metadata("refresh_token.txt").is_ok(){
        initialize_auth(&client);
    }
    std::fs::read_to_string("refresh_token.txt").expect("unable to read refresh token")
}

pub fn write_refresh_token(refresh_token: &str) -> io::Result<()>{
    fs::write("refresh_token.txt", refresh_token)?;
    Ok(())
}
pub fn get_access_token_from_rt(client: &BasicClient) -> String{
    let refresh_token = read_refresh_token(&client);

    let token_result = client.exchange_refresh_token(&RefreshToken::new(refresh_token)).request(http_client).expect("unable to request token(RT expired)");
    println!("token expires in {:?}", token_result.expires_in().unwrap());
    token_result.access_token().secret().to_string()
}

pub fn initialize_auth(client : &BasicClient) -> String{
    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("files.read".to_string()))
        .add_scope(oauth2::Scope::new("offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    let mut auth_code = String::new();
    println!("go to {}", auth_url.to_string());

    println!("enter the code: ");
    io::stdin()
        .read_line(&mut auth_code)
        .expect("failed to real line");

    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(auth_code))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)
        .expect("unable to request token");

    //println!("token result: {:#?}", token_result);
    //let access_token = token_result.access_token().secret().to_string();
    let refresh_token = token_result.refresh_token().unwrap().secret().to_string();

    let token_expiry = format!("token expires in {:?}", token_result.expires_in().unwrap());
    write_refresh_token(&refresh_token).expect("unable to write refresh token");
    token_expiry
}
