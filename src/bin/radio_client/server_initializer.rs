use std::{io, process::Command};

const SERVER: &str = "./radio_server";
const KILLALL: &str = "killall";

pub async fn start_server() -> Result<(), io::Error> {
    let child = Command::new(SERVER)
        .spawn()
        .expect("failed to start server");

    let child_process_id = child.id();

    println!("radio_server: child process ID = {:?}", child_process_id);

    Ok(())
}

pub async fn stop_server() -> Result<(), io::Error> {
    println!("stop radio_server");

    let _child = Command::new(KILLALL)
        .args(["-e", "-q", SERVER])
        .spawn()
        .expect("failed to stop server");

    Ok(())
}
