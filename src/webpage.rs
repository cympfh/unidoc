use easy_scraper::Pattern;
use reqwest;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct WebPage {
    _url: String,
    content: Option<String>,
}

impl WebPage {
    pub fn new(url: String) -> Self {
        eprintln!("Fetching {}...", url);
        let content: Option<String> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .ok()
            .unwrap()
            .get(&url)
            .send()
            .ok()
            .map(|response| response.text().ok())
            .flatten();
        Self { _url: url, content }
    }
    pub fn title(&self) -> Option<String> {
        if let Some(content) = self.content.as_ref() {
            let p = Pattern::new("<title>{{title}}</title>").unwrap();
            let ms = p.matches(&content);
            if ms.is_empty() {
                None
            } else {
                ms[0].get("title").cloned()
            }
        } else {
            None
        }
    }
    pub fn meta(&self, property: &str) -> Option<String> {
        if let Some(content) = self.content.as_ref() {
            let attributes = vec!["property", "name"];
            for attr in attributes.iter() {
                let p = Pattern::new(&format!(
                    "<meta {}={} content={{{{content}}}} />",
                    attr, property
                ))
                .unwrap();
                let ms = p.matches(&content);
                if !ms.is_empty() {
                    return ms[0].get("content").cloned();
                }
            }
        }
        None
    }
}
