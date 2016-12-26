mod oidc;

extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;
use std::ops::Deref;

fn main() {
    print!("Starting server... ");

    let config : oidc::ConfigStruct = oidc::ConfigStruct {
        redirect_endpoint : "redirectendpoint".to_owned(),
        login_endpoint : "login".to_owned(),
        use_ssl: false,
        sslcertpath : "./cert.p12".to_owned(),
        certpassword : "password".to_owned(),
        client_id : "myclientid".to_owned(),
        project_id : "myprojectid".to_owned(),
        auth_uri : "https://accounts.google.com/o/oauth2/auth".to_owned(),
        token_uri : "https://www.googleapis.com/oauth2/v1/certs".to_owned(),
        auth_provider_x509_cert_url : "https://www.googleapis.com/oauth2/v1/certs".to_owned(),
        client_secret : "mysecret".to_owned(),
        redirect_url : "redirect_url".to_owned(),
        raw_host : "localhost".to_owned(),
        port : 9123,
    };

    let config2 = config.clone();
    let local_config = config.clone();

    let stri = format!("{}:{}", local_config.raw_host, local_config.port);
    print!("{} ... ", stri);

    let mut router = Router::new();

    router.get(format!("{}{}", "/".to_owned(), local_config.login_endpoint), move |req : &mut Request| config.login_handler(req), "login");
    router.get(format!("{}{}", "/".to_owned(), local_config.redirect_endpoint), move |req : &mut Request| config2.redirection_handler(req), "redirect");


    if !local_config.use_ssl {
        Iron::new(router).http(stri.deref()).unwrap();
    }

    println!("started!")
}
