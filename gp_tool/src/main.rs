mod apps;
use apps::GPTool;
use simplelog::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_config = ConfigBuilder::new()
        .add_filter_allow_str("gp_tool")
        .add_filter_allow_str("praise_mod_lib")
        .build();

    // Setup logging
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, log_config, TerminalMode::Mixed),
        ]
    )?;

    let mut scene = GPTool::new();
    scene.run()
}