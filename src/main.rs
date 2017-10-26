extern crate librespot;
extern crate tokio_core;
extern crate toml;

use tokio_core::reactor::Core;
use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use toml::Value;

use librespot::core::authentication::Credentials;
use librespot::core::config::{PlayerConfig, SessionConfig};
use librespot::metadata::{Metadata, Track}; //Unused
use librespot::core::session::Session;
use librespot::core::util::SpotifyId;

use librespot::audio_backend;
use librespot::player::Player;

fn main() {
    //TODO(cfg) let the user pass in the config file
    let filename = "songs.toml";
    //QUESTION: This looks like a callback structure, is that correct?
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    //TODO(sptfy-setup) blackbox spotify setup and config
    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD TRACK", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    //TODO(tkn) can we use a token instead?
    let credentials = Credentials::with_password(username, password);


    //let backend = audio_backend::find(Some("pulseaudio".to_owned())).unwrap();
    let backend = audio_backend::find(None).unwrap();

    println!("Connecting ..");
    let session = core.run(Session::connect(session_config, credentials, None, handle))
        .unwrap();

    /*
    //TODO(trk-name) log info about tracks being played
    let trackmeta = core.run(Track::get(&session.to_owned(), track)).unwrap();
    println!("Track info : {}", trackmeta.name);
    */

    //TODO(sptfy-setup) this could be the return value for blackbox
    let player = Player::new(player_config,
                             session.clone(),
                             None,
                             move || (backend)(None));

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);

    let songs = contents.parse::<Value>().unwrap();

    println!("Ready for scanning.");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("Looking for song {}", input);
                match songs[input.trim()].as_str() {
                    Some(trackstr) => {
                        let track = SpotifyId::from_base62(trackstr);
                        println!("Playing...");

                        //TODO(interupt) add the ability to interupt player
                        core.run(player.load(track, true, 0)).unwrap();

                        println!("Ready for scanning.");
                    }
                    None => println!("error: {} not found", input.trim()),
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
