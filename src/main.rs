extern crate clap;
use clap::{Arg, App, ArgMatches};
use std::ffi::OsString;

fn main() {

    let cli_matches = App::new("")
        .author("lagooj")
        .version("0.1")
        .about("Check idrac status via ssh")
        .arg(Arg::with_name("hostname")
             .short("h")
             .long("hostname")
             .help("Set hostname and port ex: localhost")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("user")
             .short("u")
             .long("user")
             .help("Set drac user")
             .takes_value(true)
             .required(true))
         .arg(Arg::with_name("password")
              .short("p")
              .long("password")
              .help("Set drac user password")
              .takes_value(true)
              .required(true))
        .arg(Arg::with_name("v")
             .short("v")
             .help("Set verbose mode")
             .multiple(false))
        .get_matches();


}


pub fn get_hostname(matches :&ArgMatches) -> &str {
    matches.value_of("hostname").expect("Warning threshold must be set")
}

pub fn get_user(matches :&ArgMatches) -> &str {
    matches.value_of("user").expect("Warning threshold must be set")
}

pub fn get_password(matches :&ArgMatches) -> &str {
    matches.value_of("password").expect("Warning threshold must be set")
}


pub fn get_verbose(matches :&ArgMatches) -> bool {
    matches.is_present("v")
}
