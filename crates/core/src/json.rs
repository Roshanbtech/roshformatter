use serde_json::Value;

pub fn format(code: &str) -> Result<String, String> {
    let v: Value = serde_json::from_str(code).map_err(|e| e.to_string())?;
    Ok(serde_json::to_string_pretty(&v).unwrap())
}
