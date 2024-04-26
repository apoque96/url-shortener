use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Url {
    long_url: String,
    short_url: String,
}

impl Url {
    pub fn new() -> Url {
        Url {
            long_url: String::from(""),
            short_url: String::from(""),
        }
    }

    fn save_url(&mut self, long_url: String) {
        let short_url = hash(&long_url);
        self.long_url = long_url;
        self.short_url = short_url;
    }
}

fn hash(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hashed_value = hasher.finish();
    format!("{:x}", hashed_value)
}

#[get("/")]
pub fn index(state: &State<Mutex<Url>>) -> Template {
    let url = state.inner().lock().unwrap();
    Template::render(
        "index",
        context![
            long_url: &url.long_url,
            short_url: &url.short_url,
        ],
    )
}

#[post("/upload_url", data = "<long_url>")]
pub fn new_url(long_url: String, state: &State<Mutex<Url>>) -> Redirect {
    let mut url = state.inner().lock().unwrap();
    url.save_url(long_url);
    Redirect::to(uri!(index))
}
