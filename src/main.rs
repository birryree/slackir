#[macro_use]
extern crate clap;
extern crate toml;
extern crate irc;
extern crate rustc_serialize;
extern crate websocket;
extern crate hyper;

use std::io::Result;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::thread::spawn;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::default::Default;
use irc::client::prelude::*;
use websocket::{Message as WsMessage, Sender as WsSender, Receiver as WsReceiver};
use websocket::message::Type;
use websocket::client::request::Url;
use websocket::Client as WsClient;
use hyper::Client as HttpClient;


mod config;

use config::Configuration;

fn main() {

    let matches = clap_app!(app =>
        (version: "1.0")
        (author: "birryree")
        (about: "A Slack bridge for IRC (and XMPP, probably)")
        (@arg config: -c --config +takes_value "Configuration file")
    ).get_matches();

    let config_file = matches.value_of("config").unwrap_or("config.toml");
    println!("Using configuration file: {}", config_file);

    let config = Configuration::parse(config_file);

    println!("{}", config.slack.token);

    let (slack_tx, slack_rx) : (Sender<String>, Receiver<String>) = channel();

    let (irc_tx, irc_rx) : (Sender<String>, Receiver<String>) = channel();

    let url = format!("https://slack.com/api/rtm.start?token={}", &config.slack.token);
    println!("Connecting to RTM API at {}", &url);

    let http = HttpClient::new();
    let mut rsp = http.get(&url).send().unwrap();
    let mut body = String::new();
    rsp.read_to_string(&mut body).unwrap();
    println!("Rsp: {}", body);

    //let rtm_url = Url::parse(&url).unwrap();

    //let slack_request = Client::connect(rtm_url).unwrap();

    /*let response = slack_request.send().unwrap();
    response.validate().unwrap();*/

    let threads: Vec<_> = config.servers.into_iter().map(|c| {
        let slack = slack_tx.clone();
        spawn(move || {
            let server = IrcServer::from_config(c.to_irc_config()).unwrap();
            server.identify().unwrap();
            for message in server.iter() {
                match message {
                    Ok(message) => {
                        print!("Message received: {}", message);
                        process_message(&server, &slack, message).unwrap();
                    },
                    Err(e) => {
                        println!("Encountered error: {}", e);
                        break
                    }
                }
            }
        })
    }).collect();
    threads.into_iter().map(|h| h.join().unwrap()).count();


    /*
    let config = Config {
        nickname: Some(format!("birrybridge")),
        server: Some(String::from("irc.faxtrola.net")),
        channels: Some(vec![format!("#gotbanned?")]),
        port: Some(6697),
        use_ssl: Some(true),
        .. Default::default()
    };

    let server = IrcServer::from_config(config).unwrap();

    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap();
        print!("{}", message);
    }
    */
}

fn process_message(server: &IrcServer, slack: &Sender<String>, message: Message) -> Result<()> {
    Ok(())
}
