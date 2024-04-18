use std::io;
use std::process::Command;
use std::sync::Arc;

const PLAYER: &str = "ffplay";
const KILL: &str = "kill";

#[derive(Debug, Clone)]
pub struct Player {
    pub child_process_id: Option<u32>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            child_process_id: None,
        }
    }

    pub async fn play(&mut self, url: Arc<String>) -> Result<(), io::Error> {
        let child = Command::new(PLAYER)
            .args(["-nodisp", "-nostats", "-loglevel", "0", &url])
            .spawn()?;

        self.child_process_id = Some(child.id());

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), io::Error> {
        if let Some(id) = self.child_process_id {
            let _child = Command::new(KILL)
                .args(["-INT", id.to_string().as_ref()])
                .spawn()?;
        }
        Ok(())
    }
}
