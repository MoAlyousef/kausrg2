fn main() {
    println!("cargo:rerun-if-changed=templates");

    let dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = format!("{dir}/templates/input.css");
    let output = format!("{dir}/assets/main.css");

    let result = std::process::Command::new("npx")
        .args(["@tailwindcss/cli", "-i", &input, "-o", &output, "--minify"])
        .output()
        .ok();

    if let Some(result) = result {
        if !result.status.success() {
            let error = String::from_utf8_lossy(&result.stderr);
            println!("cargo:warning=Unable to build CSS !");
            println!("cargo:warning=Output: {error}");
        }
    } else {
        println!("cargo:warning=tailwind unavailable!");
    }
}
