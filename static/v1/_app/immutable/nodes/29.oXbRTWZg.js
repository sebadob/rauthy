import"../chunks/disclose-version.BDr9Qe-U.js";import{p as S,a3 as k,a as U,a5 as f,$ as V,g as i,j as z,a4 as a,a6 as B,a7 as t,a9 as e,a8 as W}from"../chunks/index-client.C1uYfxHd.js";import{s as r}from"../chunks/render.DlGhCWX2.js";import{a as c,t as E,b as X}from"../chunks/template.BehheIQn.js";import{h as Y}from"../chunks/svelte-head.76d9LV1R.js";import{p as j}from"../chunks/proxy.bZKuG6wn.js";import{L as Z}from"../chunks/LangSelector.Bntt9KZe.js";import{B as _}from"../chunks/Button.BCobGyYN.js";import{M as F,C as N}from"../chunks/ContentCenter.BU-nX73m.js";import{u as O}from"../chunks/i18n.svelte.C5UT6hSp.js";import{T as q}from"../chunks/Template.DxkxrmTY.js";import{t as aa,u as ta}from"../chunks/helpers.EoWe9H8J.js";import{T as ea}from"../chunks/ThemeSwitch.B8t8VQsu.js";var ra=E('<div class="container svelte-1g19pwu"><h1> </h1> <p class="svelte-1g19pwu"> <br> <b> </b> <b> </b></p> <p class="svelte-1g19pwu"> </p> <div class="btn svelte-1g19pwu"><!></div></div> <!> <!>',1),sa=E("<!> <!> <!>",1);function oa(y,A){S(A,!0);let s=O(),l=B("old@mail.org"),m=B("new@mail.org");var d=sa();Y(o=>{f(()=>V.title=s.emailChange.title||"E-Mail Change Confirm")});var h=k(d);q(h,{id:aa,get value(){return i(l)},set value(o){z(l,j(o))}});var $=a(h,2);q($,{id:ta,get value(){return i(m)},set value(o){z(m,j(o))}});var D=a($,2);F(D,{children:(o,ia)=>{N(o,{children:(G,la)=>{var b=ra(),n=k(b),v=t(n),H=t(v,!0);e(v);var p=a(v,2),C=t(p),u=a(C,3),I=t(u,!0);e(u);var w=a(u),x=a(w),J=t(x,!0);e(x),e(p);var g=a(p,2),K=t(g,!0);e(g);var L=a(g,2),P=t(L);_(P,{$$events:{click:()=>window.location.replace("/auth/v1/account")},children:(R,ma)=>{W();var T=X();f(()=>r(T,s.authorize.login)),c(R,T)},$$slots:{default:!0}}),e(L),e(n);var M=a(n,2);ea(M,{absolute:!0});var Q=a(M,2);Z(Q,{absolute:!0}),f(()=>{r(H,s.emailChange.title),r(C,`${s.emailChange.textChanged??""}:`),r(I,i(l)),r(w,` ${s.emailChange.to??""} `),r(J,i(m)),r(K,s.emailChange.textLogin)}),c(G,b)},$$slots:{default:!0}})},$$slots:{default:!0}}),c(y,d),U()}export{oa as component};
