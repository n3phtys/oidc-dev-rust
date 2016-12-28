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
use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();
    let configpath : String = if args.len() > 1 {
        format!("{}", args[1])
    } else {
        "config.json".to_owned()
    };

    println!("Starting server with config at {}... ", configpath);

    let mut f = (File::open(configpath)).unwrap();
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
    } else {
        // Avoid unused errors due to conditional compilation ('ssl' feature is not default)
        use iron::Iron;
        use std::path::{Path};

        let key = Path::new(&local_config.sslkeypath).to_path_buf();
        let cert = Path::new(&local_config.sslcertpath).to_path_buf();

        Iron::new(router).https(stri.deref(), cert, key).unwrap();

    }

    println!("did not start!")
}
