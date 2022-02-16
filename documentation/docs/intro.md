---
title: Installation
sidebar_position: 1
---

# Pufferfish

Pufferfish is an extensible html templating engine that generates raw html, meaning that it will not affect load times of websites.

A full-blown javascript framework is sometimes a bit overkill for a static website. Pufferfish adds some simple templating to html so you don't have to use such a framework for small projects or for pages that require fast loading. Pufferfish will compile your files to raw html.

## Download
You can download Pufferfish with the following command:

```bash
curl "https://raw.githubusercontent.com/Jomy10/pufferfish/master/installation/install.sh" | sh
```

Or, you can download Pufferfish from **npm**:

```bash
npm i -g pufferfish-html
```

Test if the package was installed using `puf --version`.

### Manual installation
You can also install Pufferfish manually:


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
