import"../chunks/disclose-version.BDr9Qe-U.js";import"../chunks/legacy.Dmu0D13_.js";import{p as q,y as e,z as t,ap as B,A as w,c as C,s as I,r as x,ar as A,t as N,a as z,ab as ee,f as Q}from"../chunks/index-client.CVQra9Cu.js";import{s as O}from"../chunks/render.CkKFmY3E.js";import{e as ae}from"../chunks/each.D7w_Y1pv.js";import{d as M,a as n,t as b,e as te}from"../chunks/template.4ZWi6dE6.js";import{p as l}from"../chunks/proxy.Dc6q2r8X.js";import{c as re,e as se,f as le,h as ie}from"../chunks/dataFetchingAdmin.DA15Jmej.js";import{i as R}from"../chunks/if._4yhN2kP.js";import{t as V,s as W}from"../chunks/index.CkVfMYgG.js";import{p as P}from"../chunks/props.BVYR8gU-.js";import{E as G,T as H,a as oe,O as ne,P as ve}from"../chunks/Pagination.AI5NcTku.js";import{r as J}from"../chunks/legacy-client.nffOF2oE.js";import{c as K,a as U,I as X,B as F}from"../chunks/Input.BLzcZ_Un.js";import{W as Y,q as Z}from"../chunks/helpers.BweJi6s8.js";var de=b('<div class="success svelte-17in7jd">Success</div>'),ce=b('<div class="mainErr err svelte-17in7jd"> </div>'),ue=b('<div class="container svelte-17in7jd"><div class="unit svelte-17in7jd"><div class="label font-label svelte-17in7jd">ID</div> <div class="value font-mono svelte-17in7jd"> </div></div> <!> <!> <!> <!></div>');function me(k,j){q(j,!0);let u=P(j,"role",31,()=>l({})),m=w(""),o=w(!1),y=w(void 0);J(()=>{e(o)&&t(y,l(setTimeout(()=>{t(o,!1),j.onSave()},2e3)))}),B(()=>()=>clearTimeout(e(y)));let h=w(l({}));const D=K().shape({name:U().trim().matches(Y,"Can only contain: 'a-z0-9-_/:*', length: 2-64")});async function L(){if(t(m,""),!await T()){t(m,"Invalid input");return}let a={role:u().name.trim()},$=await re(u().id,a);if($.ok)t(o,!0);else{let _=await $.json();t(m,l(_.message))}}async function T(){try{return await D.validate(u(),{abortEarly:!1}),t(h,l({})),!0}catch(a){return t(h,l(Z(a))),!1}}var v=ue(),d=C(v),i=I(C(d),2),S=C(i,!0);x(i),x(d);var E=I(d,2),g=ee(()=>u().name==="rauthy_admin");X(E,{autocomplete:"off",placeholder:"Role Name",get disabled(){return e(g)},get value(){return u().name},set value(a){u(u().name=a,!0)},get error(){return e(h).name},set error(a){e(h).name=a},$$events:{input:T},children:(a,$)=>{A();var _=M("ROLE NAME");n(a,_)},$$slots:{default:!0}});var r=I(E,2);F(r,{level:1,width:"4rem",$$events:{click:L},children:(a,$)=>{A();var _=M("SAVE");n(a,_)},$$slots:{default:!0}});var f=I(r,2);{var s=a=>{var $=de();n(a,$)};R(f,a=>{e(o)&&a(s)})}var p=I(f,2);{var c=a=>{var $=ce(),_=C($,!0);x($),N(()=>O(_,e(m))),n(a,$)};R(p,a=>{e(m)&&a(c)})}x(v),N(()=>O(S,u().id)),n(k,v),z()}var fe=b('<div class="label svelte-1jljqe9">Anti-Lockout Rule: The rauthy_admin role cannot be deleted.</div>'),he=b('<div class="err svelte-1jljqe9"> </div>'),pe=b('<div class="label svelte-1jljqe9">Are you sure, you want to delete this role?</div> <!> <!>',1),$e=b('<div class="container svelte-1jljqe9"><!></div>');function ge(k,j){q(j,!0);let u=P(j,"role",19,()=>({})),m=w(!1),o=w("");async function y(){t(o,""),t(m,!0);let v=await se(u().id);if(v.ok)j.onSave();else{let d=await v.json();t(o,l(d.message))}t(m,!1)}var h=$e(),D=C(h);{var L=v=>{var d=fe();n(v,d)},T=v=>{var d=pe(),i=I(Q(d),2);F(i,{level:1,get isLoading(){return e(m)},set isLoading(g){t(m,l(g))},$$events:{click:y},children:(g,r)=>{A();var f=M("DELETE");n(g,f)},$$slots:{default:!0}});var S=I(i,2);{var E=g=>{var r=he(),f=C(r,!0);x(r),N(()=>O(f,e(o))),n(g,r)};R(S,g=>{e(o)&&g(E)})}n(v,d)};R(D,v=>{u().name==="rauthy_admin"?v(L):v(T,!1)})}x(h),n(k,h),z()}var we=b('<div class="data font-mono svelte-115hohv"> </div>'),be=b('<div class="data svelte-115hohv"> </div>'),Se=b('<div class="header svelte-115hohv"><!> <!></div>'),je=b("<div><!></div>"),ye=b("<div><!></div>"),xe=b("<div><!> <!></div>");function Ee(k,j){q(j,!0);let u=P(j,"role",31,()=>l({})),m=P(j,"onSave",15),o=w(void 0);const y=["Config","Delete"];let h=w(l(y[0]));const D=200,L=D/2;function T(){t(o,!1),m()()}G(k,{get show(){return e(o)},set show(v){t(o,l(v))},header:v=>{var d=Se(),i=C(d);H(i,{text:"Role ID",children:(E,g)=>{var r=we(),f=C(r,!0);x(r),N(()=>O(f,u().id)),n(E,r)},$$slots:{default:!0}});var S=I(i,2);H(S,{text:"Role Name",children:(E,g)=>{var r=be(),f=C(r,!0);x(r),N(()=>O(f,u().name)),n(E,r)},$$slots:{default:!0}}),x(d),n(v,d)},body:v=>{var d=xe(),i=C(d);oe(i,{labels:y,get selected(){return e(h)},set selected(r){t(h,l(r))}});var S=I(i,2);{var E=r=>{var f=je(),s=C(f);me(s,{get role(){return u()},set role(p){u(p)},get onSave(){return m()},set onSave(p){m(p)}}),x(f),V(5,f,()=>W,()=>({delay:L,duration:D})),V(6,f,()=>W,()=>({duration:D})),n(r,f)},g=r=>{var f=te(),s=Q(f);{var p=c=>{var a=ye(),$=C(a);ge($,{get role(){return u()},onSave:T}),x(a),V(5,a,()=>W,()=>({delay:L,duration:D})),V(6,a,()=>W,()=>({duration:D})),n(c,a)};R(s,c=>{e(h)==="Delete"&&c(p)},!0)}n(r,f)};R(S,r=>{e(h)==="Config"?r(E):r(g,!1)})}x(d),n(v,d)},$$slots:{header:!0,body:!0}}),z()}var Ce=b('<div class="header font-label svelte-r30xs6">ADD NEW ROLE</div>'),Ie=b('<div class="success svelte-r30xs6">Success</div>'),De=b('<div class="err svelte-r30xs6"> </div>'),Le=b('<div class="container svelte-r30xs6"><!> <!> <!> <!></div>');function ke(k,j){q(j,!0);let u=P(j,"idx",31,()=>-1),m=w(void 0),o=w(l({role:""})),y=w(""),h=w(!1),D=w(void 0),L=w(l({}));const T=K().shape({role:U().trim().matches(Y,"Can only contain: 'a-z0-9-_/:*', length: 2-64")});J(()=>{e(h)&&t(D,l(setTimeout(()=>{t(h,!1),t(o,l({role:""})),t(m,!1),j.onSave()},1500)))}),B(()=>()=>clearTimeout(e(D)));async function v(){if(t(y,""),!await d()){t(y,"Invalid input");return}e(o).role=e(o).role.trim();let i=await le(e(o));if(i.ok)t(h,!0);else{let S=await i.json();t(y,l(S.message))}}async function d(){try{return await T.validate(e(o),{abortEarly:!1}),t(L,l({})),!0}catch(i){return t(L,l(Z(i))),!1}}G(k,{get idx(){return u()},set idx(i){u(i)},get show(){return e(m)},set show(i){t(m,l(i))},header:i=>{var S=Ce();n(i,S)},body:i=>{var S=Le(),E=C(S);X(E,{autocomplete:"off",placeholder:"Role Name",get value(){return e(o).role},set value(c){e(o).role=c},get error(){return e(L).role},set error(c){e(L).role=c},$$events:{input:d},children:(c,a)=>{A();var $=M("ROLE NAME");n(c,$)},$$slots:{default:!0}});var g=I(E,2);F(g,{level:1,width:"4rem",$$events:{click:v},children:(c,a)=>{A();var $=M("SAVE");n(c,$)},$$slots:{default:!0}});var r=I(g,2);{var f=c=>{var a=Ie();n(c,a)};R(r,c=>{e(h)&&c(f)})}var s=I(r,2);{var p=c=>{var a=De(),$=C(a,!0);x(a),N(()=>O($,e(y))),n(c,a)};R(s,c=>{e(y)&&c(p)})}x(S),n(i,S)},$$slots:{header:!0,body:!0}}),z()}var Te=b('<div class="svelte-db30dl"><!></div>'),_e=b(' <div class="content"><!> <!> <div id="roles" class="svelte-db30dl"></div> <!> <div style="height: 20px"></div></div>',1);function Re(k,j){q(j,!0);let u=w(""),m=w(l([])),o=w(l([])),y=w(l([])),h=w(""),D=[{label:"Name",callback:(s,p)=>s.name.toLowerCase().includes(p.toLowerCase())},{label:"ID",callback:(s,p)=>s.id.toLowerCase().includes(p.toLowerCase())}],L=[{label:"Name",callback:(s,p)=>s.name.localeCompare(p.name)},{label:"ID",callback:(s,p)=>s.id.localeCompare(p.id)}];B(async()=>{T()});async function T(){let s=await ie(),p=await s.json();s.ok?t(m,l([...p])):t(u,l(p.message))}function v(){T(),t(h,"")}A();var d=_e(),i=Q(d),S=I(i),E=C(S);ne(E,{get items(){return e(m)},searchOptions:D,orderOptions:L,get resItems(){return e(o)},set resItems(s){t(o,l(s))},get search(){return e(h)},set search(s){t(h,l(s))}});var g=I(E,2);ke(g,{onSave:v});var r=I(g,2);ae(r,23,()=>e(y),s=>s.id,(s,p,c)=>{var a=Te(),$=C(a);Ee($,{onSave:v,get role(){return e(m)[e(c)]},set role(_){e(m)[e(c)]=_}}),x(a),n(s,a)}),x(r);var f=I(r,2);ve(f,{get items(){return e(o)},set items(s){t(o,l(s))},get resItems(){return e(y)},set resItems(s){t(y,l(s))}}),A(2),x(S),N(()=>O(i,`${e(u)??""} `)),n(k,d),z()}function Ae(k){Re(k,{})}export{Ae as component};
