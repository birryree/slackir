use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};
use toml;
use irc;


/// Configuration structs hold the configuration
/// for an instance of a Slack Bridge
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Configuration {
    /// Slack configuration
    pub slack: SlackConfiguration,

    /// IRC configurations - multiple IRC configurations
    /// can be specified
    pub servers: Vec<IrcConfiguration>
}

impl Configuration {
    /// Returns a new default `Configuration` object (which will not be
    /// particularly useful by itself).
    pub fn new() -> Configuration {
        Configuration {
            slack: SlackConfiguration::new(),
            servers: Vec::with_capacity(5)
        }
    }

    /// Reads a configuration file from the supplied `path` and
    /// returns a `Configuration`.
    pub fn parse(path: &str) -> Configuration {
        let mut file = match File::open(path) {
            Err(why) => panic!("Couldn't open {}: {}", path,
                                                       Error::description(&why)),
            Ok(file) => file,
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).
            unwrap_or_else(|e| panic!("Error reading config: {}", e));

        let mut parser = toml::Parser::new(&contents);
        let toml = match parser.parse() {
            Some(toml) => toml,
            None => {
                for err in &parser.errors {
                    let (loline, locol) = parser.to_linecol(err.lo);
                    let (hiline, hicol) = parser.to_linecol(err.hi);
                    println!("{}:{}:{}-{}:{} error: {}",
                             path, loline, locol, hiline, hicol, err.desc);
                }
                panic!("TOML parsing errors detected!");
            }
        };

        let config = Value::Table(toml);
        toml::decode(config).unwrap()
    }
}

/// IrcConfiguration structs hold configurations
/// for a specific IRC network
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct IrcConfiguration {
    pub network: String,
    pub server: String,
    pub nick: String,
    pub use_ssl: bool,
    pub port: u16,
    pub channels: Vec<String>
}

impl IrcConfiguration {
    pub fn new() -> IrcConfiguration {
        IrcConfiguration {         
            network: String::from("EFNet"),
            server: String::from("irc.efnet.org"),
            nick: String::from("bridgebot"),
            use_ssl: false,
            port: 6667,
            channels: vec![]
        }
    }

    pub fn to_irc_config(&self) -> irc::client::data::config::Config {
        irc::client::data::config::Config {
            nickname: Some(self.nick.clone()),
            server: Some(self.server.clone()),
            channels: Some(self.channels.clone()),
            port: Some(self.port.clone()),
            use_ssl: Some(self.use_ssl.clone()),
            .. Default::default()
        }
    }
}


/// SlackConfiguration structs hold the configuration
/// for a specific SlackBot
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct SlackConfiguration {
    pub token: String
}

impl SlackConfiguration {
    pub fn new() -> SlackConfiguration {
        SlackConfiguration {
            token: String::from("")
        }
    }
}
