---
title: Nested templates
sidebar_position: 1
---

# Nested templates
You can nest temlates as much as you want, pufferfish will always look in your templates to expand other templates.

## Example
Consider the following file structure:
```
Project
└ html
  └ index.html
└ templates
  └ footer
    └ contact.html
    └ social.html
    └ copyright.html
  └ footer.html
└ output
└ Config.rb
```

This folder structure is the recommended way to structure a project. The `Config.rb` file is the build file explained in the [Compiling html](https://pufferfish.jonaseveraert.be/docs/compiling_html#build-file) chapter, in this example we are using the exact same file.

Our files are the following:
```html title=index.html
<html>
    <head></head>
    <body>
        <main>
            <p>This is my main content!</p>
        </main>
        %footer%
    </body>
</html>
```

```html title=footer.html
%footer/contact%
%footer/social%
%footer/copyright%
```

The example above will replace `%footer%` in our html with
```html
<!--Contents of contact.html-->
<!--Contents of social.html-->
<!--Contents of copyright.html-->
```
