import{a as i,t as c,e as z}from"../chunks/disclose-version.BP4T4xdV.js";import{a5 as ie,f as q,p as le,a as ne,k as s,s as o,c as r,j as e,l as v,r as a,a7 as L,t as de}from"../chunks/index-client.hxMyB_b4.js";import{s as A}from"../chunks/render.yNX9q8-B.js";import{i as _}from"../chunks/if.BeBa1Kxb.js";import{p as C}from"../chunks/proxy.DaVNR9vk.js";import{B as E}from"../chunks/Button.BR1gOfNn.js";import{I as G}from"../chunks/pow.BMnvYon0.js";import{C as K}from"../chunks/CheckIcon.7gB29NsI.js";import{M as ce}from"../chunks/Main.Rc9oYaw1.js";import{C as ve}from"../chunks/ContentCenter.lHeEkgTE.js";import{a as N}from"../chunks/patterns.BRCxk8by.js";import{F as ue}from"../chunks/Form.BnvI_UPN.js";import{g as fe}from"../chunks/helpers.CchIKxjM.js";var pe=c("<br>-> enable idp registration:<br> <code>chrome://flags/#fedcm-idp-registration</code>",1),ge=c("<p><!> <!> <!></p>"),me=c('<p><!></p> <!> <div class="row svelte-wuqbvo"><b>Logged In:</b> <div class="check svelte-wuqbvo"><!></div></div>',1),be=c('<div class="flex-col"><h1>FedCM Testing</h1> <div class="row svelte-wuqbvo"><div><b>FedCM supported:</b></div> <div class="check svelte-wuqbvo"><!></div></div> <!> <!></div>'),he=c('<div class="row svelte-wuqbvo"><b>Credential Type:</b> <span> </span></div> <div class="token svelte-wuqbvo"> </div>',1),we=c("<!> <!>",1);function Ce(O,Q){ne(Q,!0);let b=v(!1),$=v(!1),y=v(""),P=v(""),u=v("fedcm"),n=v("any");ie(async()=>{s(n,`${window.location.origin}/auth/v1/fed_cm/config`),s(u,`${window.location.origin}/auth/v1/fed_cm/client_config`),(await fetch("/auth/v1/fed_cm/status")).status===200?(console.log("FedCM status is: logged-in"),s($,!0)):console.log("FedCM status is: logged-out"),window.IdentityProvider&&IdentityProvider.register!=null?(console.log("FedCM is supported"),s(b,!0)):console.error("FedCM is not supported")});async function V(){console.log("using credentials get values: configUrl: "+e(n)+" / clientId: "+e(u));try{let t=await navigator.credentials.get({identity:{mode:"button",providers:[{configURL:e(n),clientId:e(u),nonce:fe(48)}]}});console.log(t),s(P,C(t.type)),s(y,C(t.token)),s($,!0)}catch(t){console.error("FedCM credentials error: "+t)}}function X(){IdentityProvider.register(e(n))}var R=we(),U=q(R);ce(U,{children:(t,I)=>{ve(t,{children:(f,k)=>{var p=be(),d=o(r(p),2),h=o(r(d),2),J=r(h);K(J,{get check(){return e(b)}}),a(h),a(d);var T=o(d,2);{var W=l=>{var w=pe();L(4),i(l,w)};_(T,l=>{e(b)||l(W)})}var Y=o(T,2);{var ee=l=>{var w=me(),F=q(w),oe=r(F);E(oe,{onclick:X,level:2,children:(M,re)=>{L();var g=z("Register IdP");i(M,g)},$$slots:{default:!0}}),a(F);var D=o(F,2);ue(D,{action:"",onSubmit:V,children:(M,re)=>{var g=ge(),B=r(g);G(B,{autocomplete:"off",label:"Client ID",placeholder:"Client ID",required:!0,pattern:N,get value(){return e(u)},set value(m){s(u,C(m))}});var S=o(B,2);G(S,{autocomplete:"off",label:"Config URL",placeholder:"Config URL",pattern:N,get value(){return e(n)},set value(m){s(n,C(m))}});var ae=o(S,2);E(ae,{type:"submit",children:(m,$e)=>{L();var se=z("Login");i(m,se)},$$slots:{default:!0}}),a(g),i(M,g)},$$slots:{default:!0}});var j=o(D,2),x=o(r(j),2),te=r(x);K(te,{get check(){return e($)}}),a(x),a(j),i(l,w)};_(Y,l=>{e(b)&&l(ee)})}a(p),i(f,p)},$$slots:{default:!0}})},$$slots:{default:!0}});var Z=o(U,2);{var H=t=>{var I=he(),f=q(I),k=o(r(f),2),p=r(k,!0);a(k),a(f);var d=o(f,2),h=r(d,!0);a(d),de(()=>{A(p,e(P)),A(h,e(y))}),i(t,I)};_(Z,t=>{e(y)&&t(H)})}i(O,R),le()}export{Ce as component};
