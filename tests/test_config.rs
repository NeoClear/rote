use rote::resolve::parser;
use std::fs;
use std::env;

#[derive(Clone)]
struct Star {
    tag: String,
    file: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmdtype() {

    }
}