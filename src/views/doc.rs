use axum::{response::Html, Extension};
use sqlx::SqlitePool;

use crate::errors::AppError;

pub async fn get_all_docs(Extension(pool): Extension<SqlitePool>)
    -> Result<Html<String>, AppError> {

    let docs = crate::controllers::doc::get_all_docs(&pool).await?;

    let templ_env = super::RELOADER.acquire_env().unwrap();
    let templ = templ_env.get_template("doclist.tmpl").unwrap();
    let r = templ.render(minijinja::context!(doclist => docs))?;

    Ok(Html(r))

}