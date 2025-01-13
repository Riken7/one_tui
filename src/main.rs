use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};
mod tui;
use tui::app;
use std::process;
mod auth;
mod api;

fn main() {
    //dotenv().ok();
    //let client_id = env::var("CLIENT_ID").expect("clientid not set");
    //let auth_url = env::var("AUTH_URL").expect("authurl not set");
    //let token_url = env::var("TOKEN_URL").expect("tokenurl not set");
    //let redirect_url = env::var("REDIRECT_URL").expect("redirecturl not set");
    let client_id = "61ef5691-c900-4a66-8395-2be25dbec9b6".to_string();
    let auth_url = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string();
    let token_url = "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string();
    let redirect_url = "http://localhost:8080".to_string();
    let client = BasicClient::new(
        ClientId::new(client_id),
        None,
        AuthUrl::new(auth_url.to_string()).expect("unable to parse Auth url"),
        Some(TokenUrl::new(token_url.to_string()).expect("unable to grab token")),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).expect("unable to setup redirect url"));

    let access_token = auth::get_access_token_from_rt(&client);
    match access_token {
        Ok(token) => {
            //println!("Access token {:?}", token);
            let _ = app::run(token);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
