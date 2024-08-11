use axum::{response::Html, Extension};
use sqlx::SqlitePool;

use crate::errors::AppError;

pub async fn get_all_types(Extension(pool): Extension<SqlitePool>)
    -> Result<Html<String>, AppError> {

    let types = crate::controllers::r#type::get_all_types(&pool).await?;

    let templ_env = super::RELOADER.acquire_env().unwrap();
    let templ = templ_env.get_template("typelist.tmpl").unwrap();
    let r = templ.render(minijinja::context!(typelist => types))?;

    Ok(Html(r))

}