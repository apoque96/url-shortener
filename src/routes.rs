use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use url::Url;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UrlStruct {
    long_url: String,
    short_url: String,
    valid: bool,
}

impl UrlStruct {
    pub fn new() -> UrlStruct {
        UrlStruct {
            long_url: String::from(""),
            short_url: String::from(""),
            valid: true,
        }
    }

    fn save_url(&mut self, long_url: String) {
        let short_url = hash(&long_url);
        self.long_url = long_url;
        self.short_url = short_url;
        self.valid = true;
    }

    fn invalid_url(&mut self) {
        self.long_url = String::from("");
        self.short_url = String::from("");
        self.valid = false;
    }
}

fn hash(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hashed_value = hasher.finish();
    format!("{:x}", hashed_value)
}

#[get("/")]
pub fn index(state: &State<Mutex<UrlStruct>>) -> Template {
    let mut url = state.inner().lock().unwrap();
    let render = Template::render(
        "index",
        context![
            long_url: &url.long_url,
            short_url: &url.short_url,
            valid: &url.valid.clone(),
        ],
    );
    url.valid = true;
    render
}

#[get("/upload_url?<long_url>")]
pub fn new_url(long_url: String, state: &State<Mutex<UrlStruct>>) -> Redirect {
    let mut url = state.inner().lock().unwrap();
    match Url::parse(long_url.as_str()) {
        Ok(_) => {
            url.save_url(long_url);
        }
        Err(e) => {
            println!("{}", e);
            url.invalid_url();
        }
    }

    Redirect::to(uri!(index))
}
