use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{GRPClient, ListInstanceArgs};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = ctx.data.read().await;
    let mut client_rpc = data.get::<GRPClient>().unwrap().clone();

    let result = client_rpc
        .list_instance(ListInstanceArgs {
            created_by: interaction.user.id.to_string(),
        })
        .await?
        .into_inner();

    let mut message = CreateInteractionResponseMessage::new()
        .content(format!("My Instances : {}", result.instances.len()))
        .ephemeral(true);

    for i in result.instances {
        message = message.add_embed(
            CreateEmbed::new()
                .field("UUID", i.uuid.clone(), false)
                .field("Type", i.itype.to_string(), false)
                .field("Image", i.image.to_string(), false)
                .field("MAC", i.mac.clone(), false)
                .field("IPv4", i.ipv4.clone(), false)
                .field("CreatedAt", i.created_at.unwrap().to_string(), false)
        );
    }

    interaction
        .create_response(ctx, CreateInteractionResponse::Message(message))
        .await
        .unwrap();

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("list").description("List all instance")
}
