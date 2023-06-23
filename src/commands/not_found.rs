use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    interaction
        .create_interaction_response(&ctx, |m| {
            m.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.ephemeral(true);
                    message.content("Cette commande n'existe pas !")
                })
        })
        .await
        .unwrap();
}
