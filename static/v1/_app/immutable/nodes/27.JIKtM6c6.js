import"../chunks/disclose-version.BDr9Qe-U.js";import{p as U,F as M,f as T,a as V,g as l,h as x,t as n,$ as W,s as a,i as B,c as m,r as c,ap as F}from"../chunks/runtime.BsghBUX9.js";import{s as u}from"../chunks/render.CTp9lacC.js";import{i as X}from"../chunks/if.6Ov_EDLa.js";import{a as v,t as H,d as z}from"../chunks/template.jENAUvUX.js";import{h as Y}from"../chunks/svelte-head.cS_wx7Ri.js";import{p as A}from"../chunks/proxy.YH42NwLs.js";import{c as Z,o as E,q as K}from"../chunks/helpers.CBl6QeM5.js";import{b as tt,B as D}from"../chunks/fetch.BvIeF4JA.js";import{u as ot}from"../chunks/i18n.svelte.D8rR00qK.js";import{M as at}from"../chunks/Main.DdVUe3_l.js";import{C as rt}from"../chunks/ContentCenter.1a7TMW4_.js";import{T as st,L as it}from"../chunks/LangSelector.TD0HCe5U.js";import{T as et}from"../chunks/Template.ZFmGCuZr.js";import{u as h}from"../chunks/param.svelte.BhHpzHur.js";import{u as lt}from"../chunks/is_dev.svelte.BUWsjuLN.js";var nt=H('<div class="container svelte-7qsjv3"><h1> </h1> <p> </p> <div class="btn svelte-7qsjv3"><!> <!></div> <!></div> <!> <!>',1),mt=H("<!> <!>",1);function ct(G,I){U(I,!0);let t=ot(),_=B(!1),i=B(""),$=A({post_logout_redirect_uri:h("post_logout_redirect_uri").get(),id_token_hint:h("id_token_hint").get(),state:h("state").get()});M(()=>{l(i)&&Z(l(i))}),M(()=>{$.id_token_hint&&k()});async function w(){window.location.replace("/auth/v1")}async function k(){x(_,!0);let o="/auth/v1/oidc/logout";lt().get()&&(o="/auth/v1/dev/logout");let q=await tt(o,$,"form");E();let r=q.headers.get("location");console.log("loc",r),r?window.location.replace(r):await w()}var b=mt();Y(o=>{n(()=>W.title=(t==null?void 0:t.logout.logout)||"Logout")});var L=T(b);et(L,{id:K,get value(){return l(i)},set value(o){x(i,A(o))}});var J=a(L,2);at(J,{children:(o,q)=>{rt(o,{children:(r,ut)=>{var j=nt(),p=T(j),d=m(p),N=m(d,!0);c(d);var f=a(d,2),O=m(f,!0);c(f);var g=a(f,2),y=m(g);D(y,{onclick:k,get isLoading(){return l(_)},children:(e,S)=>{F();var s=z();n(()=>u(s,t.logout.logout)),v(e,s)},$$slots:{default:!0}});var P=a(y,2);D(P,{level:3,onclick:w,children:(e,S)=>{F();var s=z();n(()=>u(s,t.common.cancel)),v(e,s)},$$slots:{default:!0}}),c(g);var Q=a(g,2);X(Q,e=>{}),c(p);var C=a(p,2);st(C,{absolute:!0});var R=a(C,2);it(R,{absolute:!0}),n(()=>{u(N,t.logout.logout),u(O,t.logout.confirmMsg)}),v(r,j)},$$slots:{default:!0}})},$$slots:{default:!0}}),v(G,b),V()}export{ct as component};
