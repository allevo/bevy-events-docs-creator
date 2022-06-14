
mod parse_functions;
mod generate_markdown;

pub use generate_markdown::generate_markdown;
pub use parse_functions::{parse_files, EventLinks};

pub fn parse(path: &str) -> EventLinks {
    parse_files(path)
}
