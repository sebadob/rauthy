import{e as P,a as m,t as y,d as X}from"../chunks/disclose-version.BP4T4xdV.js";import{a as W,aa as Y,j as e,f as R,k as n,s as q,t as k,p as H,l as w,c as L,a7 as J,r as M,a9 as K,a5 as ae}from"../chunks/index-client.hxMyB_b4.js";import{s as x}from"../chunks/render.yNX9q8-B.js";import{i as B}from"../chunks/if.BeBa1Kxb.js";import{e as se}from"../chunks/each.BUp7aLCc.js";import{B as V,a as ne}from"../chunks/Button.BR1gOfNn.js";import{p as i}from"../chunks/proxy.DaVNR9vk.js";import{u as Q}from"../chunks/i18n_admin.svelte.lt3dD7TQ.js";import{B as ie}from"../chunks/ButtonAddModal.DsDc6nUh.js";import{C as le}from"../chunks/ContentAdmin.YSUtpxca.js";import{N as me}from"../chunks/NavButtonTile.DRctQzgA.js";import{N as de}from"../chunks/NavSub.DILgx03i.js";import{O as ve}from"../chunks/OrderSearchBar.wIdu7A5l.js";import{b as ue,d as ce,c as fe,f as pe}from"../chunks/fetch.Bq59EV_b.js";import{u as ge}from"../chunks/param.svelte.BbzO27-O.js";import{I as Z}from"../chunks/pow.BMnvYon0.js";import{f as ee}from"../chunks/patterns.BRCxk8by.js";import{F as re}from"../chunks/Form.BnvI_UPN.js";import{u as U}from"../chunks/i18n.svelte.Ci9wvqOq.js";import{T as he}from"../chunks/Tabs.DUIpSMWU.js";import{h as oe}from"../chunks/html.704f8A94.js";import{I as be}from"../chunks/IconCheck.C6tGCpWy.js";import{L as $e}from"../chunks/LabeledValue.CxxfjE_4.js";var ye=y('<div class="err"> </div>'),we=y("<!> <!> <!>",1),Ce=y('<div class="container svelte-s1196z"><!></div>');function Se(A,t){W(t,!0);let E=U(),p=Q(),v=w(void 0),s=w(""),g=w("");Y(()=>{requestAnimationFrame(()=>{var c;(c=e(v))==null||c.focus()})});async function h(c,f){var b;if(t.roles.find(O=>O.name===e(g))){n(s,i(p.common.nameExistsAlready));return}n(s,"");let d={role:e(g)},o=await ue(c.action,d);o.body?t.onSave(o.body.id):n(s,i(((b=o.error)==null?void 0:b.message)||"Error"))}var $=Ce(),F=L($);re(F,{action:"/auth/v1/roles",onSubmit:h,children:(c,f)=>{var d=we(),o=R(d);Z(o,{autocomplete:"off",get label(){return p.roles.name},get placeholder(){return p.roles.name},required:!0,pattern:ee,get ref(){return e(v)},set ref(u){n(v,i(u))},get value(){return e(g)},set value(u){n(g,i(u))}});var b=q(o,2);V(b,{type:"submit",children:(u,C)=>{J();var _=P();k(()=>x(_,E.common.save)),m(u,_)},$$slots:{default:!0}});var O=q(b,2);{var D=u=>{var C=ye(),_=L(C,!0);M(C),k(()=>x(_,e(s))),m(u,C)};B(O,u=>{e(s)&&u(D)})}m(c,d)},$$slots:{default:!0}}),M($),m(A,$),H()}var Ie=y("<p><!></p>"),Ne=y('<div class="flex gap-05"><!> <!></div>'),Oe=y('<div class="err"> </div>'),_e=y("<!> <!> <!> <!>",1);function qe(A,t){W(t,!0);let E=U(),p=Q(),v=w(""),s=w(!1),g=K(()=>t.role.name==="rauthy_admin"),h=w(i(t.role.name));Y(()=>{t.role.id&&n(h,i(t.role.name))});async function $(c,f){if(n(v,""),e(h)!==t.role.name&&t.roles.find(b=>b.name===e(h))){n(v,i(p.common.nameExistsAlready));return}let d={role:e(h)},o=await ce(c.action,d);o.error?n(v,i(o.error.message)):(n(s,!0),t.onSave(),setTimeout(()=>{n(s,!1)},2e3))}var F=K(()=>`/auth/v1/roles/${t.role.id}`);re(A,{get action(){return e(F)},onSubmit:$,children:(c,f)=>{var d=_e(),o=R(d);$e(o,{label:"ID",mono:!0,children:(r,a)=>{J();var l=P();k(()=>x(l,t.role.id)),m(r,l)},$$slots:{default:!0}});var b=q(o,2);Z(b,{autocomplete:"off",get label(){return p.scopes.name},get placeholder(){return p.scopes.name},get disabled(){return e(g)},width:"14.5rem",required:!0,pattern:ee,get value(){return e(h)},set value(r){n(h,i(r))}});var O=q(b,2);{var D=r=>{var a=Ie(),l=L(a);oe(l,()=>p.roles.adminNoMod),M(a),m(r,a)},u=r=>{var a=Ne(),l=L(a);V(l,{type:"submit",children:(N,T)=>{J();var j=P();k(()=>x(j,E.common.save)),m(N,j)},$$slots:{default:!0}});var S=q(l,2);{var I=N=>{be(N,{})};B(S,N=>{e(s)&&N(I)})}M(a),m(r,a)};B(O,r=>{e(g)?r(D):r(u,!1)})}var C=q(O,2);{var _=r=>{var a=Oe(),l=L(a,!0);M(a),k(()=>x(l,e(v))),m(r,a)};B(C,r=>{e(v)&&r(_)})}m(c,d)},$$slots:{default:!0}}),H()}var Fe=y("<p><!></p>"),Te=y('<div class="err"> </div>'),Ae=y("<p> </p> <!> <!>",1);function Ee(A,t){W(t,!0);let E=U(),p=Q(),v=w(""),s=K(()=>t.role.name==="rauthy_admin");async function g(){n(v,"");let f=await fe(`/auth/v1/roles/${t.role.id}`);f.error?n(v,i(f.error.message)):t.onSave()}var h=X(),$=R(h);{var F=f=>{var d=Fe(),o=L(d);oe(o,()=>p.roles.adminNoMod),M(d),m(f,d)},c=f=>{var d=Ae(),o=R(d),b=L(o,!0);M(o);var O=q(o,2);V(O,{level:-1,onclick:g,children:(C,_)=>{J();var r=P();k(()=>x(r,E.common.delete)),m(C,r)},$$slots:{default:!0}});var D=q(O,2);{var u=C=>{var _=Te(),r=L(_,!0);M(_),k(()=>x(r,e(v))),m(C,_)};B(D,C=>{e(v)&&C(u)})}k(()=>x(b,p.roles.delete1)),m(f,d)};B($,f=>{e(s)?f(F):f(c,!1)})}m(A,h),H()}var Le=y('<div class="flex"><!></div> <!>',1);function Me(A,t){W(t,!0);let E=U(),p=Q();const v=[p.nav.config,E.common.delete];let s=w(i(v[0])),g=w(void 0);Y(()=>{t.role.id&&requestAnimationFrame(()=>{var o;(o=e(g))==null||o()})});var h=Le(),$=R(h),F=L($);he(F,{tabs:v,get selected(){return e(s)},set selected(o){n(s,i(o))},get focusFirst(){return e(g)},set focusFirst(o){n(g,i(o))}}),M($);var c=q($,2);{var f=o=>{qe(o,{get role(){return t.role},get roles(){return t.roles},get onSave(){return t.onSave}})},d=o=>{var b=X(),O=R(b);{var D=u=>{Ee(u,{get role(){return t.role},get onSave(){return t.onSave}})};B(O,u=>{e(s)===E.common.delete&&u(D)},!0)}m(o,b)};B(c,o=>{e(s)===p.nav.config?o(f):o(d,!1)})}m(A,h),H()}var Re=y("<div></div> <!>",1),ke=y("<!> <!>",1),xe=y('<div class="err"> </div>'),Be=y('<!> <div id="groups"><!></div>',1),De=y("<!> <!>",1);function Ge(A,t){W(t,!0);let E=Q(),p=w(void 0),v=w(""),s=w(i([])),g=w(i([])),h=w(void 0),$=ge("rid");const F=[E.common.name,"ID"];let c=w(i(F[0])),f=w("");const d=[E.common.name,"ID"];ae(()=>{o()}),Y(()=>{n(h,i(e(s).find(r=>r.id===$.get())))}),Y(()=>{let r=e(f).toLowerCase();r?e(c)===F[0]?n(g,i(e(s).filter(a=>{var l;return(l=a.name)==null?void 0:l.toLowerCase().includes(r)}))):e(c)===F[1]&&n(g,i(e(s).filter(a=>a.id.toLowerCase().includes(r)))):n(g,i(e(s)))});async function o(){var a;let r=await pe("/auth/v1/roles");r.body?n(s,i(r.body)):n(v,i(((a=r.error)==null?void 0:a.message)||"Error"))}function b(r,a){let l=a==="up";r===d[0]?e(s).sort((S,I)=>l?S.name.localeCompare(I.name):I.name.localeCompare(S.name)):r===d[1]&&e(s).sort((S,I)=>l?S.id.localeCompare(I.id):I.id.localeCompare(S.id))}function O(){o()}async function D(r){var a;(a=e(p))==null||a(),await o(),$.set(r)}var u=De(),C=R(u);de(C,{paddingTop:"2.1rem",buttonTilesAriaControls:"groups",width:"min(20rem, 100dvw)",thresholdNavSub:700,buttonTiles:r=>{var a=Re(),l=R(a);ne(l,"height",".5rem");var S=q(l,2);se(S,17,()=>e(g),I=>I.id,(I,N)=>{var T=K(()=>$.get()===e(N).id);me(I,{onclick:()=>$.set(e(N).id),get selected(){return e(T)},children:(j,G)=>{J();var z=P();k(()=>x(z,e(N).name)),m(j,z)},$$slots:{default:!0}})}),m(r,a)},children:(r,a)=>{var l=ke(),S=R(l),I=K(()=>e(s).length===0?1:2);ie(S,{get level(){return e(I)},alignRight:!0,get closeModal(){return e(p)},set closeModal(T){n(p,i(T))},children:(T,j)=>{Se(T,{onSave:D,get roles(){return e(s)}})},$$slots:{default:!0}});var N=q(S,2);ve(N,{searchOptions:F,orderOptions:d,onChangeOrder:b,searchWidth:"min(19.5rem, 100dvw - .5rem)",get searchOption(){return e(c)},set searchOption(T){n(c,i(T))},get value(){return e(f)},set value(T){n(f,i(T))}}),m(r,l)},$$slots:{buttonTiles:!0,default:!0}});var _=q(C,2);le(_,{children:(r,a)=>{var l=Be(),S=R(l);{var I=G=>{var z=xe(),te=L(z,!0);M(z),k(()=>x(te,e(v))),m(G,z)};B(S,G=>{e(v)&&G(I)})}var N=q(S,2),T=L(N);{var j=G=>{Me(G,{get role(){return e(h)},get roles(){return e(s)},onSave:O})};B(T,G=>{e(h)&&G(j)})}M(N),m(r,l)},$$slots:{default:!0}}),m(A,u),H()}function je(A){Ge(A,{})}export{je as component};
