use async_std::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type RadioError = Box<dyn Error + Send + Sync + 'static>;

pub type RadioResult<T> = Result<T, RadioError>;

pub async fn send_json<O, P>(sender: &mut O, packet: &P) -> RadioResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');

    sender.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn receive<R, T>(receiver: R) -> impl Stream<Item = RadioResult<T>>
where
    R: async_std::io::BufRead + Unpin,
    T: DeserializeOwned,
{
    receiver.lines().map(|line| -> RadioResult<T> {
        let l = line?;
        let msg = serde_json::from_str::<T>(&l)?;
        Ok(msg)
    })
}
