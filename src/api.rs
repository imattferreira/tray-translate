use std::collections::HashMap;

async fn api() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("http://localhost:3000")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(())
}

pub async fn get_translation_of(
    phrase: String,
    i_lang: String,
    o_lang: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} - from {} to {}", phrase, i_lang, o_lang);

    let _ = api().await;

    Ok(())
}
