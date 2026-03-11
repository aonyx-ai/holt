use std::path::Path;

use doco::{Doco, Mount, Server, WaitFor};

static CADDYFILE: &str = include_str!("Caddyfile");

// Force the library (and its #[doco::test] functions) to be linked in.
extern crate holt_kit_docs;

#[doco::main]
async fn main() -> Doco {
    let dist = Path::new("dist")
        .canonicalize()
        .expect("dist/ not found — run `trunk build --release` first");
    let dist_str = dist.to_str().expect("dist path not UTF-8");

    let caddyfile_path = std::env::temp_dir().join("holt-kit-docs-e2e-Caddyfile");
    std::fs::write(&caddyfile_path, CADDYFILE).expect("failed to write Caddyfile");
    let caddyfile_str = caddyfile_path.to_str().expect("path not UTF-8").to_string();

    let server = Server::builder()
        .image("caddy")
        .tag("alpine")
        .port(80)
        .wait(WaitFor::message_on_stderr("server running"))
        .mount(Mount::bind_mount(dist_str, "/srv"))
        .mount(Mount::bind_mount(&caddyfile_str, "/etc/caddy/Caddyfile"))
        .build();

    Doco::builder()
        .server(server)
        .headless(true)
        .viewport(doco::Viewport::new(1280, 720))
        .build()
}
