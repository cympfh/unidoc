# Fully Example Document - [[https://github.com/cympfh/unidoc]]

## Styled text

Emphasis *this* or _this_ is italic.
Strong **this** or __this__ is bold.
Using `***`, ***both***.

[Link](//cympfh.cc) and ![image](https://cympfh.cc/favicon.ico).
Link can have image as text: [![](https://cympfh.cc/favicon.ico)](//cympfh.cc)

~~This is deleted.~~

## List, Task List

1. list
1. tasks:
    - [ ] TODO
    - [x] DONE

### Deep Nested List

- ul
    + child
        * mago
            1. ol
                - a. alphabet

## Table

`mindoc` adopts common (or GitHub) style Table Notation.

| A | B | Left | Center | Right |
|---| - |:-----|:---:| --: |
| a | b | left | c | r |
|1|2|3|4|5|

Tables without headers

| A | B | C |
| a | b | c |
| 1 | 2 | 3 |

## Codes

Inline code is `like this`.
Code block are expressed with \`\`\`.

```
def main():
    pass
```

This is python code. You can express the language name:

```python
def main():
    pass
```

This block is styled with [Prism.js](https://prismjs.com/#basic-usage).

## Paragraph.

Almost plaintexts are interpreted as paragraph.
Paragraphs are separated with one or more empty lines.

Any    whitespaces (including newlines) are
used to	tokenize and   become just one space.
If you wish make a newline explicitly  
you must put two spaces `  `  at the end of line.

> Quote is another paragraph.
> Ofcource, you can use **any markdown** in quoting.

## (Ex) MathJax

```markdown
$inline-math-tex$
$$display-math-tex$$
```

When $a=2$, solve
$$\sum_{n=1}^\infty (x-a)^n = 1.$$

## (Ex) Import Another Markdown

```markdown
@(list.md)
```

@(list.md)

## (Ex) Import Another as a Code block

```markdown
@[rust](sample.rs)
```

@[rust](sample.rs)

## (Ex) Inner HyperLink

`[[url]]` makes a nicely link: [[http://example.com]].
The text for link is the `<title>` of the web page.

If something error while fetching web page, url will be used alternatively.
[[http://this.is.not.existing-site.co.com.tokyo.jp]]

## (Ex) Block HyperLink a.k.a BlogCard

{{ https://cympfh.cc/taglibro/2022/03/31 }}

{{https://www.youtube.com/watch?v=_FKRL-t8aM8}}

{{https://twitter.com/Jack/status/20}}
