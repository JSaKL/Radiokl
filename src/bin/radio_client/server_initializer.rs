use std::io;

const KILLALL: &str = "killall";

pub async fn start_server() -> Result<(), io::Error> {
    // Try to find the server binary in common locations
    let server_paths = [
        "./target/release/radio_server", // Running from project root
        "./radio_server",                // Running from target/release
        "../radio_server",               // Alternative location
    ];

    let mut server_path = None;
    for path in &server_paths {
        if std::path::Path::new(path).exists() {
            server_path = Some(*path);
            break;
        }
    }

    let server_path = server_path.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "radio_server binary not found. Please run 'cargo build --release' first.",
        )
    })?;

    let child = tokio::process::Command::new(server_path)
        .stdin(std::process::Stdio::null())
        .spawn()?;

    let child_process_id = child.id();

    println!("radio_server: child process ID = {:?}", child_process_id);

    Ok(())
}

pub async fn stop_server() -> Result<(), io::Error> {
    println!("stop radio_server");

    let _child = tokio::process::Command::new(KILLALL)
        .args(["-e", "-q", "radio_server"])
        .spawn()?;

    Ok(())
}
