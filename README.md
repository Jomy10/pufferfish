# Pufferfish

Pufferfish is an extensible html templating engine that generates raw html, meaning that it will not affect load times of websites.

A full-blown javascript framework is sometimes a bit overkill for a static website. Pufferfish adds some simple templating to html so you don't have to use such a framework for small projects or for pages that require fast loading. Pufferfish will compile your files to raw html.

[To the documentation!](https://pufferfish.jonaseveraert.be)

<details>
    <summary>Table of contents</summary>

- [Overview](#overview)
    - [Syntax](#syntax)
    - [Compiling html](#compiling-your-html)
    - [Build file](#build-file)
- [Download](#download)
- [Integrations](#integrations)
- [Contributing](#contributing)
- [License](#license)
</details>

## Overview
### Syntax
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

### Compiling your html
You can either use the cli, or the [build file](#build-file) for compiling html. The latter is the easiest and most complete.

**Usage:**
```bash
puf <filename> [output_filename] -d [template_dir] -p
```

- `filename`: the file name of your html to be compiled
- `output_filename`: the file name of the compiled html file (default: stdout)
- `-d`: specifies a directory where pufferfish will look for templates. (default: .)
- `-p`: prettify html

### Build file
Pufferfish also includes a build file you can specify.

You can name the file anyway you want. In the following example, it will be called `Config.rb`.

*Config.rb*
```ruby
require 'pufferfish'

Pufferfish::Builder.new(lambda { |b|
    b.html_dir = "html" # default: .
    b.template_dir = "templates" # default: .
    b.output_dir = "output" # default: .
    b.pretty = false # default: false
    b.minify = true # default: false
    b.minify_flags = "--collapse-whitespace --remove-comments --minify-css true --minify-js true --case-sensitive" # default: ""
    b.verbose = true # default: false
})
```

- `html_dir`: the directory where you the files will live that will be compiled to raw html.
- `template_dir`: the directory where pufferfish will look for templates you use inside of your html.
- `output_dir`: the directory where pufferfish will put the compiled html.
- `pretty`: if set to true, the html will be prettified first.
- `minify`: if set to true, the html will be minified first, requires that you have [html-minifier](https://github.com/kangax/html-minifier) installed. You can do this with `npm install html-minifier -g` (you might have to run it as sudo).
- `minify_flags`: specify the flags for the minify command. All flags can be found [here](https://github.com/kangax/html-minifier). If no options are specified, almost nothing will happen. The above example shows a good starting point.
- `verbose`: will show you what's going on during compilation.

To compile your html, run
```bash
ruby Config.rb
```

## Download
You can get Pufferfish from ruby gems.
```bash
gem install pufferfish
```

You can also download it from GitHub packages:
```bash
gem install pufferfish --source "https://rubygems.pkg.github.com/jomy10"
```

<!--
If you want to, you can download it from npm `npm i pufferfish-html -g`, but I would recommend getting the Gems version.
-->

## Integrations
Pufferfish is made so it can be included in other build processes. It also includes integrations with [htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier) and [html-minifier](https://github.com/kangax/html-minifier) when using the [build file](#build-file).

## Contributing
Contributions are always welcome. Read the [CONTRIBUTING](.github/CONTRIBUTING.md) file for more information!

## License
Pufferfish is licensed under the [MIT license](LICENSE).

© Jonas Everaert 2022
