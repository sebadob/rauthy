import"../chunks/disclose-version.BDr9Qe-U.js";import"../chunks/legacy.BVESbwbj.js";import{p as q,u as e,v as t,w,c as C,s as I,r as x,ap as N,t as A,a as z,O as ee,f as B}from"../chunks/runtime.BONl2e88.js";import{s as O}from"../chunks/render.AqJWcVDb.js";import{e as ae}from"../chunks/each.Bce--_O8.js";import{d as M,a as n,t as b,e as te}from"../chunks/template.B7ldVCvc.js";import{p as l}from"../chunks/proxy.0BECmVET.js";import{o as F}from"../chunks/index-client.Bjp_k6xO.js";import{r as re,s as se,t as le,u as oe}from"../chunks/OptionSelect.TkTfcdPh.js";import{i as R}from"../chunks/if.DS90EFHs.js";import{a as V,c as W}from"../chunks/index.BJiFQ8Di.js";import{p as P}from"../chunks/props.BlbrNjWf.js";import{E as H}from"../chunks/ExpandContainer.2O3yvOBK.js";import{T as J}from"../chunks/Tooltip.BZNUXfH5.js";import{T as ie}from"../chunks/TabBar.u0W3le5w.js";import{r as K}from"../chunks/legacy-client.D71ZcBDy.js";import{c as Q,a as U}from"../chunks/index.esm.B2KJ0uBO.js";import{W as X,l as Y}from"../chunks/helpers.CQGRRstl.js";import{B as G}from"../chunks/Button.JPJkNEUa.js";import{I as Z}from"../chunks/Input.KbCPDMgh.js";import{O as ne}from"../chunks/OrderSearchBar.DcBjpUC6.js";import{P as ve}from"../chunks/Pagination.DjghmVzr.js";var de=b('<div class="success svelte-17in7jd">Success</div>'),ce=b('<div class="mainErr err svelte-17in7jd"> </div>'),me=b('<div class="container svelte-17in7jd"><div class="unit svelte-17in7jd"><div class="label font-label svelte-17in7jd">ID</div> <div class="value font-mono svelte-17in7jd"> </div></div> <!> <!> <!> <!></div>');function ue(T,j){q(j,!0);let m=P(j,"role",31,()=>l({})),u=w(""),i=w(!1),y=w(void 0);K(()=>{e(i)&&t(y,l(setTimeout(()=>{t(i,!1),j.onSave()},2e3)))}),F(()=>()=>clearTimeout(e(y)));let h=w(l({}));const D=Q().shape({name:U().trim().matches(X,"Can only contain: 'a-z0-9-_/:*', length: 2-64")});async function L(){if(t(u,""),!await k()){t(u,"Invalid input");return}let a={role:m().name.trim()},$=await re(m().id,a);if($.ok)t(i,!0);else{let _=await $.json();t(u,l(_.message))}}async function k(){try{return await D.validate(m(),{abortEarly:!1}),t(h,l({})),!0}catch(a){return t(h,l(Y(a))),!1}}var v=me(),d=C(v),o=I(C(d),2),S=C(o,!0);x(o),x(d);var E=I(d,2),g=ee(()=>m().name==="rauthy_admin");Z(E,{autocomplete:"off",placeholder:"Role Name",get disabled(){return e(g)},get value(){return m().name},set value(a){m(m().name=a,!0)},get error(){return e(h).name},set error(a){e(h).name=a},$$events:{input:k},children:(a,$)=>{N();var _=M("ROLE NAME");n(a,_)},$$slots:{default:!0}});var r=I(E,2);G(r,{level:1,width:"4rem",$$events:{click:L},children:(a,$)=>{N();var _=M("SAVE");n(a,_)},$$slots:{default:!0}});var f=I(r,2);{var s=a=>{var $=de();n(a,$)};R(f,a=>{e(i)&&a(s)})}var p=I(f,2);{var c=a=>{var $=ce(),_=C($,!0);x($),A(()=>O(_,e(u))),n(a,$)};R(p,a=>{e(u)&&a(c)})}x(v),A(()=>O(S,m().id)),n(T,v),z()}var fe=b('<div class="label svelte-1jljqe9">Anti-Lockout Rule: The rauthy_admin role cannot be deleted.</div>'),he=b('<div class="err svelte-1jljqe9"> </div>'),pe=b('<div class="label svelte-1jljqe9">Are you sure, you want to delete this role?</div> <!> <!>',1),$e=b('<div class="container svelte-1jljqe9"><!></div>');function ge(T,j){q(j,!0);let m=P(j,"role",19,()=>({})),u=w(!1),i=w("");async function y(){t(i,""),t(u,!0);let v=await se(m().id);if(v.ok)j.onSave();else{let d=await v.json();t(i,l(d.message))}t(u,!1)}var h=$e(),D=C(h);{var L=v=>{var d=fe();n(v,d)},k=v=>{var d=pe(),o=I(B(d),2);G(o,{level:1,get isLoading(){return e(u)},set isLoading(g){t(u,l(g))},$$events:{click:y},children:(g,r)=>{N();var f=M("DELETE");n(g,f)},$$slots:{default:!0}});var S=I(o,2);{var E=g=>{var r=he(),f=C(r,!0);x(r),A(()=>O(f,e(i))),n(g,r)};R(S,g=>{e(i)&&g(E)})}n(v,d)};R(D,v=>{m().name==="rauthy_admin"?v(L):v(k,!1)})}x(h),n(T,h),z()}var we=b('<div class="data font-mono svelte-115hohv"> </div>'),be=b('<div class="data svelte-115hohv"> </div>'),Se=b('<div class="header svelte-115hohv"><!> <!></div>'),je=b("<div><!></div>"),ye=b("<div><!></div>"),xe=b("<div><!> <!></div>");function Ee(T,j){q(j,!0);let m=P(j,"role",31,()=>l({})),u=P(j,"onSave",15),i=w(void 0);const y=["Config","Delete"];let h=w(l(y[0]));const D=200,L=D/2;function k(){t(i,!1),u()()}H(T,{get show(){return e(i)},set show(v){t(i,l(v))},header:v=>{var d=Se(),o=C(d);J(o,{text:"Role ID",children:(E,g)=>{var r=we(),f=C(r,!0);x(r),A(()=>O(f,m().id)),n(E,r)},$$slots:{default:!0}});var S=I(o,2);J(S,{text:"Role Name",children:(E,g)=>{var r=be(),f=C(r,!0);x(r),A(()=>O(f,m().name)),n(E,r)},$$slots:{default:!0}}),x(d),n(v,d)},body:v=>{var d=xe(),o=C(d);ie(o,{labels:y,get selected(){return e(h)},set selected(r){t(h,l(r))}});var S=I(o,2);{var E=r=>{var f=je(),s=C(f);ue(s,{get role(){return m()},set role(p){m(p)},get onSave(){return u()},set onSave(p){u(p)}}),x(f),V(5,f,()=>W,()=>({delay:L,duration:D})),V(6,f,()=>W,()=>({duration:D})),n(r,f)},g=r=>{var f=te(),s=B(f);{var p=c=>{var a=ye(),$=C(a);ge($,{get role(){return m()},onSave:k}),x(a),V(5,a,()=>W,()=>({delay:L,duration:D})),V(6,a,()=>W,()=>({duration:D})),n(c,a)};R(s,c=>{e(h)==="Delete"&&c(p)},!0)}n(r,f)};R(S,r=>{e(h)==="Config"?r(E):r(g,!1)})}x(d),n(v,d)},$$slots:{header:!0,body:!0}}),z()}var Ce=b('<div class="header font-label svelte-r30xs6">ADD NEW ROLE</div>'),Ie=b('<div class="success svelte-r30xs6">Success</div>'),De=b('<div class="err svelte-r30xs6"> </div>'),Le=b('<div class="container svelte-r30xs6"><!> <!> <!> <!></div>');function Te(T,j){q(j,!0);let m=P(j,"idx",31,()=>-1),u=w(void 0),i=w(l({role:""})),y=w(""),h=w(!1),D=w(void 0),L=w(l({}));const k=Q().shape({role:U().trim().matches(X,"Can only contain: 'a-z0-9-_/:*', length: 2-64")});K(()=>{e(h)&&t(D,l(setTimeout(()=>{t(h,!1),t(i,l({role:""})),t(u,!1),j.onSave()},1500)))}),F(()=>()=>clearTimeout(e(D)));async function v(){if(t(y,""),!await d()){t(y,"Invalid input");return}e(i).role=e(i).role.trim();let o=await le(e(i));if(o.ok)t(h,!0);else{let S=await o.json();t(y,l(S.message))}}async function d(){try{return await k.validate(e(i),{abortEarly:!1}),t(L,l({})),!0}catch(o){return t(L,l(Y(o))),!1}}H(T,{get idx(){return m()},set idx(o){m(o)},get show(){return e(u)},set show(o){t(u,l(o))},header:o=>{var S=Ce();n(o,S)},body:o=>{var S=Le(),E=C(S);Z(E,{autocomplete:"off",placeholder:"Role Name",get value(){return e(i).role},set value(c){e(i).role=c},get error(){return e(L).role},set error(c){e(L).role=c},$$events:{input:d},children:(c,a)=>{N();var $=M("ROLE NAME");n(c,$)},$$slots:{default:!0}});var g=I(E,2);G(g,{level:1,width:"4rem",$$events:{click:v},children:(c,a)=>{N();var $=M("SAVE");n(c,$)},$$slots:{default:!0}});var r=I(g,2);{var f=c=>{var a=Ie();n(c,a)};R(r,c=>{e(h)&&c(f)})}var s=I(r,2);{var p=c=>{var a=De(),$=C(a,!0);x(a),A(()=>O($,e(y))),n(c,a)};R(s,c=>{e(y)&&c(p)})}x(S),n(o,S)},$$slots:{header:!0,body:!0}}),z()}var ke=b('<div class="svelte-db30dl"><!></div>'),_e=b(' <div class="content"><!> <!> <div id="roles" class="svelte-db30dl"></div> <!> <div style="height: 20px"></div></div>',1);function Re(T,j){q(j,!0);let m=w(""),u=w(l([])),i=w(l([])),y=w(l([])),h=w(""),D=[{label:"Name",callback:(s,p)=>s.name.toLowerCase().includes(p.toLowerCase())},{label:"ID",callback:(s,p)=>s.id.toLowerCase().includes(p.toLowerCase())}],L=[{label:"Name",callback:(s,p)=>s.name.localeCompare(p.name)},{label:"ID",callback:(s,p)=>s.id.localeCompare(p.id)}];F(async()=>{k()});async function k(){let s=await oe(),p=await s.json();s.ok?t(u,l([...p])):t(m,l(p.message))}function v(){k(),t(h,"")}N();var d=_e(),o=B(d),S=I(o),E=C(S);ne(E,{get items(){return e(u)},searchOptions:D,orderOptions:L,get resItems(){return e(i)},set resItems(s){t(i,l(s))},get search(){return e(h)},set search(s){t(h,l(s))}});var g=I(E,2);Te(g,{onSave:v});var r=I(g,2);ae(r,23,()=>e(y),s=>s.id,(s,p,c)=>{var a=ke(),$=C(a);Ee($,{onSave:v,get role(){return e(u)[e(c)]},set role(_){e(u)[e(c)]=_}}),x(a),n(s,a)}),x(r);var f=I(r,2);ve(f,{get items(){return e(i)},set items(s){t(i,l(s))},get resItems(){return e(y)},set resItems(s){t(y,l(s))}}),N(2),x(S),A(()=>O(o,`${e(m)??""} `)),n(T,d),z()}function Ne(T){Re(T,{})}export{Ne as component};
