import"../chunks/disclose-version.BDr9Qe-U.js";import"../chunks/legacy.DUFkTaXF.js";import{p as H,a4 as Y,s as f,c as m,a as W,g as e,aq as K,t as D,r as p,F as fe,a9 as F,i as S,h as n,f as U,ao as ke}from"../chunks/index-client.CzvcfCLI.js";import{s as E,r as Se}from"../chunks/render.BjwlFNsb.js";import{i as R}from"../chunks/if.Dlf0zx3_.js";import{e as ge,i as Ce}from"../chunks/each.BtetwUOf.js";import{a as h,t as A,d as M,e as ue}from"../chunks/template.CQASMEo4.js";import{s as oe,d as Te}from"../chunks/class.CZuJD32d.js";import{t as ee,a as ae,s as ne}from"../chunks/index.DIq6rU-s.js";import{p as k}from"../chunks/proxy.CHyqegA9.js";import{c as he,d as Pe,b as Be,f as Ae}from"../chunks/fetch.jldij1e_.js";import{C as Ge}from"../chunks/ContentAdmin.CEcZ6LAl.js";import{I as re,k as ye,B as Ne,N as De}from"../chunks/InputCheckbox.q8l3g2HO.js";import{N as Ee}from"../chunks/NavSub.CIK-mXQ0.js";import{u as Ve}from"../chunks/param.svelte.BMhroPqj.js";import{u as te}from"../chunks/i18n_admin.svelte.BIwIWUz9.js";import{O as je}from"../chunks/OrderSearchBar.CdfasuLu.js";import{p as J,r as Oe}from"../chunks/props.BRvF51YW.js";import{B as be}from"../chunks/helpers.DC_VHWHA.js";import{B as Q}from"../chunks/Button.6IA1Uwqq.js";import{I as qe}from"../chunks/IconCheck.CP7b1sho.js";import{L as Ke}from"../chunks/LabeledValue.CJfzY9sv.js";import{I as we}from"../chunks/InputDateTimeCombo.C63yl3I1.js";import{u as se}from"../chunks/i18n.svelte.CQpTABT5.js";import{f as ie,a as me}from"../chunks/form.DekAF1-C.js";import{h as pe}from"../chunks/html.DAmxUMld.js";import{I as Ue,a as Re,b as Fe}from"../chunks/PasswordInput.CjkMUDBI.js";import{b as Me}from"../chunks/input.DcPY3vts.js";import{I as Xe}from"../chunks/InputPassword.BNYdHBG0.js";import{T as He}from"../chunks/Tabs.CvJGPoeP.js";import{I as We}from"../chunks/Input.rIT8d5f_.js";import{c as Je}from"../chunks/patterns.Dy_ah9_D.js";import{F as Qe}from"../chunks/Form.BuIMl9-6.js";var Ye=A('<div class="center svelte-opxw4z"><!></div>'),Ze=A("<div><!></div>"),ea=A('<div class="row svelte-opxw4z"><!> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div></div>'),aa=A('<div class="matrix svelte-opxw4z"><div class="row svelte-opxw4z"><div></div> <!> <!> <!> <!></div> <!></div>');function $e(V,i){H(i,!0),J(i,"finalize",15)(I);let y=k([["Blacklist",!1,!1,!1,!1],["Clients",!1,!1,!1,!1],["Events",!1,!1,!1,!1],["Generic",!1,!1,!1,!1],["Groups",!1,!1,!1,!1],["Roles",!1,!1,!1,!1],["Secrets",!1,!1,!1,!1],["Sessions",!1,!1,!1,!1],["Scopes",!1,!1,!1,!1],["UserAttributes",!1,!1,!1,!1],["Users",!1,!1,!1,!1]]);Y(()=>{if(C(),i.key)for(let s of i.key.access){let a;switch(s.group){case"Blacklist":a=0;break;case"Clients":a=1;break;case"Events":a=2;break;case"Generic":a=3;break;case"Groups":a=4;break;case"Roles":a=5;break;case"Secrets":a=6;break;case"Sessions":a=7;break;case"Scopes":a=8;break;case"UserAttributes":a=9;break;case"Users":a=10;break}if(a===void 0){console.error("invalid idx during access key buildup");return}s.access_rights.includes("create")&&(y[a][1]=!0),s.access_rights.includes("read")&&(y[a][2]=!0),s.access_rights.includes("update")&&(y[a][3]=!0),s.access_rights.includes("delete")&&(y[a][4]=!0)}});function I(){let s=[];for(let a of y){let $=[];a[1]&&$.push("create"),a[2]&&$.push("read"),a[3]&&$.push("update"),a[4]&&$.push("delete"),$.length>0&&s.push({group:a[0],access_rights:$})}return s}function C(){for(let s of y)s[1]=!1,s[2]=!1,s[3]=!1,s[4]=!1}function d(s){for(let a of y)if(a[0]===s){a[1]&&a[2]&&a[3]&&a[4]?(a[1]=!1,a[2]=!1,a[3]=!1,a[4]=!1):(a[1]=!0,a[2]=!0,a[3]=!0,a[4]=!0);break}}function x(s){let a;switch(s){case"create":a=1;break;case"read":a=2;break;case"update":a=3;break;case"delete":a=4;break}if(!a){console.error("logic error in toggleRight, idx is undefined");return}let $=!0;for(let l of y)if(!l[a]){$=!1;break}for(let l of y)l[a]=!$}var b=aa();{const s=($,l=fe)=>{var w=Ye(),P=m(w),o=F(()=>`Toggle: ${l()}`);Q(P,{get ariaLabel(){return e(o)},invisible:!0,onclick:()=>x(l()),children:(c,u)=>{K();var _=M();D(()=>E(_,l())),h(c,_)},$$slots:{default:!0}}),p(w),h($,w)},a=($,l=fe)=>{var w=Ze(),P=m(w),o=F(()=>`Toggle: ${l()}`);Q(P,{get ariaLabel(){return e(o)},invisible:!0,onclick:()=>d(l()),children:(c,u)=>{K();var _=M();D(()=>E(_,l())),h(c,_)},$$slots:{default:!0}}),p(w),h($,w)};var T=m(b),z=f(m(T),2);s(z,()=>"create");var L=f(z,2);s(L,()=>"read");var G=f(L,2);s(G,()=>"update");var g=f(G,2);s(g,()=>"delete"),p(T);var B=f(T,2);ge(B,17,()=>y,Ce,($,l,w)=>{var P=ea(),o=m(P);a(o,()=>e(l)[0]);var c=f(o,2),u=m(c),_=F(()=>`${e(l)[0]}: create`);re(u,{get ariaLabel(){return e(_)},get checked(){return y[w][1]},set checked(q){y[w][1]=q}}),p(c);var r=f(c,2),t=m(r),v=F(()=>`${e(l)[0]}: read`);re(t,{get ariaLabel(){return e(v)},get checked(){return y[w][2]},set checked(q){y[w][2]=q}}),p(r);var N=f(r,2),O=m(N),j=F(()=>`${e(l)[0]}: update`);re(O,{get ariaLabel(){return e(j)},get checked(){return y[w][3]},set checked(q){y[w][3]=q}}),p(N);var X=f(N,2),le=m(X),Z=F(()=>`${e(l)[0]}: delete`);re(le,{get ariaLabel(){return e(Z)},get checked(){return y[w][4]},set checked(q){y[w][4]=q}}),p(X),p(P),h($,P)}),p(b)}h(V,b),W()}var ra=A("<div><!></div>"),ta=A('<div class="err"> </div>'),sa=A('<!> <!> <!> <!> <div class="btn svelte-1j50zcx"><!> <!> <!></div>',1);function ia(V,i){H(i,!0);let y=se(),I=te();const C=ie();let d=S(""),x=S(!1),b=S(!!i.key.expires),T=S(k(ie())),z=S(k(me())),L=S(void 0);Y(()=>{i.key.name&&n(b,!!i.key.expires)}),Y(()=>{if(e(b)){let r;i.key.expires?r=new Date(i.key.expires*1e3):r=new Date,n(T,k(ie(r))),n(z,k(me(r)))}});async function G(){if(n(d,""),!e(L)){console.error("finalizeMatrix is undefined");return}let r={name:i.key.name,access:e(L)()};if(r.access.length===0){n(d,"Grant the API Key at least one permission");return}if(e(b)){let v=be(e(T),e(z));if(!v){n(d,"Invalid Date Input: User Expires");return}r.exp=v}let t=await he(`/auth/v1/api_keys/${i.key.name}`,r);t.error?n(d,k(t.error.message)):(i.onSave(),n(x,!0),setTimeout(()=>{n(x,!1),i.onSave()},2e3))}var g=sa(),B=U(g);Ke(B,{get label(){return I.api_key.keyName},children:(r,t)=>{K();var v=M();D(()=>E(v,i.key.name)),h(r,v)},$$slots:{default:!0}});var s=f(B,2);re(s,{get ariaLabel(){return I.api_key.limitedValidity},get checked(){return e(b)},set checked(r){n(b,k(r))},children:(r,t)=>{K();var v=M();D(()=>E(v,I.api_key.limitedValidity)),h(r,v)},$$slots:{default:!0}});var a=f(s,2);{var $=r=>{var t=ra(),v=m(t);we(v,{get label(){return I.api_key.expires},withTime:!0,min:C,required:!0,get value(){return e(T)},set value(N){n(T,k(N))},get timeValue(){return e(z)},set timeValue(N){n(z,k(N))}}),p(t),ee(3,t,()=>ae),h(r,t)};R(a,r=>{e(b)&&r($)})}var l=f(a,2);$e(l,{get key(){return i.key},get finalize(){return e(L)},set finalize(r){n(L,k(r))}});var w=f(l,2),P=m(w);Q(P,{onclick:G,children:(r,t)=>{K();var v=M();D(()=>E(v,y.common.save)),h(r,v)},$$slots:{default:!0}});var o=f(P,2);{var c=r=>{qe(r,{})};R(o,r=>{e(x)&&r(c)})}var u=f(o,2);{var _=r=>{var t=ta(),v=m(t,!0);p(t),D(()=>E(v,e(d))),h(r,t)};R(u,r=>{e(d)&&r(_)})}p(w),h(V,g),W()}var oa=A("<div><!></div>"),na=A("<div><!></div>"),la=A("<div><!></div>"),ca=A('<div><div class="iconsOuter svelte-1pc4fk7"><div class="iconsInner svelte-1pc4fk7"><!> <!></div></div> <textarea></textarea></div>');function xe(V,i){H(i,!0);let y=J(i,"ariaLabel",3,""),I=J(i,"value",3,""),C=J(i,"rows",3,10),d=J(i,"cols",3,60),x=J(i,"show",15,!1),b=J(i,"width",3,"min(25rem, calc(100dvw - .5rem))"),T=Oe(i,["$$slots","$$events","$$legacy","ariaLabel","value","rows","cols","show","width"]),z=se(),L=S(""),G=S("");Y(()=>{x()?n(G,I()):n(G,k(e(L)))}),ke(()=>{for(let u=0;u<I().length;u++)n(L,e(L)+"*");n(G,k(e(L))),x(!1)});function g(){navigator.clipboard.writeText(I())}function B(){x(!x())}var s=ca(),a=m(s),$=m(a),l=m($),w=F(()=>x()?z.common.hide:z.common.show);Q(l,{get ariaLabel(){return e(w)},invisible:!0,onclick:B,children:(u,_)=>{var r=ue(),t=U(r);{var v=O=>{var j=oa(),X=m(j);Re(X,{width:22}),p(j),D(()=>oe(j,"title",z.common.hide)),h(O,j)},N=O=>{var j=na(),X=m(j);Fe(X,{width:22}),p(j),D(()=>oe(j,"title",z.common.show)),h(O,j)};R(t,O=>{x()?O(v):O(N,!1)})}h(u,r)},$$slots:{default:!0}});var P=f(l,2);Q(P,{get ariaLabel(){return z.common.copyToClip},invisible:!0,onclick:g,children:(u,_)=>{var r=la(),t=m(r);Ue(t,{}),p(r),D(()=>oe(r,"title",z.common.copyToClip)),h(u,r)},$$slots:{default:!0}}),p($),p(a);var o=f(a,2);Se(o);let c;p(s),D(()=>{ne(s,"width",`${b()}`),c=Te(o,c,{"aria-label":y(),disabled:!0,rows:C(),cols:d(),...T},"svelte-1pc4fk7"),ne(o,"width",b()),ne(o,"padding-right","2.75rem")}),Me(o,()=>e(G),u=>n(G,u)),h(V,s),W()}var va=A('<div class="secret font-mono svelte-1ii2aob"><!></div> <div><p><!></p> <p><span class="headerCode"><b><code></code></b></span></p></div> <div><p><!></p> <!></div> <div><p><!></p> <!></div>',1),da=A('<p> </p> <p> </p> <div class="btn svelte-1ii2aob"><!></div> <!> <div class="err"> </div>',1);function ua(V,i){H(i,!0);let y=se(),I=te(),C=S(""),d=S(""),x=S(""),b=S("");Y(()=>{i.key.name&&n(d,"")});async function T(){var u;let c=await he(`/auth/v1/api_keys/${i.key.name}/secret`);c.text?(n(d,k(c.text)),n(x,`curl -s -H 'Authorization: API-Key ${c.body}' ${window.location.origin}/auth/v1/api_keys/${i.key.name}/test`),n(b,`${e(x)} | jq`)):n(C,k(((u=c.error)==null?void 0:u.message)||"Error"))}var z=da(),L=U(z),G=m(L,!0);p(L);var g=f(L,2),B=m(g,!0);p(g);var s=f(g,2),a=m(s),$=F(()=>e(d)?3:1);Q(a,{get level(){return e($)},onclick:T,children:(c,u)=>{K();var _=M();D(()=>E(_,y.passwordReset.generate)),h(c,_)},$$slots:{default:!0}}),p(s);var l=f(s,2);{var w=c=>{var u=va(),_=U(u),r=m(_);Xe(r,{autocomplete:"off",get value(){return e(d)},label:"API Key",placeholder:"API Key",disabled:!0,showCopy:!0,width:"min(25rem, calc(100dvw - .5rem))"}),p(_);var t=f(_,2),v=m(t),N=m(v);pe(N,()=>I.api_key.generate3),p(v);var O=f(v,2),j=m(O),X=m(j),le=m(X);le.textContent="API-Key <api_key>",p(X),p(j),p(O),p(t);var Z=f(t,2),q=m(Z),_e=m(q);pe(_e,()=>I.api_key.generate4),p(q);var ze=f(q,2);ye(ze,()=>e(b),de=>{xe(de,{ariaLabel:"curl text command with jq",rows:3,get value(){return e(b)}})}),p(Z);var ce=f(Z,2),ve=m(ce),Ie=m(ve);pe(Ie,()=>I.api_key.generate5),p(ve);var Le=f(ve,2);ye(Le,()=>e(b),de=>{xe(de,{ariaLabel:"curl text command simple",rows:3,get value(){return e(x)}})}),p(ce),ee(3,_,()=>ae,()=>({duration:150})),ee(3,t,()=>ae,()=>({duration:150})),ee(3,Z,()=>ae,()=>({duration:150})),ee(3,ce,()=>ae,()=>({duration:150})),h(c,u)};R(l,c=>{e(d)&&c(w)})}var P=f(l,2),o=m(P,!0);p(P),D(()=>{E(G,I.api_key.generate1),E(B,I.api_key.generate2),E(o,e(C))}),h(V,z),W()}var ma=A('<div class="err"> </div>'),pa=A("<p> </p> <!> <!>",1);function fa(V,i){H(i,!0);let y=se(),I=te(),C=S("");async function d(){n(C,"");let g=await Pe(`/auth/v1/api_keys/${i.key.name}`);g.error?n(C,k(g.error.message)):i.onSave()}var x=pa(),b=U(x),T=m(b,!0);p(b);var z=f(b,2);Q(z,{level:-1,onclick:d,children:(g,B)=>{K();var s=M();D(()=>E(s,y.common.delete)),h(g,s)},$$slots:{default:!0}});var L=f(z,2);{var G=g=>{var B=ma(),s=m(B,!0);p(B),D(()=>E(s,e(C))),h(g,B)};R(L,g=>{e(C)&&g(G)})}D(()=>E(T,I.api_key.delete1)),h(V,x),W()}var ka=A('<div><div class="flex"><!></div> <!></div>');function ga(V,i){H(i,!0);let y=J(i,"key",15),I=te();const C=[I.tabs.config,"Secret",I.tabs.delete];let d=S(k(C[0]));var x=ka(),b=m(x),T=m(b);He(T,{tabs:C,get selected(){return e(d)},set selected(g){n(d,k(g))}}),p(b);var z=f(b,2);{var L=g=>{ia(g,{get onSave(){return i.onSave},get key(){return y()},set key(B){y(B)}})},G=g=>{var B=ue(),s=U(B);{var a=l=>{ua(l,{get key(){return y()}})},$=l=>{var w=ue(),P=U(w);{var o=c=>{fa(c,{get key(){return y()},get onSave(){return i.onSave}})};R(P,c=>{e(d)===I.tabs.delete&&c(o)},!0)}h(l,w)};R(s,l=>{e(d)==="Secret"?l(a):l($,!1)},!0)}h(g,B)};R(z,g=>{e(d)===I.tabs.config?g(L):g(G,!1)})}p(x),h(V,x),W()}var ha=A("<div><!></div>"),ya=A('<!> <!> <!> <!> <!> <div class="err"> </div>',1),ba=A('<div class="container svelte-1dweyfr"><!></div>');function wa(V,i){H(i,!0);const y=ie();let I=se(),C=te(),d=S(""),x=S(!1),b=S(void 0),T=S(""),z=S(k(ie())),L=S(k(me()));async function G(s,a){var P;n(d,"");for(let o of i.keys)if(o.name===e(T)){n(d,"Name already exists");return}let $=(P=e(b))==null?void 0:P();if(!$){console.error("access rights is undefined");return}let l={name:e(T),access:$};if(l.access.length===0){n(d,"Grant the new API Key at least one permission");return}if(e(x)){if(console.log(),!e(z)||!e(L)){n(d,"Disable expiry or provide an valid date and time");return}let o=be(e(z),e(L));if(!o){n(d,"Invalid Date Input: User Expires");return}l.exp=o}let w=await Be(s.action,l);w.error?n(d,k(w.error.message)):i.onSave()}var g=ba(),B=m(g);Qe(B,{action:"/auth/v1/api_keys",onSubmit:G,children:(s,a)=>{var $=ya(),l=U($);We(l,{get label(){return C.api_key.keyName},get placeholder(){return C.api_key.keyName},autocomplete:"off",required:!0,min:"2",max:"24",pattern:Je,get value(){return e(T)},set value(t){n(T,k(t))}});var w=f(l,2);re(w,{get ariaLabel(){return C.api_key.limitedValidity},get checked(){return e(x)},set checked(t){n(x,k(t))},children:(t,v)=>{K();var N=M();D(()=>E(N,C.api_key.limitedValidity)),h(t,N)},$$slots:{default:!0}});var P=f(w,2);{var o=t=>{var v=ha(),N=m(v);we(N,{get label(){return C.api_key.expires},withTime:!0,min:y,required:!0,get value(){return e(z)},set value(O){n(z,k(O))},get timeValue(){return e(L)},set timeValue(O){n(L,k(O))}}),p(v),ee(3,v,()=>ae),h(t,v)};R(P,t=>{e(x)&&t(o)})}var c=f(P,2);$e(c,{get finalize(){return e(b)},set finalize(t){n(b,k(t))}});var u=f(c,2);Q(u,{type:"submit",children:(t,v)=>{K();var N=M();D(()=>E(N,I.common.save)),h(t,N)},$$slots:{default:!0}});var _=f(u,2),r=m(_,!0);p(_),D(()=>E(r,e(d))),h(s,$)},$$slots:{default:!0}}),p(g),h(V,g),W()}var $a=A("<div></div> <!>",1),xa=A("<!> <!>",1),_a=A('<div id="keys"><!></div>'),za=A(" <!> <!>",1);function Ia(V,i){H(i,!0);let y=te(),I=S(void 0),C=S(""),d=S(k([])),x=S(k([])),b=S(void 0),T=Ve("kn");const z=["Name"],L=["Name"];let G=S(k(L[0])),g=S("");ke(()=>{B()}),Y(()=>{n(b,k(e(d).find(o=>o.name===T.get())))}),Y(()=>{let o=e(g).toLowerCase();o?e(G)===L[0]&&n(x,k(e(d).filter(c=>{var u;return(u=c.name)==null?void 0:u.includes(o)}))):n(x,k(e(d)))});async function B(){var c;let o=await Ae("/auth/v1/api_keys");o.body?n(d,k(o.body.keys)):n(C,k(((c=o.error)==null?void 0:c.message)||"Error"))}function s(o,c){let u=c==="up";o===z[0]&&e(d).sort((_,r)=>u?_.name.localeCompare(r.name):r.name.localeCompare(_.name))}function a(){var o;(o=e(I))==null||o(),B()}K();var $=za(),l=U($),w=f(l);Ee(w,{paddingTop:"2.1rem",buttonTilesAriaControls:"keys",width:"min(20rem, 100dvw)",thresholdNavSub:700,buttonTiles:o=>{var c=$a(),u=U(c);ne(u,"height",".5rem");var _=f(u,2);ge(_,17,()=>e(x),r=>r.name,(r,t)=>{var v=F(()=>T.get()===e(t).name);De(r,{onclick:()=>T.set(e(t).name),get selected(){return e(v)},children:(N,O)=>{K();var j=M();D(()=>E(j,e(t).name)),h(N,j)},$$slots:{default:!0}})}),h(o,c)},children:(o,c)=>{var u=xa(),_=U(u),r=F(()=>e(d).length===0?1:2);Ne(_,{get level(){return e(r)},alignRight:!0,get closeModal(){return e(I)},set closeModal(v){n(I,k(v))},children:(v,N)=>{wa(v,{get keys(){return e(d)},onSave:a})},$$slots:{default:!0}});var t=f(_,2);je(t,{searchOptions:L,orderOptions:z,onChangeOrder:s,searchWidth:"min(19.5rem, 100dvw - .5rem)",get searchOption(){return e(G)},set searchOption(v){n(G,k(v))},get value(){return e(g)},set value(v){n(g,k(v))}}),h(o,u)},$$slots:{buttonTiles:!0,default:!0}});var P=f(w,2);Ge(P,{children:(o,c)=>{var u=_a(),_=m(u);{var r=t=>{ga(t,{onSave:B,get key(){return e(b)},set key(v){n(b,k(v))}})};R(_,t=>{e(b)&&t(r)})}p(u),D(()=>oe(u,"aria-label",y.common.details)),h(o,u)},$$slots:{default:!0}}),D(()=>E(l,`${e(C)??""} `)),h(V,$),W()}function La(V){Ia(V,{})}export{La as component};
