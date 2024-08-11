use minijinja_autoreload::AutoReloader;
use once_cell::sync::Lazy;

pub static RELOADER: Lazy<AutoReloader> = Lazy::new(|| AutoReloader::new(|notifier| {
    let template_path = "resources/template";

    let mut env = minijinja::Environment::new();

    env.set_loader(minijinja::path_loader(template_path));
    notifier.watch_path(template_path, true);
    Ok(env)
}));

pub mod doc;
pub mod tag;
pub mod r#type;
pub mod person;