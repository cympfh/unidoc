use easy_scraper::Pattern;
use reqwest;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct WebPage {
    url: String,
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
        Self { url, content }
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

    /// Convert relative URL to absolute URL based on the page's base URL
    pub fn absolute_url(&self, relative_url: &str) -> String {
        if relative_url.starts_with("http://") || relative_url.starts_with("https://") {
            // Already absolute
            relative_url.to_string()
        } else if relative_url.starts_with("/") {
            // Absolute path, need to get the base URL (scheme + host)
            if let Some(base_url) = self.get_base_url() {
                format!("{}{}", base_url, relative_url)
            } else {
                relative_url.to_string()
            }
        } else {
            // Relative path - not implemented for now
            relative_url.to_string()
        }
    }

    /// Extract base URL (scheme + host) from the page URL
    fn get_base_url(&self) -> Option<String> {
        // Parse URL to extract scheme and host
        // e.g., "https://arxiv.org/abs/2502.14541" -> "https://arxiv.org"
        if let Some(scheme_end) = self.url.find("://") {
            let after_scheme = &self.url[scheme_end + 3..];
            if let Some(path_start) = after_scheme.find('/') {
                Some(format!("{}{}", &self.url[..scheme_end + 3], &after_scheme[..path_start]))
            } else {
                // No path, entire URL is the base
                Some(self.url.clone())
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_webpage {
    use super::*;

    #[test]
    fn test_get_base_url() {
        let webpage = WebPage {
            url: String::from("https://arxiv.org/abs/2502.14541"),
            content: None,
        };
        assert_eq!(webpage.get_base_url(), Some(String::from("https://arxiv.org")));

        let webpage2 = WebPage {
            url: String::from("https://example.com"),
            content: None,
        };
        assert_eq!(webpage2.get_base_url(), Some(String::from("https://example.com")));
    }

    #[test]
    fn test_absolute_url() {
        let webpage = WebPage {
            url: String::from("https://arxiv.org/abs/2502.14541"),
            content: None,
        };

        // Test relative path starting with /
        assert_eq!(
            webpage.absolute_url("/static/browse/0.3.4/images/arxiv-logo-fb.png"),
            "https://arxiv.org/static/browse/0.3.4/images/arxiv-logo-fb.png"
        );

        // Test already absolute URL
        assert_eq!(
            webpage.absolute_url("https://example.com/image.png"),
            "https://example.com/image.png"
        );

        // Test http URL
        assert_eq!(
            webpage.absolute_url("http://example.com/image.png"),
            "http://example.com/image.png"
        );
    }
}
