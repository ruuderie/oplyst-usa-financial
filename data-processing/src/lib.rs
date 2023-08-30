pub mod data_scripts;
mod db;

use std::{
    collections::{HashMap, HashSet},
    //error::Error,
    fs::{self, File, OpenOptions},
    path::Path,
   // str::FromStr,
};
use tokio_postgres::{NoTls};
use csv::{Reader,QuoteStyle,Writer,WriterBuilder};
use polars::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::future::Future;
use std::pin::Pin;
extern crate log;
extern crate env_logger;
use log::debug;




