---
title: Quick look
sidebar_position: 2
---

# Quick look
Pufferfish allows you to use templates inside of your html and compile it to regular html.

### Example
```html
<html>
    <head>
        <!--...-->
    </head>
    <body>
        <h1>The text below was inserted by Pufferfish!</h1>
        %myTemplate%
    </body>
</html>
```

```html myTemplate.html
<p>I am inserted by Pufferfish!</p>
```

**Result after compiling**:
```html
<html>
    <head></head>
    <body>
        <h1>The text below was inserted by Pufferfish!</h1>
        <p>I am inserted by Pufferfish!</p>
    </body>
</html>
```

I hope I have peaked your interest here, go ahead and continue learning by going to the next chapter!