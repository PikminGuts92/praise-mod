mod apps;
use apps::GPTool;
use simplelog::*;

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_config = ConfigBuilder::new()
        .add_filter_allow_str("gp_tool")
        .add_filter_allow_str("praise_mod_lib")
        .build();

    // Setup logging
    CombinedLogger::init(
        vec![
            TermLogger::new(LOG_LEVEL, log_config, TerminalMode::Mixed, ColorChoice::Auto),
        ]
    )?;

    let mut scene = GPTool::new();
    scene.run()
}