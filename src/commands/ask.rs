use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serenity::all::*;
use tracing::error;

#[derive(Serialize)]
struct Input {
    input: String,
}

#[derive(Serialize)]
struct InvokeRequest {
    input: Input,
}

#[derive(Deserialize)]
struct Output {
    answer: String,
}

#[derive(Deserialize)]
struct InvokeResponse {
    output: Output,
}

async fn invoke(question: String) -> Result<String, Box<dyn std::error::Error>> {
    let invoke_endpoint = std::env::var("INVOKE_ENDPOINT")
        .ok()
        .unwrap_or("http://127.0.0.1:8000/invoke".to_string());

    let client = reqwest::Client::new();

    let invoke_request = InvokeRequest {
        input: Input {
            input: question.to_string(),
        },
    };

    let x_api_key = std::env::var("X_API_KEY").ok().unwrap_or_default();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("x-api-key", HeaderValue::from_str(&x_api_key)?);

    let request = client
        .post(invoke_endpoint)
        .json(&invoke_request)
        .headers(headers)
        .send()
        .await?;

    let invoke_response = request.json::<InvokeResponse>().await?;

    Ok(invoke_response.output.answer)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
        )
        .await;

    let options = interaction.data.options();

    let content = if let Some(ResolvedOption {
        value: ResolvedValue::String(question),
        ..
    }) = options.first()
    {
        let response = invoke(question.to_string()).await;

        response.unwrap_or_else(|err| {
            error!("{}", err);
            "Une erreur est survenue !".to_string()
        })
    } else {
        "Merci de poser une question.".to_string()
    };

    let _ = interaction
        .create_followup(
            &ctx,
            CreateInteractionResponseFollowup::new().content(content),
        )
        .await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ask")
        .description("Poser une question à l'assistant virtuel de Pickaria.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "question",
                "La question à poser à l'assistant",
            )
            .required(true),
        )
}
