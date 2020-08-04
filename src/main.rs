use std::env;
use std::net::TcpStream;
use std::io::prelude::*;

use serenity::{
    prelude::*,
    model::prelude::*,
    utils::MessageBuilder,
};

//Build a simple struct to impl the EventHandler trait on
//That will serve the responses to the message event
struct Handler;

//Add impl for all served events
impl EventHandler for Handler {

    fn message(&self, ctx: Context, mes: Message) {

        if mes.author.bot != true {
            if mes.content.contains("!players") {
                if let Err(err) = mes.channel_id.say(ctx.http,
                                        format!("{} out of 20 players online.",
                                        query_players("corvuscorax.org:25565").unwrap())) {
                    println!("{:?}", err);
                }

            }

            else if mes.content.contains("I'm") {
                let _ = match mes.channel_id.to_channel(&ctx) {
                    Ok(channel) => channel,
                    Err(why) => {
                        println!("Error getting channel: {:?}", why);
                        return;
                    },
                };
                let response = MessageBuilder::new()
                    .push("Hey ")
                    .mention(&mes.author)
                    .push(" , I'm daddy! O.o")
                    .build();
                if let Err(e) = mes.channel_id.say(&ctx.http, &response) {
                    println!("{}", e);
                }
            }
        }
    }

    //Dev info to console
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected\nID: {}", ready.user.name, ready.user.id);
    }
}

//Full of panics! for good reason, all these should halt startup
fn main() {
    let token = env::var("TOKEN")
        .expect("Expected a token!\nFormat - $env:TOKEN=TOKEN_HERE\n");

    let mut client = Client::new(&token, Handler)
        .expect("Could not create client!");

    client.start()
        .expect("Failed to establish a connection to the API");
}

// Never workin, except maybe now
fn query_players(addr: &str) -> Result<String, std::io::Error> {
    let mut stream = match TcpStream::connect(&addr) {
        Ok(s) => s,
        Err(err) => panic!("{}", err),
    };
    println!("Successful connection");

    let buf = [0xFE, 0x001];
    stream.write(&buf)?;

    let temp = stream.bytes()
        .map(|b| match b {
            Ok(d) => char::from(d),
            Err(_) => char::from(0u8),
        })
        .collect::<String>();
    let result = temp.split("\x00\x00\x00")
        .nth(4)
        .unwrap();

    Ok(result.to_string())
}
