use github_flows::{get_octo, listen_to_event, EventPayload};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use slack_flows::send_message_to_channel;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::*;

use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event(
        "jaykchen",
        "vitesse-lite",
        vec!["issue_comment", "issues", "created"],
        handler,
    )
    .await;

    Ok(())
}

async fn handler(payload: EventPayload) {
    let owner = "jaykchen";
    let repo = "vitesse-lite";

    let octo = get_octo(Some(String::from("jaykchen")));
    let issues = octo.issues(owner, repo);

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            if e.comment.user.r#type != "Bot" {
                if let Some(b) = e.comment.body {
                    send_message_to_channel("ik8", "general", b);
                    // if let Some(r) = chat_completion(&b) {
                    //     if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                    //         println!("Error: {}", e.to_string());
                    //     }
                    // }
                }
            }
        }
        _ => (),
    };
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub new_conversation: bool,
    pub choice: String,
}

impl Default for ChatResponse {
    fn default() -> ChatResponse {
        ChatResponse {
            new_conversation: true,
            choice: String::new(),
        }
    }
}

pub fn chat_completion(prompt: &str) -> Option<ChatResponse> {
    let api_token = env::var("OPENAI_API_TOKEN").unwrap();

    // let prompt = "How can I reply to comment on Issues at GitHub repository with rest API?";

    let params = serde_json::json!({
                "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.7,
        "top_p": 1,
        "n": 1,
        "stream": false,
        "max_tokens": 512,
        "presence_penalty": 0,
        "frequency_penalty": 0,
    });

    let uri = "https://api.openai.com/v1/chat/completions";

    let uri = Uri::try_from(uri).unwrap();
    let mut writer = Vec::new();
    let body_str = params.to_string();
    let body = body_str.as_bytes();
    match Request::new(&uri)
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Content-Length", &body.len())
        .body(&body)
        .send(&mut writer)
    {
        Ok(res) => {
            if !res.status_code().is_success() {
                return None;
            }
            serde_json::from_slice::<ChatResponse>(&writer).ok()
        }
        Err(_) => None,
    }

    // let response = reqwest::Client::new()
    //     .post()
    //     .header("Content-Type", "application/json")
    //     .header(
    //         "Authorization",
    //         "Bearer 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",
    //     )
    //     // .header("content-length", len)
    //     .json(&params)
    //     // .send(&mut writer)
    //     .send()
    //     .await
    //     .expect("send");

    // let text = response.text().await.unwrap();
    // // let text = String::from_utf8(&response).unwrap_or("failed to parse response".to_string());
    // let raw: ChatResponse = serde_json::from_str(&text).unwrap();
    // let answer = raw.choices[0].message.content.to_string();

    // println!("Answer: {}", answer);
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ChatResponse {
//     created: i64,
//     choices: Vec<Choice>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Choice {
//     message: Message,
//     index: i64,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Message {
//     role: String,
//     content: String,
// }
