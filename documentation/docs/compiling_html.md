---
title: Compiling html
sidebar_position: 4
---

Inside of the directory of the project, run `puf build`. This will take into account the the [config file](#config-file) and build html to the output directory.

## Config file

Every Pufferfish project needs a `pufferfish.toml` file.

The minimum config file looks like this:

```toml
[project]
```

Here is a complete config file with all possible settings and their default values:

```toml
[project]
html_dir = "html"
template_dir = "templates"
output_dir = "output"
dev_dir = "output" # Default: set to `output_dir`
pretty = false
minify = false
verbose = false
 
[minify]
method = "default" # values: (default | onepass)
minify_doctype = true
ensure_spec_compliant_unquoted_attribute_values = true
keep_closing_tags = true
keep_html_and_head_opening_tags = true
keep_spaces_between_attributes = false
keep_comments = false
minify_css = true
minify_js = true
remove_bangs = false
remove_processing_instructions = false

[server]
port = "8080"
```
