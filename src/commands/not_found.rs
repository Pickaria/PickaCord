use serenity::all::*;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Cette commande n'existe pas !")
                    .ephemeral(true),
            ),
        )
        .await;
}
