"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[525],{3905:function(e,t,n){n.d(t,{Zo:function(){return c},kt:function(){return m}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var p=r.createContext({}),u=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},c=function(e){var t=u(e.components);return r.createElement(p.Provider,{value:t},e.children)},s={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,i=e.originalType,p=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),f=u(n),m=o,d=f["".concat(p,".").concat(m)]||f[m]||s[m]||i;return n?r.createElement(d,a(a({ref:t},c),{},{components:n})):r.createElement(d,a({ref:t},c))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=n.length,a=new Array(i);a[0]=f;var l={};for(var p in t)hasOwnProperty.call(t,p)&&(l[p]=t[p]);l.originalType=e,l.mdxType="string"==typeof e?e:o,a[1]=l;for(var u=2;u<i;u++)a[u]=n[u];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},1563:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return l},contentTitle:function(){return p},metadata:function(){return u},toc:function(){return c},default:function(){return f}});var r=n(7462),o=n(3366),i=(n(7294),n(3905)),a=["components"],l={title:"Syntax",sidebar_position:2},p=void 0,u={unversionedId:"syntax",id:"syntax",title:"Syntax",description:"To include a template file inside of html, simply write %filename%.",source:"@site/docs/syntax.md",sourceDirName:".",slug:"/syntax",permalink:"/docs/syntax",editUrl:"https://github.com/jomy10/pufferfish/documentation/docs/syntax.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{title:"Syntax",sidebar_position:2},sidebar:"tutorialSidebar",previous:{title:"Installation",permalink:"/docs/intro"},next:{title:"Compiling html",permalink:"/docs/compiling_html"}},c=[],s={toc:c};function f(e){var t=e.components,n=(0,o.Z)(e,a);return(0,i.kt)("wrapper",(0,r.Z)({},s,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,"To include a template file inside of html, simply write ",(0,i.kt)("inlineCode",{parentName:"p"},"%filename%"),"."),(0,i.kt)("p",null,(0,i.kt)("strong",{parentName:"p"},"Example")),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-html"},"<html>\n  <body>\n    %menu%\n    %header.html%\n    %footer.handlebars%\n  </body>\n</html>\n")),(0,i.kt)("p",null,"If the filename does not include a file extension, ",(0,i.kt)("inlineCode",{parentName:"p"},".html")," will be used. You can also specify files with other file extensions."),(0,i.kt)("p",null,"When compiled, the html above will expand to include the ",(0,i.kt)("inlineCode",{parentName:"p"},"menu.html"),", ",(0,i.kt)("inlineCode",{parentName:"p"},"header.html")," and ",(0,i.kt)("inlineCode",{parentName:"p"},"footer.handlebars")," file contents."),(0,i.kt)("p",null,(0,i.kt)("em",{parentName:"p"},"In the future, Pufferfish will support passing variabls to html. Pufferfish is still in early development. If you have any suggestions for its future, please suggest them by opening an issue!")))}f.isMDXComponent=!0}}]);