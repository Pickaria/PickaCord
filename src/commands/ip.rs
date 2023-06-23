use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    interaction
        .create_interaction_response(&ctx, |m| {
            m.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.ephemeral(true);
                    message.content("L'adresse du serveur est `play.pickaria.fr` !")
                })
        })
        .await
        .unwrap();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ip")
        .description("Affiche l'adresse IP du serveur")
}
