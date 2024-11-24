use async_std::prelude::*;

use serde::de::DeserializeOwned;
use serde::Serialize;
// serlialization and deserializaion is the process of converting data from it's in memory representation to byte streams and vice versa.
use std::error::Error;
use std::marker::Unpin;

//some custom types

// so ChatError is a Box that contains a dyn trait object that implements Error trait and is safe to send and share between threads, additionally chatError type must have a lifetime that is at least as long as the lifetime of the program.
// this type alias is used to define error , that can be returned from functions that may encounter errors during the execution
// by defininf type Error we can make our code more readable and easier to understand
pub type ChatError = Box<dyn Error + Send + Sync + 'static>; // type is used to define a type alias 
pub type ChatResult<T> = Result<T, ChatError>;


pub async fn send_json<O,P>(leaving : &mut O, packet : &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');

    leaving.write_all(json.as_bytes()).await?;
    Ok(())
} 

pub async fn receive<I,T> (incoming : I) -> impl Stream<Item = ChatResult<T>> 
where 
    I : async_std::io::BufRead + Unpin,
    T : DeserializeOwned,
{
    incoming.lines().map(|line| -> ChatResult<T> {
        let li = line?;
        let msg = serde_json::from_str::<T> (&li)?;
        Ok(msg)
    })
}
