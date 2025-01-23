import"../chunks/disclose-version.BDr9Qe-U.js";import{p as oe,a3 as h,a4 as C,a as me,a6 as c,$ as le,g as t,j as s,a5 as m,aa as ne,a7 as y,a9 as k,a8 as E}from"../chunks/index-client.Cqn5bTAA.js";import{s as l}from"../chunks/render.BxZ_8nPF.js";import{i as M}from"../chunks/if.CfO-_epY.js";import{a as n,t as b,b as _,c as ve}from"../chunks/template.A7ni-K4i.js";import{h as ce}from"../chunks/svelte-head.DTZb8QfM.js";import{p as v}from"../chunks/proxy.Ce5Ub1RB.js";import{r as ue}from"../chunks/legacy-client.B3KsQmEJ.js";import{I as A,c as de,a as F}from"../chunks/Input.DikYy5qt.js";import{r as ge,n as pe,t as fe,u as $e,v as Ne}from"../chunks/dataFetching.BTF7Y2Cd.js";import{B as he}from"../chunks/Button.Ch35LVs2.js";import{L as ye}from"../chunks/LangSelector.BJ1Lurmb.js";import{f as ke}from"../chunks/pow.dgjdRmCA.js";import{M as we,C as be}from"../chunks/ContentCenter.DAqX6TIP.js";import{u as Ue}from"../chunks/i18n.svelte.CZcZQkVy.js";import{T as xe}from"../chunks/Template.DXDiqOj9.js";import{u as Ce}from"../chunks/param.svelte.BjpJyl57.js";import{T as Ee}from"../chunks/ThemeSwitch.Nnphyg3I.js";var _e=b(" <br> <code> </code>",1),Te=b('<div class="success svelte-1ks9oi1"> <br> </div>'),qe=b('<div class="err svelte-1ks9oi1"> </div>'),Ie=b('<div class="container svelte-1ks9oi1"><div class="domainTxt svelte-1ks9oi1"><h1> </h1> <!></div> <!> <!> <!> <!> <!></div> <!> <!>',1),Le=b("<!> <!>",1);function Re(X,Y){oe(Y,!0);let a=Ue(),f=h(""),J=Ce("redirect_uri"),U=h(!1),g=h(""),T=h(!1),o=v({email:"",givenName:"",familyName:""}),u=h(v({})),P=h(v({}));ue(()=>{a&&s(P,v(de().shape({email:F().required(a.common.required).email(a.register.emailBadFormat),givenName:F().required(a.common.required).matches($e,a.register.regexName),familyName:F().matches(Ne,a.register.regexName)})))});function q(d){d.detail.code==="Enter"&&Q()}async function Q(){s(T,!1),s(g,"");try{await t(P).validate(o,{abortEarly:!1}),s(u,v({}))}catch(p){s(u,v(pe(p)));return}if(t(f)&&!o.email.endsWith(t(f))){s(g,v(a.register.domainErr));return}s(U,!0),await ne();let d=await ke();const I={email:o.email,given_name:o.givenName,family_name:o.familyName.length>0?o.familyName:void 0,pow:d};let x=J.get();x&&(I.redirect_uri=x);const L=await fe(I);if(L.ok)s(g,""),s(T,!0),setTimeout(()=>{window.location.replace(J.get()||"/auth/v1/account")},3e3);else{const p=await L.json();p.message.includes("UNIQUE constraint")?s(g,"E-Mail is already registered"):s(g,v(p.message))}s(U,!1)}var W=Le();ce(d=>{c(()=>le.title=(a==null?void 0:a.register)||"Register")});var D=C(W);xe(D,{id:ge,get value(){return t(f)},set value(d){s(f,v(d))}});var Z=m(D,2);we(Z,{children:(d,I)=>{be(d,{children:(x,L)=>{var p=Ie(),R=C(p),j=y(R),B=y(j),z=y(B,!0);k(B);var H=m(B,2);{var ee=e=>{var i=_e(),r=C(i,!0),$=m(r,2),N=m($),w=y(N);k(N),c(()=>{l(r,a.register.domainRestricted),l($,` ${a.register.domainAllowed??""} `),l(w,`@${t(f)??""}`)}),n(e,i)};M(H,e=>{t(f)&&e(ee)})}k(j);var G=m(j,2);A(G,{type:"email",autocomplete:"email",get placeholder(){return a.common.email},get value(){return o.email},set value(e){o.email=e},get error(){return t(u).email},set error(e){t(u).email=e},$$events:{keypress:q},children:(e,i)=>{E();var r=_();c(()=>l(r,a.common.email.toUpperCase())),n(e,r)},$$slots:{default:!0}});var K=m(G,2);A(K,{autocomplete:"given-name",get placeholder(){return a.account.givenName},get value(){return o.givenName},set value(e){o.givenName=e},get error(){return t(u).givenName},set error(e){t(u).givenName=e},$$events:{keypress:q},children:(e,i)=>{E();var r=_();c(()=>l(r,a.account.givenName.toUpperCase())),n(e,r)},$$slots:{default:!0}});var O=m(K,2);A(O,{autocomplete:"family-name",get placeholder(){return a.account.familyName},get value(){return o.familyName},set value(e){o.familyName=e},get error(){return t(u).familyName},set error(e){t(u).familyName=e},$$events:{keypress:q},children:(e,i)=>{E();var r=_();c(()=>l(r,a.account.familyName.toUpperCase())),n(e,r)},$$slots:{default:!0}});var S=m(O,2);he(S,{get isLoading(){return t(U)},set isLoading(e){s(U,v(e))},$$events:{click:Q},children:(e,i)=>{E();var r=_();c(()=>l(r,a.register.register)),n(e,r)},$$slots:{default:!0}});var ae=m(S,2);{var re=e=>{var i=Te(),r=y(i,!0),$=m(r,2);k(i),c(()=>{l(r,a.register.success),l($,` ${a.register.emailCheck??""}`)}),n(e,i)},te=e=>{var i=ve(),r=C(i);{var $=N=>{var w=qe(),ie=y(w,!0);k(w),c(()=>l(ie,t(g))),n(N,w)};M(r,N=>{t(g)&&N($)},!0)}n(e,i)};M(ae,e=>{t(T)?e(re):e(te,!1)})}k(R);var V=m(R,2);Ee(V,{absolute:!0});var se=m(V,2);ye(se,{absolute:!0}),c(()=>l(z,a.register.userReg)),n(x,p)},$$slots:{default:!0}})},$$slots:{default:!0}}),n(X,W),me()}export{Re as component};
