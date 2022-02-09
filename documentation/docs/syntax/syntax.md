---
title: Syntax
sidebar_position: 1
---

To include a template file inside of html, simply write `%filename%`.

**Example**
```html
<html>
  <body>
    %menu%
    %header.html%
    %footer.handlebars%
  </body>
</html>
```

If the filename does not include a file extension, `.html` will be used. You can also specify files with other file extensions.

When compiled, the html above will expand to include the `menu.html`, `header.html` and `footer.handlebars` file contents.

These paths to these files is determined by the `template_dir` specified in the [build file](https://pufferfish.jonaseveraert.be/docs/compiling_html#build-file), or the `-d` tag in the [cli](https://pufferfish.jonaseveraert.be/docs/compiling_html#cli).

*In the future, Pufferfish will support passing variables to html. Pufferfish is still in early development. If you have any suggestions for its future, please suggest them by opening an issue!*

