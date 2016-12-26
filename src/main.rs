mod oidc;

extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use router::Router;
use std::ops::Deref;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    print!("Starting server... ");

    //let json = r##"{"redirect_endpoint":"redirectendpoint","login_endpoint":"login","use_ssl":false,"sslcertpath":"./cert.p12","certpassword":"password","client_id":"myclientid","project_id":"myprojectid","auth_uri":"https://accounts.google.com/o/oauth2/auth","token_uri":"https://www.googleapis.com/oauth2/v1/certs","auth_provider_x509_cert_url":"https://www.googleapis.com/oauth2/v1/certs","client_secret":"mysecret","redirect_url":"redirect_url","raw_host":"localhost","port":9123}"##;
    let mut f = (File::open("config.json")).unwrap();
    let mut buffer = String::new();
    let size = f.read_to_string(&mut buffer);
    println!("Have read in {0} bytes from file! As String: {1}", size.unwrap(), buffer);

    let config : oidc::ConfigStruct = json::decode(&buffer).unwrap();

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

    println!("did not start!")
}
