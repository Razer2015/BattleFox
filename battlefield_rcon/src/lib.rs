#![warn(missing_debug_implementations, rust_2018_idioms)]
/*!
# Example
```ignore
#[tokio::main]
async fn main() -> rcon::RconResult<()> {
    let rcon = RconClient::connect(("127.0.0.1", 47200, "smurf")).await?;
    let bf4 = Bf4Client::new(rcon).await.unwrap();

    bf4.kill("player").await.unwrap_err();

    let mut event_stream = bf4.event_stream();
    while let Some(ev) = event_stream.next().await {
        match ev {
            Ok(Event::Kill{killer, victim, headshot: _, weapon}) => {
                println!("{} killed {} with a {}!", killer, victim, weapon);
            },
            Ok(_) => {}, // ignore other events.
            Err(err) => {
                println!("Got error: {:?}", err);
            },

        }
    }

    sleep(Duration::from_secs(60)).await;

    Ok(())
}
```
*/

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

#[macro_use]
pub mod macros;
#[cfg(feature = "bf4")]
pub mod bf4;
pub mod rcon;

#[cfg(test)]
mod tests {
    use crate::bf4::{Bf4Client, ServerInfoError};

    #[tokio::test]
    #[ignore]
    async fn test_server_info() {
        let bf4 = Bf4Client::connect_restricted(
            "127.0.0.1:47200", false,
        )
        .await
        .unwrap();

        // Server info test
        let serverinfo = match bf4.server_info().await {
            Ok(info) => info,
            Err(ServerInfoError::Rcon(rconerr)) => panic!("{:?}", rconerr),
        };

        // println!("ServerInfo {:?}", serverinfo);

        let json_string = serde_json::to_string(&serverinfo).unwrap();
        println!("{}", json_string);

        assert_eq!("IN_GAME", serverinfo.blaze_game_state);
    }
}
