# unidoc: Unite all Markdown.

## Install & Usage

```sh
$ cargo install unidoc

$ cat example.md | unidoc
$ unidoc -i example.md -o output.md

$ unidoc --help  # for more detail
```

## Syntax

### Text

```markdown
*emphasis*
_emphasis_
**strong**
__strong__
***Emphasis and Strong***
~~deleted~~

![alt](image-link-Path-or-URL)

[text](link)

`inline code`

<!-- this is hidden comment -->
```

### Headings

```markdown
# h1
## h2
...
###### h6
```

### List

```markdown
- one
- two
- three

- any bullet is ok
    + plus
        * astersk

1. ordered with numbers
2. hgoehoge

a. with alphabets
a. hogehoge

- task list
    - [ ] not yet
    - [x] already done
```

### Paragraph, Quoting

Text blocks separated with empty lines are paragraphs.

```markdown
this is paragraph1.
this is paragraph1.
this is paragraph1.

this is paragraph2.  this is paragraph2.
this is paragraph2.

> This is quoted.  This is quoted.
> This is quoted.
```

### Code block

````markdown
```
def code():
    return code
```

```python
def code():
    return code
```
````

### Table

```markdown
| A | B |   C   |   D |
|---|:--| :----:| --: |
| a | b |   c   |  d  |
| 1 | 2 |   3   |  4  |
```

### Hr

```markdown
---
```

## Extended Syntax

### Import Another Markdown

```markdown
@(./relative/another.md)
```

### Import Code

```markdown
@[rust](./sample.rs)
```

### Hyperlink

```markdown
This is inline hyperlink: [[http://example.com]].
```

```markdown
Hyperlink as a block generates Blogcard:

{{ https://cympfh.cc/ }}
```
