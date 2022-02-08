"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[561],{3905:function(e,t,n){n.d(t,{Zo:function(){return p},kt:function(){return f}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function l(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function a(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var c=r.createContext({}),u=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):l(l({},t),e)),n},p=function(e){var t=u(e.components);return r.createElement(c.Provider,{value:t},e.children)},s={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,i=e.originalType,c=e.parentName,p=a(e,["components","mdxType","originalType","parentName"]),m=u(n),f=o,d=m["".concat(c,".").concat(f)]||m[f]||s[f]||i;return n?r.createElement(d,l(l({ref:t},p),{},{components:n})):r.createElement(d,l({ref:t},p))}));function f(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=n.length,l=new Array(i);l[0]=m;var a={};for(var c in t)hasOwnProperty.call(t,c)&&(a[c]=t[c]);a.originalType=e,a.mdxType="string"==typeof e?e:o,l[1]=a;for(var u=2;u<i;u++)l[u]=n[u];return r.createElement.apply(null,l)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},1657:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return a},contentTitle:function(){return c},metadata:function(){return u},toc:function(){return p},default:function(){return m}});var r=n(7462),o=n(3366),i=(n(7294),n(3905)),l=["components"],a={title:"Quick look",sidebar_position:2},c="Quick look",u={unversionedId:"quick_look",id:"quick_look",title:"Quick look",description:"Pufferfish allows you to use templates inside of your html and compile it to regular html.",source:"@site/docs/quick_look.md",sourceDirName:".",slug:"/quick_look",permalink:"/docs/quick_look",editUrl:"https://github.com/jomy10/pufferfish/tree/master/documentation/docs/quick_look.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{title:"Quick look",sidebar_position:2},sidebar:"tutorialSidebar",previous:{title:"Installation",permalink:"/docs/intro"},next:{title:"Compiling html",permalink:"/docs/compiling_html"}},p=[{value:"Example",id:"example",children:[],level:3}],s={toc:p};function m(e){var t=e.components,n=(0,o.Z)(e,l);return(0,i.kt)("wrapper",(0,r.Z)({},s,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"quick-look"},"Quick look"),(0,i.kt)("p",null,"Pufferfish allows you to use templates inside of your html and compile it to regular html."),(0,i.kt)("h3",{id:"example"},"Example"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-html"},"<html>\n    <head>\n        \x3c!--...--\x3e\n    </head>\n    <body>\n        <h1>The text below was inserted by Pufferfish!</h1>\n        %myTemplate%\n    </body>\n</html>\n")),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-html",metastring:"myTemplate.html","myTemplate.html":!0},"<p>I am inserted by Pufferfish!</p>\n")),(0,i.kt)("p",null,(0,i.kt)("strong",{parentName:"p"},"Results after compiling"),":"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-html"},"<html>\n    <head>\n        \x3c!--...--\x3e\n    </head>\n    <body>\n        <h1>The text below was inserted by Pufferfish!</h1>\n        <p>I am inserted by Pufferfish!</p>\n    </body>\n</html>\n")),(0,i.kt)("p",null,"I hope I have peaked your interest here, go ahead and continue learning by going to the next chapter!"))}m.isMDXComponent=!0}}]);