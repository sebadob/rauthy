import"../chunks/disclose-version.BDr9Qe-U.js";import"../chunks/legacy.DUFkTaXF.js";import{p as X,ao as ee,s as y,c as n,g as t,r as v,t as B,a as F,a9 as L,f as J,i as _,aq as G,h as r}from"../chunks/index-client.CzvcfCLI.js";import{d as oe,e as te,s as R,k as fe}from"../chunks/render.BJBkrb2f.js";import{i as O}from"../chunks/if.Dlf0zx3_.js";import{e as ae,i as re}from"../chunks/each.BtetwUOf.js";import{a as f,t as N,e as se,d as V}from"../chunks/template.CQASMEo4.js";import{t as W,a as H,s as le}from"../chunks/index.CqGLefLa.js";import{p as $}from"../chunks/proxy.CHyqegA9.js";import{T as he,a as ge,P as xe}from"../chunks/Pagination.B865uVVG.js";import{B as Q}from"../chunks/Button.BMVss8ej.js";import{y as ye,z as we,A as $e,B as be,C as ke}from"../chunks/dataFetchingAdmin.D6QJ_imf.js";import{p as M,r as Se}from"../chunks/props.BRvF51YW.js";import{r as ie}from"../chunks/legacy-client.CQT_jR_Y.js";import{c as Ke,a as Ae,I as ne}from"../chunks/Input.BmqIhqeF.js";import{$ as Ee,B as ce,o as Ie,k as je}from"../chunks/helpers.kNqlWGzE.js";import{S as de}from"../chunks/Switch.CwDfiuqK.js";import{r as _e,d as ze}from"../chunks/class.DOSp4p8N.js";import{a as Ne,b as Ce}from"../chunks/input.mmO6A0Hj.js";import{E as De}from"../chunks/ExpandContainer.96GzOZVL.js";import{T as qe}from"../chunks/TabBar.DO8vwRCx.js";import{I as Te,a as Pe,b as Oe,P as Ue}from"../chunks/PasswordInput.DZqvQQpj.js";var Me=N('<div role="button" tabindex="0" class="cell pointer svelte-lgl56a"> </div>'),Ye=N('<div class="cell svelte-lgl56a"><input class="pointer svelte-lgl56a" type="checkbox"></div>'),Be=N('<div class="mr svelte-lgl56a"><div role="button" tabindex="0" class="label pointer svelte-lgl56a"> </div> <!></div>'),Ge=N('<div class="matrix svelte-lgl56a"><b>Access Rights</b> <div class="mr svelte-lgl56a"><div class="label svelte-lgl56a"></div> <!></div> <!></div>');function pe(P,p){X(p,!0);let u=M(p,"accessMatrix",15);M(p,"finalize",15)(o);const l=["Blacklist","Clients","Events","Generic","Groups","Roles","Secrets","Sessions","Scopes","UserAttributes","Users"],h=["create","read","update","delete"];let b=new Array(l.length).fill(!1),w=new Array(h.length).fill(!1);ee(()=>{var s;S(),(s=p.apiKey)!=null&&s.access&&D()});function S(){let s=new Array(l.length);for(let d=0;d<l.length;d++){s[d]={group:l[d]};for(let i of h)s[d][i]=!1}u(s)}function D(){for(let s of p.apiKey.access){let d=l.findIndex(i=>i===s.group);for(let i of s.access_rights){let g=h.findIndex(I=>I===i);u(u()[d][h[g]]=!0,!0)}}}function o(){let s=[];for(let d=0;d<l.length;d++){let i=[];for(let g of h)u()[d][g]&&i.push(g);i.length>0&&s.push({group:l[d],access_rights:i})}return s}function c(s){b[s]=!b[s];const d=b[s];let i=u()[s];for(let g=0;g<h.length;g++)for(let I of Object.keys(i))I!=="group"&&(i[I]=d);u(u()[s]=i,!0)}function E(s){w[s]=!w[s];const d=w[s];for(let i=0;i<l.length;i++)u(u()[i][h[s]]=d,!0)}var j=Ge(),z=y(n(j),2),K=y(n(z),2);ae(K,17,()=>h,re,(s,d,i)=>{var g=Me(),I=L(()=>E.bind(this,i));g.__click=function(...A){var e;(e=t(I))==null||e.apply(this,A)};var k=L(()=>E.bind(this,i)),T=n(g,!0);v(g),B(()=>R(T,h[i])),te("keypress",g,function(...A){var e;(e=t(k))==null||e.apply(this,A)}),f(s,g)}),v(z);var C=y(z,2);{var m=s=>{var d=se(),i=J(d);ae(i,17,()=>l,re,(g,I,k)=>{var T=Be(),A=n(T),e=L(()=>c.bind(this,k));A.__click=function(...Y){var U;(U=t(e))==null||U.apply(this,Y)};var a=L(()=>c.bind(this,k)),x=n(A,!0);v(A);var q=y(A,2);ae(q,17,()=>h,re,(Y,U)=>{var Z=Ye(),ve=n(Z);_e(ve),v(Z),Ne(ve,()=>u()[k][t(U)],me=>u(u()[k][t(U)]=me,!0)),f(Y,Z)}),v(T),B(()=>R(x,l[k])),te("keypress",A,function(...Y){var U;(U=t(a))==null||U.apply(this,Y)}),f(g,T)}),f(s,d)};O(C,s=>{u()&&s(m)})}return v(j),f(P,j),F({finalizeMatrix:o})}oe(["click"]);var Re=N("<div><!></div>"),We=N('<div class="container svelte-ubtepy"><!> <div class="switch svelte-ubtepy">Key Expires: <!></div> <!> <!> <div class="saveBtn svelte-ubtepy"><!></div> <div class="err svelte-ubtepy"> </div></div>');function He(P,p){X(p,!0);const u=new Date().toISOString().split(".")[0];let l=M(p,"apiKeys",19,()=>[]),h=M(p,"onSave",3,()=>{}),b=_(""),w=_(!1),S=_(void 0),D=_(void 0),o=$({name:"",exp:""}),c=_($({}));const E=Ke().shape({name:Ae().required("Name is required").min(2).max(24).matches(Ee,"Format: [a-zA-Z0-9_/-]{2,24}")});ie(()=>{t(w)&&(o.exp=new Date().toISOString().split(".")[0])});async function j(){if(r(b,""),!await z())return;let e={name:o.name,access:t(S)()};if(e.access.length===0){r(b,"Grant the new API Key at least one permission");return}if(t(w)){let x=ce(o.exp);if(!x){r(b,"Invalid Date Input: User Expires");return}e.exp=x}let a=await ye(e);if(a.ok)h()();else{let x=await a.json();r(b,$(x.message))}}async function z(){try{await E.validate(o,{abortEarly:!1}),r(c,$({}))}catch(e){return r(c,$(Ie(e))),!1}for(let e of l())if(e.name===o.name)return t(c).name="Name already exists",!1;return!0}var K=We(),C=n(K);ne(C,{width:"18rem",autocomplete:"off",placeholder:"Name",get value(){return o.name},set value(e){o.name=e},get error(){return t(c).name},set error(e){t(c).name=e},children:(e,a)=>{G();var x=V("NAME");f(e,x)},$$slots:{default:!0}});var m=y(C,2),s=y(n(m));de(s,{get selected(){return t(w)},set selected(e){r(w,$(e))}}),v(m);var d=y(m,2);{var i=e=>{var a=Re(),x=n(a);ne(x,{type:"datetime-local",step:"60",width:"18rem",min:u,max:"2099-01-01T00:00",get value(){return o.exp},set value(q){o.exp=q},children:(q,Y)=>{G();var U=V("EXPIRES");f(q,U)},$$slots:{default:!0}}),v(a),W(3,a,()=>H),f(e,a)};O(d,e=>{t(w)&&e(i)})}var g=y(d,2);pe(g,{get accessMatrix(){return t(D)},set accessMatrix(e){r(D,$(e))},get finalize(){return t(S)},set finalize(e){r(S,$(e))}});var I=y(g,2),k=n(I);Q(k,{level:1,$$events:{click:j},children:(e,a)=>{G();var x=V("SAVE");f(e,x)},$$slots:{default:!0}}),v(I);var T=y(I,2),A=n(T,!0);v(T),v(K),B(()=>R(A,t(b))),f(P,K),F()}var Ve=N("<div><!></div>"),Xe=N('<div class="success svelte-1a3jnb3">Success</div>'),Fe=N('<div class="mainErr err svelte-1a3jnb3"> </div>'),Ze=N('<div class="container svelte-1a3jnb3"><div class="unit svelte-1a3jnb3"><div class="label svelte-1a3jnb3">Name</div> <div class="value svelte-1a3jnb3"> </div></div> <div class="switch svelte-1a3jnb3">Key Expires: <!></div> <!> <!> <!> <!> <!></div>');function Je(P,p){X(p,!0);const u=new Date().toISOString().split(".")[0];let l=M(p,"apiKey",27,()=>$({})),h=_(""),b=_(!1),w=_(void 0),S=_(!!l().expires),D=_(void 0),o=_(void 0);ie(()=>{t(b)&&r(w,$(setTimeout(()=>{r(b,!1),p.onSave()},2e3)))}),ee(()=>()=>clearTimeout(t(w)));let c=$({exp:""});ie(()=>{t(S)&&(c.exp=new Date().toISOString().split(".")[0])});async function E(){r(h,"");let a={name:l().name,access:t(o)()};if(a.access.length===0){r(h,"Grant the API Key at least one permission");return}if(t(S)){let q=ce(c.exp);if(!q){r(h,"Invalid Date Input: User Expires");return}a.exp=q}let x=await we(a);if(x.ok)p.onSave(),r(b,!0);else{let q=await x.json();r(h,$(q.message))}}var j=Ze(),z=n(j),K=y(n(z),2),C=n(K,!0);v(K),v(z);var m=y(z,2),s=y(n(m));de(s,{get selected(){return t(S)},set selected(a){r(S,$(a))}}),v(m);var d=y(m,2);{var i=a=>{var x=Ve(),q=n(x);ne(q,{type:"datetime-local",step:"60",width:"18rem",min:u,max:"2099-01-01T00:00",get value(){return c.exp},set value(Y){c.exp=Y},children:(Y,U)=>{G();var Z=V("EXPIRES");f(Y,Z)},$$slots:{default:!0}}),v(x),W(3,x,()=>H),f(a,x)};O(d,a=>{t(S)&&a(i)})}var g=y(d,2);pe(g,{get apiKey(){return l()},get accessMatrix(){return t(D)},set accessMatrix(a){r(D,$(a))},get finalize(){return t(o)},set finalize(a){r(o,$(a))}});var I=y(g,2);Q(I,{level:1,width:"4rem",$$events:{click:E},children:(a,x)=>{G();var q=V("SAVE");f(a,q)},$$slots:{default:!0}});var k=y(I,2);{var T=a=>{var x=Xe();f(a,x)};O(k,a=>{t(b)&&a(T)})}var A=y(k,2);{var e=a=>{var x=Fe(),q=n(x,!0);v(x),B(()=>R(q,t(h))),f(a,x)};O(A,a=>{t(h)&&a(e)})}v(j),B(()=>R(C,l().name)),f(P,j),F()}var Le=N('<div class="err svelte-1jljqe9"> </div>'),Qe=N('<div class="container svelte-1jljqe9"><div class="label svelte-1jljqe9">Are you sure, you want to delete this ApiKey?</div> <!> <!></div>');function et(P,p){X(p,!0);let u=M(p,"apiKey",19,()=>({})),l=_("");async function h(){r(l,"");let o=await $e(u().name);if(o.ok)p.onSave();else{let c=await o.json();r(l,$(c.message))}}var b=Qe(),w=y(n(b),2);Q(w,{level:1,$$events:{click:h},children:(o,c)=>{G();var E=V("DELETE");f(o,E)},$$slots:{default:!0}});var S=y(w,2);{var D=o=>{var c=Le(),E=n(c,!0);v(c),B(()=>R(E,t(l))),f(o,c)};O(S,o=>{t(l)&&o(D)})}v(b),f(P,b),F()}var tt=N('<div><div class="iconsOuter svelte-s24qr4"><div class="iconsInner svelte-s24qr4"><div role="button" tabindex="0" class="show svelte-s24qr4"><!></div> <div role="button" tabindex="0"><!></div></div></div> <textarea></textarea></div>');function ue(P,p){X(p,!0);let u=M(p,"value",3,""),l=M(p,"rows",3,10),h=M(p,"cols",3,60),b=M(p,"name",3,"default"),w=M(p,"show",15,!1),S=M(p,"width",3,"40rem"),D=Se(p,["$$slots","$$events","$$legacy","value","rows","cols","name","show","width"]),o=_(""),c=_("");ie(()=>{w()?r(c,u()):r(c,$(t(o)))}),ee(()=>{for(let A=0;A<u().length;A++)r(o,t(o)+"*");r(c,$(t(o)))});function E(){navigator.clipboard.writeText(u())}function j(){w(!w())}var z=tt(),K=n(z),C=n(K),m=n(C);m.__click=j;var s=n(m);{var d=A=>{Pe(A,{width:22})},i=A=>{Oe(A,{width:22})};O(s,A=>{w()?A(d):A(i,!1)})}v(m);var g=y(m,2);g.__click=E;var I=n(g);Te(I,{}),v(g),v(C),v(K);var k=y(K,2);fe(k);let T;v(z),B(()=>{le(z,"width",`${S()}`),T=ze(k,T,{disabled:!0,name:b(),rows:l(),cols:h(),...D},"svelte-s24qr4"),le(k,"width",`${S()}`),le(k,"padding-right","2.75rem")}),te("keypress",m,j),te("keypress",g,E),Ce(k,()=>t(c),A=>r(c,A)),f(P,z),F()}oe(["click"]);var at=N(`<div class="secret font-mono svelte-1vixcxf"><!></div> <p class="svelte-1vixcxf">An API Key must be provided in the HTTP <span class="headerCodeSmall svelte-1vixcxf"><code>Authorization</code></span> header in the following format:</p> <p class="svelte-1vixcxf"><span class="headerCode svelte-1vixcxf"><b><code>API-Key YourSecretApiKeyHere</code></b></span></p> <p class="svelte-1vixcxf">You can use the following <code>curl</code> request to test your new Key:</p> <!> <p class="svelte-1vixcxf">If you don't have <code>jq</code> installed and the above fails:</p> <!>`,1),st=N(`<div class="container svelte-1vixcxf"><p class="svelte-1vixcxf">You Can generate a new secret for this API Key here.</p> <p class="svelte-1vixcxf">You will only see this secret once after the generation.
        When a new one has been generated, the old secret will be overridden permanently.
        This operation cannot be reverted!</p> <!> <!> <div class="err svelte-1vixcxf"> </div></div>`);function it(P,p){X(p,!0);let u=_(""),l=_(""),h=_(""),b=_("");async function w(){let K=await be(p.apiKey.name);if(K.ok)r(l,$(await K.text())),r(h,`curl -s -H 'Authorization: API-Key ${t(l)}' ${window.location.origin}/auth/v1/api_keys/${p.apiKey.name}/test`),r(b,`${t(h)} | jq`);else{let C=await K.json();r(u,$(C.message))}}var S=st(),D=y(n(S),4),o=L(()=>t(l)?3:1);Q(D,{get level(){return t(o)},width:"130px",$$events:{click:w},children:(K,C)=>{G();var m=V("GENERATE NEW");f(K,m)},$$slots:{default:!0}});var c=y(D,2);{var E=K=>{var C=at(),m=J(C),s=n(m);Ue(s,{autocomplete:"off",disabled:!0,showCopy:!0,width:"inherit",maxWidth:"800px",get value(){return t(l)},set value(g){r(l,$(g))},children:(g,I)=>{G();var k=V("API KEY");f(g,k)},$$slots:{default:!0}}),v(m);var d=y(m,8);ue(d,{name:"id",rows:3,get value(){return t(b)}});var i=y(d,4);ue(i,{name:"id",rows:3,get value(){return t(h)}}),W(3,m,()=>H),f(K,C)};O(c,K=>{t(l)&&K(E)})}var j=y(c,2),z=n(j,!0);v(j),v(S),B(()=>R(z,t(u))),f(P,S),F()}var rt=N('<div class="data svelte-115hohv"> </div>'),lt=N('<div class="header svelte-115hohv"><div class="data svelte-115hohv"> </div> <!></div>'),nt=N("<div><!></div>"),vt=N("<div><!></div>"),ot=N("<div><!></div>"),ct=N("<div><!> <!></div>");function dt(P,p){X(p,!0);let u=M(p,"apiKey",31,()=>$({})),l=M(p,"onSave",11,()=>{}),h=_(void 0);const b=["Config","Secret","Delete"];let w=_($(b[0]));const S=200,D=S/2;function o(){r(h,!1),l()()}De(P,{get show(){return t(h)},set show(c){r(h,$(c))},header:c=>{var E=lt(),j=n(E),z=n(j,!0);v(j);var K=y(j,2);{var C=m=>{he(m,{text:"Expiry",children:(s,d)=>{var i=rt(),g=n(i,!0);B(()=>R(g,je(u().expires))),v(i),f(s,i)},$$slots:{default:!0}})};O(K,m=>{u().expires&&m(C)})}v(E),B(()=>R(z,u().name)),f(c,E)},body:c=>{var E=ct(),j=n(E);qe(j,{labels:b,get selected(){return t(w)},set selected(m){r(w,$(m))}});var z=y(j,2);{var K=m=>{var s=nt(),d=n(s);Je(d,{get onSave(){return l()},get apiKey(){return u()},set apiKey(i){u(i)}}),v(s),W(1,s,()=>H,()=>({delay:D,duration:S})),W(2,s,()=>H,()=>({duration:S})),f(m,s)},C=m=>{var s=se(),d=J(s);{var i=I=>{var k=vt(),T=n(k);it(T,{get apiKey(){return u()}}),v(k),W(1,k,()=>H,()=>({delay:D,duration:S})),W(2,k,()=>H,()=>({duration:S})),f(I,k)},g=I=>{var k=se(),T=J(k);{var A=e=>{var a=ot(),x=n(a);et(x,{get apiKey(){return u()},onSave:o}),v(a),W(1,a,()=>H,()=>({delay:D,duration:S})),W(2,a,()=>H,()=>({duration:S})),f(e,a)};O(T,e=>{t(w)==="Delete"&&e(A)},!0)}f(I,k)};O(d,I=>{t(w)==="Secret"?I(i):I(g,!1)},!0)}f(m,s)};O(z,m=>{t(w)==="Config"?m(K):m(C,!1)})}v(E),f(c,E)},$$slots:{header:!0,body:!0}}),F()}var pt=N("<div><!></div>"),ut=N('<div class="svelte-1fzx60h">No Api Keys</div>'),mt=N(' <div class="content"><div class="top svelte-1fzx60h"><!> <div class="addNew svelte-1fzx60h"><!></div></div> <!> <div id="keys" class="svelte-1fzx60h"><!></div> <!> <div style="height: 20px"></div></div>',1);function ft(P,p){X(p,!0);let u=_(""),l=_($([])),h=_($([])),b=_($([])),w=_(!1);const S=[{label:"Name",callback:(e,a)=>e.name.includes(a)}];let D=[{label:"Name",callback:(e,a)=>e.name.localeCompare(a.name)}];ee(()=>{o()});async function o(){let e=await ke(),a=await e.json();e.ok?(r(l,$(a.keys)),r(w,!1)):r(u,$(a.message))}G();var c=mt(),E=J(c),j=y(E),z=n(j),K=n(z);ge(K,{get items(){return t(l)},searchOptions:S,orderOptions:D,get resItems(){return t(h)},set resItems(e){r(h,$(e))}});var C=y(K,2),m=n(C);Q(m,{level:3,$$events:{click:()=>r(w,!t(w))},children:(e,a)=>{G();var x=V("NEW KEY");f(e,x)},$$slots:{default:!0}}),v(C),v(z);var s=y(z,2);{var d=e=>{var a=pt(),x=n(a);He(x,{onSave:o,get apiKeys(){return t(l)}}),v(a),W(3,a,()=>H),f(e,a)};O(s,e=>{t(w)&&e(d)})}var i=y(s,2),g=n(i);{var I=e=>{var a=ut();f(e,a)},k=e=>{var a=se(),x=J(a);ae(x,19,()=>t(b),q=>q.name,(q,Y,U)=>{dt(q,{onSave:o,get apiKey(){return t(b)[t(U)]},set apiKey(Z){t(b)[t(U)]=Z}})}),f(e,a)};O(g,e=>{t(l).length===0?e(I):e(k,!1)})}v(i);var T=y(i,2);{var A=e=>{xe(e,{get items(){return t(h)},set items(a){r(h,$(a))},get resItems(){return t(b)},set resItems(a){r(b,$(a))}})};O(T,e=>{t(l).length>0&&e(A)})}G(2),v(j),B(()=>R(E,`${t(u)??""} `)),f(P,c),F()}function ht(P){ft(P,{})}export{ht as component};
