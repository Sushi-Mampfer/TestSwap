use std::{fs::File, io::Write, path::Path};

use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};

use crate::api::datatypes::Upload;

pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    let mut files = Vec::new();
    let mut data = Upload {
        name: "".to_string(),
        description: "".to_string(),
        teacher: "".to_string(),
        subject: "".to_string(),
        year: "".to_string(),
    };


    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");

        match name {
            "files" => {
                let file_type = Path::new(field.file_name().unwrap()).extension().unwrap().to_string_lossy().to_string();
                if file_type.is_empty() {
                    return (StatusCode::BAD_REQUEST, "File type is required".to_string());
                }
                if file_type != "png" && file_type != "jpg" && file_type != "jpeg" {
                    return (StatusCode::BAD_REQUEST, "Only images are allowed".to_string());
                }
                let file_bytes = field.bytes().await.unwrap();
                files.push((file_type, file_bytes));
            }
            "name" => {
                data.name = field.text().await.unwrap_or_default();
            }
            "description" => {
                data.description = field.text().await.unwrap_or_default();
            }
            "teacher" => {
                data.teacher = field.text().await.unwrap_or_default();
            }
            "subject" => {
                data.subject = field.text().await.unwrap_or_default();
            }
            "year" => {
                data.year = field.text().await.unwrap_or_default();
            }
            _ => println!("Unknown field: {}", name),
        }
    }
    if data.name.is_empty() || data.description.is_empty() || data.teacher.is_empty() || data.subject.is_empty() || data.year.is_empty() || files.is_empty() {
        return (StatusCode::BAD_REQUEST, "All fields are required".to_string());
    }
    let mut last = files[0].0.clone();
    for i in &files {
        if i.0 != last {
            return (StatusCode::BAD_REQUEST, "All files must be the same type.".to_string());
        }
        last = i.0.clone();
    }
    let files_len = files.len() as i64;
    let id = sqlx::query!(
        "INSERT INTO tests (name, description, teacher, subject, year, files, extension) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id",
        data.name,
        data.description,
        data.teacher,
        data.subject,
        data.year,
        files_len,
        last
    ).fetch_one(&*crate::DB).await.unwrap();
    for i in 0..files_len {
        File::create(format!("files/{}_{}.{}", id.id, i, files[i as usize].0)).unwrap()
            .write_all(&files[i as usize].1).unwrap();
    }
    (StatusCode::OK, id.id.to_string())
}