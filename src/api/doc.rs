use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::extract::Path;

use sqlx::SqlitePool;

use crate::controllers;
use crate::models;
use crate::errors::AppError;

pub async fn get_all_docs(Extension(pool): Extension<SqlitePool>)
    -> Result<(StatusCode, Json<Vec<models::Doc>>), AppError> {

    let docs = controllers::doc::get_all_docs(&pool)
        .await?;

    Ok((StatusCode::OK, Json(docs)))

}

pub async fn get_doc(Path(id): Path<i32>, Extension(pool): Extension<SqlitePool>)
    -> Result<(StatusCode, Json<models::Doc>), AppError> {

    let doc = controllers::doc::get_doc(&pool, id)
        .await?;

    Ok((StatusCode::OK, Json(doc)))
}

pub async fn post_doc(Extension(pool): Extension<SqlitePool>, Json(doc): Json<models::DocAPI>)
    -> Result<(StatusCode, Json<models::Doc>), AppError> {

    if doc.id.is_some() {
        return Err(AppError::BadRequest(String::from("Use PUT to update a document")));
    }

    let doc = controllers::doc::post_doc(&pool, doc)
        .await?;

    Ok((StatusCode::CREATED, Json(doc)))

}

pub async fn put_doc(Extension(pool): Extension<SqlitePool>, Json(doc): Json<models::DocAPI>)
    -> Result<(StatusCode, Json<models::Doc>), AppError> {

    if doc.id.is_none() {
        return Err(AppError::BadRequest(String::from("Use POST to insert new document")));
    }

    let doc = controllers::doc::put_doc(&pool, doc)
        .await?;

    Ok((StatusCode::ACCEPTED, Json(doc)))

}