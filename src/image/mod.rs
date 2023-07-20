pub struct ImageImpl {}

impl ImageImpl {
    pub fn handle(source: String, format: Option<String>, width: Option<u32>, height: Option<u32>) {
        println!("[Commands::Image] source: '{source}' format: '{format:?}' width: '{width:?}' height: '{height:?}'");
    }
}