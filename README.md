# Pufferfish

Pufferfish is an extensible html templating engine that generates raw html, meaning that it will not affect load times of websites.

A full-blown javascript framework is sometimes a bit overkill for a static website. Pufferfish adds some simple templating to html so you don't have to use such a framework for small projects or for pages that require fast loading. Pufferfish will compile your files to raw html.

**Pufferfish is still under heavy development**

**The documentation is currently out of date with new releases**

[To the documentation!](https://pufferfish.jonaseveraert.be)

<details>
    <summary>Table of contents</summary>

- [Overview](#overview)
    - [Syntax](#syntax)
    - [Setting up a Pufferfish project](#setting-up-a-pufferfish-project)
    - [Compiling html](#compiling-html)
    - [Config file](#config-file)
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

### Setting up a Pufferfish project

The recommended folder structure for a Pufferfish project is the followng: 

```
project_name
├── html
├── output
├── templates
└── pufferfish.toml
```

- `html`: contains the html of your website (like the file shown in the [Syntax](#syntax) section as example).
- `output`: contains the output of pufferfish.
- `templates`: contains the files to be used as templates (e.g. `menu.html` in the example)
- `Pufferfish.toml`: the configuration file for the project

These directories can also be set using the [config file](#config-file).

#### Compiling html
Inside of the directory of the project, run `puf build`. This will take into account the the [config file](#config-file) and build html to the output directory.

### Config file

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

## Further documentation
Go to the dedicated [documentation](pufferfish.jonaseveraert.be/docs) or [tutorial](pufferfish.jonaseveraert.be/tutorial) for more information on Pufferfish (**Currently not maintained due to refactor to Rust**).

## Download

You can download Pufferfish with the following command:

```bash
curl "https://raw.githubusercontent.com/Jomy10/pufferfish/master/installation/install.sh" | sh
```

Or, you can download the Pufferfish from **npm**:

```bash
npm i -g pufferfish-html
```

Test if the package was installed using `puf --version`.

### Manual installation
Head over to the [Github releases](https://github.com/Jomy10/pufferfish/releases/latest) page and download the correct build for your operatin system. You now have an executable which can bee moved to the correct directory.

### Manual compilation
Pufferfish can be compiled for any platform, to do so, copy this repository:

```bash
git clone https://github.com/Jomy10/pufferfish.git
```

Then, go into the directory containing the project:

```bash
cd pufferfish/pufferfish
```

Run `cargo build --release` and the executable will be put in the `target` directory.

<!--
If you want to, you can download it from npm `npm i pufferfish-html -g`, but I would recommend getting the Gems version.
-->

## Integrations
Pufferfish is made so it can be included in other build processes. It also includes integrations with [htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier) and [minify-html](https://crates.io/crates/minify-html) (and [minify-html-onepass](https://crates.io/crates/minify-html-onepass)).

## Contributing
Contributions are always welcome. Read the [CONTRIBUTING](.github/CONTRIBUTING.md) file for more information!

## License
Pufferfish is licensed under the [MIT license](LICENSE).

© Jonas Everaert 2022
