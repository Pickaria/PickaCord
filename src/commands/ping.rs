use serenity::builder::CreateApplicationCommand;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::Timestamp;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    interaction
        .create_interaction_response(&ctx, |m| {
            m.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.title("Title");
                    message.custom_id("3");
                    message.ephemeral(true);
                    message
                        .content("Choose an animal")
                        .components(|c| {
                            c.create_action_row(|row| {
                                // An action row can only contain one select menu!
                                row.create_select_menu(|menu| {
                                    menu.custom_id("animal_select");
                                    menu.placeholder("No animal selected");
                                    menu.options(|f| {
                                        f.create_option(|o| o.label("🐈 meow").value("Cat"));
                                        f.create_option(|o| o.label("🐕 woof").value("Dog"));
                                        f.create_option(|o| o.label("🐎 neigh").value("Horse"));
                                        f.create_option(|o| {
                                            o.label("🦙 hoooooooonk").value("Alpaca")
                                        });
                                        f.create_option(|o| o.label("🦀 crab rave").value("Ferris"))
                                    })
                                })
                            });
                            c.create_action_row(|row| {
                                row.create_button(|button| {
                                    button.style(ButtonStyle::Primary);
                                    button.custom_id("Primary");
                                    button.label("Primary")
                                });
                                row.create_button(|button| {
                                    button.style(ButtonStyle::Secondary);
                                    button.custom_id("Secondary");
                                    button.label("Secondary")
                                });
                                row.create_button(|button| {
                                    button.style(ButtonStyle::Success);
                                    button.custom_id("Success");
                                    button.label("Success")
                                });
                                row.create_button(|button| {
                                    button.style(ButtonStyle::Danger);
                                    button.custom_id("Danger");
                                    button.label("Danger")
                                });
                                row.create_button(|button| {
                                    button.style(ButtonStyle::Link);
                                    button.url("https://www.pickaria.fr");
                                    button.label("Link")
                                })
                            })
                        })
                        .embed(|e| {
                            e.title("This is a title")
                                .description("This is a description")
                                .image("attachment://ferris_eyes.png")
                                .fields(vec![
                                    ("This is the first field", "This is a field body", true),
                                    ("This is the second field", "Both fields are inline", true),
                                ])
                                .field(
                                    "This is the third field",
                                    "This is not an inline field",
                                    false,
                                )
                                .footer(|f| f.text("This is a footer"))
                                // Add a timestamp for the current time
                                // This also accepts a rfc3339 Timestamp
                                .timestamp(Timestamp::now())
                        })
                })
        })
        .await
        .unwrap();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
