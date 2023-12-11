use reqwest;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn load_input_data(year: u32, day: u8) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;

    let request_builder = client.get(url);

    let mut cookie = reqwest::header::HeaderMap::new();

    cookie.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str("session=")?,
    );

    let response = request_builder.headers(cookie).send();

    match response {
        Ok(res) => {
            let body: String = res.text()?;
            let lines: Vec<String> = body.lines().map(|v| v.to_string()).collect();
            Ok(lines)
        }
        Err(e) => Err(e.into()),
    }
}

pub fn load_sample_data(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();

    match lines {
        Ok(lines) => Ok(lines),
        Err(e) => Err(e.into()),
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {

    pub const NORTH: Point = Point::new(0,-1);
    pub const SOUTH: Point = Point::new(0,1);
    pub const EAST: Point = Point::new(1,0);
    pub const WEST: Point = Point::new(-1,0);

    pub const fn new(x: isize, y: isize) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    pub fn add(&self, p: &Point) -> Point {
        return Point{x:self.x + p.x, y:self.y + p.y};
    }

    pub fn distance(&self, p: &Point) -> isize {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }

}
