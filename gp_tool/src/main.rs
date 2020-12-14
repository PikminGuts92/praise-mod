mod apps;
use apps::GPTool;
use simplelog::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        ]
    )?;

    let mut scene = GPTool::new();
    scene.run()
}