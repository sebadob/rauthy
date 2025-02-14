import"../chunks/disclose-version.BDr9Qe-U.js";import"../chunks/legacy.C93jtn63.js";import{p as V,i as $,a as L,g as r,h as s,s as y,c as N,aq as k,r as A,t as z,ao as Z,f as K}from"../chunks/index-client.ijM6xFk7.js";import{s as M}from"../chunks/render.BjVgqyVR.js";import{e as _}from"../chunks/each.K_FVK-xy.js";import{a as d,d as q,t as b,e as ee}from"../chunks/template.DOyJZ41v.js";import{p as o}from"../chunks/proxy.DmesnD8z.js";import{b as ae,c as te,e as re,f as se}from"../chunks/dataFetchingAdmin.1Vn3g5PB.js";import{E as W,T as ie,a as oe,P as ne}from"../chunks/Pagination.8F_y-ekl.js";import{i as x}from"../chunks/if.BunKWniS.js";import{p as O}from"../chunks/props.YiozBtSG.js";import{c as F,a as R,I as P}from"../chunks/Input.QQtpeOHH.js";import{U as G,V as H,q as Q}from"../chunks/helpers.DMpx17S8.js";import{B as J}from"../chunks/Button.BgUaD8Uz.js";import{t as U,a as B}from"../chunks/index.R935SZZb.js";import{T as le}from"../chunks/TabBar.DtAcwHiX.js";import{r as ve}from"../chunks/legacy-client.YTFnMCQ3.js";var ce=b('<div class="header font-label svelte-1mi5990">ADD NEW USER ATTRIBUTE</div>'),de=b('<div class="mainErr err svelte-1mi5990"> </div>'),me=b('<div class="container svelte-1mi5990"><div class="desc svelte-1mi5990">You can add a new custom user attribute.<br> These attributes can be set for every user and mapped to an existing scope.<br> They are simple Key / JsonValue pairs.</div> <!> <!> <!> <!> <!></div>');function ue(I,p){V(p,!0);let w=O(p,"idx",31,()=>-1),v=$(""),c=$(void 0),u=o({name:"",desc:""}),n=$(o({}));const m=F().shape({name:R().trim().required("Name is required").matches(G,"Invalid characters: [a-z0-9-_/]{2,32}"),desc:R().trim().matches(H,"Invalid characters: [a-zA-Z0-9\\-_/\\s]{0,128}")});async function C(){if(!await T())return;s(v,"");let t=await ae(u);if(t.ok)s(c,!1),p.onSave();else{let i=await t.json();s(v,o(i.message))}}async function T(){try{return await m.validate(u,{abortEarly:!1}),s(n,o({})),!0}catch(t){return s(n,o(Q(t))),!1}}W(I,{get idx(){return w()},set idx(t){w(t)},get show(){return r(c)},set show(t){s(c,o(t))},header:t=>{var i=ce();d(t,i)},body:t=>{var i=me(),g=y(N(i),2);P(g,{autocomplete:"off",placeholder:"Name",get value(){return u.name},set value(e){u.name=e},get error(){return r(n).name},set error(e){r(n).name=e},$$events:{input:T},children:(e,a)=>{k();var l=q("NAME");d(e,l)},$$slots:{default:!0}});var j=y(g,2);P(j,{autocomplete:"off",placeholder:"Description",get value(){return u.desc},set value(e){u.desc=e},get error(){return r(n).desc},set error(e){r(n).desc=e},$$events:{input:T},children:(e,a)=>{k();var l=q("DESCRIPTION");d(e,l)},$$slots:{default:!0}});var D=y(j,2);J(D,{level:1,$$events:{click:C},children:(e,a)=>{k();var l=q("SAVE");d(e,l)},$$slots:{default:!0}});var E=y(D,2);x(E,e=>{});var h=y(E,2);{var S=e=>{var a=de(),l=N(a,!0);A(a),z(()=>M(l,r(v))),d(e,a)};x(h,e=>{r(v)&&e(S)})}A(i),d(t,i)},$$slots:{header:!0,body:!0}}),L()}var fe=b('<div class="success svelte-2o3w7i">Success</div>'),he=b('<div class="mainErr err svelte-2o3w7i"> </div>'),pe=b('<div class="container svelte-2o3w7i"><!> <!> <!> <!> <!></div>');function ge(I,p){V(p,!0);let w=O(p,"attr",19,()=>({})),v=$(""),c=$(!1),u=$(void 0);ve(()=>{r(c)&&s(u,o(setTimeout(()=>{s(c,!1),p.onSave()},2e3)))}),Z(()=>()=>clearTimeout(r(u)));let n=$(o({})),m=o({});const C=F().shape({name:R().trim().required("Name is required").matches(G,"Valid characters: [a-z0-9-_/]{2,32}"),desc:R().nullable().trim().matches(H,"Valid characters: [a-zA-Z0-9\\-_/\\s]{0,128}")});Z(()=>{m.name=w().name,m.desc=w().desc});async function T(){if(s(v,""),!await t()){s(v,"Invalid input");return}let a={name:m.name,desc:m.desc},l=await te(w().name,a);if(l.ok)s(c,!0);else{let f=await l.json();s(v,o(f.message))}}async function t(){try{return await C.validate(m,{abortEarly:!1}),s(n,o({})),!0}catch(a){return s(n,o(Q(a))),!1}}var i=pe(),g=N(i);P(g,{autocomplete:"off",placeholder:"Name",get value(){return m.name},set value(a){m.name=a},get error(){return r(n).name},set error(a){r(n).name=a},$$events:{input:t},children:(a,l)=>{k();var f=q("NAME");d(a,f)},$$slots:{default:!0}});var j=y(g,2);P(j,{autocomplete:"off",placeholder:"Description",get value(){return m.desc},set value(a){m.desc=a},get error(){return r(n).desc},set error(a){r(n).desc=a},$$events:{input:t},children:(a,l)=>{k();var f=q("DESCRIPTION");d(a,f)},$$slots:{default:!0}});var D=y(j,2);J(D,{level:1,width:"4rem",$$events:{click:T},children:(a,l)=>{k();var f=q("SAVE");d(a,f)},$$slots:{default:!0}});var E=y(D,2);{var h=a=>{var l=fe();d(a,l)};x(E,a=>{r(c)&&a(h)})}var S=y(E,2);{var e=a=>{var l=he(),f=N(l,!0);A(l),z(()=>M(f,r(v))),d(a,l)};x(S,a=>{r(v)&&a(e)})}A(i),d(I,i),L()}var $e=b('<div class="err svelte-1jljqe9"> </div>'),ye=b('<div class="container svelte-1jljqe9"><div class="label svelte-1jljqe9">Are you sure, you want to delete this custom attribute?</div> <!> <!></div>');function we(I,p){V(p,!0);let w=O(p,"attr",19,()=>({})),v=$(!1),c=$("");async function u(){s(c,""),s(v,!0);let t=await re(w().name);if(t.ok)p.onSave();else{let i=await t.json();s(c,o(i.message))}s(v,!1)}var n=ye(),m=y(N(n),2);J(m,{level:1,get isLoading(){return r(v)},set isLoading(t){s(v,o(t))},$$events:{click:u},children:(t,i)=>{k();var g=q("DELETE");d(t,g)},$$slots:{default:!0}});var C=y(m,2);{var T=t=>{var i=$e(),g=N(i,!0);A(i),z(()=>M(g,r(c))),d(t,i)};x(C,t=>{r(c)&&t(T)})}A(n),d(I,n),L()}var be=b('<div class="data svelte-115hohv"> </div>'),Ee=b('<div class="header svelte-115hohv"><!></div>'),Se=b("<div><!></div>"),Ae=b("<div><!></div>"),Ne=b("<div><!> <!></div>");function Te(I,p){V(p,!0);let w=O(p,"attr",27,()=>o({})),v=O(p,"onSave",15),c=$(void 0);const u=["Config","Delete"];let n=$(o(u[0]));const m=200,C=m/2;function T(){s(c,!1),v()()}W(I,{get show(){return r(c)},set show(t){s(c,o(t))},header:t=>{var i=Ee(),g=N(i);ie(g,{text:"Custom Attribute Name",children:(j,D)=>{var E=be(),h=N(E,!0);A(E),z(()=>M(h,w().name)),d(j,E)},$$slots:{default:!0}}),A(i),d(t,i)},body:t=>{var i=Ne(),g=N(i);le(g,{labels:u,get selected(){return r(n)},set selected(h){s(n,o(h))}});var j=y(g,2);{var D=h=>{var S=Se(),e=N(S);ge(e,{get attr(){return w()},get onSave(){return v()},set onSave(a){v(a)}}),A(S),U(5,S,()=>B,()=>({delay:C,duration:m})),U(6,S,()=>B,()=>({duration:m})),d(h,S)},E=h=>{var S=ee(),e=K(S);{var a=l=>{var f=Ae(),Y=N(f);we(Y,{get attr(){return w()},onSave:T}),A(f),U(5,f,()=>B,()=>({delay:C,duration:m})),U(6,f,()=>B,()=>({duration:m})),d(l,f)};x(e,l=>{r(n)==="Delete"&&l(a)},!0)}d(h,S)};x(j,h=>{r(n)==="Config"?h(D):h(E,!1)})}A(i),d(t,i)},$$slots:{header:!0,body:!0}}),L()}var je=b('<div class="svelte-mjg702"><!></div>'),Ie=b(' <div class="content"><!> <!> <div id="attrs" class="svelte-mjg702"></div> <!> <div style="height: 20px"></div></div>',1);function De(I,p){V(p,!0);let w=$(""),v=$(o([])),c=$(o([])),u=$(o([])),n=$(""),m=[{label:"NAME",callback:(e,a)=>e.name.toLowerCase().includes(a.toLowerCase())}],C=[{label:"NAME",callback:(e,a)=>e.name.localeCompare(a.name)}];Z(()=>{T()});async function T(){let e=await se();if(!e.ok)s(w,"Error fetching user attr: "+e.body.message);else{let a=await e.json();s(v,o([...a.values]))}}function t(){r(v).length<2&&(s(v,o([])),s(c,o([])),s(u,o([]))),T(),s(n,"")}k();var i=Ie(),g=K(i),j=y(g),D=N(j);oe(D,{get items(){return r(v)},searchOptions:m,orderOptions:C,get resItems(){return r(c)},set resItems(e){s(c,o(e))},get search(){return r(n)},set search(e){s(n,o(e))}});var E=y(D,2);ue(E,{onSave:t});var h=y(E,2);_(h,23,()=>r(u),e=>e.name,(e,a,l)=>{var f=je(),Y=N(f);Te(Y,{onSave:t,get attr(){return r(u)[r(l)]},set attr(X){r(u)[r(l)]=X}}),A(f),d(e,f)}),A(h);var S=y(h,2);ne(S,{get items(){return r(c)},set items(e){s(c,o(e))},get resItems(){return r(u)},set resItems(e){s(u,o(e))}}),k(2),A(j),z(()=>M(g,`${r(w)??""} `)),d(I,i),L()}function Ce(I){De(I,{})}export{Ce as component};
