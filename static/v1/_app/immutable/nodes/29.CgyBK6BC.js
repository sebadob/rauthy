import"../chunks/disclose-version.BDr9Qe-U.js";import{t as $,p as Sr,A as n,ap as Tr,a6 as se,f as V,a as qr,y as e,z as t,aq as Fr,s as c,c as v,r as d,ar as fe,ab as he}from"../chunks/index-client.CVQra9Cu.js";import{s as S}from"../chunks/render.CkKFmY3E.js";import{i as g}from"../chunks/if._4yhN2kP.js";import{e as Wr}from"../chunks/each.D7w_Y1pv.js";import{a as l,n as Cr,t as p,d as ge,e as rr}from"../chunks/template.4ZWi6dE6.js";import{h as Gr}from"../chunks/svelte-head.Cv719s3a.js";import{s as j}from"../chunks/class.BnbJV1WX.js";import{p as i}from"../chunks/proxy.Dc6q2r8X.js";import{p as Hr}from"../chunks/dataFetching.DHlo1LT-.js";import{f as tr,D as Rr,F as Ur,G as Ar,u as Br,H as Vr,k as jr,l as Dr,_ as Er,P as Nr,I as Jr}from"../chunks/helpers.BweJi6s8.js";import{B as we}from"../chunks/Button.B6DffihK.js";import{W as Zr}from"../chunks/WebauthnRequest.D7AXCzJU.js";import{I as Or}from"../chunks/Input.E2qlCEoO.js";import{T as Pr,L as Qr}from"../chunks/LangSelector.WQX7yGNj.js";import{p as Ge}from"../chunks/props.BVYR8gU-.js";import{M as Xr}from"../chunks/Main.Caps8oUu.js";import{C as Yr}from"../chunks/ContentCenter.DfrWDKi1.js";import{u as Kr}from"../chunks/i18n.svelte.DVZudxvm.js";import{T as Z}from"../chunks/Template.Rw5R688S.js";import{u as G}from"../chunks/param.svelte.CAlaz0Tl.js";import{I as et}from"../chunks/InputPassword.BoC-LeDj.js";import{b as _e}from"../chunks/fetch.BnpwKXAn.js";import{u as ar}from"../chunks/is_dev.svelte.DPSayiST.js";import{F as rt}from"../chunks/Form.BiuDxuNH.js";import{B as tt}from"../chunks/ButtonAuthProvider.avA5-tta.js";var at=Cr(`<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125
            1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504
            1.125-1.125V9.75M8.25 21h8.25"></path></svg>`);function ot(be,O){let ie=Ge(O,"opacity",3,.9),u=Ge(O,"width",3,"1.5rem"),P=Ge(O,"color",3,"currentColor");var H=at();j(H,"stroke-width",2),$(()=>{j(H,"stroke",P()),j(H,"width",u()),j(H,"opacity",ie())}),l(be,H)}var st=p('<img alt="No Logo Available">'),it=p('<a class="home svelte-1rplcwr"><!></a>'),lt=p('<div class="forgotten svelte-1rplcwr"><!></div>'),nt=p("<!> <!>",1),ct=p('<div class="btn flex-col svelte-1rplcwr"><!></div>'),vt=p('<div class="btn flex-col svelte-1rplcwr"><!></div>'),dt=p("<!> <!> <!>",1),ut=p('<a class="reg svelte-1rplcwr" target="_blank"> </a>'),pt=p('<a class="reg svelte-1rplcwr" href="/auth/v1/users/register" target="_blank"> </a>'),mt=p("<!> <!>",1),ft=p('<div class="errMsg svelte-1rplcwr"> </div>'),ht=p('<div class="success svelte-1rplcwr"> </div>'),gt=p('<div class="btn flex-col svelte-1rplcwr"><!></div>'),wt=p('<div class="providers flex-col svelte-1rplcwr"><div class="providersSeparator svelte-1rplcwr"><div class="separator svelte-1rplcwr"></div> <div class="loginWith svelte-1rplcwr"><div class="svelte-1rplcwr"> </div></div></div> <!></div>'),_t=p('<div class="container svelte-1rplcwr"><div class="head svelte-1rplcwr"><div class="logo svelte-1rplcwr"><!></div> <!></div> <div class="name svelte-1rplcwr"><h2> </h2></div> <!> <!> <!> <!> <!> <!></div> <!> <!>',1),bt=p("<!> <!> <!> <!> <!> <!> <!>",1);function $t(be,O){var Je;Sr(O,!0);const ie="18rem";let u=Kr(),P=ar().get(),H=he(()=>P?"/auth/v1/dev/authorize":"/auth/v1/oidc/authorize"),_=G("client_id").get(),le=n(""),T=n(i(P?"/auth/v1":"")),Q=G("redirect_uri").get(),$e=G("nonce").get(),ye=((Je=G("scope").get())==null?void 0:Je.split(" "))||[],ze=n(void 0),xe=n(void 0),ke=G("state").get(),X=G("code_challenge").get(),I=G("code_challenge_method").get(),He=!1,Ie=n(void 0),ne=n(i([])),ce=n(void 0),D=n(!1),m=n(""),Y=n(""),ve=n(""),q=n(!1),R=n(!1),de=n(!1),ue=n(!1),pe=n(!1),U=n(!1),Re=n(""),Le=n(!1),y=n(i(G("login_hint").get()||"")),F=n(""),Me=n("");Tr(()=>{var r;e(q)||((r=e(ze))==null||r.focus())}),se(()=>{He&&(_!=null&&_.length)&&ir()}),se(()=>{e(pe)&&setTimeout(()=>{t(pe,!1),t(de,!1),t(ue,!1)},3e3)}),se(()=>{var r;(r=e(xe))==null||r.focus()}),se(()=>{var r;if(e(Y)==="Refresh")He=!0;else if((r=e(Y))!=null&&r.startsWith("MfaLogin ")){let o=e(Y).replace("MfaLogin ","");t(y,i(o)),t(Ie,i(o))}}),se(()=>{P?or():e(ve)&&tr(e(ve))});async function or(){var o;let r=await _e("/auth/v1/oidc/session");console.log("manual session",r.body),(o=r.body)!=null&&o.csrf_token?tr(r.body.csrf_token):console.error(r.error)}function sr(){t(m,""),t(de,!0),t(F,"")}async function ir(){if(!_){console.error("clientId is undefined");return}if(!Q){console.error("redirectUri is undefined");return}t(D,!0);const r={client_id:_,redirect_uri:Q,state:ke,nonce:$e,scopes:ye};X&&I&&(I==="plain"||I==="S256")&&(r.code_challenge=X,r.code_challenge_method=I);let o=await _e("/auth/v1/oidc/authorize/refresh",r);await Ue(o)}async function lr(r,o){if(t(m,""),!_){console.error("clientId is undefined");return}if(!Q){console.error("redirectUri is undefined");return}const w={email:e(y),client_id:_,redirect_uri:Q,state:ke,nonce:$e,scopes:ye};if(X&&I&&(I==="plain"||I==="S256")&&(w.code_challenge=X,w.code_challenge_method=I),e(q)&&e(y)!==e(Ie)){if(!e(F)){t(m,i(u.authorize.passwordRequired));return}if(e(F).length>256){t(m,"max 256");return}w.password=e(F)}t(D,!0);let b="/auth/v1/oidc/authorize";ar().get()&&(b="/auth/v1/dev/authorize");let x=await _e(b,w);await Ue(x)}async function Ue(r){if(r.status===202){let o=r.headers.get("location");if(!o){console.error("location header missing");return}window.location.replace(o)}else if(r.status===200){t(m,"");let o=r.body;o&&"user_id"in o&&"code"in o?(t(Me,i(o.user_id)),t(ce,i({Login:o.code}))):console.error("did not receive a proper WebauthnLoginResponse after HTTP200")}else if(r.status===406)t(m,i(u.authorize.clientForceMfa)),t(R,!0);else if(r.status===429){let o=r.headers.get("x-retry-not-before");if(!o){console.error("x-retry-not-before header missing");return}let w=Number.parseInt(o),b=Dr(w),x=w*1e3-new Date().getTime();t(U,!0),t(m,`${u.authorize.http429} ${b}`),t(y,""),t(F,""),t(q,!1),setTimeout(()=>{t(U,!1),t(m,"")},x)}else e(q)?(t(m,i(u.authorize.invalidCredentials)),t(ue,!0)):(t(q,!0),t(Re,i(e(y))));t(D,!1)}function nr(){e(q)&&e(Re)!==e(y)&&(t(q,!1),t(F,""),t(m,""))}function cr(r){Er(64,(o,{challenge:w,verifier:b})=>{o||(localStorage.setItem(Nr,b),vr(r,w))})}async function vr(r,o){let w={email:e(y)||null,client_id:_,redirect_uri:Q,scopes:ye,state:ke,nonce:$e,code_challenge:X,code_challenge_method:I,provider_id:r,pkce_challenge:o},b=await Hr(w);if(b.ok){const x=await b.text();Jr(x);let E=b.headers.get("location");if(!E){console.error("no location header set for provider login");return}window.location.href=E}else{let x=await b.json();t(m,i(x.message))}}function dr(r){t(ce,void 0),t(m,i(r))}function ur(r){console.log("onWebauthnSuccess",r),r&&"loc"in r&&window.location.replace(r.loc)}async function pr(){let r={email:e(y)};e(T)&&(r.redirect_uri=encodeURI(e(T))),t(D,!0);let o=await _e("/auth/v1/users/request_reset",r);o.error?t(m,i(o.error.message)):t(pe,!0),t(D,!1)}var Ae=bt();Gr(r=>{$(()=>Fr.title=`Login ${(e(le)||_)??""}`)});var Be=V(Ae);Z(Be,{id:Rr,get value(){return e(ne)},set value(r){t(ne,i(r))}});var Ve=c(Be,2);Z(Ve,{id:Ur,get value(){return e(le)},set value(r){t(le,i(r))}});var je=c(Ve,2);Z(je,{id:Ar,get value(){return e(T)},set value(r){t(T,i(r))}});var De=c(je,2);Z(De,{id:Br,get value(){return e(ve)},set value(r){t(ve,i(r))}});var Ee=c(De,2);Z(Ee,{id:Vr,get value(){return e(Y)},set value(r){t(Y,i(r))}});var Ne=c(Ee,2);Z(Ne,{id:jr,get value(){return e(Le)},set value(r){t(Le,i(r))}});var mr=c(Ne,2);Xr(mr,{children:(r,o)=>{Yr(r,{children:(w,b)=>{var x=_t(),E=V(x),Se=v(E),Te=v(Se),fr=v(Te);{var hr=a=>{var s=st();j(s,"src",`/auth/v1/clients/${_}/logo`),l(a,s)};g(fr,a=>{_&&a(hr)})}d(Te);var gr=c(Te,2);{var wr=a=>{var s=it(),h=v(s);ot(h,{color:"hsla(var(--text) / .4)"}),d(s),$(()=>j(s,"href",e(T))),l(a,s)};g(gr,a=>{e(T)&&a(wr)})}d(Se);var qe=c(Se,2),Ze=v(qe),_r=v(Ze,!0);d(Ze),d(qe);var Oe=c(qe,2);{var br=a=>{Zr(a,{get userId(){return e(Me)},get purpose(){return e(ce)},onSuccess:ur,onError:dr})};g(Oe,a=>{e(ce)&&e(Me)&&a(br)})}var Pe=c(Oe,2);{var $r=a=>{var s=mt(),h=V(s);rt(h,{get action(){return e(H)},onSubmit:lr,children:(L,ee)=>{var W=dt(),A=V(W),re=he(()=>e(U)||e(R));Or(A,{typ:"email",name:"email",autocomplete:"email",get label(){return u.common.email},get placeholder(){return u.common.email},get disabled(){return e(re)},onInput:nr,width:ie,required:!0,get ref(){return e(ze)},set ref(z){t(ze,i(z))},get value(){return e(y)},set value(z){t(y,i(z))}});var M=c(A,2);{var k=z=>{var ae=nt(),me=V(ae),Fe=he(()=>e(U)||e(R));et(me,{name:"password",autocomplete:"current-password",get label(){return u.common.password},get placeholder(){return u.common.password},maxLength:256,get disabled(){return e(Fe)},width:ie,required:!0,get ref(){return e(xe)},set ref(f){t(xe,i(f))},get value(){return e(F)},set value(f){t(F,i(f))}});var We=c(me,2);{var C=f=>{var B=lt(),oe=v(B);we(oe,{invisible:!0,onclick:sr,children:(Ce,J)=>{fe();var er=ge();$(()=>S(er,u.authorize.passwordForgotten)),l(Ce,er)},$$slots:{default:!0}}),d(B),l(f,B)};g(We,f=>{e(ue)&&!e(U)&&f(C)})}l(z,ae)};g(M,z=>{e(q)&&e(Ie)!==e(y)&&!e(de)&&z(k)})}var te=c(M,2);{var Mr=z=>{var ae=rr(),me=V(ae);{var Fe=C=>{var f=ct(),B=v(f);we(B,{onclick:pr,children:(oe,Ce)=>{fe();var J=ge();$(()=>S(J,u.authorize.passwordRequest)),l(oe,J)},$$slots:{default:!0}}),d(f),l(C,f)},We=C=>{var f=vt(),B=v(f);we(B,{type:"submit",get isLoading(){return e(D)},children:(oe,Ce)=>{fe();var J=ge();$(()=>S(J,u.authorize.login)),l(oe,J)},$$slots:{default:!0}}),d(f),l(C,f)};g(me,C=>{e(de)?C(Fe):C(We,!1)})}l(z,ae)};g(te,z=>{!e(U)&&!e(R)&&z(Mr)})}l(L,W)},$$slots:{default:!0}});var N=c(h,2);{var K=L=>{var ee=rr(),W=V(ee);{var A=M=>{var k=ut(),te=v(k,!0);d(k),$(()=>{j(k,"href",`/auth/v1/users/register?redirect_uri=${e(T)??""}`),S(te,u.authorize.signUp)}),l(M,k)},re=M=>{var k=pt(),te=v(k,!0);d(k),$(()=>S(te,u.authorize.signUp)),l(M,k)};g(W,M=>{e(T)?M(A):M(re,!1)})}l(L,ee)};g(N,L=>{e(Le)&&!e(ue)&&!e(U)&&L(K)})}l(a,s)};g(Pe,a=>{e(R)||a($r)})}var Qe=c(Pe,2);{var yr=a=>{var s=ft(),h=v(s,!0);d(s),$(()=>S(h,e(m))),l(a,s)};g(Qe,a=>{e(m)&&a(yr)})}var Xe=c(Qe,2);{var zr=a=>{var s=ht(),h=v(s,!0);d(s),$(()=>S(h,u.authorize.emailSentMsg)),l(a,s)};g(Xe,a=>{e(pe)&&a(zr)})}var Ye=c(Xe,2);{var xr=a=>{var s=gt(),h=v(s);we(h,{onclick:()=>window.location.href="/auth/v1/account",children:(N,K)=>{fe();var L=ge("Account");l(N,L)},$$slots:{default:!0}}),d(s),l(a,s)};g(Ye,a=>{e(R)&&a(xr)})}var kr=c(Ye,2);{var Ir=a=>{var s=wt(),h=v(s),N=c(v(h),2),K=v(N),L=v(K,!0);d(K),d(N),d(h);var ee=c(h,2);Wr(ee,17,()=>e(ne),W=>W.id,(W,A)=>{var re=he(()=>`Login: ${e(A).name}`);tt(W,{get ariaLabel(){return e(re)},get provider(){return e(A)},onclick:cr})}),d(s),$(()=>S(L,u.authorize.orLoginWith)),l(a,s)};g(kr,a=>{!e(R)&&e(ne)&&a(Ir)})}d(E);var Ke=c(E,2);Pr(Ke,{absolute:!0});var Lr=c(Ke,2);Qr(Lr,{absolute:!0}),$(()=>S(_r,e(le)||_)),l(w,x)},$$slots:{default:!0}})},$$slots:{default:!0}}),l(be,Ae),qr()}export{$t as component};
