import"../chunks/disclose-version.BDr9Qe-U.js";import"../chunks/legacy.Dmu0D13_.js";import{p as he,s as a,c as o,r as l,t as C,a as ge,a6 as Ne,y as e,z as n,A as S,f as ee,ar as F,ab as de,ap as yr,a0 as Vr}from"../chunks/index-client.CVQra9Cu.js";import{s as u,e as Yr}from"../chunks/render.CkKFmY3E.js";import{i as D}from"../chunks/if._4yhN2kP.js";import{e as Er}from"../chunks/each.D7w_Y1pv.js";import{a as c,t as E,n as Hr,d as G,e as Xe}from"../chunks/template.4ZWi6dE6.js";import{s as re}from"../chunks/class.BnbJV1WX.js";import{a as Be,t as Ze,s as er}from"../chunks/index.CkVfMYgG.js";import{p as k}from"../chunks/proxy.Dc6q2r8X.js";import{u as Kr,d as Or,f as Jr,c as Qr,b as pr}from"../chunks/fetch.BnpwKXAn.js";import{C as Wr}from"../chunks/ContentAdmin.p5jUo2m-.js";import{N as Xr}from"../chunks/NavSub.CPeQMNZ5.js";import{u as Zr}from"../chunks/param.svelte.CAlaz0Tl.js";import{N as et}from"../chunks/NavButtonTile.B87bIUim.js";import{B as rt}from"../chunks/ButtonAddModal.B1jk33x3.js";import{p as oe}from"../chunks/props.BVYR8gU-.js";import{k as tt}from"../chunks/key.BF1drcZy.js";import{h as ye}from"../chunks/html.DBwFzVLh.js";import{B as Ve}from"../chunks/Button.B6DffihK.js";import{I as j}from"../chunks/Input.E2qlCEoO.js";import{i as at}from"../chunks/lifecycle.Cb2DdOTg.js";import{u as Ee}from"../chunks/i18n_admin.svelte.DBF4IRUg.js";import{g as it}from"../chunks/helpers.mrWtKrwW.js";import{I as Tr}from"../chunks/IconCheck.jhQYM7vu.js";import{I as ur}from"../chunks/InputArea.CPIIXA8-.js";import{I as Lr}from"../chunks/InputPassword.BoC-LeDj.js";import{F as Ar}from"../chunks/Form.BiuDxuNH.js";import{u as mr}from"../chunks/i18n.svelte.DVZudxvm.js";import{L as _r}from"../chunks/LabeledValue.DfaotEKr.js";import{I as le}from"../chunks/InputCheckbox.DJbEwO1k.js";import{f as B,g as Pr,c as Nr,h as fr}from"../chunks/patterns.CpCrQ-Dx.js";import{c as ot}from"../chunks/input.DW316tIw.js";import{a as hr}from"../chunks/helpers.BweJi6s8.js";import{L as lt}from"../chunks/Loading.BhsEZHIf.js";import{E as st}from"../chunks/Expandable.BcrRxLXT.js";import{T as nt}from"../chunks/Tabs.BGGYqHwk.js";import"../chunks/Switch.BxcKRi58.js";import{C as Mr}from"../chunks/CheckIcon.BEZpWWv8.js";import{O as ct}from"../chunks/Options.DNEfnOhW.js";var dt=E('<div class="desc svelte-6fcb3i"><h4 class="svelte-6fcb3i">ID token claim mappings</h4> <p class="svelte-6fcb3i"> </p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p> <p class="svelte-6fcb3i"><!></p></div>');function Ur(V,p){he(p,!1);let r=Ee();at();var d=dt(),s=a(o(d),2),m=o(s,!0);l(s);var $=a(s,2),h=o($);ye(h,()=>r.providers.config.jsonPath.p2),l($);var U=a($,2),N=o(U);ye(N,()=>r.providers.config.jsonPath.p3),l(U);var w=a(U,2),t=o(w);ye(t,()=>r.providers.config.jsonPath.p4),l(w);var x=a(w,2),_=o(x);ye(_,()=>r.providers.config.jsonPath.p5),l(x);var y=a(x,2),f=o(y);ye(f,()=>r.providers.config.jsonPath.p6),l(y),l(d),C(()=>u(m,r.providers.config.jsonPath.p1)),c(V,d),ge()}var vt=E('<img alt="No Logo Available">');function pt(V,p){he(p,!0);var r=vt();C(()=>re(r,"src",`/auth/v1/providers/${p.providerId}/img?${it(6)}`)),re(r,"width",20),re(r,"height",20),c(V,r),ge()}var ut=Hr('<svg fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"></path></svg>');function mt(V,p){let r=oe(p,"opacity",3,.9),d=oe(p,"width",3,"1.5rem"),s=oe(p,"color",3,"currentColor");var m=ut();re(m,"stroke-width",2),C(()=>{re(m,"stroke",s()),re(m,"width",d()),re(m,"opacity",r())}),c(V,m)}var _t=E('<div class="flex space-between loading svelte-1k623j2"><span class="flex gap-05"><span> </span> <span> </span> <span>/</span> <span> </span></span> <!></div>'),ft=E('<label class="svelte-1k623j2"><!></label> <input type="file" class="svelte-1k623j2">',1),ht=E('<form aria-dropeffect="move" aria-label="Upload" class="flex svelte-1k623j2"><!></form>');function gt(V,p){he(p,!0);let r=oe(p,"method",3,"PUT"),d=oe(p,"accept",19,()=>[".image/jpg","image/jpeg","image/png","image/svg"]),s=oe(p,"buttonOnly",3,!1),m=oe(p,"buttonSize",3,"1.5rem"),$=oe(p,"multiple",3,!1),h=oe(p,"width",3,"17rem"),U=oe(p,"height",3,"2.5rem");const N=hr();let w=Ee(),t=S(!1),x=S(void 0),_=S(0),y=S(0);Ne(()=>{!e(t)&&e(x)&&e(x).length>0&&A(e(x))});function f(P){var I;n(x,k((I=P.dataTransfer)==null?void 0:I.files))}async function A(P){var I;n(_,k(P.length)),n(y,0),n(t,!0);for(let R of P){if(console.log("uploading file",R),d()&&!d().includes(R.type)){n(y,e(y)+1);continue}let W=await Kr(r(),p.url,R,p.fileName||R.name);W.error?console.error(W.error):(I=p.onSuccess)==null||I.call(p),n(y,e(y)+1)}n(x,void 0),n(t,!1)}var b=ht(),q=o(b);{var X=P=>{var I=_t(),R=o(I),W=o(R),H=o(W);l(W);var se=a(W,2),pe=o(se,!0);l(se);var Y=a(se,4),z=o(Y,!0);l(Y),l(R);var Q=a(R,2);lt(Q,{}),l(I),C(()=>{u(H,`${w.common.loading??""}:`),u(pe,e(y)),u(z,e(_))}),c(P,I)},J=P=>{var I=ft(),R=ee(I);re(R,"for",N);var W=o(R);mt(W,{get width(){return m()}}),l(R);var H=a(R,2);re(H,"id",N),C(()=>re(H,"accept",d()&&d().join(", "))),C(()=>{re(R,"aria-disabled",p.disabled),re(H,"aria-hidden",s()),re(H,"aria-disabled",p.disabled),H.disabled=p.disabled,H.multiple=$()}),ot(H,()=>e(x),se=>n(x,se)),c(P,I)};D(q,P=>{e(t)?P(X):P(J,!1)})}l(b),C(()=>{re(b,"data-nopad",s()),Be(b,"width",h()),Be(b,"height",U())}),Yr("drop",b,f),c(V,b),ge()}var bt=E("<div><!></div>"),kt=E('<div class="checkbox svelte-iec8od"><!></div>'),wt=E('<div class="err"> </div>'),Ct=E('<div class="svelte-iec8od"><!></div>'),$t=E('<span class="err"> </span>'),It=E('<!> <div></div> <div class="checkbox svelte-iec8od"><!></div> <div class="checkbox svelte-iec8od"><!></div> <!> <!> <!> <!> <!> <div class="checkbox svelte-iec8od"><!></div> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p class="desc svelte-iec8od"> </p> <!> <p><!></p> <div class="checkbox svelte-iec8od"><!></div> <div class="checkbox svelte-iec8od"><!></div> <!> <!> <p class="desc svelte-iec8od"> </p> <!> <!> <p class="desc svelte-iec8od"> </p> <!> <!> <div class="logo svelte-iec8od"><!> <!></div> <div class="flex gap-05"><!> <!> <!></div>',1),yt=E('<div class="container svelte-iec8od"><!></div>');function Et(V,p){he(p,!0);let r=oe(p,"provider",15),d=mr(),s=Ee();const m="min(calc(100dvw - .5rem), 30rem)";let $=!1,h=S(""),U=S(!1),N=S(!!r().root_pem),w=S(k(hr()));Ne(()=>{r().id&&(r(r().client_secret=r().client_secret||"",!0),r(r().admin_claim_path=r().admin_claim_path||"",!0),r(r().admin_claim_value=r().admin_claim_value||"",!0),r(r().mfa_claim_path=r().mfa_claim_path||"",!0),r(r().mfa_claim_value=r().mfa_claim_value||"",!0),r(r().root_pem=r().root_pem||"",!0))}),Ne(()=>{r().scope&&r(r().scope=r().scope.replaceAll("+"," "),!0)});async function t(f,A){if(n(h,""),r().client_secret&&!(r().client_secret_basic||r().client_secret_post)){n(h,k(s.providers.config.errNoAuthMethod));return}if(!r().use_pkce&&!r().client_secret){n(h,k(s.providers.config.errConfidential));return}let b={name:r().name,typ:r().typ,enabled:r().enabled,issuer:r().issuer,authorization_endpoint:r().authorization_endpoint,token_endpoint:r().token_endpoint,userinfo_endpoint:r().userinfo_endpoint,danger_allow_insecure:e(N)&&r().root_pem?!1:r().danger_allow_insecure,use_pkce:r().use_pkce,client_secret_basic:r().client_secret_basic,client_secret_post:r().client_secret_post,client_id:r().client_id,client_secret:r().client_secret||void 0,scope:r().scope.trim(),root_pem:e(N)&&r().root_pem?r().root_pem.trim():void 0,admin_claim_path:r().admin_claim_path||void 0,admin_claim_value:r().admin_claim_value||void 0,mfa_claim_path:r().mfa_claim_path||void 0,mfa_claim_value:r().mfa_claim_value||void 0},q=await Or(f.action,b);q.error?n(h,k(q.error.message)):(n(U,!0),p.onSave(),setTimeout(()=>{n(U,!1)},3e3))}var x=yt(),_=o(x),y=de(()=>`/auth/v1/providers/${r().id}`);Ar(_,{get action(){return e(y)},onSubmit:t,children:(f,A)=>{var b=It(),q=ee(b);_r(q,{label:"ID",mono:!0,children:(i,T)=>{F();var L=G();C(()=>u(L,r().id)),c(i,L)},$$slots:{default:!0}});var X=a(q,2);Be(X,"height",".15rem");var J=a(X,2),P=o(J);le(P,{get ariaLabel(){return s.common.enabled},get checked(){return r().enabled},set checked(i){r(r().enabled=i,!0)},children:(i,T)=>{F();var L=G();C(()=>u(L,s.common.enabled)),c(i,L)},$$slots:{default:!0}}),l(J);var I=a(J,2),R=o(I);le(R,{get ariaLabel(){return s.providers.config.custRootCa},get checked(){return e(N)},set checked(i){n(N,k(i))},children:(i,T)=>{F();var L=G();C(()=>u(L,s.providers.config.custRootCa)),c(i,L)},$$slots:{default:!0}}),l(I);var W=a(I,2);{var H=i=>{var T=bt(),L=o(T);ur(L,{rows:15,name:"rootPem",get label(){return s.providers.config.rootPemCert},placeholder:`-----BEGIN CERTIFICATE-----
...
 -----END CERTIFICATE-----`,errMsg:"-----BEGIN CERTIFICATE----- ...",width:"min(40rem, calc(100dvw - .5rem))",fontMono:!0,pattern:fr,get value(){return r().root_pem},set value(me){r(r().root_pem=me,!0)}}),l(T),Ze(3,T,()=>er,()=>({duration:150})),c(i,T)},se=i=>{var T=kt(),L=o(T);le(L,{get ariaLabel(){return s.providers.config.allowInsecureTls},get checked(){return r().danger_allow_insecure},set checked(me){r(r().danger_allow_insecure=me,!0)},children:(me,gr)=>{F();var Fe=G();C(()=>u(Fe,s.providers.config.allowInsecureTls)),c(me,Fe)},$$slots:{default:!0}}),l(T),c(i,T)};D(W,i=>{e(N)?i(H):i(se,!1)})}var pe=a(W,2);j(pe,{autocomplete:"off",label:"Issuer URL",placeholder:"Issuer URL",required:!0,pattern:B,width:m,get value(){return r().issuer},set value(i){r(r().issuer=i,!0)}});var Y=a(pe,2);j(Y,{typ:"url",autocomplete:"off",label:"Authorization Endpoint",placeholder:"Authorization Endpoint",required:!0,pattern:B,width:m,get value(){return r().authorization_endpoint},set value(i){r(r().authorization_endpoint=i,!0)}});var z=a(Y,2);j(z,{typ:"url",autocomplete:"off",label:"Token Endpoint",placeholder:"Token Endpoint",required:!0,pattern:B,width:m,get value(){return r().token_endpoint},set value(i){r(r().token_endpoint=i,!0)}});var Q=a(z,2);j(Q,{typ:"url",autocomplete:"off",label:"Userinfo Endpoint",placeholder:"Userinfo Endpoint",required:!0,pattern:B,width:m,get value(){return r().userinfo_endpoint},set value(i){r(r().userinfo_endpoint=i,!0)}});var g=a(Q,2),be=o(g);le(be,{ariaLabel:"PKCE",get checked(){return r().use_pkce},set checked(i){r(r().use_pkce=i,!0)},children:(i,T)=>{F();var L=G("PKCE");c(i,L)},$$slots:{default:!0}}),l(g);var ue=a(g,2),ke=o(ue,!0);l(ue);var we=a(ue,2);j(we,{autocomplete:"off",label:"Scope",placeholder:"openid profile email",required:!0,pattern:Pr,width:m,get value(){return r().scope},set value(i){r(r().scope=i,!0)}});var Ce=a(we,2),Ye=o(Ce,!0);l(Ce);var Me=a(Ce,2);j(Me,{autocomplete:"off",get label(){return s.providers.config.clientName},get placeholder(){return s.providers.config.clientName},required:!0,pattern:Nr,width:m,get value(){return r().name},set value(i){r(r().name=i,!0)}});var Te=a(Me,2),rr=o(Te,!0);l(Te);var Ue=a(Te,2);j(Ue,{autocomplete:"off",label:"Client ID",placeholder:"Client ID",required:!0,pattern:B,width:m,get value(){return r().client_id},set value(i){r(r().client_id=i,!0)}});var qe=a(Ue,2),tr=o(qe,!0);l(qe);var He=a(qe,2),M=de(()=>!r().use_pkce);Lr(He,{autocomplete:"off",label:"Client Secret",placeholder:"Client Secret",maxLength:256,get errMsg(){return s.providers.config.errConfidential},get required(){return e(M)},width:m,get value(){return r().client_secret},set value(i){r(r().client_secret=i,!0)}});var K=a(He,2),te=o(K);ye(te,()=>s.providers.config.descAuthMethod),l(K);var ve=a(K,2),Re=o(ve);le(Re,{ariaLabel:"client_secret_basic",get checked(){return r().client_secret_basic},set checked(i){r(r().client_secret_basic=i,!0)},children:(i,T)=>{F();var L=G("client_secret_basic");c(i,L)},$$slots:{default:!0}}),l(ve);var ne=a(ve,2),fe=o(ne);le(fe,{ariaLabel:"client_secret_post",get checked(){return r().client_secret_post},set checked(i){r(r().client_secret_post=i,!0)},children:(i,T)=>{F();var L=G("client_secret_post");c(i,L)},$$slots:{default:!0}}),l(ne);var Z=a(ne,2);{var ae=i=>{var T=wt(),L=o(T,!0);l(T),C(()=>u(L,s.providers.config.errNoAuthMethod)),Ze(3,T,()=>er,()=>({duration:150})),c(i,T)};D(Z,i=>{!r().use_pkce&&!r().client_secret_basic&&!r().client_secret_post&&i(ae)})}var ie=a(Z,2);Ur(ie,{});var ce=a(ie,2),Se=o(ce,!0);l(ce);var $e=a(ce,2);j($e,{autocomplete:"off",get label(){return s.providers.config.pathAdminClaim},placeholder:"$.roles.*",pattern:B,width:m,get value(){return r().admin_claim_path},set value(i){r(r().admin_claim_path=i,!0)}});var Ke=a($e,2);j(Ke,{autocomplete:"off",get label(){return s.providers.config.valueAdminClaim},placeholder:"rauthy_admin",pattern:B,width:m,get value(){return r().admin_claim_value},set value(i){r(r().admin_claim_value=i,!0)}});var Le=a(Ke,2),Oe=o(Le,!0);l(Le);var xe=a(Le,2);j(xe,{autocomplete:"off",get label(){return s.providers.config.pathMfaClaim},placeholder:"$.amr.*",pattern:B,width:m,get value(){return r().mfa_claim_path},set value(i){r(r().mfa_claim_path=i,!0)}});var ze=a(xe,2);j(ze,{autocomplete:"off",get label(){return s.providers.config.valueMfaClaim},placeholder:"mfa",pattern:B,width:m,get value(){return r().mfa_claim_value},set value(i){r(r().mfa_claim_value=i,!0)}});var Ie=a(ze,2),Je=o(Ie);tt(Je,()=>e(w),i=>{var T=Ct(),L=o(T);pt(L,{get providerId(){return r().id}}),l(T),c(i,T)});var ar=a(Je,2),ir=de(()=>`/auth/v1/providers/${r().id}/img`);gt(ar,{method:"PUT",get url(){return e(ir)},fileName:"logo",onSuccess:()=>n(w,k(hr()))}),l(Ie);var Ae=a(Ie,2),Qe=o(Ae);Ve(Qe,{type:"submit",get isLoading(){return $},children:(i,T)=>{F();var L=G();C(()=>u(L,d.common.save)),c(i,L)},$$slots:{default:!0}});var De=a(Qe,2);{var je=i=>{Tr(i,{})};D(De,i=>{e(U)&&i(je)})}var or=a(De,2);{var We=i=>{var T=$t(),L=o(T,!0);l(T),C(()=>u(L,e(h))),c(i,T)};D(or,i=>{e(h)&&i(We)})}l(Ae),C(()=>{u(ke,s.providers.config.descScope),u(Ye,s.providers.config.descClientName),u(rr,s.providers.config.descClientId),u(tr,s.providers.config.descClientSecret),u(Se,s.providers.config.mapUser),u(Oe,s.providers.config.mapMfa)}),c(f,b)},$$slots:{default:!0}}),l(x),c(V,x),ge()}var Tt=E('<div class="user svelte-1vu6qeb"> <span class="muted font-mono svelte-1vu6qeb"> </span></div>'),Lt=E('<div class="forceDelete svelte-1vu6qeb"> </div>'),At=E('<p class="err"><b> </b></p> <p> </p> <!> <p> </p> <!> <!>',1),Pt=E("<p> </p> <!>",1),Nt=E('<div class="err"> </div>'),Mt=E('<div class="container svelte-1vu6qeb"><!> <!></div>');function Ut(V,p){he(p,!0);let r=mr(),d=Ee(),s=S(!0),m=S(""),$=S(!1),h=S(k([]));yr(async()=>{let f=await Jr(`/auth/v1/providers/${p.provider.id}/delete_safe`);f.status===406&&f.error&&n(h,k(f.error)),n(s,!1)});async function U(){n(m,""),n(s,!0);let f=await Qr(`/auth/v1/providers/${p.provider.id}`);f.error?n(m,k(f.error.message)):p.onSave(),n(s,!1)}var N=Mt(),w=o(N);{var t=f=>{var A=At(),b=ee(A),q=o(b),X=o(q,!0);l(q),l(b);var J=a(b,2),P=o(J,!0);l(J);var I=a(J,2);st(I,{summary:Y=>{F();var z=G();C(()=>u(z,d.providers.delete.linkedUsers)),c(Y,z)},details:Y=>{var z=Xe(),Q=ee(z);Er(Q,17,()=>e(h),g=>g.id,(g,be)=>{var ue=Tt(),ke=o(ue),we=a(ke),Ce=o(we);l(we),l(ue),C(()=>{u(ke,`${e(be).email??""} `),u(Ce,`/ ${e(be).id??""}`)}),c(g,ue)}),c(Y,z)},$$slots:{summary:!0,details:!0}});var R=a(I,2),W=o(R,!0);l(R);var H=a(R,2);le(H,{get ariaLabel(){return d.providers.delete.forceDelete},get checked(){return e($)},set checked(Y){n($,k(Y))},children:(Y,z)=>{var Q=Lt(),g=o(Q,!0);l(Q),C(()=>u(g,d.providers.delete.forceDelete)),c(Y,Q)},$$slots:{default:!0}});var se=a(H,2);{var pe=Y=>{Ve(Y,{level:-1,onclick:U,get isLoading(){return e(s)},children:(z,Q)=>{F();var g=G();C(()=>u(g,r.common.delete)),c(z,g)},$$slots:{default:!0}})};D(se,Y=>{e($)&&Y(pe)})}C(()=>{u(X,d.providers.delete.isInUse1),u(P,d.providers.delete.isInUse2),u(W,d.providers.delete.areYouSure)}),c(f,A)},x=f=>{var A=Pt(),b=ee(A),q=o(b,!0);l(b);var X=a(b,2);Ve(X,{level:-1,onclick:U,get isLoading(){return e(s)},children:(J,P)=>{F();var I=G();C(()=>u(I,r.common.delete)),c(J,I)},$$slots:{default:!0}}),C(()=>u(q,d.providers.delete.areYouSure)),c(f,A)};D(w,f=>{e(h).length>0?f(t):f(x,!1)})}var _=a(w,2);{var y=f=>{var A=Nt(),b=o(A,!0);l(A),C(()=>u(b,e(m))),c(f,A)};D(_,f=>{e(m)&&f(y)})}l(N),c(V,N),ge()}var qt=E('<div class="flex"><!></div> <!>',1);function Rt(V,p){he(p,!0);let r=oe(p,"provider",15),d=oe(p,"onSave",15),s=Ee();const m=[s.tabs.config,s.tabs.delete];let $=S(k(m[0]));var h=qt(),U=ee(h),N=o(U);nt(N,{tabs:m,get selected(){return e($)},set selected(_){n($,k(_))}}),l(U);var w=a(U,2);{var t=_=>{Et(_,{get provider(){return r()},set provider(y){r(y)},get onSave(){return d()},set onSave(y){d(y)}})},x=_=>{var y=Xe(),f=ee(y);{var A=b=>{Ut(b,{get provider(){return r()},get onSave(){return d()},set onSave(q){d(q)}})};D(f,b=>{e($)===s.tabs.delete&&b(A)},!0)}c(_,y)};D(w,_=>{e($)===s.tabs.config?_(t):_(x,!1)})}c(V,h),ge()}var St=E("<div><!></div>"),xt=E('<div class="checkbox svelte-f8e9ia"><!></div>'),zt=E('<div class="checkbox svelte-f8e9ia"><!></div> <!>',1),Dt=E('<div class="ml mb"><!></div>'),jt=E('<div class="err"> </div>'),Ft=E('<!> <!> <!> <!> <!> <div class="checkbox svelte-f8e9ia"><!></div> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p class="desc svelte-f8e9ia"> </p> <!> <p><!></p> <div class="checkbox svelte-f8e9ia"><!></div> <div class="checkbox svelte-f8e9ia"><!></div> <!> <!> <p class="desc svelte-f8e9ia"> </p> <!> <!> <p class="desc svelte-f8e9ia"> </p> <!> <!>',1),Gt=E('<div class="err"> </div>'),Bt=E('<!> <!> <div class="flex gap-05"><!> <!> <!> <!></div>',1),Vt=E('<div class="container svelte-f8e9ia"><!> <div></div> <!></div>');function Yt(V,p){he(p,!0);let r=mr(),d=Ee();const s="min(calc(100dvw - 1.75rem), 30rem)";let m=S(!1),$=S(""),h=S(!1),U=S(!1),N=S(!1),w=S(k({issuer:"",metadata_url:"",danger_allow_insecure:!1,root_pem:""})),t=S(k({enabled:!0,typ:"oidc",issuer:"",danger_allow_insecure:!1,authorization_endpoint:"",token_endpoint:"",token_auth_method_basic:!1,userinfo_endpoint:"",use_pkce:!0,client_secret_basic:!0,client_secret_post:!1,name:"",client_id:"",client_secret:"",scope:"",root_pem:"",admin_claim_path:"",admin_claim_value:"",mfa_claim_path:"",mfa_claim_value:""})),x=["OIDC","Auto","Custom","Github","Google"],_=S(k(x[0])),y=de(()=>e(_)==="Auto"),f=de(()=>e(_)==="Custom"),A=de(()=>e(_)==="OIDC"),b=de(()=>!e(y)&&!e(f)&&!e(A)),q=de(()=>!(e(b)||e(f)||e(h)));const X="/auth/v1/providers/lookup";let J=de(()=>e(q)?X:"/auth/v1/providers/create");Ne(()=>{if(e(_))switch(n(h,!1),n(w,k({issuer:"",metadata_url:"",danger_allow_insecure:!1,root_pem:""})),e(_)){case"Github":n(t,k({enabled:!0,issuer:"github.com",typ:"github",danger_allow_insecure:!1,authorization_endpoint:"https://github.com/login/oauth/authorize",token_endpoint:"https://github.com/login/oauth/access_token",client_secret_basic:!0,client_secret_post:!0,userinfo_endpoint:"https://api.github.com/user",use_pkce:!1,name:"Github",client_id:"",client_secret:"",scope:"user:email",root_pem:"",admin_claim_path:"",admin_claim_value:"",mfa_claim_path:"$.two_factor_authentication",mfa_claim_value:"true"}));break;case"Google":n(w,k({issuer:"accounts.google.com",metadata_url:"",danger_allow_insecure:!1,root_pem:""})),Vr(()=>{R(X)});break;default:n(t,k({enabled:!0,issuer:"",typ:"oidc",danger_allow_insecure:!1,authorization_endpoint:"",token_endpoint:"",client_secret_basic:!0,client_secret_post:!1,userinfo_endpoint:"",use_pkce:!0,name:"",client_id:"",client_secret:"",scope:"",root_pem:"",admin_claim_path:"",admin_claim_value:"",mfa_claim_path:"",mfa_claim_value:""}))}}),Ne(()=>{e(U)&&setTimeout(()=>{p.onSave(),W()},1500)});async function P(z,Q){e(q)?await R(z.action):await I(z.action)}async function I(z){if(e(t).client_secret&&!(e(t).client_secret_basic||e(t).client_secret_post)){n($,k(d.providers.config.errNoAuthMethod));return}if(!e(t).use_pkce&&!e(t).client_secret){n($,k(d.providers.config.errConfidential));return}n($,""),n(m,!0);let Q={name:e(t).name,typ:e(y)?"custom":e(_).toLowerCase(),enabled:e(t).enabled,issuer:e(t).issuer,authorization_endpoint:e(t).authorization_endpoint,token_endpoint:e(t).token_endpoint,userinfo_endpoint:e(t).userinfo_endpoint,danger_allow_insecure:e(N)&&e(t).root_pem?!1:e(t).danger_allow_insecure,use_pkce:e(t).use_pkce,client_secret_basic:e(t).client_secret_basic,client_secret_post:e(t).client_secret_post,client_id:e(t).client_id,client_secret:e(t).client_secret,scope:e(t).scope.trim(),root_pem:e(N)&&e(t).root_pem?e(t).root_pem.trim():void 0,admin_claim_path:e(t).admin_claim_path||void 0,admin_claim_value:e(t).admin_claim_value||void 0,mfa_claim_path:e(t).mfa_claim_path||void 0,mfa_claim_value:e(t).mfa_claim_value||void 0},g=await pr(z,Q);g.error?g.error.message.includes("InvalidCertificate")?n($,"Insecure connection not allowed"):n($,k(g.error.message)):n(U,!0),n(m,!1)}async function R(z){if(!e(w).issuer&&!e(w).metadata_url){n($,"Provide at least one of Issuer / Metadata URL");return}n($,""),n(m,!0);let Q={issuer:e(w).issuer||void 0,metadata_url:e(w).metadata_url||void 0,danger_allow_insecure:e(w).danger_allow_insecure,root_pem:e(w).root_pem||void 0},g=await pr(z,Q);g.body?(e(t).issuer=g.body.issuer,e(t).authorization_endpoint=g.body.authorization_endpoint,e(t).danger_allow_insecure=g.body.danger_allow_insecure,e(t).token_endpoint=g.body.token_endpoint,e(t).userinfo_endpoint=g.body.userinfo_endpoint,e(t).client_secret_basic=g.body.client_secret_basic,e(t).client_secret_post=g.body.client_secret_post,e(t).use_pkce=g.body.use_pkce,e(t).client_secret_basic=g.body.client_secret_basic,e(t).client_secret_post=!g.body.client_secret_basic&&g.body.client_secret_post,e(t).scope=g.body.scope,e(t).root_pem=g.body.root_pem,n(h,!0)):g.error&&(g.error.message.includes("InvalidCertificate")?n($,"Insecure connection not allowed"):n($,k(g.error.message))),n(m,!1)}function W(){n(w,k({issuer:"",metadata_url:"",danger_allow_insecure:!1,root_pem:""})),n(t,k({enabled:!0,issuer:"",typ:"oidc",danger_allow_insecure:!1,authorization_endpoint:"",token_endpoint:"",userinfo_endpoint:"",use_pkce:!0,name:"",client_id:"",client_secret_basic:!0,client_secret_post:!1,scope:"",root_pem:"",admin_claim_path:"",admin_claim_value:"",mfa_claim_path:"",mfa_claim_value:""})),n(U,!1),n(h,!1),n(N,!1)}var H=Vt(),se=o(H);ct(se,{ariaLabel:"Select Mode",options:x,get value(){return e(_)},set value(z){n(_,k(z))}});var pe=a(se,2);Be(pe,"height",".5rem");var Y=a(pe,2);Ar(Y,{get action(){return e(J)},onSubmit:P,children:(z,Q)=>{var g=Bt(),be=ee(g);{var ue=M=>{var K=zt(),te=ee(K),ve=o(te);le(ve,{get ariaLabel(){return d.providers.config.custRootCa},get checked(){return e(N)},set checked(Z){n(N,k(Z))},children:(Z,ae)=>{F();var ie=G();C(()=>u(ie,d.providers.config.custRootCa)),c(Z,ie)},$$slots:{default:!0}}),l(te);var Re=a(te,2);{var ne=Z=>{var ae=St(),ie=o(ae);ur(ie,{rows:15,name:"rootPem",get label(){return d.providers.config.rootPemCert},placeholder:`-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----`,errMsg:"-----BEGIN CERTIFICATE----- ...",width:"min(40rem, calc(100dvw - 1.75rem))",fontMono:!0,pattern:fr,get value(){return e(w).root_pem},set value(ce){e(w).root_pem=ce}}),l(ae),Ze(3,ae,()=>er,()=>({duration:150})),c(Z,ae)},fe=Z=>{var ae=xt(),ie=o(ae);le(ie,{get ariaLabel(){return d.providers.config.allowInsecureTls},get checked(){return e(w).danger_allow_insecure},set checked(ce){e(w).danger_allow_insecure=ce},children:(ce,Se)=>{F();var $e=G();C(()=>u($e,d.providers.config.allowInsecureTls)),c(ce,$e)},$$slots:{default:!0}}),l(ae),c(Z,ae)};D(Re,Z=>{e(N)?Z(ne):Z(fe,!1)})}c(M,K)};D(be,M=>{e(h)||M(ue)})}var ke=a(be,2);{var we=M=>{j(M,{name:"issuer",label:"Issuer URL",placeholder:"Issuer URL",width:s,required:!0,pattern:B,get value(){return e(w).issuer},set value(K){e(w).issuer=K}})},Ce=M=>{var K=Xe(),te=ee(K);{var ve=ne=>{j(ne,{typ:"url",name:"metadata",label:"Metadata URL",placeholder:".../.well-known/openid-configuration",width:s,required:!0,pattern:B,get value(){return e(w).metadata_url},set value(fe){e(w).metadata_url=fe}})},Re=ne=>{var fe=Xe(),Z=ee(fe);{var ae=ie=>{var ce=Ft(),Se=ee(ce);{var $e=v=>{ur(v,{rows:17,name:"rootPem",get label(){return d.providers.config.rootPemCert},placeholder:`-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----`,get disabled(){return e(h)},errMsg:"-----BEGIN CERTIFICATE----- ...",width:"min(40rem, calc(100dvw - 1.75rem))",fontMono:!0,pattern:fr,get value(){return e(t).root_pem},set value(O){e(t).root_pem=O}})},Ke=v=>{var O=Dt(),_e=o(O);{var vr=Pe=>{_r(Pe,{get label(){return d.providers.config.allowInsecureTls},children:(Ge,Br)=>{Mr(Ge,{get check(){return e(t).danger_allow_insecure}})},$$slots:{default:!0}})},Gr=Pe=>{le(Pe,{get ariaLabel(){return d.providers.config.allowInsecureTls},get checked(){return e(t).danger_allow_insecure},set checked(Ge){e(t).danger_allow_insecure=Ge},children:(Ge,Br)=>{F();var Ir=G();C(()=>u(Ir,d.providers.config.allowInsecureTls)),c(Ge,Ir)},$$slots:{default:!0}})};D(_e,Pe=>{e(h)?Pe(vr):Pe(Gr,!1)})}l(O),c(v,O)};D(Se,v=>{e(N)?v($e):v(Ke,!1)})}var Le=a(Se,2);j(Le,{name:"issuer",label:"Issuer URL",placeholder:"Issuer URL",width:s,get disabled(){return e(h)},required:!0,pattern:B,get value(){return e(t).issuer},set value(v){e(t).issuer=v}});var Oe=a(Le,2);j(Oe,{typ:"url",name:"auth_endpoint",label:"Authorization Endpoint",placeholder:"Authorization Endpoint",width:s,get disabled(){return e(h)},required:!0,pattern:B,get value(){return e(t).authorization_endpoint},set value(v){e(t).authorization_endpoint=v}});var xe=a(Oe,2);j(xe,{typ:"url",name:"token_endpoint",label:"Token Endpoint",placeholder:"Token Endpoint",width:s,get disabled(){return e(h)},required:!0,pattern:B,get value(){return e(t).token_endpoint},set value(v){e(t).token_endpoint=v}});var ze=a(xe,2);j(ze,{typ:"url",name:"userinfo_endpoint",label:"Userinfo Endpoint",placeholder:"Userinfo Endpoint",width:s,get disabled(){return e(h)},required:!0,pattern:B,get value(){return e(t).userinfo_endpoint},set value(v){e(t).userinfo_endpoint=v}});var Ie=a(ze,2),Je=o(Ie);{var ar=v=>{_r(v,{label:"PKCE",children:(O,_e)=>{Mr(O,{get check(){return e(t).use_pkce}})},$$slots:{default:!0}})},ir=v=>{le(v,{ariaLabel:"PKCE",get checked(){return e(t).use_pkce},set checked(O){e(t).use_pkce=O},children:(O,_e)=>{F();var vr=G("PKCE");c(O,vr)},$$slots:{default:!0}})};D(Je,v=>{e(h)?v(ar):v(ir,!1)})}l(Ie);var Ae=a(Ie,2),Qe=o(Ae,!0);l(Ae);var De=a(Ae,2);j(De,{name:"scope",label:"Scope",placeholder:"openid profile email",width:s,required:!0,pattern:Pr,get value(){return e(t).scope},set value(v){e(t).scope=v}});var je=a(De,2),or=o(je,!0);l(je);var We=a(je,2);j(We,{name:"client_name",label:"Client Name",placeholder:"Client Name",width:s,required:!0,pattern:Nr,get value(){return e(t).name},set value(v){e(t).name=v}});var i=a(We,2),T=o(i,!0);l(i);var L=a(i,2);j(L,{name:"client_id",autocomplete:"off",label:"Client ID",placeholder:"Client ID",width:s,required:!0,pattern:B,get value(){return e(t).client_id},set value(v){e(t).client_id=v}});var me=a(L,2),gr=o(me,!0);l(me);var Fe=a(me,2),qr=de(()=>!e(t).use_pkce);Lr(Fe,{name:"client_secret",autocomplete:"off",label:"Client Secret",placeholder:"Client Secret",width:s,get errMsg(){return d.providers.config.errConfidential},maxLength:256,get required(){return e(qr)},get value(){return e(t).client_secret},set value(v){e(t).client_secret=v}});var lr=a(Fe,2),Rr=o(lr);ye(Rr,()=>d.providers.config.descAuthMethod),l(lr);var sr=a(lr,2),Sr=o(sr);le(Sr,{ariaLabel:"client_secret_basic",get checked(){return e(t).client_secret_basic},set checked(v){e(t).client_secret_basic=v},children:(v,O)=>{F();var _e=G("client_secret_basic");c(v,_e)},$$slots:{default:!0}}),l(sr);var nr=a(sr,2),xr=o(nr);le(xr,{ariaLabel:"client_secret_post",get checked(){return e(t).client_secret_post},set checked(v){e(t).client_secret_post=v},children:(v,O)=>{F();var _e=G("client_secret_post");c(v,_e)},$$slots:{default:!0}}),l(nr);var br=a(nr,2);{var zr=v=>{var O=jt(),_e=o(O,!0);l(O),C(()=>u(_e,d.providers.config.errNoAuthMethod)),Ze(3,O,()=>er,()=>({duration:150})),c(v,O)};D(br,v=>{!e(t).use_pkce&&!e(t).client_secret_basic&&!e(t).client_secret_post&&v(zr)})}var kr=a(br,2);Ur(kr,{});var cr=a(kr,2),Dr=o(cr,!0);l(cr);var wr=a(cr,2);j(wr,{name:"admin_claim_path",get label(){return d.providers.config.pathAdminClaim},placeholder:"$.roles.*",width:s,pattern:B,get value(){return e(t).admin_claim_path},set value(v){e(t).admin_claim_path=v}});var Cr=a(wr,2);j(Cr,{name:"admin_claim_value",get label(){return d.providers.config.valueAdminClaim},placeholder:"rauthy_admin",width:s,pattern:B,get value(){return e(t).admin_claim_value},set value(v){e(t).admin_claim_value=v}});var dr=a(Cr,2),jr=o(dr,!0);l(dr);var $r=a(dr,2);j($r,{name:"mfa_claim_path",get label(){return d.providers.config.pathMfaClaim},placeholder:"$.amr.*",width:s,pattern:B,get value(){return e(t).mfa_claim_path},set value(v){e(t).mfa_claim_path=v}});var Fr=a($r,2);j(Fr,{name:"mfa_claim_value",get label(){return d.providers.config.valueMfaClaim},placeholder:"mfa",width:s,pattern:B,get value(){return e(t).mfa_claim_value},set value(v){e(t).mfa_claim_value=v}}),C(()=>{u(Qe,d.providers.config.descScope),u(or,d.providers.config.descClientName),u(T,d.providers.config.descClientId),u(gr,d.providers.config.descClientId),u(Dr,d.providers.config.mapUser),u(jr,d.providers.config.mapMfa)}),c(ie,ce)};D(Z,ie=>{(e(b)||e(f)||e(h))&&ie(ae)},!0)}c(ne,fe)};D(te,ne=>{e(y)&&!e(h)?ne(ve):ne(Re,!1)},!0)}c(M,K)};D(ke,M=>{e(A)&&!e(h)?M(we):M(Ce,!1)})}var Ye=a(ke,2),Me=o(Ye);Ve(Me,{type:"submit",get isLoading(){return e(m)},children:(M,K)=>{F();var te=G();C(()=>u(te,e(q)?d.providers.config.lookup:r.common.save)),c(M,te)},$$slots:{default:!0}});var Te=a(Me,2);{var rr=M=>{Ve(M,{level:3,onclick:W,get isLoading(){return e(m)},children:(K,te)=>{F();var ve=G();C(()=>u(ve,d.common.reset)),c(K,ve)},$$slots:{default:!0}})};D(Te,M=>{e(q)||M(rr)})}var Ue=a(Te,2);{var qe=M=>{Tr(M,{})};D(Ue,M=>{e(U)&&M(qe)})}var tr=a(Ue,2);{var He=M=>{var K=Gt(),te=o(K,!0);l(K),C(()=>u(te,e($))),c(M,K)};D(tr,M=>{e($)&&M(He)})}l(Ye),c(z,g)},$$slots:{default:!0}}),l(H),c(V,H),ge()}var Ht=E("<div></div> <!>",1),Kt=E('<div class="err"> </div>'),Ot=E('<div id="federation"><!></div> <!>',1),Jt=E("<!> <!>",1);function Qt(V,p){he(p,!0);let r=Ee(),d=S(void 0),s=Zr("pid"),m=S(""),$=S(k([])),h=S(void 0);yr(()=>{U()}),Ne(()=>{n(h,k(e($).find(_=>_.id===s.get())))});async function U(){var y;let _=await pr("/auth/v1/providers");_.body?n($,k(_.body)):n(m,k(((y=_.error)==null?void 0:y.message)||"Error"))}function N(){var _;s.set(void 0),(_=e(d))==null||_(),U()}var w=Jt(),t=ee(w);Xr(t,{width:"11rem",buttonTilesAriaControls:"federation",buttonTiles:_=>{var y=Ht(),f=ee(y);Be(f,"height",".5rem");var A=a(f,2);Er(A,17,()=>e($),b=>b.id,(b,q)=>{var X=de(()=>s.get()===e(q).id);et(b,{onclick:()=>s.set(e(q).id),get selected(){return e(X)},children:(J,P)=>{F();var I=G();C(()=>u(I,e(q).name)),c(J,I)},$$slots:{default:!0}})}),c(_,y)},children:(_,y)=>{var f=de(()=>e($).length===0?1:2);rt(_,{get level(){return e(f)},get closeModal(){return e(d)},set closeModal(A){n(d,k(A))},children:(A,b)=>{Yt(A,{onSave:N})},$$slots:{default:!0}})},$$slots:{buttonTiles:!0,default:!0}});var x=a(t,2);Wr(x,{children:(_,y)=>{var f=Ot(),A=ee(f),b=o(A);{var q=P=>{Rt(P,{onSave:U,get provider(){return e(h)},set provider(I){n(h,k(I))}})};D(b,P=>{e(h)&&P(q)})}l(A);var X=a(A,2);{var J=P=>{var I=Kt(),R=o(I,!0);l(I),C(()=>u(R,e(m))),c(P,I)};D(X,P=>{e(m)&&P(J)})}C(()=>re(A,"aria-label",r.common.details)),c(_,f)},$$slots:{default:!0}}),c(V,w),ge()}function Wt(V){Qt(V,{})}export{Wt as component};
