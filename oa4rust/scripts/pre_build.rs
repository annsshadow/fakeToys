//! Pre-build script: ensures web/dist exists before Rust compilation.
//! Called via Cargo.toml build = "scripts/pre_build.rs"

fn main() {
    let web_dist = match std::env::var("OA4RUST_WEB_DIST") {
        Ok(v) => v,
        Err(_) => "../../web/dist/web".to_string(),
    };
    let dist_path = format!("../../{}", web_dist);
    println!("cargo:rerun-if-env-changed=OA4RUST_WEB_DIST");
    println!("cargo:rerun-if-changed={}", dist_path);
    if !std::path::Path::new(&dist_path).exists() {
        eprintln!(
            "Warning: Frontend not built. Run `cd web && pnpm build` first, or set OA4RUST_WEB_DIST."
        );
    }
}
