use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{FilterInstanceArgs, GRPClient};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = ctx.data.read().await;
    let mut client_rpc = data.get::<GRPClient>().unwrap().clone();

    let options = interaction.data.options();

    let uuid = match options.get(0).unwrap().value {
        ResolvedValue::String(d) => d,
        _ => "",
    };

    let mut message = CreateInteractionResponseMessage::new()
        .ephemeral(true);

    match client_rpc
        .start_instance(FilterInstanceArgs {
            uuid: uuid.to_string(),
            created_by: interaction.user.id.to_string(),
        })
        .await
    {
        Ok(_) => message = message.content(format!("Started instance : {}", uuid.to_string())),
        Err(e) => message = message.content(format!("Cannot start instance : {}", e.message())),
    }

    interaction
        .create_response(ctx, CreateInteractionResponse::Message(message))
        .await
        .unwrap();

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("start")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "uuid", "Instance UUID")
                .required(true),
        )
        .description("Start instance by uuid")
}
