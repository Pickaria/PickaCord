use std::env;
use std::error::Error;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::guild::Guild;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::model::prelude::{Activity, User};
use serenity::prelude::*;

mod commands;

struct Handler;

#[allow(dead_code)]
async fn delete_all_commands(ctx: Context) -> Result<(), Box<dyn Error>> {
    for command in Command::get_global_application_commands(&ctx.http).await? {
        Command::delete_global_application_command(&ctx.http, command.id).await?
    }

    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Ok(guild) = Guild::get(&ctx.http, new_member.guild_id).await {
            if let Some(system_channel) = guild.system_channel_id {
                system_channel
                    .send_message(&ctx.http, |m| {
                        m.content(format!("{} a rejoint le serveur", new_member.mention()))
                    })
                    .await
                    .unwrap();
            }
        }
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, kicked: User) {
        if let Ok(guild) = Guild::get(&ctx.http, guild_id).await {
            if let Some(system_channel) = guild.system_channel_id {
                system_channel
                    .send_message(&ctx.http, |m| {
                        m.content(format!("{} a quitté le serveur", kicked.mention()))
                    })
                    .await
                    .unwrap();
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::playing("Minecraft")).await;

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::register(command));
            commands.create_application_command(|command| commands::ip::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            match command.data.name.as_str() {
                "ping" => {
                    commands::ping::run(&ctx, &command).await;
                }
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
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
