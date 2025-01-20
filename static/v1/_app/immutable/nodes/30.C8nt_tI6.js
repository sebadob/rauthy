import{a as p,b as U,c as ie,t as k}from"../chunks/disclose-version.DZhRUPAc.js";import{p as es,_ as g,o as ss,a0 as V,a as rs,j as s,a2 as m,$ as ge,g as e,a1 as d,a3 as _,a5 as R,a4 as C,h as as}from"../chunks/index-client.CGppiJvc.js";import{h as ts,s as w}from"../chunks/render.D4MwAWTL.js";import{i as y}from"../chunks/if.DUA6Hpfn.js";import{L as os,t as je,b as qe,s as Ae}from"../chunks/Loading.FF_EHyHU.js";import{y as is,v as ds,A as ns,m as Se,B as ps,D as ls,E as cs,F as he,G as $e,H as us}from"../chunks/dataFetching.Dto1xJW6.js";import{p as l}from"../chunks/proxy.DB4Km9hY.js";import{r as ue}from"../chunks/legacy-client.CgqodnwN.js";import{c as Ue,a as ye,I as ws}from"../chunks/Input.Bj_i2TLU.js";import{B}from"../chunks/Button.BH2_dPry.js";import{P as Oe}from"../chunks/PasswordPolicy.D9rZurKD.js";import{P as we}from"../chunks/PasswordInput.6uXzPAl-.js";import{W as Fe}from"../chunks/WebauthnRequest.CHVZELhw.js";import{L as vs}from"../chunks/LangSelector.mSh8cV6Q.js";import"../chunks/is_dev.svelte.sP_MjoCE.js";import{u as ms}from"../chunks/i18n.svelte.BTGsPiz_.js";var fs=U('<div class="success svelte-189rvns"> <br> </div>'),gs=U("<div><!> <!> <!> <!> <!> <!></div>"),hs=U('<div class="success svelte-189rvns"><p> </p> <p> </p> <!></div>'),$s=U("<div><!> <!> <!></div>"),ys=U('<!> <h1> </h1> <p> </p> <p> <a target="_blank" class="svelte-189rvns">FIDO Alliance</a></p> <div><!> <!></div> <!>',1),bs=U('<div class="success svelte-189rvns"> <br> <br> <a class="svelte-189rvns">Link</a></div>'),ks=U("<!> <h1>Password Reset</h1> <!> <!> <!> <!> <!> <!>",1),_s=U('<div class="err svelte-189rvns"> </div>'),Rs=U('<!> <div class="container svelte-189rvns"><!> <!></div> <!>',1);function Cs(Ve,Be){es(Be,!0);const O="150px",Q="320px";let r=ms(),de="",W=g(void 0),be=g(!0),ke=!1,N=g(!1),I=g(""),X="",Y=g(""),Z=g(""),ne="",ee=g(void 0),f=g(!1),se=g(!1),re=g(!1),j=g(void 0),n=g(l({passkeyName:"",password:"",passwordConfirm:""})),x=g(l({})),_e=g(void 0),Re=g(void 0);ss(async()=>{let a;a="10, 128, 1, 1, 1, 1, 3";const t=[];a.split(",").forEach(i=>t.push(i)),s(W,l({length_min:Number.parseInt(t[0]),length_max:Number.parseInt(t[1]),include_lower_case:Number.parseInt(t[2]),include_upper_case:Number.parseInt(t[3]),include_digits:Number.parseInt(t[4]),include_special:Number.parseInt(t[5]),not_recently_used:Number.parseInt(t[6])})),ke=t[7]=="true",de=window.document.getElementsByName("rauthy-csrf-token")[0].id,X=window.location.href.split("/users/")[1].split("/")[0],ne=window.location.href.split("/reset/")[1].split("?")[0],s(Y,"password_reset"),s(be,!0)});function Ce(){window.location.replace("/auth/v1/account")}function Ne(){const a=e(W).length_min>24?e(W).length_min:24;let t=ns(a,e(W).include_lower_case,e(W).include_upper_case,e(W).include_digits,e(W).include_special);e(n).password=t,e(n).passwordConfirm=t}async function Ie(){s(I,"");try{await e(_e).validate(e(n),{abortEarly:!1}),s(x,l({}))}catch(i){s(x,l(Se(i)));return}const a=e(n).passkeyName;if(a.length<2){s(I,l(r.mfa.passkeyNameErr));return}let t=await cs(X,{passkey_name:a,magic_link_id:ne},de);if(t.status===200){let i=await t.json();i.publicKey.authenticatorSelection.userVerification="required",i.publicKey.challenge=he(i.publicKey.challenge),i.publicKey.user.id=he(i.publicKey.user.id),i.publicKey.excludeCredentials=i.publicKey.excludeCredentials,i.publicKey.excludeCredentials&&(i.publicKey.excludeCredentials=i.publicKey.excludeCredentials.map(h=>(h.id=he(h.id),h)));let b=await navigator.credentials.create(i),D={passkey_name:a,data:{id:b.id,rawId:$e(b.rawId),response:{attestationObject:$e(b.response.attestationObject),clientDataJSON:$e(b.response.clientDataJSON)},type:b.type},magic_link_id:ne};t=await us(X,D,de),t.status===201?(s(n,l({passkeyName:"",password:"",passwordConfirm:""})),s(f,!0)):(pe(),console.error(t))}else{pe();let i=await t.json();console.error(i.error),console.error(i.message)}}async function Le(){try{await e(Re).validate(e(n),{abortEarly:!1}),s(x,l({}))}catch(a){s(x,l(Se(a)));return}if(e(se)){if(e(n).password.length>256){s(I,"max 256");return}if(e(n).password!==e(n).passwordConfirm){s(I,l(r.passwordReset.passwordNoMatch));return}else s(I,"");if(ke){let a=await ps(X,{purpose:"PasswordReset"}),t=await a.json();if(!a.ok){s(I,l(t.message)),s(N,!1);return}if(t.user_id!==X){s(I,"MFA user ID does not match - this should never happen"),s(N,!1);return}s(j,l(t))}else await De()}}async function De(a){s(N,!0);const t={password:e(n).password,magic_link_id:ne,mfa_code:a},i=await ls(X,t,de);if(i.ok)s(I,""),s(n,l({passkeyName:"",password:"",passwordConfirm:""})),s(ee,l(i.headers.get("Location"))),console.log("redirectUri: "+e(ee)),s(f,!0);else{const b=await i.json();s(I,l(b.message))}s(N,!1)}function pe(){s(j,void 0),s(I,l(r.mfa.errorReg))}function Ee(a){a&&(s(j,void 0),De(a.code))}ue(()=>{r&&(s(_e,l(Ue().shape({passkeyName:ye().required(r.common.required).matches(ds,r.mfa.passkeyNameErr)}))),s(Re,l(Ue().shape({password:ye().required(r.common.required),passwordConfirm:ye().required(r.common.required)}))))}),ue(()=>{e(Z)&&s(n,l({passkeyName:"",password:"",passwordConfirm:""}))}),ue(()=>{var a;((a=e(n).password)==null?void 0:a.length)>0&&e(n).password===e(n).passwordConfirm&&s(re,!0)}),ue(()=>{e(f)&&setTimeout(()=>{e(ee)?window.location.replace(e(ee)):Ce()},5e3)});var xe=Rs();ts(a=>{var t=ie(),i=V(t);{var b=h=>{var q=ie(),ae=V(q);{var F=K=>{m(()=>ge.title=r.passwordReset.newAccount)},G=K=>{var P=ie(),J=V(P);{var te=z=>{m(()=>ge.title=r.passwordReset.passwordReset)};y(J,z=>{e(Y)==="password_reset"&&z(te)},!0)}p(K,P)};y(ae,K=>{e(Y).startsWith("new_user")?K(F):K(G,!1)})}p(h,q)},D=h=>{ge.title="Password"};y(i,h=>{r?h(b):h(D,!1)})}p(a,t)});var Ke=V(xe);{var Ge=a=>{os(a,{})};y(Ke,a=>{e(be)||a(Ge)})}var ve=d(Ke,2),Pe=_(ve);{var Je=a=>{var t=ys(),i=V(t);{var b=o=>{Fe(o,{onSuccess:Ee,onError:pe,get data(){return e(j)},set data($){s(j,l($))}})};y(i,o=>{e(j)&&o(b)})}var D=d(i,2),h=_(D,!0);R(D);var q=d(D,2),ae=_(q,!0);R(q);var F=d(q,2),G=_(F,!0),K=d(G);R(F);var P=d(F,2);is(P,"margin-bottom","1rem");var J=_(P);B(J,{width:O,level:2,get isDisabled(){return e(f)},get isLoading(){return e(N)},set isLoading(o){s(N,l(o))},$$events:{click:()=>s(Z,"passkey")},children:(o,$)=>{C();var L=k();m(()=>w(L,r.passwordReset.passwordless)),p(o,L)},$$slots:{default:!0}});var te=d(J,2);B(te,{width:O,level:3,get isDisabled(){return e(f)},get isLoading(){return e(N)},set isLoading(o){s(N,l(o))},$$events:{click:()=>s(Z,"password")},children:(o,$)=>{C();var L=k();m(()=>w(L,r.passwordReset.password)),p(o,L)},$$slots:{default:!0}}),R(P);var z=d(P,2);{var u=o=>{var $=gs(),L=_($);Oe(L,{get policy(){return e(W)},get password(){return e(n).password},get accepted(){return e(se)},set accepted(v){s(se,l(v))}});var le=d(L,2);we(le,{get error(){return e(x).password},autocomplete:"new-password",get placeholder(){return r.passwordReset.password},width:Q,get showCopy(){return e(re)},get disabled(){return e(f)},get value(){return e(n).password},set value(v){e(n).password=v},children:(v,A)=>{C();var c=k();m(()=>w(c,r.passwordReset.password.toUpperCase())),p(v,c)},$$slots:{default:!0}});var H=d(le,2);we(H,{get error(){return e(x).passwordConfirm},autocomplete:"new-password",get placeholder(){return r.passwordReset.passwordConfirm},width:Q,get showCopy(){return e(re)},get disabled(){return e(f)},get value(){return e(n).passwordConfirm},set value(v){e(n).passwordConfirm=v},children:(v,A)=>{C();var c=k();m(()=>w(c,r.passwordReset.passwordConfirm.toUpperCase())),p(v,c)},$$slots:{default:!0}});var M=d(H,2);B(M,{width:O,level:3,get isDisabled(){return e(f)},$$events:{click:Ne},children:(v,A)=>{C();var c=k();m(()=>w(c,r.passwordReset.generate)),p(v,c)},$$slots:{default:!0}});var oe=d(M,2);B(oe,{width:O,level:2,get isDisabled(){return e(f)},get isLoading(){return e(N)},set isLoading(v){s(N,l(v))},$$events:{click:Le},children:(v,A)=>{C();var c=k();m(()=>w(c,r.common.save)),p(v,c)},$$slots:{default:!0}});var ce=d(oe,2);{var me=v=>{var A=fs(),c=_(A),T=d(c,2);R(A),m(()=>{w(c,`${r.passwordReset.success1??""} `),w(T,` ${r.passwordReset.success2??""}`)}),p(v,A)};y(ce,v=>{e(f)&&v(me)})}R($),je(3,$,()=>qe),p(o,$)},E=o=>{var $=ie(),L=V($);{var le=H=>{var M=$s(),oe=_(M);ws(oe,{autocomplete:"off",get placeholder(){return r.mfa.passkeyName},width:Q,get disabled(){return e(f)},get value(){return e(n).passkeyName},set value(c){e(n).passkeyName=c},get error(){return e(x).passkeyName},set error(c){e(x).passkeyName=c},$$events:{enter:Ie},children:(c,T)=>{C();var S=k();m(()=>w(S,r.mfa.passkeyName)),p(c,S)},$$slots:{default:!0}});var ce=d(oe,2),me=as(()=>e(f)?2:1);B(ce,{width:O,get level(){return e(me)},get isDisabled(){return e(f)},$$events:{click:Ie},children:(c,T)=>{C();var S=k();m(()=>w(S,r.mfa.register)),p(c,S)},$$slots:{default:!0}});var v=d(ce,2);{var A=c=>{var T=hs(),S=_(T),He=_(S,!0);R(S);var fe=d(S,2),Qe=_(fe,!0);R(fe);var Ye=d(fe,2);B(Ye,{width:O,level:1,$$events:{click:Ce},children:(Ze,Ns)=>{C();var We=k();m(()=>w(We,r.passwordReset.accountLogin)),p(Ze,We)},$$slots:{default:!0}}),R(T),m(()=>{w(He,r.passwordReset.successPasskey1),w(Qe,r.passwordReset.successPasskey2)}),p(c,T)};y(v,c=>{e(f)&&c(A)})}R(M),je(3,M,()=>qe),p(H,M)};y(L,H=>{e(Z)==="passkey"&&H(le)},!0)}p(o,$)};y(z,o=>{e(Z)==="password"?o(u):o(E,!1)})}m(()=>{w(h,r.passwordReset.newAccount),w(ae,r.passwordReset.newAccDesc1),w(G,r.passwordReset.newAccDesc2),Ae(K,"href",r.passwordReset.fidoLink)}),p(a,t)},Me=a=>{var t=ie(),i=V(t);{var b=D=>{var h=ks(),q=V(h);{var ae=u=>{Fe(u,{purpose:"PasswordReset",onSuccess:Ee,onError:pe,get data(){return e(j)},set data(E){s(j,l(E))}})};y(q,u=>{e(j)&&u(ae)})}var F=d(q,4);Oe(F,{get policy(){return e(W)},get password(){return e(n).password},get accepted(){return e(se)},set accepted(u){s(se,l(u))}});var G=d(F,2);we(G,{get error(){return e(x).password},autocomplete:"new-password",get placeholder(){return r.passwordReset.password},width:Q,get showCopy(){return e(re)},get value(){return e(n).password},set value(u){e(n).password=u},children:(u,E)=>{C();var o=k();m(()=>w(o,r.passwordReset.password.toUpperCase())),p(u,o)},$$slots:{default:!0}});var K=d(G,2);we(K,{get error(){return e(x).passwordConfirm},autocomplete:"new-password",get placeholder(){return r.passwordReset.passwordConfirm},width:Q,get showCopy(){return e(re)},get value(){return e(n).passwordConfirm},set value(u){e(n).passwordConfirm=u},children:(u,E)=>{C();var o=k();m(()=>w(o,r.passwordReset.passwordConfirm.toUpperCase())),p(u,o)},$$slots:{default:!0}});var P=d(K,2);B(P,{width:O,level:3,$$events:{click:Ne},children:(u,E)=>{C();var o=k();m(()=>w(o,r.passwordReset.generate)),p(u,o)},$$slots:{default:!0}});var J=d(P,2);B(J,{width:O,level:2,get isLoading(){return e(N)},set isLoading(u){s(N,l(u))},$$events:{click:Le},children:(u,E)=>{C();var o=k();m(()=>w(o,r.common.save)),p(u,o)},$$slots:{default:!0}});var te=d(J,2);{var z=u=>{var E=bs(),o=_(E),$=d(o,2),L=d($,3);R(E),m(()=>{w(o,`${r.passwordReset.success1??""}
                ${r.passwordReset.success2??""} `),w($,` ${r.passwordReset.success3??""} `),Ae(L,"href",e(ee)||"/auth/v1/account")}),p(u,E)};y(te,u=>{e(f)&&u(z)})}p(D,h)};y(i,D=>{e(Y).startsWith("password_reset")&&D(b)},!0)}p(a,t)};y(Pe,a=>{e(Y).startsWith("new_user")?a(Je):a(Me,!1)})}var Te=d(Pe,2);{var Xe=a=>{var t=_s(),i=_(t,!0);R(t),m(()=>w(i,e(I))),p(a,t)};y(Te,a=>{e(I)&&a(Xe)})}R(ve);var ze=d(ve,2);vs(ze,{absolute:!0}),p(Ve,xe),rs()}export{Cs as component};
