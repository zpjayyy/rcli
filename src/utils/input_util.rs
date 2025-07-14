use std::path::Path;

pub fn validate_input_exists(input: &str) -> Result<String, String> {
    if input == "-" || Path::new(input).exists() {
        Ok(input.to_string())
    } else {
        Err(format!("Input file {} does not exist", input))
    }
}
