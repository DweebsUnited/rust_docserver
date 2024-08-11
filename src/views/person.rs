use axum::{response::Html, Extension};
use sqlx::SqlitePool;

use crate::errors::AppError;

pub async fn get_all_people(Extension(pool): Extension<SqlitePool>)
    -> Result<Html<String>, AppError> {

    let people = crate::controllers::person::get_all_people(&pool).await?;

    let templ_env = super::RELOADER.acquire_env().unwrap();
    let templ = templ_env.get_template("personlist.tmpl").unwrap();
    let r = templ.render(minijinja::context!(personlist => people))?;

    Ok(Html(r))

}