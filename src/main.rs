/*
bot made by avery using Serenity examples
*/
mod commands;
use commands::{owner::*};

use serenity::{async_trait, http::Http};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::{
    StandardFramework,
    help_commands,
    Args,
    HelpOptions,
    CommandGroup,
    CommandResult,
    macros::{
        command,
        group,
        help
    }
};
use std::{collections::HashSet, sync::Arc,env};
use tokio::sync::Mutex;
use serenity::model::prelude::*;
use serenity::prelude::*;
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
////////////////////

#[group]
#[commands(ping, quit)]
struct General;   

////////////////////

struct Handler;
#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    // Login with a bot token from the environment
    //let token = env::var("DISCORD_TOKEN").expect("token");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new_with_token(&token);
    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut _owners = HashSet::new();
            _owners.insert(info.owner.id);

            (_owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let framework = StandardFramework::new().configure(|c| c.owners(_owners).prefix("~")).group(&GENERAL_GROUP).help(&THE_HELP);
    let mut client = Client::builder(token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }
    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
        println!("\nBot shutting down");
    });

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[help]
async fn the_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    _owners: HashSet<UserId>
    ) -> CommandResult {
        let _ = help_commands::plain(context, msg, args, help_options, groups, _owners).await;
        Ok(())
 }
