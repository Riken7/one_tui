use dotenv::dotenv;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};
mod tui;
use std::{env, process};
mod auth;
mod files_list;
fn main() {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").expect("clientid not set");
    let auth_url = env::var("AUTH_URL").expect("authurl not set");
    let token_url = env::var("TOKEN_URL").expect("tokenurl not set");
    let redirect_url = env::var("REDIRECT_URL").expect("redirecturl not set");
    let client = BasicClient::new(
        ClientId::new(client_id),
        None,
        AuthUrl::new(auth_url.to_string()).expect("unable to parse Auth url"),
        Some(TokenUrl::new(token_url.to_string()).expect("unable to grab token")),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).expect("unable to setup redirect url"));

    let access_token = auth::get_access_token_from_rt(&client);
    if !access_token.is_ok() {
        eprintln!("Error getting access token: {:?}", access_token.err());
        process::exit(1);
    }else {
        //tui::app::main();
        let _ = files_list::list_files(access_token);
    }
}
