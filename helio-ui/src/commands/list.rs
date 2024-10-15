use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{GRPClient, InstanceState, ListInstanceArgs, ListInstanceResult};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = ctx.data.read().await;
    let mut client_rpc = data.get::<GRPClient>().unwrap().clone();

    let result = match client_rpc
        .list_instance(ListInstanceArgs {
            created_by: interaction.user.id.to_string(),
        })
        .await
    {
        Ok(v) => v.into_inner(),
        Err(_) => ListInstanceResult { instances: vec![] },
    };

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
                .field(
                    "State",
                    match i.state {
                        0 => Some(InstanceState::InstanceNone),
                        1 => Some(InstanceState::InstanceRunning),
                        2 => Some(InstanceState::InstanceSuspended),
                        _ => None,
                    }
                    .unwrap_or(InstanceState::InstanceNone)
                    .as_str_name(),
                    false,
                ),
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
