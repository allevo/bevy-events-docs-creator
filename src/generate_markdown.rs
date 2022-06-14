use std::io::Write;
use markdown::*;

use crate::parse_functions::{EventLinks, FunctionPointer};

pub fn generate_markdown(path: &str, event_links: &EventLinks) {
    let mut file = std::fs::File::create(path).unwrap();

    let mut blocks = vec![];
    blocks.push(Block::Header(vec![Span::Text("Events".to_owned())], 1));

    for (event_name, event_link) in event_links {
        blocks.push(Block::Header(vec![Span::Text(event_name.clone())], 2));

        blocks.push(Block::Header(vec![Span::Text("Writers".to_owned())], 3));
        
        let items = event_link
            .writers
            .iter()
            .map(|fp| ListItem::Simple(vec![fp.into()]))
            .collect();
        blocks.push(Block::UnorderedList(items));

        blocks.push(Block::Header(vec![Span::Text("Readers".to_owned())], 3));
        
        let items = event_link
            .readers
            .iter()
            .map(|fp| ListItem::Simple(vec![fp.into()]))
            .collect();
        blocks.push(Block::UnorderedList(items));
    }

    let output = markdown::generate_markdown(blocks);

    file.write(output.as_bytes()).unwrap();
}

impl From<&FunctionPointer> for Span {
    fn from(fp: &FunctionPointer) -> Self {
        let text = if let Some(doc) = fp.docs.get(0) {
            format!("{} ({}) - {}", fp.function_name, fp.file_path, doc)
        } else {
            format!("{} ({})", fp.function_name, fp.file_path)
        };
        Span::Text(text)
    }
}