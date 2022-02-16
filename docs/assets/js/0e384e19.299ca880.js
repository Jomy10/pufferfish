"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[671],{3905:function(e,t,n){n.d(t,{Zo:function(){return c},kt:function(){return m}});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var s=r.createContext({}),u=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},c=function(e){var t=u(e.components);return r.createElement(s.Provider,{value:t},e.children)},f={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},p=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),p=u(n),m=a,d=p["".concat(s,".").concat(m)]||p[m]||f[m]||o;return n?r.createElement(d,i(i({ref:t},c),{},{components:n})):r.createElement(d,i({ref:t},c))}));function m(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,i=new Array(o);i[0]=p;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:a,i[1]=l;for(var u=2;u<o;u++)i[u]=n[u];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}p.displayName="MDXCreateElement"},9881:function(e,t,n){n.r(t),n.d(t,{frontMatter:function(){return l},contentTitle:function(){return s},metadata:function(){return u},toc:function(){return c},default:function(){return p}});var r=n(7462),a=n(3366),o=(n(7294),n(3905)),i=["components"],l={title:"Installation",sidebar_position:1},s="Pufferfish",u={unversionedId:"intro",id:"intro",title:"Installation",description:"Pufferfish is an extensible html templating engine that generates raw html, meaning that it will not affect load times of websites.",source:"@site/docs/intro.md",sourceDirName:".",slug:"/intro",permalink:"/docs/intro",editUrl:"https://github.com/jomy10/pufferfish/tree/master/documentation/docs/intro.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{title:"Installation",sidebar_position:1},sidebar:"tutorialSidebar",next:{title:"Quick look",permalink:"/docs/quick_look"}},c=[{value:"Download",id:"download",children:[{value:"Manual installation",id:"manual-installation",children:[],level:3},{value:"Manual compilation",id:"manual-compilation",children:[],level:3}],level:2}],f={toc:c};function p(e){var t=e.components,n=(0,a.Z)(e,i);return(0,o.kt)("wrapper",(0,r.Z)({},f,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"pufferfish"},"Pufferfish"),(0,o.kt)("p",null,"Pufferfish is an extensible html templating engine that generates raw html, meaning that it will not affect load times of websites."),(0,o.kt)("p",null,"A full-blown javascript framework is sometimes a bit overkill for a static website. Pufferfish adds some simple templating to html so you don't have to use such a framework for small projects or for pages that require fast loading. Pufferfish will compile your files to raw html."),(0,o.kt)("h2",{id:"download"},"Download"),(0,o.kt)("p",null,"You can download Pufferfish with the following command:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},'curl "https://raw.githubusercontent.com/Jomy10/pufferfish/master/installation/install.sh" | sh\n')),(0,o.kt)("p",null,"Or, you can download Pufferfish from ",(0,o.kt)("strong",{parentName:"p"},"npm"),":"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"npm i -g pufferfish-html\n")),(0,o.kt)("p",null,"Test if the package was installed using ",(0,o.kt)("inlineCode",{parentName:"p"},"puf --version"),"."),(0,o.kt)("h3",{id:"manual-installation"},"Manual installation"),(0,o.kt)("p",null,"You can also install Pufferfish manually:"),(0,o.kt)("p",null,"Head over to the ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/Jomy10/pufferfish/releases/latest"},"Github releases")," page and download the correct build for your operatin system. You now have an executable which can bee moved to the correct directory."),(0,o.kt)("h3",{id:"manual-compilation"},"Manual compilation"),(0,o.kt)("p",null,"Pufferfish can be compiled for any platform, to do so, copy this repository:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"git clone https://github.com/Jomy10/pufferfish.git\n")),(0,o.kt)("p",null,"Then, go into the directory containing the project:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-bash"},"cd pufferfish/pufferfish\n")),(0,o.kt)("p",null,"Run ",(0,o.kt)("inlineCode",{parentName:"p"},"cargo build --release")," and the executable will be put in the ",(0,o.kt)("inlineCode",{parentName:"p"},"target")," directory."))}p.isMDXComponent=!0}}]);