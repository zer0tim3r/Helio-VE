use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use uuid::Uuid;

use crate::{CreateInstanceArgs, GRPClient};

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = ctx.data.read().await;
    let mut client_rpc = data.get::<GRPClient>().unwrap().clone();

    let options = interaction.data.options();

    let uuid = Uuid::new_v4();

    let itype = match options.get(0).unwrap().value {
        ResolvedValue::Integer(d) => d,
        _ => 0,
    };

    let image = match options.get(1).unwrap().value {
        ResolvedValue::Integer(d) => d,
        _ => 0,
    };

    let mut message = CreateInteractionResponseMessage::new().ephemeral(false);

    match client_rpc
        .create_instance(CreateInstanceArgs {
            uuid: uuid.to_string(),
            label: None,
            itype: itype as i32,
            image: image as i32,
            created_by: interaction.user.id.to_string(),
        })
        .await
    {
        Ok(_) => message = message.content(format!("Created instance : {}", uuid.to_string())),
        Err(e) => message = message.content(format!("Cannot create instance : {}", e.message())),
    }

    interaction
        .create_response(ctx, CreateInteractionResponse::Message(message))
        .await
        .unwrap();

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("create")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "type", "Instance Type")
                .add_int_choice("(1 vCPU, 1GiB Memory) p1.micro", 0)
                .add_int_choice("(1 vCPU, 2GiB Memory) p1.small", 1)
                .add_int_choice("(2 vCPU, 4GiB Memory) p1.medium", 2)
                .add_int_choice("(4 vCPU, 8GiB Memory) p1.large", 3)
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "image", "Instance Image")
                .add_int_choice("Ubuntu 24.04 (Noble)", 0)
                .add_int_choice("Debian 12 (Bookworm)", 1)
                .add_int_choice("Rocky Linux 9.4", 2)
                .add_int_choice("Arch Linux (Latest)", 3)
                .required(true),
        )
        .description("Create instance")
}
