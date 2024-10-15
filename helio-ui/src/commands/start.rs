use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{GRPClient, ProcessInstanceArgs};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = ctx.data.read().await;
    let mut client_rpc = data.get::<GRPClient>().unwrap().clone();

    let options = interaction.data.options();

    if let Some(ResolvedOption {
        name: "uuid",
        value: ResolvedValue::String(uuid),
        ..
    }) = options.get(0)
    {
        client_rpc
            .start_instance(ProcessInstanceArgs {
                uuid: uuid.to_string(),
                created_by: interaction.user.id.to_string(),
            })
            .await?;

        let message = CreateInteractionResponseMessage::new()
            .content(format!("Started instance : {}", uuid.to_string()))
            .ephemeral(true);

        interaction
            .create_response(ctx, CreateInteractionResponse::Message(message))
            .await
            .unwrap();

        Ok(())
    } else {
        Err("option error".into())
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("start")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "uuid", "Instance UUID")
                .required(true),
        )
        .description("Start instance by uuid")
}
