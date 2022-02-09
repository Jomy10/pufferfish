"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[869],{3905:function(e,t,i){i.d(t,{Zo:function(){return m},kt:function(){return s}});var n=i(7294);function l(e,t,i){return t in e?Object.defineProperty(e,t,{value:i,enumerable:!0,configurable:!0,writable:!0}):e[t]=i,e}function r(e,t){var i=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),i.push.apply(i,n)}return i}function a(e){for(var t=1;t<arguments.length;t++){var i=null!=arguments[t]?arguments[t]:{};t%2?r(Object(i),!0).forEach((function(t){l(e,t,i[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(i)):r(Object(i)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(i,t))}))}return e}function o(e,t){if(null==e)return{};var i,n,l=function(e,t){if(null==e)return{};var i,n,l={},r=Object.keys(e);for(n=0;n<r.length;n++)i=r[n],t.indexOf(i)>=0||(l[i]=e[i]);return l}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(n=0;n<r.length;n++)i=r[n],t.indexOf(i)>=0||Object.prototype.propertyIsEnumerable.call(e,i)&&(l[i]=e[i])}return l}var u=n.createContext({}),p=function(e){var t=n.useContext(u),i=t;return e&&(i="function"==typeof e?e(t):a(a({},t),e)),i},m=function(e){var t=p(e.components);return n.createElement(u.Provider,{value:t},e.children)},f={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},c=n.forwardRef((function(e,t){var i=e.components,l=e.mdxType,r=e.originalType,u=e.parentName,m=o(e,["components","mdxType","originalType","parentName"]),c=p(i),s=l,d=c["".concat(u,".").concat(s)]||c[s]||f[s]||r;return i?n.createElement(d,a(a({ref:t},m),{},{components:i})):n.createElement(d,a({ref:t},m))}));function s(e,t){var i=arguments,l=t&&t.mdxType;if("string"==typeof e||l){var r=i.length,a=new Array(r);a[0]=c;var o={};for(var u in t)hasOwnProperty.call(t,u)&&(o[u]=t[u]);o.originalType=e,o.mdxType="string"==typeof e?e:l,a[1]=o;for(var p=2;p<r;p++)a[p]=i[p];return n.createElement.apply(null,a)}return n.createElement.apply(null,i)}c.displayName="MDXCreateElement"},3153:function(e,t,i){i.r(t),i.d(t,{frontMatter:function(){return o},contentTitle:function(){return u},metadata:function(){return p},toc:function(){return m},default:function(){return c}});var n=i(7462),l=i(3366),r=(i(7294),i(3905)),a=["components"],o={title:"Compiling html",sidebar_position:3},u=void 0,p={unversionedId:"compiling_html",id:"compiling_html",title:"Compiling html",description:"You can either use the cli, or the build file for compiling html. The latter is the easiest and most complete.",source:"@site/docs/compiling_html.md",sourceDirName:".",slug:"/compiling_html",permalink:"/docs/compiling_html",editUrl:"https://github.com/jomy10/pufferfish/tree/master/documentation/docs/compiling_html.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{title:"Compiling html",sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"Quick look",permalink:"/docs/quick_look"},next:{title:"Syntax",permalink:"/docs/syntax/"}},m=[{value:"Build file",id:"build-file",children:[],level:2},{value:"CLI",id:"cli",children:[],level:2}],f={toc:m};function c(e){var t=e.components,i=(0,l.Z)(e,a);return(0,r.kt)("wrapper",(0,n.Z)({},f,i,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"You can either use the cli, or the ",(0,r.kt)("a",{parentName:"p",href:"#build-file"},"build file")," for compiling html. The latter is the easiest and most complete."),(0,r.kt)("h2",{id:"build-file"},"Build file"),(0,r.kt)("p",null,"Pufferfish includes a build file. You can name the file anyway you want. In the following example, it will be called ",(0,r.kt)("inlineCode",{parentName:"p"},"Config.rb"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-ruby",metastring:"title=Config.rb",title:"Config.rb"},'require \'pufferfish\'\n\nPufferfish::Builder.new(lambda { |b|\n    b.html_dir = "html" # default: .\n    b.template_dir = "templates" # default: .\n    b.output_dir = "output" # default: .\n    b.pretty = false # default: false\n    b.minify = true # default: false\n    b.minify_flags = "--collapse-whitespace --remove-comments --minify-css true --minify-js true --case-sensitive" # default: ""\n    b.verbose = true # default: false\n})\n')),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"html_dir"),": the directory where you the files will live that will be compiled to raw html."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"template_dir"),": the directory where pufferfish will look for templates you use inside of your html."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"output_dir"),": the directory where pufferfish will put the compiled html."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"pretty"),": if set to true, the html will be prettified first."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"minify"),": if set to true, the html will be minified first, requires that you have ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/kangax/html-minifier"},"html-minifier")," installed. You can do this with ",(0,r.kt)("inlineCode",{parentName:"li"},"npm install html-minifier -g")," (you might have to run it as sudo)."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"minify_flags"),": specify the flags for the minify command. All flags can be found ",(0,r.kt)("a",{parentName:"li",href:"https://github.com/kangax/html-minifier"},"here"),". If no options are specified, almost nothing will happen. The above example shows a good starting point."),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"verbose"),": will show you what's going on during compilation.")),(0,r.kt)("p",null,"To compile your html, run"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"ruby Config.rb\n")),(0,r.kt)("h2",{id:"cli"},"CLI"),(0,r.kt)("p",null,"The CLI is a good fit if you just want to compile a single file. It does not contain a minify option, though."),(0,r.kt)("p",null,(0,r.kt)("strong",{parentName:"p"},"Usage:")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"puf <filename> [output_filename] -d [template_dir] -p\n")),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"filename"),": the file name of your html to be compiled"),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"output_filename"),": the file name of the compiled html file (default: stdout)"),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"-d"),": specifies a directory where pufferfish will look for templates. (default: .)"),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"-p"),": prettify html")))}c.isMDXComponent=!0}}]);