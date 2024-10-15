mod commands;

use std::env;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "list" => {
                    commands::list::run(&ctx, &command).await.unwrap();
                    None
                },
                "create" => {
                    commands::create::run(&ctx, &command).await.unwrap();
                    None
                },
                "delete" => {
                    commands::delete::run(&ctx, &command).await.unwrap();
                    None
                },
                "start" => {
                    commands::start::run(&ctx, &command).await.unwrap();
                    None
                },
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![commands::list::register(), commands::create::register(), commands::delete::register(), commands::start::register()])
            .await.unwrap();

        println!("I now have the following guild slash commands: {:?}", commands.iter().map(|c| c.name.to_string()).collect::<Vec<_>>());

        // let guild_command =
        //     Command::create_global_command(&ctx.http, commands::wonderful_command::register())
        //         .await;

        // println!("I created the following global slash command: {guild_command:#?}");
    }
}

tonic::include_proto!("rpc");
use helio_client::HelioClient;
use tonic::transport::Channel;

pub type GRPClient = HelioClient<Channel>;

impl TypeMapKey for GRPClient {
    type Value = GRPClient;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let client_rpc = HelioClient::connect("http://127.0.0.1:8080").await?;

        let mut data = client.data.write().await;
        data.insert::<GRPClient>(client_rpc);
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(())
}
