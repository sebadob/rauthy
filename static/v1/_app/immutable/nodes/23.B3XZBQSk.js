import{a as n,b as h,t as y}from"../chunks/disclose-version.C0dpEWi_.js";import{p as le,Z as p,o as ce,_ as R,a as de,j as c,a0 as r,a2 as a,g as t,a4 as s,a3 as b,a1 as ve}from"../chunks/index-client.LkyNpeLM.js";import{s as Z}from"../chunks/render.D21ACvZ4.js";import{i as _}from"../chunks/if.BQqAvi9S.js";import{p as m}from"../chunks/proxy.bVSoDcUp.js";import{g as ue}from"../chunks/helpers.DVmWh-6p.js";import{B as z}from"../chunks/Button.BDI_bIrm.js";import{c as fe,a as J,I as K}from"../chunks/Input.BWHohn37.js";import{l as Q,m as ge,n as pe}from"../chunks/dataFetching.C4rEqQY1.js";import{C as S}from"../chunks/CheckIcon.D95u0ggr.js";import{M as me,C as $e}from"../chunks/ContentCenter.BsVUI9il.js";var he=h("<br>-> enable idp registration:<br> <code>chrome://flags/#fedcm-idp-registration</code>",1),be=h('<p><!></p> <p><!> <!> <!></p> <div class="row svelte-wuqbvo"><b>Logged In:</b> <div class="check svelte-wuqbvo"><!></div></div>',1),Ie=h('<div class="flex-col"><h1>FedCM Testing</h1> <div class="row svelte-wuqbvo"><div><b>FedCM supported:</b></div> <div class="check svelte-wuqbvo"><!></div></div> <!> <!></div>'),we=h('<div class="row svelte-wuqbvo"><b>Credential Type:</b> <span> </span></div> <div class="token svelte-wuqbvo"> </div>',1),Ce=h("<!> <!>",1);function ye(V,W){le(W,!0);let E=p(""),I=p(!1),U=p(!1),M=p(""),T=p(""),l=m({clientId:"fedcm",configUrl:"any"}),u=p(m({}));const X=fe().shape({clientId:J().required("Client ID is required").trim().matches(Q,"Must be URL safe"),configUrl:J().nullable().trim().matches(Q,"Must be URL safe")});ce(async()=>{if(c(E,`${window.location.origin}/auth/v1/fed_cm/config`),l.clientId=`${window.location.origin}/auth/v1/fed_cm/client_config`,(await ge()).ok?(console.log("FedCM status is: logged-in"),c(U,!0)):console.log("FedCM status is: logged-out"),window.IdentityProvider&&IdentityProvider.register!=null)console.log("FedCM is supported"),c(I,!0);else{console.error("FedCM is not supported");return}});async function Y(){if(!await k())return;let i=l.configUrl||void 0,f=l.clientId;console.log("using credentials get values: configUrl: "+i+" / clientId: "+f);try{let o=await navigator.credentials.get({identity:{mode:"button",providers:[{configURL:i,clientId:f,nonce:ue(48)}]}});console.log(o),c(T,m(o.type)),c(M,m(o.token)),c(U,!0)}catch(o){console.error("FedCM credentials error: "+o)}}function A(){IdentityProvider.register(t(E))}async function k(){try{return await X.validate(l,{abortEarly:!1}),c(u,m({})),!0}catch(i){return c(u,m(pe(i))),!1}}var x=Ce(),D=R(x);me(D,{children:(i,f)=>{$e(i,{children:(o,q)=>{var $=Ie(),g=r(a($),2),w=r(a(g),2),re=a(w);S(re,{get check(){return t(I)}}),s(w),s(g);var N=r(g,2);{var te=d=>{var C=he();b(4),n(d,C)};_(N,d=>{t(I)||d(te)})}var oe=r(N,2);{var ie=d=>{var C=be(),F=R(C),ae=a(F);z(ae,{level:3,$$events:{click:A},children:(e,P)=>{b();var v=y("Register IdP");n(e,v)},$$slots:{default:!0}}),s(F);var L=r(F,2),j=a(L);K(j,{autocomplete:"off",placeholder:"Client Id",get value(){return l.clientId},set value(e){l.clientId=e},get error(){return t(u).clientId},set error(e){t(u).clientId=e},$$events:{input:k},children:(e,P)=>{b();var v=y("CLIENT ID");n(e,v)},$$slots:{default:!0}});var B=r(j,2);K(B,{autocomplete:"off",placeholder:"Config URL",get value(){return l.configUrl},set value(e){l.configUrl=e},get error(){return t(u).configUrl},set error(e){t(u).configUrl=e},$$events:{input:k},children:(e,P)=>{b();var v=y("CONFIG URL");n(e,v)},$$slots:{default:!0}});var se=r(B,2);z(se,{$$events:{click:Y},children:(e,P)=>{b();var v=y("Login");n(e,v)},$$slots:{default:!0}}),s(L);var G=r(L,2),O=r(a(G),2),ne=a(O);S(ne,{get check(){return t(U)}}),s(O),s(G),n(d,C)};_(oe,d=>{t(I)&&d(ie)})}s($),n(o,$)},$$slots:{default:!0}})},$$slots:{default:!0}});var H=r(D,2);{var ee=i=>{var f=we(),o=R(f),q=r(a(o),2),$=a(q,!0);s(q),s(o);var g=r(o,2),w=a(g,!0);s(g),ve(()=>{Z($,t(T)),Z(w,t(M))}),n(i,f)};_(H,i=>{t(M)&&i(ee)})}n(V,x),de()}export{ye as component};
