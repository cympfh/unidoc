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
        static ref SOUNDCLOUD: Regex =
            Regex::new(r"https://soundcloud.com/([^/]*)/([a-zA-Z0-9\-_]*)").unwrap();
    }
    if let Some(caps) = YOUTUBE.captures(&url) {
        let id = caps.get(1).unwrap().as_str();
        leaf!("<div class=\"youtube\" src-id=\"{}\"></div>", id)
    } else if TWITTER.is_match(&url) {
        leaf!("<blockquote class=\"twitter-tweet\"><p lang=\"ja\" dir=\"ltr\"></p>
                            <a href=\"{}\"></a></blockquote>
                            <script async src=\"https://platform.twitter.com/widgets.js\" charset=\"utf-8\"></script>",
                            url)
    } else if let Some(caps) = SOUNDCLOUD.captures(&url) {
        let user_id = caps.get(1).unwrap().as_str().to_string();
        blogcard_soundcloud(url, user_id)
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

#[derive(Serialize)]
struct SoundCloudContext {
    url: String,
    title: String,
    user_id: String,
    sound_id: String,
}

fn blogcard_soundcloud(url: String, user_id: String) -> Html {
    let webpage = WebPage::new(url.to_string());
    let title = webpage.meta("og:title").unwrap_or(url.to_string());
    lazy_static! {
        static ref INTENT: Regex = Regex::new(r"soundcloud://sounds:([0-9]*)").unwrap();
    }
    let sound_id = webpage
        .meta("al:ios:url")
        .map(|link| {
            INTENT
                .captures(&link)
                .map(|caps| caps.get(1).unwrap().as_str().to_string())
        })
        .flatten()
        .unwrap();
    const TEMPLATE: &str = r#"
<iframe width="100%" height="300" scrolling="no" frameborder="no" allow="autoplay" src="https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/{{sound_id}}&color=%23ff5500&auto_play=false&hide_related=false&show_comments=true&show_user=true&show_reposts=false&show_teaser=true&visual=true"></iframe><div style="font-size: 10px; color: #cccccc;line-break: anywhere;word-break: normal;overflow: hidden;white-space: nowrap;text-overflow: ellipsis; font-family: Interstate,Lucida Grande,Lucida Sans Unicode,Lucida Sans,Garuda,Verdana,Tahoma,sans-serif;font-weight: 100;"><a href="https://soundcloud.com/{{user_id}}" title="{{user_id}}" target="_blank" style="color: #cccccc; text-decoration: none;">{{user_id}}</a> · <a href="{{url}}" title="{{title}}" target="_blank" style="color: #cccccc; text-decoration: none;">{{title}}</a></div>
    "#;
    let reg = Handlebars::new();
    let context = SoundCloudContext {
        url,
        title,
        user_id,
        sound_id,
    };
    let html = reg.render_template(TEMPLATE, &context).unwrap();
    Html::Leaf(html)
}
