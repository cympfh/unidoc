use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Context {
    title: String,
    body: String,
    css: Vec<String>,
    js: Vec<String>,
    mathjax: String,
    headers: Vec<String>,
    befores: Vec<String>,
    afters: Vec<String>,
    variable: HashMap<String, String>,
}

impl Context {
    pub fn new(
        title: String,
        body: String,
        css: Vec<String>,
        js: Vec<String>,
        mathjax: String,
        headers: Vec<String>,
        befores: Vec<String>,
        afters: Vec<String>,
        variable: HashMap<String, String>,
    ) -> Self {
        Self {
            title,
            body,
            css,
            js,
            mathjax,
            headers,
            befores,
            afters,
            variable,
        }
    }
}

const DEFAULT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
  <meta http-equiv="Content-Style-Type" content="text/css" />
  <meta name="generator" content="unidoc" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes">
  <title>{{{title}}}</title>
  <style type="text/css">code{white-space: pre;}</style>
  <link rel="stylesheet" href="https://cympfh.cc/resources/css/youtube.css" />
  <link href="https://unpkg.com/prismjs@1/themes/prism.css" rel="stylesheet" />
  <script id="MathJax-script" async src="https://unpkg.com/mathjax@3/es5/{{{mathjax}}}.js"></script>
  {{#each css}}
  <link href="{{this}}" rel="stylesheet" />
  {{/each}}
  {{#each js}}
  <script src="{{this}}"></script>
  {{/each}}
{{#each headers}}{{{this}}}{{/each}}
</head>
<body>
{{#each befores}}{{{this}}}{{/each}}
{{{body}}}
{{#each afters}}{{{this}}}{{/each}}
  <script src="https://cympfh.cc/resources/js/youtube.js"></script>
  <script src="https://unpkg.com/prismjs@1/components/prism-core.min.js"></script>
  <script src="https://unpkg.com/prismjs@1/plugins/autoloader/prism-autoloader.min.js"></script>
</body>
</html>"#;

pub fn get_default_template() -> &'static str {
    DEFAULT_TEMPLATE
}

pub fn simple(context: Context) -> Result<String, RenderError> {
    let reg = Handlebars::new();
    reg.render_template(DEFAULT_TEMPLATE, &context)
}

pub fn custom(htmltemplate: &str, context: Context) -> Result<String, RenderError> {
    let reg = Handlebars::new();
    reg.render_template(htmltemplate, &context)
}
