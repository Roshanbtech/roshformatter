mod js;
mod json;

pub fn format(code: &str, lang: &str) -> Result<String, String> {
    match lang {
        "js" => js::format(code),
        "json" => json::format(code),
        _ => Err("Unsupported language".to_string()),
    }
}
