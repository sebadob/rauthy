import{a as y,t as P,e as F,d as ue}from"../chunks/disclose-version.BP4T4xdV.js";import{a as H,aa as W,s as f,c as u,p as J,j as e,a7 as j,t as N,r as m,a8 as fe,a9 as M,l as A,k as s,f as R,a5 as ke}from"../chunks/index-client.hxMyB_b4.js";import{s as D,r as Ce}from"../chunks/render.yNX9q8-B.js";import{i as B}from"../chunks/if.BeBa1Kxb.js";import{e as ge,i as Ae}from"../chunks/each.BUp7aLCc.js";import{s as oe,d as Le}from"../chunks/class.CF2J4YCf.js";import{B as Q,t as ee,s as ae,a as ne}from"../chunks/Button.BR1gOfNn.js";import{p as g}from"../chunks/proxy.DaVNR9vk.js";import{d as he,c as Ve,b as Te,f as Pe}from"../chunks/fetch.Bq59EV_b.js";import{C as qe}from"../chunks/ContentAdmin.YSUtpxca.js";import{N as Ge}from"../chunks/NavButtonTile.DRctQzgA.js";import{N as Ne}from"../chunks/NavSub.DILgx03i.js";import{B as De}from"../chunks/ButtonAddModal.DsDc6nUh.js";import{u as Ee}from"../chunks/param.svelte.BbzO27-O.js";import{u as re}from"../chunks/i18n_admin.svelte.lt3dD7TQ.js";import{O as Oe}from"../chunks/OrderSearchBar.wIdu7A5l.js";import{p as X,r as Ke}from"../chunks/props.DNAkfByl.js";import{B as ye}from"../chunks/helpers.CchIKxjM.js";import{I as Ue}from"../chunks/IconCheck.C6tGCpWy.js";import{L as je}from"../chunks/LabeledValue.CxxfjE_4.js";import{I as te}from"../chunks/InputCheckbox.BFLz2Hte.js";import{I as be}from"../chunks/InputDateTimeCombo.VZN-6SG8.js";import{u as ie}from"../chunks/i18n.svelte.Ci9wvqOq.js";import{f as se,a as me}from"../chunks/form.DekAF1-C.js";import{k as $e}from"../chunks/key.DIUIfgzP.js";import{h as pe}from"../chunks/html.704f8A94.js";import{b as Re}from"../chunks/input.BqbgywMu.js";import{a as Be,b as Fe,c as Me,I as Ze}from"../chunks/InputPassword.DvK4K3vC.js";import{T as He}from"../chunks/Tabs.DUIpSMWU.js";import{I as We}from"../chunks/pow.BMnvYon0.js";import{j as Je}from"../chunks/patterns.BRCxk8by.js";import{F as Qe}from"../chunks/Form.BnvI_UPN.js";var Xe=P('<div class="center svelte-opxw4z"><!></div>'),Ye=P("<div><!></div>"),ea=P('<div class="row svelte-opxw4z"><!> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div> <div class="center svelte-opxw4z"><!></div></div>'),aa=P('<div class="matrix svelte-opxw4z"><div class="row svelte-opxw4z"><div></div> <!> <!> <!> <!></div> <!></div>');function we(E,o){H(o,!0),X(o,"finalize",15)(z);let h=g([["Blacklist",!1,!1,!1,!1],["Clients",!1,!1,!1,!1],["Events",!1,!1,!1,!1],["Generic",!1,!1,!1,!1],["Groups",!1,!1,!1,!1],["Roles",!1,!1,!1,!1],["Secrets",!1,!1,!1,!1],["Sessions",!1,!1,!1,!1],["Scopes",!1,!1,!1,!1],["UserAttributes",!1,!1,!1,!1],["Users",!1,!1,!1,!1]]);W(()=>{if(T(),o.key)for(let t of o.key.access){let a;switch(t.group){case"Blacklist":a=0;break;case"Clients":a=1;break;case"Events":a=2;break;case"Generic":a=3;break;case"Groups":a=4;break;case"Roles":a=5;break;case"Secrets":a=6;break;case"Sessions":a=7;break;case"Scopes":a=8;break;case"UserAttributes":a=9;break;case"Users":a=10;break}if(a===void 0){console.error("invalid idx during access key buildup");return}t.access_rights.includes("create")&&(h[a][1]=!0),t.access_rights.includes("read")&&(h[a][2]=!0),t.access_rights.includes("update")&&(h[a][3]=!0),t.access_rights.includes("delete")&&(h[a][4]=!0)}});function z(){let t=[];for(let a of h){let w=[];a[1]&&w.push("create"),a[2]&&w.push("read"),a[3]&&w.push("update"),a[4]&&w.push("delete"),w.length>0&&t.push({group:a[0],access_rights:w})}return t}function T(){for(let t of h)t[1]=!1,t[2]=!1,t[3]=!1,t[4]=!1}function v(t){for(let a of h)if(a[0]===t){a[1]&&a[2]&&a[3]&&a[4]?(a[1]=!1,a[2]=!1,a[3]=!1,a[4]=!1):(a[1]=!0,a[2]=!0,a[3]=!0,a[4]=!0);break}}function x(t){let a;switch(t){case"create":a=1;break;case"read":a=2;break;case"update":a=3;break;case"delete":a=4;break}if(!a){console.error("logic error in toggleRight, idx is undefined");return}let w=!0;for(let k of h)if(!k[a]){w=!1;break}for(let k of h)k[a]=!w}var b=aa();{const t=(w,k=fe)=>{var p=Xe(),V=u(p),n=M(()=>`Toggle: ${k()}`);Q(V,{get ariaLabel(){return e(n)},invisible:!0,onclick:()=>x(k()),children:(d,l)=>{j();var _=F();N(()=>D(_,k())),y(d,_)},$$slots:{default:!0}}),m(p),y(w,p)},a=(w,k=fe)=>{var p=Ye(),V=u(p),n=M(()=>`Toggle: ${k()}`);Q(V,{get ariaLabel(){return e(n)},invisible:!0,onclick:()=>v(k()),children:(d,l)=>{j();var _=F();N(()=>D(_,k())),y(d,_)},$$slots:{default:!0}}),m(p),y(w,p)};var L=u(b),S=f(u(L),2);t(S,()=>"create");var I=f(S,2);t(I,()=>"read");var q=f(I,2);t(q,()=>"update");var C=f(q,2);t(C,()=>"delete"),m(L);var $=f(L,2);ge($,17,()=>h,Ae,(w,k,p)=>{var V=ea(),n=u(V);a(n,()=>e(k)[0]);var d=f(n,2),l=u(d),_=M(()=>`${e(k)[0]}: create`);te(l,{get ariaLabel(){return e(_)},get checked(){return h[p][1]},set checked(U){h[p][1]=U}}),m(d);var r=f(d,2),i=u(r),c=M(()=>`${e(k)[0]}: read`);te(i,{get ariaLabel(){return e(c)},get checked(){return h[p][2]},set checked(U){h[p][2]=U}}),m(r);var G=f(r,2),K=u(G),O=M(()=>`${e(k)[0]}: update`);te(K,{get ariaLabel(){return e(O)},get checked(){return h[p][3]},set checked(U){h[p][3]=U}}),m(G);var Z=f(G,2),le=u(Z),Y=M(()=>`${e(k)[0]}: delete`);te(le,{get ariaLabel(){return e(Y)},get checked(){return h[p][4]},set checked(U){h[p][4]=U}}),m(Z),m(V),y(w,V)}),m(b)}y(E,b),J()}var ra=P("<div><!></div>"),ta=P('<div class="err"> </div>'),ia=P('<!> <!> <!> <!> <div class="btn svelte-1j50zcx"><!> <!> <!></div>',1);function sa(E,o){H(o,!0);let h=ie(),z=re();const T=se();let v=A(""),x=A(!1),b=A(!!o.key.expires),L=A(g(se())),S=A(g(me())),I=A(void 0);W(()=>{o.key.name&&s(b,!!o.key.expires)}),W(()=>{if(e(b)){let r;o.key.expires?r=new Date(o.key.expires*1e3):r=new Date,s(L,g(se(r))),s(S,g(me(r)))}});async function q(){if(s(v,""),!e(I)){console.error("finalizeMatrix is undefined");return}let r={name:o.key.name,access:e(I)()};if(r.access.length===0){s(v,"Grant the API Key at least one permission");return}if(e(b)){let c=ye(e(L),e(S));if(!c){s(v,"Invalid Date Input: User Expires");return}r.exp=c}let i=await he(`/auth/v1/api_keys/${o.key.name}`,r);i.error?s(v,g(i.error.message)):(o.onSave(),s(x,!0),setTimeout(()=>{s(x,!1),o.onSave()},2e3))}var C=ia(),$=R(C);je($,{get label(){return z.api_key.keyName},children:(r,i)=>{j();var c=F();N(()=>D(c,o.key.name)),y(r,c)},$$slots:{default:!0}});var t=f($,2);te(t,{get ariaLabel(){return z.api_key.limitedValidity},get checked(){return e(b)},set checked(r){s(b,g(r))},children:(r,i)=>{j();var c=F();N(()=>D(c,z.api_key.limitedValidity)),y(r,c)},$$slots:{default:!0}});var a=f(t,2);{var w=r=>{var i=ra(),c=u(i);be(c,{get label(){return z.api_key.expires},withTime:!0,min:T,required:!0,get value(){return e(L)},set value(G){s(L,g(G))},get timeValue(){return e(S)},set timeValue(G){s(S,g(G))}}),m(i),ee(3,i,()=>ae),y(r,i)};B(a,r=>{e(b)&&r(w)})}var k=f(a,2);we(k,{get key(){return o.key},get finalize(){return e(I)},set finalize(r){s(I,g(r))}});var p=f(k,2),V=u(p);Q(V,{onclick:q,children:(r,i)=>{j();var c=F();N(()=>D(c,h.common.save)),y(r,c)},$$slots:{default:!0}});var n=f(V,2);{var d=r=>{Ue(r,{})};B(n,r=>{e(x)&&r(d)})}var l=f(n,2);{var _=r=>{var i=ta(),c=u(i,!0);m(i),N(()=>D(c,e(v))),y(r,i)};B(l,r=>{e(v)&&r(_)})}m(p),y(E,C),J()}var oa=P("<div><!></div>"),na=P("<div><!></div>"),la=P("<div><!></div>"),ca=P('<div><div class="iconsOuter svelte-1pc4fk7"><div class="iconsInner svelte-1pc4fk7"><!> <!></div></div> <textarea></textarea></div>');function xe(E,o){H(o,!0);let h=X(o,"ariaLabel",3,""),z=X(o,"value",3,""),T=X(o,"rows",3,10),v=X(o,"cols",3,60),x=X(o,"show",15,!1),b=X(o,"width",3,"min(25rem, calc(100dvw - .5rem))"),L=Ke(o,["$$slots","$$events","$$legacy","ariaLabel","value","rows","cols","show","width"]),S=ie(),I=A(""),q=A("");W(()=>{x()?s(q,z()):s(q,g(e(I)))}),ke(()=>{for(let l=0;l<z().length;l++)s(I,e(I)+"*");s(q,g(e(I))),x(!1)});function C(){navigator.clipboard.writeText(z())}function $(){x(!x())}var t=ca(),a=u(t),w=u(a),k=u(w),p=M(()=>x()?S.common.hide:S.common.show);Q(k,{get ariaLabel(){return e(p)},invisible:!0,onclick:$,children:(l,_)=>{var r=ue(),i=R(r);{var c=K=>{var O=oa(),Z=u(O);Fe(Z,{width:22}),m(O),N(()=>oe(O,"title",S.common.hide)),y(K,O)},G=K=>{var O=na(),Z=u(O);Me(Z,{width:22}),m(O),N(()=>oe(O,"title",S.common.show)),y(K,O)};B(i,K=>{x()?K(c):K(G,!1)})}y(l,r)},$$slots:{default:!0}});var V=f(k,2);Q(V,{get ariaLabel(){return S.common.copyToClip},invisible:!0,onclick:C,children:(l,_)=>{var r=la(),i=u(r);Be(i,{}),m(r),N(()=>oe(r,"title",S.common.copyToClip)),y(l,r)},$$slots:{default:!0}}),m(w),m(a);var n=f(a,2);Ce(n);let d;m(t),N(()=>{ne(t,"width",`${b()}`),d=Le(n,d,{"aria-label":h(),disabled:!0,rows:T(),cols:v(),...L},"svelte-1pc4fk7"),ne(n,"width",b()),ne(n,"padding-right","2.75rem")}),Re(n,()=>e(q),l=>s(q,l)),y(E,t),J()}var va=P('<div class="secret font-mono svelte-1ii2aob"><!></div> <div><p><!></p> <p><span class="headerCode"><b><code></code></b></span></p></div> <div><p><!></p> <!></div> <div><p><!></p> <!></div>',1),da=P('<p> </p> <p> </p> <div class="btn svelte-1ii2aob"><!></div> <!> <div class="err"> </div>',1);function ua(E,o){H(o,!0);let h=ie(),z=re(),T=A(""),v=A(""),x=A(""),b=A("");W(()=>{o.key.name&&s(v,"")});async function L(){var l;let d=await he(`/auth/v1/api_keys/${o.key.name}/secret`);d.text?(s(v,g(d.text)),s(x,`curl -s -H 'Authorization: API-Key ${d.text}' ${window.location.origin}/auth/v1/api_keys/${o.key.name}/test`),s(b,`${e(x)} | jq`)):s(T,g(((l=d.error)==null?void 0:l.message)||"Error"))}var S=da(),I=R(S),q=u(I,!0);m(I);var C=f(I,2),$=u(C,!0);m(C);var t=f(C,2),a=u(t),w=M(()=>e(v)?3:1);Q(a,{get level(){return e(w)},onclick:L,children:(d,l)=>{j();var _=F();N(()=>D(_,h.passwordReset.generate)),y(d,_)},$$slots:{default:!0}}),m(t);var k=f(t,2);{var p=d=>{var l=va(),_=R(l),r=u(_);Ze(r,{autocomplete:"off",get value(){return e(v)},label:"API Key",placeholder:"API Key",disabled:!0,showCopy:!0,width:"min(25rem, calc(100dvw - .5rem))"}),m(_);var i=f(_,2),c=u(i),G=u(c);pe(G,()=>z.api_key.generate3),m(c);var K=f(c,2),O=u(K),Z=u(O),le=u(Z);le.textContent="API-Key <api_key>",m(Z),m(O),m(K),m(i);var Y=f(i,2),U=u(Y),_e=u(U);pe(_e,()=>z.api_key.generate4),m(U);var Se=f(U,2);$e(Se,()=>e(b),de=>{xe(de,{ariaLabel:"curl text command with jq",rows:5,get value(){return e(b)}})}),m(Y);var ce=f(Y,2),ve=u(ce),ze=u(ve);pe(ze,()=>z.api_key.generate5),m(ve);var Ie=f(ve,2);$e(Ie,()=>e(b),de=>{xe(de,{ariaLabel:"curl text command simple",rows:5,get value(){return e(x)}})}),m(ce),ee(3,_,()=>ae,()=>({duration:150})),ee(3,i,()=>ae,()=>({duration:150})),ee(3,Y,()=>ae,()=>({duration:150})),ee(3,ce,()=>ae,()=>({duration:150})),y(d,l)};B(k,d=>{e(v)&&d(p)})}var V=f(k,2),n=u(V,!0);m(V),N(()=>{D(q,z.api_key.generate1),D($,z.api_key.generate2),D(n,e(T))}),y(E,S),J()}var ma=P('<div class="err"> </div>'),pa=P("<p> </p> <!> <!>",1);function fa(E,o){H(o,!0);let h=ie(),z=re(),T=A("");async function v(){s(T,"");let C=await Ve(`/auth/v1/api_keys/${o.key.name}`);C.error?s(T,g(C.error.message)):o.onSave()}var x=pa(),b=R(x),L=u(b,!0);m(b);var S=f(b,2);Q(S,{level:-1,onclick:v,children:(C,$)=>{j();var t=F();N(()=>D(t,h.common.delete)),y(C,t)},$$slots:{default:!0}});var I=f(S,2);{var q=C=>{var $=ma(),t=u($,!0);m($),N(()=>D(t,e(T))),y(C,$)};B(I,C=>{e(T)&&C(q)})}N(()=>D(L,z.api_key.delete1)),y(E,x),J()}var ka=P('<div><div class="flex"><!></div> <!></div>');function ga(E,o){H(o,!0);let h=X(o,"key",15),z=re();const T=[z.tabs.config,"Secret",z.tabs.delete];let v=A(g(T[0])),x=A(void 0);W(()=>{var $;h().name&&(($=e(x))==null||$())});var b=ka(),L=u(b),S=u(L);He(S,{tabs:T,get selected(){return e(v)},set selected($){s(v,g($))},get focusFirst(){return e(x)},set focusFirst($){s(x,g($))}}),m(L);var I=f(L,2);{var q=$=>{sa($,{get onSave(){return o.onSave},get key(){return h()},set key(t){h(t)}})},C=$=>{var t=ue(),a=R(t);{var w=p=>{ua(p,{get key(){return h()}})},k=p=>{var V=ue(),n=R(V);{var d=l=>{fa(l,{get key(){return h()},get onSave(){return o.onSave}})};B(n,l=>{e(v)===z.tabs.delete&&l(d)},!0)}y(p,V)};B(a,p=>{e(v)==="Secret"?p(w):p(k,!1)},!0)}y($,t)};B(I,$=>{e(v)===z.tabs.config?$(q):$(C,!1)})}m(b),y(E,b),J()}var ha=P("<div><!></div>"),ya=P('<!> <!> <!> <!> <!> <div class="err"> </div>',1),ba=P('<div class="container svelte-1dweyfr"><!></div>');function $a(E,o){H(o,!0);const h=se();let z=ie(),T=re(),v=A(""),x=A(!1),b=A(void 0),L=A(""),S=A(g(se())),I=A(g(me()));async function q(t,a){var V;s(v,"");for(let n of o.keys)if(n.name===e(L)){s(v,"Name already exists");return}let w=(V=e(b))==null?void 0:V();if(!w){console.error("access rights is undefined");return}let k={name:e(L),access:w};if(k.access.length===0){s(v,"Grant the new API Key at least one permission");return}if(e(x)){if(console.log(),!e(S)||!e(I)){s(v,"Disable expiry or provide an valid date and time");return}let n=ye(e(S),e(I));if(!n){s(v,"Invalid Date Input: User Expires");return}k.exp=n}let p=await Te(t.action,k);p.error?s(v,g(p.error.message)):o.onSave()}var C=ba(),$=u(C);Qe($,{action:"/auth/v1/api_keys",onSubmit:q,children:(t,a)=>{var w=ya(),k=R(w);We(k,{get label(){return T.api_key.keyName},get placeholder(){return T.api_key.keyName},autocomplete:"off",required:!0,min:"2",max:"24",pattern:Je,get value(){return e(L)},set value(i){s(L,g(i))}});var p=f(k,2);te(p,{get ariaLabel(){return T.api_key.limitedValidity},get checked(){return e(x)},set checked(i){s(x,g(i))},children:(i,c)=>{j();var G=F();N(()=>D(G,T.api_key.limitedValidity)),y(i,G)},$$slots:{default:!0}});var V=f(p,2);{var n=i=>{var c=ha(),G=u(c);be(G,{get label(){return T.api_key.expires},withTime:!0,min:h,required:!0,get value(){return e(S)},set value(K){s(S,g(K))},get timeValue(){return e(I)},set timeValue(K){s(I,g(K))}}),m(c),ee(3,c,()=>ae),y(i,c)};B(V,i=>{e(x)&&i(n)})}var d=f(V,2);we(d,{get finalize(){return e(b)},set finalize(i){s(b,g(i))}});var l=f(d,2);Q(l,{type:"submit",children:(i,c)=>{j();var G=F();N(()=>D(G,z.common.save)),y(i,G)},$$slots:{default:!0}});var _=f(l,2),r=u(_,!0);m(_),N(()=>D(r,e(v))),y(t,w)},$$slots:{default:!0}}),m(C),y(E,C),J()}var wa=P("<div></div> <!>",1),xa=P("<!> <!>",1),_a=P('<div id="keys"><!></div>'),Sa=P(" <!> <!>",1);function za(E,o){H(o,!0);let h=re(),z=A(void 0),T=A(""),v=A(g([])),x=A(g([])),b=A(void 0),L=Ee("kn");const S=["Name"],I=["Name"];let q=A(g(I[0])),C=A("");ke(()=>{$()}),W(()=>{s(b,g(e(v).find(n=>n.name===L.get())))}),W(()=>{let n=e(C).toLowerCase();n?e(q)===I[0]&&s(x,g(e(v).filter(d=>{var l;return(l=d.name)==null?void 0:l.includes(n)}))):s(x,g(e(v)))});async function $(){var d;let n=await Pe("/auth/v1/api_keys");n.body?s(v,g(n.body.keys)):s(T,g(((d=n.error)==null?void 0:d.message)||"Error"))}function t(n,d){let l=d==="up";n===S[0]&&e(v).sort((_,r)=>l?_.name.localeCompare(r.name):r.name.localeCompare(_.name))}function a(){var n;(n=e(z))==null||n(),$()}j();var w=Sa(),k=R(w),p=f(k);Ne(p,{paddingTop:"2.1rem",buttonTilesAriaControls:"keys",width:"min(20rem, 100dvw)",thresholdNavSub:700,buttonTiles:n=>{var d=wa(),l=R(d);ne(l,"height",".5rem");var _=f(l,2);ge(_,17,()=>e(x),r=>r.name,(r,i)=>{var c=M(()=>L.get()===e(i).name);Ge(r,{onclick:()=>L.set(e(i).name),get selected(){return e(c)},children:(G,K)=>{j();var O=F();N(()=>D(O,e(i).name)),y(G,O)},$$slots:{default:!0}})}),y(n,d)},children:(n,d)=>{var l=xa(),_=R(l),r=M(()=>e(v).length===0?1:2);De(_,{get level(){return e(r)},alignRight:!0,get closeModal(){return e(z)},set closeModal(c){s(z,g(c))},children:(c,G)=>{$a(c,{get keys(){return e(v)},onSave:a})},$$slots:{default:!0}});var i=f(_,2);Oe(i,{searchOptions:I,orderOptions:S,onChangeOrder:t,searchWidth:"min(19.5rem, 100dvw - .5rem)",get searchOption(){return e(q)},set searchOption(c){s(q,g(c))},get value(){return e(C)},set value(c){s(C,g(c))}}),y(n,l)},$$slots:{buttonTiles:!0,default:!0}});var V=f(p,2);qe(V,{children:(n,d)=>{var l=_a(),_=u(l);{var r=i=>{ga(i,{onSave:$,get key(){return e(b)},set key(c){s(b,g(c))}})};B(_,i=>{e(b)&&i(r)})}m(l),N(()=>oe(l,"aria-label",h.common.details)),y(n,l)},$$slots:{default:!0}}),N(()=>D(k,`${e(T)??""} `)),y(E,w),J()}function Ia(E){za(E,{})}export{Ia as component};
