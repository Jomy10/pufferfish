---
title: Syntax
sidebar_position: 2
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

*In the future, Pufferfish will support passing variabls to html. Pufferfish is still in early development. If you have any suggestions for its future, please suggest them by opening an issue!*