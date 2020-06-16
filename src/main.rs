pub mod parser;

fn main() {
    let _ = parser::parse_line("loop      ".to_string());
    let _ = parser::parse_line("     mov     r0, r1".to_string());
    let _ = parser::parse_line("     beq     suca".to_string());
}
