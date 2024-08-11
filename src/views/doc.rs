use axum::{response::Html, Extension};
use minijinja::render;
use minijinja_autoreload::AutoReloader;
use sqlx::SqlitePool;

use crate::errors::AppError;

use once_cell::sync::Lazy;

// TODO: Cache all the templates in memory, rather than reading from disk each time..

static RELOADER: Lazy<AutoReloader> = Lazy::new(|| AutoReloader::new(|notifier| {
    let template_path = "resources/template";

    let mut env = minijinja::Environment::new();

    env.set_loader(minijinja::path_loader(template_path));
    notifier.watch_path(template_path, true);
    Ok(env)
}));


pub async fn get_all_docs(Extension(pool): Extension<SqlitePool>)
    -> Result<Html<String>, AppError> {

    let docs = crate::controllers::doc::get_all_docs(&pool).await?;

    let templ_env = RELOADER.acquire_env().unwrap();
    let templ = templ_env.get_template("doclist.tmpl").unwrap();
    let r = templ.render(minijinja::context!(doclist => docs))?;

    Ok(Html(r))

}