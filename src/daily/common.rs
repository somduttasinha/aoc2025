pub fn read(path: &str) -> String {
    return std::fs::read_to_string(path).expect("Expected content in file");
}
