---
title: Compiling html
sidebar_position: 3
---

You can either use the cli, or the [build file](#build-file) for compiling html. The latter is the easiest and most complete.

## Build file
Pufferfish also includes a build file you can specify.

You can name the file anyway you want. In the following example, it will be called `Config.rb`.

```ruby title=Config.rb
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

## CLI

**Usage:**
```bash
puf <filename> [output_filename] -d [template_dir] -p
```

- `filename`: the file name of your html to be compiled
- `output_filename`: the file name of the compiled html file (default: stdout)
- `-d`: specifies a directory where pufferfish will look for templates. (default: .)
- `-p`: prettify html