import"../chunks/disclose-version.BDr9Qe-U.js";import{p as U,b as T,a3 as x,a as V,a5 as l,$ as W,g as n,j as B,a4 as o,a6 as C,a7 as u,a9 as m,a8 as H}from"../chunks/index-client.C1uYfxHd.js";import{s as c}from"../chunks/render.DlGhCWX2.js";import{i as X}from"../chunks/if.DL6pUnI4.js";import{a as v,t as z,b as A}from"../chunks/template.BehheIQn.js";import{h as Y}from"../chunks/svelte-head.76d9LV1R.js";import{p as D}from"../chunks/proxy.bZKuG6wn.js";import{m as Z,b as F,p as K}from"../chunks/helpers.EoWe9H8J.js";import{B as E,T as tt,a as at}from"../chunks/ThemeSwitch.B8t8VQsu.js";import{u as ot}from"../chunks/i18n.svelte.C5UT6hSp.js";import{M as st,C as rt}from"../chunks/ContentCenter.BU-nX73m.js";import{L as et}from"../chunks/LangSelector.Bntt9KZe.js";import{T as it,u as lt}from"../chunks/Template.DxkxrmTY.js";import{u as h}from"../chunks/param.svelte.vEd3x5XR.js";var nt=z('<div class="container svelte-7qsjv3"><h1> </h1> <p> </p> <div class="btn svelte-7qsjv3"><!> <!></div> <!></div> <!> <!>',1),ut=z("<!> <!>",1);function mt(G,I){U(I,!0);let t=ot(),_=C(!1),e=C(""),$=D({post_logout_redirect_uri:h("post_logout_redirect_uri").get(),id_token_hint:h("id_token_hint").get(),state:h("state").get()});T(()=>{n(e)&&F(n(e))}),T(()=>{$.id_token_hint&&b()});async function w(){window.location.replace("/auth/v1")}async function b(){B(_,!0);let a="/auth/v1/oidc/logout";lt().get()&&(a="/auth/v1/dev/logout");let j=await at(a,$,"form");K();let s=j.headers.get("location");console.log("loc",s),s?window.location.replace(s):await w()}var k=ut();Y(a=>{l(()=>W.title=(t==null?void 0:t.logout.logout)||"Logout")});var L=x(k);it(L,{id:Z,get value(){return n(e)},set value(a){B(e,D(a))}});var J=o(L,2);st(J,{children:(a,j)=>{rt(a,{children:(s,ct)=>{var q=nt(),p=x(q),d=u(p),N=u(d,!0);m(d);var g=o(d,2),O=u(g,!0);m(g);var f=o(g,2),y=u(f);E(y,{onclick:b,get isLoading(){return n(_)},children:(i,S)=>{H();var r=A();l(()=>c(r,t.logout.logout)),v(i,r)},$$slots:{default:!0}});var P=o(y,2);E(P,{level:3,onclick:w,children:(i,S)=>{H();var r=A();l(()=>c(r,t.common.cancel)),v(i,r)},$$slots:{default:!0}}),m(f);var Q=o(f,2);X(Q,i=>{}),m(p);var M=o(p,2);tt(M,{absolute:!0});var R=o(M,2);et(R,{absolute:!0}),l(()=>{c(N,t.logout.logout),c(O,t.logout.confirmMsg)}),v(s,q)},$$slots:{default:!0}})},$$slots:{default:!0}}),v(G,k),V()}export{mt as component};
