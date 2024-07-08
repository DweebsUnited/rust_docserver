pub fn now() -> String {
    let now = chrono::Utc::now();

    now.format("%FT%T").to_string()
}