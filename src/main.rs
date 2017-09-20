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
use librespot::metadata::{Metadata, Track};
use librespot::core::session::Session;
use librespot::core::util::SpotifyId;

use librespot::audio_backend;
use librespot::player::Player;

fn main() {
    let filename = "songs.toml";
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let session_config = SessionConfig::default();
    let player_config = PlayerConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD TRACK", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let credentials = Credentials::with_password(username, password);


    let backend = audio_backend::find(Some("pulseaudio".to_owned())).unwrap();

    println!("Connecting ..");
    let session = core.run(Session::connect(session_config, credentials, None, handle))
        .unwrap();
    /*
    let trackmeta = core.run(Track::get(&session.to_owned(), track)).unwrap();
    println!("Track info : {}", trackmeta.name);
    */
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
                        println!("{} bytes read", n);
                        println!("Playing...");

                        core.run(player.load(track, true, 0)).unwrap();

                        println!("Done");
                    }
                    None => println!("error: {} not found", input.trim()),
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
