use clap::{Clap};
use std::error::Error;

mod mid2xml;
mod packcreator;
pub use self::mid2xml::*;
pub use self::packcreator::*;

// From Cargo.toml
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Clap, Debug)]
#[clap(name = PKG_NAME, version = VERSION, about = "Use this tool for modding guitar praise")]
struct Options {
    #[clap(subcommand)]
    commands: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "mid2xml", about = "Convert gh/rb mid to guitar praise xml")]
    Mid2Xml(Mid2XmlApp),
    #[clap(name = "packcreate", about = "Create guitar praise pack from CH song directory")]
    PackCreator(PackCreatorApp),
}

#[derive(Debug)]
pub struct GPTool {
    options: Options,
}

impl GPTool {
    pub fn new() -> GPTool {
        GPTool {
            options: Options::parse()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.options.commands {
            SubCommand::Mid2Xml(app) => app.process(),
            SubCommand::PackCreator(app) => app.process(),
        }
    }
}