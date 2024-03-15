use serenity::all::*;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("L'adresse du serveur est `play.pickaria.fr` !")
                    .ephemeral(true),
            ),
        )
        .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ip").description("Affiche l'adresse IP du serveur.")
}
