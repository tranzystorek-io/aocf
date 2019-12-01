#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::{Utc, Datelike};
use failure::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod http;

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Level {
    First,
    Second,
}

impl Default for Level {
    fn default() -> Self {
        Self::First
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Aoc {
    year: Option<i32>,
    day: Option<u32>,
    input: Option<String>,
    level: Level,
    brief: HashMap<Level, String>,
    solution: HashMap<Level, String>,
    cookie: String,
    cache_path: Option<PathBuf>,
}

impl Aoc {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the year
    pub fn year(&mut self, year: Option<i32>) -> &mut Self {
        self.year = year;
        self
    }

    /// Set the day
    pub fn day(&mut self, day: Option<u32>) -> &mut Self {
        self.day = day;
        self
    }

    pub fn cookie(&mut self, cookie: String) -> &mut Self {
        self.cookie = cookie;
        self
    }

    /// Set the cache path
//    pub fn cache<P>(&mut self, path: P) -> &mut Self
//        where P: AsRef<Path> + std::clone::Clone,
//    {
    pub fn cache<P>(&mut self, path: &Path) -> &mut Self {
        self.cache_path = Some(path.to_path_buf());
        self
    }

    /// Initialise (finish building)
    pub fn init(&mut self) -> Self {
        let now = Utc::now();
        self.year = self.year.or_else(|| Some(now.year()));
        self.day = self.day.or_else(|| Some(now.day()));
        self.clone()
    }

    /// Restore the problem from JSON
    pub fn load_json_from(path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Self::default())
    }

    /// Get the problem brief as HTML and sanitise it to plain text
    pub fn get_brief(&self) -> Result<String, Error> {
        http::get_brief(self)
        //Ok(self.brief)
    }

    /// Get the input data
    pub fn get_input(&mut self) -> Result<String, Error> {
        if self.input.is_none() {
            let input = http::get_input(self)?;
            self.input = Some(input);
        }
        Ok(self.input.clone().unwrap())
    }

    /// Submit the solution
    pub fn submit(&mut self, solution: &str) -> Result<(), Error> {
        let response = http::submit(self, solution)?;
        Ok(())
    }

    /// get a JSON representation for the AoC problem
    pub fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }

    /// get an AoC problem from JSON representation
    pub fn from_json(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }

    /// Save JSON to path
    pub fn write_json_to(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut file = File::create(path)?;
        file.write_all(self.to_json()?.as_bytes())?;
        Ok(())
    }

    /// Get time until release
    pub fn get_time_until_release() {

    }
}
