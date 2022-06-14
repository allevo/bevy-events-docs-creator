use clap::Parser;

use bevy_events_docs_creator::{parse, generate_markdown};

/// Parse Bevy project and creates a markdown for documenting events
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of bevy project
    #[clap(value_parser(clap::builder::NonEmptyStringValueParser::new()))]
    path: String,

    /// output filename
    #[clap(
        short,
        long,
        value_parser,
        default_value = "events.md",
        value_parser(clap::builder::NonEmptyStringValueParser::new())
    )]
    output: String,
}

fn main() {
    let args = Args::parse();

    let event_links = parse(&args.path);
    generate_markdown(&args.output, &event_links);
}
