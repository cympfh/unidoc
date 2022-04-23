use crate::entity::html::Html;
use crate::leaf;
use crate::webpage::WebPage;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

pub fn blogcard(url: String) -> Html {
    lazy_static! {
        static ref YOUTUBE: Regex = Regex::new(r"youtube.com/watch\?v=([^&]+)$").unwrap();
        static ref TWITTER: Regex = Regex::new(r"twitter.com/.*/status/([0-9])+").unwrap();
    }
    if let Some(caps) = YOUTUBE.captures(&url) {
        let id = caps.get(1).unwrap().as_str();
        leaf!("<div class=\"youtube\" src-id=\"{}\"></div>", id)
    } else if TWITTER.is_match(&url) {
        leaf!("<blockquote class=\"twitter-tweet\"><p lang=\"ja\" dir=\"ltr\"></p>
                            <a href=\"{}\"></a></blockquote>
                            <script async src=\"https://platform.twitter.com/widgets.js\" charset=\"utf-8\"></script>",
                            url)
    } else {
        blogcard_general(url)
    }
}

#[derive(Serialize)]
struct BlogContext {
    url: String,
    title: String,
    description: String,
    image: String,
    site: String,
}

impl BlogContext {
    fn new(url: String, title: String, description: String, image: String) -> Self {
        let site = url.replace("http://", "").replace("https://", "");
        Self {
            url,
            title,
            description,
            image,
            site,
        }
    }
}

fn blogcard_general(url: String) -> Html {
    let webpage = WebPage::new(url.to_string());
    let title: String = webpage
        .meta("og:title")
        .unwrap_or_else(|| webpage.title().unwrap_or(url.to_string()));
    let image = webpage.meta("og:image").unwrap_or(String::new());
    let description = webpage.meta("og:description").unwrap_or(String::new());
    const TEMPLATE: &str = r#"
<div class="blogcard" style="width:auto;max-width:9999px;border:1px solid #E0E0E0;border-radius:3px;margin:10px 0;padding:15px;line-height:1.4;text-align:left;background:#FFFFFF;">
<a href="{{url}}" target="_blank" style="display:block;text-decoration:none;">
<span class="blogcard-image" style="float:right;width:100px;padding:0 0 0 10px;margin:0 0 5px 5px;">
<img src="{{image}}" width="100" style="width:100%;height:auto;max-height:100px;min-width:0;border:0 none;margin:0;" />
</span><br style="display:none">
<span class="blogcard-title" style="font-size:112.5%;font-weight:700;color:#333333;margin:0 0 5px 0;">{{title}}</span><br />
<span class="blogcard-content" style="font-size:87.5%;font-weight:400;color:#666666;">{{description}}</span><br />
<span style="clear:both;display:block;overflow:hidden;height:0;">&nbsp;</span>
</a><div style="font-size:75%;text-align:right;clear:both"><a href="{{url}}" target="_blank" rel="nofollow">{{site}}</a></div></div>
    "#;
    let reg = Handlebars::new();
    let context = BlogContext::new(url, title, description, image);
    let html = reg.render_template(TEMPLATE, &context).unwrap();
    Html::Leaf(html)
}
