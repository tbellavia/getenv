use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum SearchResult {
    Found(String),
    NotFound,
}

pub enum SearchError {
    FileNotExists,
}

pub const DELIMITERS: &[char] = &['\'', '"', ' '];
pub const LINE_STARTS: &[char] = &['#'];

pub fn search(envfile: &str, key: &str) -> Result<SearchResult, SearchError> {
    let file = match File::open(envfile) {
        Ok(f) => f,
        Err(_) => return Err(SearchError::FileNotExists),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.starts_with(LINE_STARTS) {
            continue;
        }

        match line.split_once('=') {
            Some((lhs, rhs)) if lhs.trim() == key => {
                let value = rhs.trim_matches(DELIMITERS);
                return Ok(SearchResult::Found(value.to_string()));
            }
            Some(_) => {}
            None => {}
        }
    }
    Ok(SearchResult::NotFound)
}
