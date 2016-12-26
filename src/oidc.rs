extern crate rustc_serialize;
extern crate iron;
extern crate params;
extern crate std;

use self::rustc_serialize::base64::FromBase64;
use self::iron::prelude::*;
use self::iron::modifiers::Redirect;
use self::iron::{Url, status};
use self::params::{Params, Value};

#[derive(Clone)]
pub struct ConfigStruct  {
    pub redirect_endpoint: String,
    pub login_endpoint : String,
    pub use_ssl: bool,
    pub sslcertpath : String,
    pub certpassword : String,
    pub client_id : String,
    pub project_id : String,
    pub auth_uri : String,
    pub token_uri : String,
    pub auth_provider_x509_cert_url : String,
    pub client_secret : String,
    pub redirect_url : String,
    pub raw_host : String,
    pub port : u16,
}

#[derive(Clone)]
pub struct RemoteVerifiedTokenStruct  {
    iss: String,
    iat: u64,
    exp: u64,
    at_hash: String,
    aud: String,
    sub: String,
    email_verified: String,
    azp: String,
    email: String,
    alg: String,
    kid: String,
}

pub fn redirection_handler(config : ConfigStruct, req : &mut Request) ->  IronResult<Response>   {

        /*
            TODO: extract token and redirect url from state, etc.
            */
        let token = String::from("abc");
        let url : String = String::from("abc.com");
        let bearer : String = format_token_as_bearer_token(&token);
        let together : String = format!("{}{}{}", url, "?token=".to_owned() , bearer);
        let url = Url::parse(&together).unwrap();
        Ok(Response::with((status::Found, Redirect(url.clone()))))

}

pub fn login_handler(config : ConfigStruct, req : &mut Request) -> IronResult<Response> {
    let map : &params::Map = req.get_ref::<Params>().unwrap();
    println!("Parameter Map: {:?}", map);
    match map.get("url") {
        Some(&Value::String(ref url64))  => {
            let para = url64.from_base64().unwrap();
            let decode = std::str::from_utf8(&para).unwrap();
            let url = Url::parse("http://rust-lang.org").unwrap(); //TODO: implement this
            Ok(Response::with((status::Found, Redirect(url.clone()))))
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}

fn exchange_code_for_access_token(config : &ConfigStruct, code : &String) -> String {
        //TODO: make HTTP post call and wait for return
    "this is returned".to_owned()
}

fn format_token_as_bearer_token(token : &String) -> String {
    //TODO: url encoded and so on
    "this is a token".to_owned()
}


/* //TODO:
 fn verify_token() {

}*/