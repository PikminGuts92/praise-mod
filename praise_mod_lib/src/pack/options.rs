#[derive(Debug)]
pub struct PackOptions {
    pub songs_path: String,
    pub output_path: String,
    pub name: Option<String>,
    pub id: u8,
}