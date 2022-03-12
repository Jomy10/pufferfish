# Pufferfish 1.0 design features

## Templates

Inside of HTML

```html
%template_name%
%template_name?attr1="some string",attr2=<
    <div>
	<p>Some arbitrary HTML>
    </div>
>%
```

Attrs Inside of templates

```
<h1>My Template</h1>
<p>#attr1#</p>
#attr2#
```

## Folders

### templates
will be used to replace temlates inside of html

### assets
will be copied to output/assets.
Any arbitrary amount of folders can be specified in the config to be copied to the output.

### output
the folder where all generated files will be located


