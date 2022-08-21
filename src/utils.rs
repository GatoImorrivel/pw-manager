use std::io::Write;

pub fn prompt_field() -> (String, String) {
    print!("Field Name: ");
    std::io::stdout().flush().unwrap();
    let field_name = read_line_sanitized();

    print!("Field Value: ");
    std::io::stdout().flush().unwrap();
    let field_value = read_line_sanitized();

    (field_name, field_value)
}

pub fn read_line_sanitized() -> String {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");

    input.pop().unwrap();
    input
}

pub fn fetch_profiles() {
}
