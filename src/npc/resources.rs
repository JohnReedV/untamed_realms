use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum Weather {
    #[default]
    Sunny,
    Rainy,
    Stormy,
}

#[derive(Resource, Default)]
pub struct WorldState {
    pub time: f32,
    pub weather: Weather,
    pub entities: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct LanguageModelAPI {
    pub api_key: String,
    pub base_url: String,
}

impl LanguageModelAPI {
    pub async fn send_request(&self, input: String) -> reqwest::Result<String> {
        let client = reqwest::Client::new();
        let response = client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&input)
            .send()
            .await?;

        let result: String = response.json().await?;
        Ok(result)
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PlayerAnimationTimer(Timer);
