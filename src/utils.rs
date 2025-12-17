use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type RadioError = Box<dyn Error + Send + Sync + 'static>;

pub type RadioResult<T> = Result<T, RadioError>;

pub async fn send_json<O, P>(sender: &mut O, packet: &P) -> RadioResult<()>
where
    O: AsyncWriteExt + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');

    sender.write_all(json.as_bytes()).await?;
    Ok(())
}

pub async fn receive_one<R, T>(receiver: &mut R) -> RadioResult<Option<T>>
where
    R: AsyncBufReadExt + Unpin,
    T: DeserializeOwned,
{
    let mut line = String::new();
    let bytes_read = receiver.read_line(&mut line).await?;
    
    if bytes_read == 0 {
        return Ok(None);
    }
    
    let msg = serde_json::from_str::<T>(&line)?;
    Ok(Some(msg))
}
