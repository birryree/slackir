use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};
use serde;
use toml;
use irc;

include!(concat!(env!("OUT_DIR"), "/config.rs"));
