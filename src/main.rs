mod oidc;

extern crate iron;
extern crate router;

use std::net::{TcpStream,ToSocketAddrs, SocketAddr};
use iron::prelude::*;
use iron::status;
use router::Router;

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

    let redirect_closure = move |req : &mut Request| oidc::redirection_handler(oidc::ConfigStruct {
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
    }, req);
    //let login_closure = move |req : &mut Request| oidc::login_handler(config, req);


    let mut router = Router::new();
    router.get(format!("{}{}", "/".to_owned(), config.login_endpoint), redirect_closure, "login");
    //router.get(format!("{}{}", "/".to_owned(), config.redirect_endpoint), oidc::redirection_handler(&config), "redirect");

    let protocol = if config.use_ssl {
        "https://"
    } else {
        "http://"
    };

    let address : std::net::SocketAddr = (format!("{}{}{}", protocol, config.raw_host, config.port)).to_socket_addrs().unwrap().nth(0).unwrap();

    Iron::new(router).http(address).unwrap();

    println!("started!")
}
