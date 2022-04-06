use easy_scraper::Pattern;
use reqwest;

pub struct WebPage {
    _url: String,
    content: Option<String>,
}

impl WebPage {
    pub fn new(url: String) -> Self {
        let content: Option<String> = reqwest::blocking::get(&url)
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
    pub fn ogtitle(&self) -> Option<String> {
        if let Some(content) = self.content.as_ref() {
            let p = Pattern::new("<meta property=og:title content={{content}} />").unwrap();
            let ms = p.matches(&content);
            if ms.is_empty() {
                None
            } else {
                ms[0].get("content").cloned()
            }
        } else {
            None
        }
    }
    pub fn ogdescription(&self) -> Option<String> {
        if let Some(content) = self.content.as_ref() {
            let p = Pattern::new("<meta property=og:description content={{content}} />").unwrap();
            let ms = p.matches(&content);
            if ms.is_empty() {
                None
            } else {
                ms[0].get("content").cloned()
            }
        } else {
            None
        }
    }
    pub fn ogimage(&self) -> Option<String> {
        if let Some(content) = self.content.as_ref() {
            let p = Pattern::new("<meta property=og:image content={{content}} />").unwrap();
            let ms = p.matches(&content);
            if ms.is_empty() {
                None
            } else {
                ms[0].get("content").cloned()
            }
        } else {
            None
        }
    }
}
