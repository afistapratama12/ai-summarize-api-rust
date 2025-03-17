use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RegisterRequest {
  pub name: String,
  pub email: String,
  pub password: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SummaryRequest {
  pub file_id: String,
  pub file_ext: String,
  pub input_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SummarizationRequest {
  pub model: String,
  pub prompt: String,
  pub max_tokens: usize,
}