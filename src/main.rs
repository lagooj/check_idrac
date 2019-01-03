extern crate clap;
extern crate ssh2;
use clap::{Arg, App, ArgMatches};

use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;

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

    println!("{}",get_hostname(&cli_matches));
    println!("{}",get_user(&cli_matches));
    println!("{}",get_password(&cli_matches));
}


pub fn get_hostname<'a>(matches :&'a ArgMatches) -> &'a str {
    matches.value_of("hostname").expect("Host muse be set")
}

pub fn get_user<'a>(matches :&'a ArgMatches) -> &'a str {
    matches.value_of("user").expect("user muste be set")
}

pub fn get_password<'a>(matches :&'a ArgMatches) -> &'a str {
    matches.value_of("password").expect("password must be set")
}


pub fn get_verbose(matches :&ArgMatches) -> bool {
    matches.is_present("v")
}


pub fn ssh_connect(hostname: &'static str, user: &'static str, password: &'static str) ->Result {

    let tcp = TcpStream::connect(format!("{}:22", hostname)).expect("Should be see");
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();

    sess.userauth_password(user, password).unwrap();
    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().expect("Should be closed");
    println!("{}", channel.exit_status().unwrap());
}
