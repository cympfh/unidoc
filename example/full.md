# :book: Full Example for [unidoc](https://github.com/cympfh/unidoc) :magnet:

## Styled Text

```markdown
*emphasis* == _italic_
**strong** == __bold__
***emphasis & strong***
```

Single `*` or `_` quotings are *emphasis* (or _italic_).
Double `**` or `__` quotings are **strong** (or __bold__).
Triple `***` can make ***Emphasis and Strong***.

```markdown
~~This is deleted.~~
```

Double `~~` quotings are ~~deleted~~ (or striked).

## Links, Images

```markdown
[link text](url)
![image alt](image url or path)

The `link text` can be an image.
[![alt](https://cympfh.cc/favicon.ico)](//cympfh.cc)
```

[![alt](https://cympfh.cc/favicon.ico)](//cympfh.cc)

## List, Task List

```markdown
- Unordered lists
- ...
  + The bullets can be one of `-` `+` `*`
  + ...

1. Ordered lists
1. The bullets can be one of `1.` `a.`
```

TODO checkboxes can be put.

```markdown
- Tasks:
  - [ ] TODO
  - [x] DONE
```

- Tasks:
  - [ ] TODO
  - [x] DONE

## Table

Github-style Table Notation can use.

```markdown
| A | B | Left | Center | Right |
|---| - |:-----|:---:| --: |
| a | b | left | c   | r   |
| 1 | 2 | 3    | 4   | 5   |
```

| A | B | Left | Center | Right |
|---| - |:-----|:---:| --: |
| a | b | left | c   | r   |
| 1 | 2 | 3    | 4   | 5   |

Headers can be omitted.

```markdown
| A | B | C |
| a | b | c |
| 1 | 2 | 3 |
```

| A | B | C |
| a | b | c |
| 1 | 2 | 3 |

## Codes

```markdown
`inline code` ```

`unidoc` is written in `Rust`.

(Fenced) Code block are quoted with \`\`\`.

```
def main():
    pass
```

This is a python code.
You can express the language name:

```python
def main():
    pass
```

When `--standalone, -s`, the code blocks (with language names) are styled with [Prism.js](https://prismjs.com/#basic-usage).
Also see [Supported languages](https://prismjs.com/#supported-languages)
as available language names.

## Paragraph

Almost plaintexts are interpreted as paragraph.
Paragraphs are separated with one or more empty lines.

Any    whitespaces (including newlines) are
used to	tokenize and   become just one space.
If you wish make a newline explicitly  
you must put two spaces `  `  at the end of line.

> Quote is another paragraph.
> Ofcource, you can use **any markdown** in quoting.

---

## (Ex) MathJax

```markdown
$inline-math-tex$
$$display-math-tex$$
```

When $a=2$, solve
$$\sum_{n=1}^\infty (x-a)^n = 1.$$

## (Ex) Import Another Markdown

```markdown
@(another markdown file path)
```

@(list.md)

## (Ex) Import Another as a Code block

```markdown
@[language name](file path)
@[rust](sample.rs)
```

@[rust](sample.rs)

## (Ex) Inline HyperLink

```markdown
[[url]]
```

`[[url]]` makes a nicely link: [[http://example.com]].
The text for link is the `<title>` of the web page (web is required).

If something error while fetching web page, url will be used alternatively.
(:point_right: [[http://this.is.not.existing-site.co.com.tokyo.jp]])

## (Ex) Block HyperLink a.k.a BlogCard

```markdown
{{url}}
```

This generates nicely blogcard.
OGP metadata are used (web is required).
Some urls have special forms for embedding (e.g. Youtube, Twitter).

**NOTE**:
This is a paragraph block.
Each paragraphs must be delimited with empty lines in markdown text.

{{https://example.com}}

{{https://www.youtube.com/watch?v=_FKRL-t8aM8}}

{{https://twitter.com/Jack/status/20}}

## :joy: Emojis

```markdown
:(emoji-shortcode):

:+1: :joy: :cry:
```

Github emoji shortcodes are available :v:
Awesome cheatsheet is [here](https://github.com/ikatyang/emoji-cheat-sheet/blob/master/README.md) :point_left:

## Executor

The code blocks can accept some special language names (@-prefixed),
and can execute the code.

| Executor Name | Output                    | Command       | Description               |
|:--------------|:--------------------------|:--------------|:--------------------------|
| `@bash`       | Text (stdout)             | `bash`        | Run as a shell code       |
| `@dot`        | PNG data (base64 encoded) | `dot -Tpng`   | graphviz dot              |
| `@gnuplot`    | SVG                       | `gnuplot`     | svg terminal will be used |

```@bash
date
yes | head -n 3
```

```@dot
digraph {
  this -> works
}
```

```@gnuplot
f(x) = sin(x) / x
plot f(x)
```
