use std::{fs::File, io::Read};
use sea_orm::DatabaseConnection;
use warp::{
  filters::multipart::FormData,
  http::StatusCode,
  Rejection,
  Reply,
};
use uuid::Uuid;
use futures::TryStreamExt;
use pdf_extract;
use docx_rust::*;
use bytes::Buf;

use super::{
  request::SummaryRequest, 
  response::{SummarizeResponse, UploadResponse}
};
use super::response::{error_resp, success_resp};
use crate::{
  common::error::InternalServerError, 
  libs::{
    jwt::Claims,
    openai::chat_completion
  },
};

#[allow(unused_variables)]
pub async fn upload_file_handler(_claims: Claims, form: FormData, db: DatabaseConnection) -> Result<impl Reply, Rejection> {
  let mut parts = form.into_stream();

  let file_id = Uuid::new_v4().to_string().replace("-", "");
  let mut file_ext = String::new();

  while let Ok(Some(p)) = parts.try_next().await {
    if p.name() == "file" {
      let content_type = p.content_type();
      match content_type {
          Some(file_type) => match file_type {
            "application/pdf" => {
              file_ext = "pdf".to_string();
            }
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
              file_ext = "docx".to_string();
            }
            "text/plain" => {
              file_ext = "txt".to_string();
            }
            v => {
              eprintln!("invalid file type found: {}", v);
              return Err(warp::reject::custom(InternalServerError{message: "invalid file type".to_string()}));
            }
          },
          None => {
            eprintln!("file type could not be determined");
            return Err(warp::reject::custom(InternalServerError{message: "file type could not be determined".to_string()}));
          }
      }

      let value = p
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
            vec.extend_from_slice(&buf_to_vec(data));
            async move { Ok(vec) }
        })
        .await
        .map_err(|e| {
            eprintln!("reading file error: {}", e);
            warp::reject::reject()
        })?;

      let file_name = format!("./upload/{}.{}", file_id, file_ext.clone());
      tokio::fs::write(&file_name, value).await.map_err(|e| {
          eprint!("error writing file: {}", e);
          warp::reject::custom(InternalServerError{message: "error writing file".to_string()})
      })?;

      println!("created file: {}", file_name);
    }
  }

  let file_resp = UploadResponse {
    file_id,
    file_ext,
  };

  Ok(success_resp(warp::reply::json(&file_resp)))
}

// handler for summarizing the content of a file
#[allow(unused_variables)]
pub async fn summarize(_claims: Claims, req: SummaryRequest, db: DatabaseConnection) -> Result<impl Reply, Rejection> {  
  let file_ext = req.file_ext.clone();
  let file_name = format!("./upload/{}.{}", req.file_id, file_ext);

  let extracted_text = match file_ext.as_str() {
    "pdf" => extract_text_from_pdf(&file_name.clone()),
    "docx" => extract_text_from_docx(&file_name.clone()),
    "txt" => {
      let mut file = File::open(&file_name).unwrap();
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
      contents
    }
    _ => "Unsupported file type".to_string(),
  };

  if extracted_text.clone() == "Unsupported file type" {
    return Ok(error_resp("Unsupported file type", StatusCode::BAD_REQUEST));
  }

  let summary = chat_completion(&extracted_text.clone()).await.unwrap();

  // process the request here
  let summary_resp = SummarizeResponse{
    summary,
    content: extracted_text,
  };

  Ok(success_resp(warp::reply::json(&summary_resp)))
}
// internal function
fn extract_text_from_pdf(file_path: &str) -> String {
  match pdf_extract::extract_text(file_path) {
      Ok(text) => text,
      Err(_) => "Error extracting text from PDF".to_string(),
  }
}

fn extract_text_from_docx(file_path: &str) -> String {
  let docx = DocxFile::from_file(file_path).unwrap();
  let docx = docx.parse().unwrap();

  let extract_data = docx.document.body.text();
  extract_data
}

fn buf_to_vec(mut buf: impl Buf) -> Vec<u8> {
  let mut vec = Vec::with_capacity(buf.remaining());
  while buf.has_remaining() {
    vec.extend_from_slice(buf.chunk());
    buf.advance(buf.chunk().len());
  }
  vec
}
