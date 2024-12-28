use std::error::Error;

use serenity::all::*;
use tracing::{error, info};

mod commands;

struct Handler;

#[allow(dead_code)]
async fn delete_all_commands(ctx: &Context) -> Result<(), Box<dyn Error>> {
    for command in Command::get_global_commands(&ctx.http).await? {
        Command::delete_global_command(&ctx.http, command.id).await?;
        info!("Deleted global command {} commands!", command.name);
    }

    let id: u64 = 533362856297496596;
    let guild_id = GuildId::new(id);
    let guild = Guild::get(ctx, guild_id).await.unwrap();
    let commands = guild.get_commands(ctx).await.unwrap();
    for command in commands {
        let _ = guild.delete_command(ctx, command.id).await;
        info!("Deleted guild command {} commands!", command.name);
    }

    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Ok(guild) = Guild::get(&ctx.http, new_member.guild_id).await {
            if let Some(system_channel) = guild.system_channel_id {
                let message = CreateMessage::new()
                    .content(format!("{} a rejoint le serveur", new_member.mention()));
                let _ = system_channel.send_message(&ctx.http, message).await;
            }
        }
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        if let Ok(guild) = Guild::get(&ctx.http, guild_id).await {
            if let Some(system_channel) = guild.system_channel_id {
                let message = CreateMessage::new().content(format!(
                    "{} a quitté le serveur",
                    user.global_name.unwrap_or(user.name)
                ));
                let _ = system_channel.send_message(&ctx.http, message).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        let activity = ActivityData::playing("Minecraft");
        ctx.set_activity(Some(activity));

        if let Err(err) = Command::create_global_command(&ctx.http, commands::ip::register()).await
        {
            error!("An error has occurred while registering a command: {err}")
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "ip" => {
                    commands::ip::run(&ctx, &command).await;
                }
                _ => {
                    commands::not_found::run(&ctx, &command).await;
                }
            };
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "pickacord=debug");
    }
    tracing_subscriber::fmt::init();

    let intents = GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
