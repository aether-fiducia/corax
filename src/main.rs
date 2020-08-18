#![allow(unused_imports, dead_code)]

use std::{
    env,
    net::TcpStream,
    io::prelude::*,
    sync::Arc,
};

use serenity::{
    prelude::*,
    model::prelude::*,
    utils::MessageBuilder,
    client::bridge::voice::ClientVoiceManager,
    voice,
    framework::{
        StandardFramework,
        standard::{
            Args,
            CommandResult,
            macros::{command, group},
        },
    },
    Result as SerenityResult,
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
            else if mes.content.contains("!clear") {
                let guild = match mes.channel_id.to_channel(&ctx) {
                    Err(e) => {
                        println!("{:?}", e);
                        return;
                    },
                    Ok(c) => {
                        match c.guild() {
                            None => {
                                println!("Failed to get GuildChannel");
                                return;
                            },
                            Some(gc) => gc,
                        }
                    },
                };
                let messages = match guild.read().messages(&ctx.http, |builder| {
                    builder.before(&mes.id).limit(100)
                }) {
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                    Ok(mess) => mess,
                };
                for line in messages {
                    if line.content.starts_with(|c| c == '!' || c == '-') || line.author.bot {
                        line.delete(&ctx.http).unwrap();
                    }
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
                let mut words = mes.content.split(' ');
                let im_pos = words.position(|w| w == "I'm")
                    .unwrap();
                let response = MessageBuilder::new()
                    .push("Hey ")
                    .push(words.nth(im_pos).unwrap())
                    .push(" (")
                    .mention(&mes.author)
                    .push(") , I'm daddy! O.o")
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

//#[group]
//#[commands(join, leave, play)]
struct Genneral;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}



//Full of panics! for good reason, all these should halt startup
fn main() {
    let token = env::var("TOKEN")
        .expect("Expected a token!\nFormat - $env:TOKEN=TOKEN_HERE\n");

    let mut client = Client::new(&token, Handler)
        .expect("Could not create client!");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

//    client.with_framework(StandardFramework::new()
//        .configure(|c| c
//            .prefix("!"))
//        .group(&GENNERAL_GROUP));

    let _ = client.start()
        .map_err(|why| println!("Failed to establish a connection to the API: {:?}", why));

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
