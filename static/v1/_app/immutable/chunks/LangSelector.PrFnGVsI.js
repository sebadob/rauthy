import{a as j,b as k}from"./disclose-version.C0dpEWi_.js";import{p as R,Z as S,d as p,b as c,a1 as y,a as A,g as i,j as f,a2 as B,a4 as C}from"./index-client.LkyNpeLM.js";import{L as d,w as O,x as l}from"./dataFetching.C4rEqQY1.js";import{p as m}from"./proxy.bVSoDcUp.js";import{p as n}from"./props.BDzDHVdA.js";import{O as E}from"./OptionSelect.CHnFxA-i.js";import{u as G}from"./language.svelte.C7Ul_DTA.js";import{a as H}from"./helpers.DVmWh-6p.js";var M=k('<div class="svelte-s2vujs"><!></div>');function P(v,t){R(t,!0);let b=n(t,"absolute",3,!1),u=n(t,"absoluteRight",3,!1),w=n(t,"updateBackend",3,!1);const L=";Path=/;SameSite=Lax;Max-Age=315360000";let e=G(),o=S(m(p(()=>e)));c(()=>{let a=H("locale"),r=p(()=>e);a!==r&&d.includes(a)&&(e=a,f(o,m(a)))}),c(()=>{i(o)&&i(o)!==e&&g(i(o))});async function g(a){if(document.cookie="locale="+a.toLowerCase()+L,w()){let r=await O();if(!r.ok){let h=await r.json();console.error(h);return}}window.location.reload()}var s=M(),x=B(s);E(x,{options:d,get value(){return i(o)},set value(a){f(o,m(a))}}),C(s),y(()=>{l(s,"absolute",b()),l(s,"absoluteLeft",!u()),l(s,"absoluteRight",u())}),j(v,s),A()}export{P as L};