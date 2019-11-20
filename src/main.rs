extern crate discord;

use discord::model::Event;
use discord::Discord;

use color_backtrace;

fn main() {
    color_backtrace::install();

    let token = include_str!("../client_id.txt").trim();
    dbg!(token);

    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(token).expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");

    println!("Ready.");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                if message.content == "!test" {
                    let _ = discord.send_message(
                        message.channel_id,
                        "This is a reply to the test.",
                        "",
                        false,
                    );
                } else if message.content == "!quit" {
                    println!("Quitting.");
                    break;
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}
