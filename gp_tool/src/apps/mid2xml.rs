use crate::apps::{SubApp};
use clap::{Clap};
use std::error::Error;

#[derive(Clap, Debug)]
pub struct Mid2XmlApp {
    #[clap(about = "Path to input mid", required = true)]
    pub mid_path: String,
    #[clap(about = "Path to output xml", required = true)]
    pub xml_path: String,
}

impl SubApp for Mid2XmlApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        println!("This is mid2xml app!");

        Ok(())
    }
}