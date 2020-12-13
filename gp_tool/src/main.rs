mod apps;
use apps::GPTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scene = GPTool::new();
    scene.run()
}