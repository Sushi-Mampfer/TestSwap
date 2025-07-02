use axum::{extract, http::StatusCode, response::IntoResponse};
use sqlx::{QueryBuilder, Row};

use crate::api::datatypes::{Search, Test};

pub async fn search(extract::Json(data): extract::Json<Search>) -> impl IntoResponse {
    let mut query = QueryBuilder::<sqlx::Sqlite>::new("SELECT * FROM tests WHERE 1=1");

    if let Some(name) = &data.name {
        query.push(" AND name LIKE ");
        query.push_bind(format!("%{}%", name));
    }

    if let Some(description) = &data.description {
        query.push(" AND description LIKE ");
        query.push_bind(format!("%{}%", description));
    }

    if let Some(teacher) = &data.teacher {
        query.push(" AND teacher LIKE ");
        query.push_bind(format!("%{}%", teacher));
    }

    if let Some(subject) = &data.subject {
        query.push(" AND subject LIKE ");
        query.push_bind(format!("%{}%", subject));
    }

    if let Some(year) = &data.year {
        query.push(" AND year = ");
        query.push_bind(year);
    }

    let results = query.build().fetch_all(&*crate::DB).await.unwrap();
    let return_data: Vec<Test> = results.into_iter().map(|row| Test {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        teacher: row.get("teacher"),
        subject: row.get("subject"),
        year: row.get("year"),
        files: row.get("files"),
        extension: row.get("extension"),
    }).collect();
    (StatusCode::OK, axum::Json(return_data))
}