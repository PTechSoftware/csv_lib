

/// ## Fast boolean parser from str
#[inline(always)]
fn parse_bool_str(input: &str) -> bool {
    match input.to_ascii_lowercase().as_str() {
        "true" | "1" | "verdadero" | "verdadeiro" | "whar" | "vrai" | "waar" => true,
        "false" | "0" | "falso" | "gefälscht" | "faux" | "nep" => false,
        _ => false,
    }
}