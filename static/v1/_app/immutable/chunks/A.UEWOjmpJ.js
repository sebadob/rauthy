import"./disclose-version.BDr9Qe-U.js";import{p as d,c as u,r as c,t as v,a as W,u as x,O as U}from"./runtime.BONl2e88.js";import{a as b,t as E}from"./template.B7ldVCvc.js";import{s as P}from"./snippet.D6O6IDbh.js";import{s,t as S}from"./index.BJiFQ8Di.js";import{p as a}from"./props.BlbrNjWf.js";import{p as e}from"./index.HLl6XeHK.js";var $=E('<a class="font-label svelte-vdcj5m"><!></a>');function j(h,r){d(r,!0);let f=a(r,"selectedStep",3,!1),l=a(r,"hideUnderline",3,!1),p=a(r,"highlight",3,!1),o=a(r,"highlightExact",3,!1),m=a(r,"highlightWithParams",3,!1),n=U(()=>{if(f())return"step";let i=e.route.id;if(m()){if(`${i}${e.url.search}`.startsWith(r.href))return"page"}else if(o()){if(i===r.href.split("?")[0])return"page"}else if(i&&r.href.split("?")[0].endsWith(i))return"page"});var t=$(),g=u(t);P(g,()=>r.children),c(t),v(()=>{s(t,"href",r.href),s(t,"target",r.target),s(t,"aria-current",x(n)),s(t,"data-highlight",p()),S(t,"hideUnderline",l())}),b(h,t),W()}export{j as A};
