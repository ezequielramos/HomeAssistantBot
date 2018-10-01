use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "~ ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say("Pong!") {
                error!("Error sending message: {:?}", why);
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
        debug!("{} is connected!", ready.user.name);
    }
}

/// Boot the home bot
pub fn start(token: &str) {
    // Login with a bot token from the environment
    let mut client = Client::new(token, Handler).expect("Error creating serenity client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        error!("An error occurred while running the client: {:?}", why);
    }
}
