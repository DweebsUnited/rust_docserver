use axum::{response::Html, Extension};
use sqlx::SqlitePool;

use crate::errors::AppError;

pub async fn get_all_tags(Extension(pool): Extension<SqlitePool>)
    -> Result<Html<String>, AppError> {

    let tags = crate::controllers::tag::get_all_tags(&pool).await?;

    let templ_env = super::RELOADER.acquire_env().unwrap();
    let templ = templ_env.get_template("taglist.tmpl").unwrap();
    let r = templ.render(minijinja::context!(taglist => tags))?;

    Ok(Html(r))

}