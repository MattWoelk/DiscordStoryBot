#![deny(rust_2018_idioms)]

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = include_str!("../client_id.txt").trim();

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

//extern crate discord;
//
//use discord::model::ChannelId;
//use discord::model::Event;
//use discord::model::Message;
//use discord::model::MessageId;
//use discord::model::Reaction;
//use discord::model::ReactionEmoji;
//use discord::Connection;
//use discord::Discord;
//
//use color_backtrace;
//
//struct Game {
//    channel_id: ChannelId,
//    most_recent_message_id: MessageId,
//}
//
//fn main() {
//    color_backtrace::install();
//
//    let token = include_str!("../client_id.txt").trim();
//
//    // Log in to Discord using a bot token from the environment
//    let discord = Discord::from_bot_token(token).expect("login failed");
//
//    // Establish and use a websocket connection
//    let (mut connection, _) = discord.connect().expect("connect failed");
//
//    println!("Ready.");
//
//    let mut game: Game = wait_for_game_start(&mut connection, &discord);
//
//    println!("Game On!");
//
//    // Wait for game to be set up
//
//    loop {
//        match connection.recv_event() {
//            Ok(Event::MessageCreate(message)) => {
//                handle_user_message(&message, &discord, &mut game);
//            }
//            Ok(Event::ReactionAdd(reaction)) => {
//                handle_user_reaction(&reaction, &discord, &mut game);
//            }
//            Ok(_) => {}
//            Err(discord::Error::Closed(code, body)) => {
//                println!("Gateway closed on us with code {:?}: {}", code, body);
//                break;
//            }
//            Err(err) => println!("Receive error: {:?}", err),
//        }
//    }
//}
//
//fn wait_for_game_start(connection: &mut Connection, discord: &Discord) -> Game {
//    loop {
//        match connection.recv_event() {
//            Ok(Event::MessageCreate(message)) => {
//                if message.content == "!play" {
//                    let mut game = Game {
//                        channel_id: message.channel_id,
//                        most_recent_message_id: message.id,
//                    };
//
//                    dbg!("start play!");
//
//                    let sent_message = discord
//                        .send_message(
//                            message.channel_id,
//                            "╔═════════════════════════════════════════\n\
//                             ║ You step into an [adjective] [location]\n\
//                             ╚═════════════════════════════════════════\n\
//                             What would you like to do?",
//                            "",
//                            false,
//                        )
//                        .expect("game start message failed");
//
//                    game.most_recent_message_id = sent_message.id;
//
//                    dbg!(discord.add_reaction(
//                        game.channel_id,
//                        sent_message.id,
//                        ReactionEmoji::Unicode("😄".to_string()),
//                    ));
//                    //message.channel.send('╔═════════════════════════════════════════╗');
//                    //message.channel.send('║ You step into an [adjective] [location] ║');
//                    //message.channel.send('╚═════════════════════════════════════════╝');
//                    //message.channel.send('What would you like to do?')
//                    //    .then(message => {
//                    //    message.react('⬅');
//                    //    message.react('⬆');
//                    //    message.react('⬇');
//                    //    message.react('➡');
//                    //});
//
//                    return game;
//                }
//            }
//            _ => {}
//        }
//    }
//}
//
//fn handle_user_message(message: &Message, discord: &Discord, game: &mut Game) {
//    println!("{} says: {}", message.author.name, message.content);
//
//    match message.content.as_str() {
//        "!play" => {}
//        "!test" => {
//            let sent_message = discord.send_message(
//                message.channel_id,
//                "This is a reply to the test.",
//                "",
//                false,
//            );
//
//            // TODO: put this in the real initialization code
//            //game.most_recent_message_id = sent_message.ok().map(|a| a.id);
//        }
//        _ => {}
//    }
//}
//
//fn handle_user_reaction(reaction: &Reaction, discord: &Discord, game: &mut Game) {
//    if reaction.message_id == game.most_recent_message_id {
//        dbg!("MOST RECENT");
//    }
//    dbg!(&reaction.user_id);
//    dbg!(&reaction.channel_id);
//    dbg!(&reaction.message_id);
//    dbg!(&reaction.emoji);
//}
