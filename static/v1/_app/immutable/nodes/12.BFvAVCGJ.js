import"../chunks/disclose-version.BDr9Qe-U.js";import"../chunks/legacy.Dmu0D13_.js";import{p as U,A as v,ap as W,f as ee,t as f,a as ae,z as t,s,y as p,c as o,r as a,ar as re}from"../chunks/index-client.CVQra9Cu.js";import{s as i}from"../chunks/render.CkKFmY3E.js";import{i as D}from"../chunks/if._4yhN2kP.js";import{e as se,i as oe}from"../chunks/each.D7w_Y1pv.js";import{a as u,t as E,d as te}from"../chunks/template.4ZWi6dE6.js";import{p as m}from"../chunks/proxy.Dc6q2r8X.js";import{B as ie}from"../chunks/Button.B6DffihK.js";import{I as pe}from"../chunks/IconCheck.jhQYM7vu.js";import{f as ce,b as ne}from"../chunks/fetch.BnpwKXAn.js";import{u as me}from"../chunks/i18n_admin.svelte.DBF4IRUg.js";import{O as ve}from"../chunks/Options.DNEfnOhW.js";var le=E('<li class="font-mono"> </li>'),ye=E('<div class="err"> </div>'),de=E('<h3> </h3> <p> </p> <p> </p> <p><span class="font-label"> </span> <span class="active font-mono svelte-lxghyr"> </span></p> <p> </p> <ul></ul> <p> </p> <p> </p> <!> <div class="btn svelte-lxghyr"><!></div> <!> <!>',1);function fe(g,F){U(F,!0);let c=me(),l=v(""),y=v(!0),I=v(""),d=v(""),K=v(m([])),b=v(!1);W(async()=>{var r;let e=await ce("/auth/v1/encryption/keys");e.body?(t(K,m(e.body.keys)),t(I,m(e.body.active)),t(d,m(e.body.active))):t(l,m(((r=e.error)==null?void 0:r.message)||"Error")),t(y,!1)});async function G(){t(y,!0);let e={key_id:p(d)},r=await ne("/auth/v1/encryption/migrate",e);r.error?t(l,m(r.error.message)):(t(b,!0),setTimeout(()=>{t(b,!1)},4e3)),t(y,!1)}var O=de(),h=ee(O),H=o(h,!0);a(h);var k=s(h,2),J=o(k,!0);a(k);var $=s(k,2),N=o($,!0);a($);var w=s($,2),x=o(w),P=o(x);a(x);var j=s(x,2),Q=o(j,!0);a(j),a(w);var A=s(w,2),R=o(A);a(A);var T=s(A,2);se(T,21,()=>p(K),oe,(e,r)=>{var n=le(),M=o(n,!0);a(n),f(()=>i(M,p(r))),u(e,n)}),a(T);var L=s(T,2),S=o(L,!0);a(L);var z=s(L,2),V=o(z);a(z);var q=s(z,2);ve(q,{get ariaLabel(){return c.docs.encKeys.migrateToKey},get options(){return p(K)},get value(){return p(d)},set value(e){t(d,m(e))}});var B=s(q,2),X=o(B);ie(X,{onclick:G,get isLoading(){return p(y)},children:(e,r)=>{re();var n=te();f(()=>i(n,c.docs.encKeys.migrate)),u(e,n)},$$slots:{default:!0}}),a(B);var C=s(B,2);{var Y=e=>{var r=ye(),n=o(r,!0);a(r),f(()=>i(n,p(l))),u(e,r)};D(C,e=>{p(l)&&e(Y)})}var Z=s(C,2);{var _=e=>{pe(e,{})};D(Z,e=>{p(b)&&e(_)})}f(()=>{i(H,c.docs.encKeys.header),i(J,c.docs.encKeys.p1),i(N,c.docs.encKeys.p2),i(P,`${c.docs.encKeys.keyActive??""}:`),i(Q,p(I)),i(R,`${c.docs.encKeys.keysAvailable??""}:`),i(S,c.docs.encKeys.p3),i(V,`${c.docs.encKeys.migrateToKey??""}:`)}),u(g,O),ae()}function ue(g){fe(g,{})}export{ue as component};
