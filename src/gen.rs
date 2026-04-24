pub fn generate(txt: &str) -> String {
    let mut code = String::new();
    let mut current: u8 = 0;

    for &byte in txt.as_bytes() {
        let diff = byte as i32 - current as i32;
        if diff > 0 {
            code.push_str(&"+".repeat(diff as usize));
        } else {
            code.push_str(&"-".repeat(diff.unsigned_abs() as usize));
        }
        code.push('.');
        current = byte;
    }

    code
}
