import"./disclose-version.BDr9Qe-U.js";import{p as M,h as P,F as U,g as l,f as O,a as W,i as B,c as n,r as c,s as S,t as x}from"./runtime.BsghBUX9.js";import{s as E}from"./render.CTp9lacC.js";import{i as j}from"./if.6Ov_EDLa.js";import{e as T,a as g,t as z}from"./template.jENAUvUX.js";import{p as G}from"./proxy.YH42NwLs.js";import{o as Q}from"./index-client.ClP8y9aT.js";import{b as _,L as V}from"./fetch.BvIeF4JA.js";import{K as X,M as Y,p as Z,N as K}from"./helpers.CBl6QeM5.js";import{u as ee}from"./i18n.svelte.D8rR00qK.js";function re(i){let t=Y(i);const s={"+":"-","/":"_","=":""};return t.replace(/[+/=]/g,e=>s[e])}function R(i){if(typeof i=="string"){const t={"-":"+",_:"/",".":"="},s=i.replace(/[-_.]/g,e=>t[e]);return X(s)}else return i}async function ae(i,t,s,e){var k;let b={purpose:t},d=await _(`/auth/v1/users/${i}/webauthn/auth/start`,b);if(d.error)return console.error(d.error),{error:d.error.message||"Error starting the Authentication"};if(!d.body){let r="Did not receive a valid webauthn body";return console.error(r),{error:r}}let y=d.body,a=y.rcr;if(!a.publicKey){let r="no publicKey in challenge from the backend";return console.error(r),{error:r}}if(a.publicKey.challenge=R(a.publicKey.challenge),a.publicKey.allowCredentials)for(let r of a.publicKey.allowCredentials)r.id=R(r.id);const f=(y.exp-1)*1e3,w=new Date().getTime()+f;let o;try{const r=await Z(navigator.credentials.get(a),f);if(r)o=r;else return{error:s}}catch{return{error:new Date().getTime()>=w?e:s}}let h={code:y.code,data:{id:o.id,rawId:K(o.rawId),response:{authenticatorData:K(o.response.authenticatorData),clientDataJSON:K(o.response.clientDataJSON),signature:K(o.response.signature)},extensions:o.getClientExtensionResults(),type:o.type}},u=await _(`/auth/v1/users/${i}/webauthn/auth/finish`,h);return u.status===202?{data:u.body}:(console.error(u),{error:((k=u.error)==null?void 0:k.message)||"Authentication Error"})}var te=z('<div class="err svelte-jfkcym"> </div>'),se=z('<div class="good svelte-jfkcym"> </div>'),oe=z('<div class="wrapperOuter svelte-jfkcym"><div class="wrapperInner svelte-jfkcym"><div class="content svelte-jfkcym"><div class="contentRow svelte-jfkcym"><div class="contentHeader svelte-jfkcym"> </div></div> <div class="contentRow svelte-jfkcym"><div><!></div></div> <div class="contentRow svelte-jfkcym"><!></div></div></div></div>');function ie(i,t){M(t,!0);let s=ee(),e=B(void 0);Q(async()=>{P(e,G(await ae(t.userId,t.purpose,s.authorize.invalidKeyUsed,s.authorize.requestExpired)))}),U(()=>{l(e)&&(l(e).error?setTimeout(()=>{var a;t.onError(((a=l(e))==null?void 0:a.error)||"Webauthn Error")},3e3):t.onSuccess(l(e).data))});var b=T(),d=O(b);{var y=a=>{var f=oe(),w=n(f),o=n(w),h=n(o),u=n(h),k=n(u,!0);c(u),c(h);var r=S(h,2),A=n(r),N=n(A);{var $=v=>{V(v,{})};j(N,v=>{l(e)||v($)})}c(A),c(r);var C=S(r,2),q=n(C);{var F=v=>{var I=T(),H=O(I);{var J=m=>{var p=te(),D=n(p,!0);c(p),x(()=>E(D,l(e).error)),g(m,p)},L=m=>{var p=se(),D=n(p,!0);c(p),x(()=>E(D,s.authorize.mfaAck)),g(m,p)};j(H,m=>{l(e).error?m(J):m(L,!1)})}g(v,I)};j(q,v=>{l(e)&&v(F)})}c(C),c(o),c(w),c(f),x(()=>E(k,s.authorize.expectingPasskey)),g(a,f)};j(d,a=>{t.purpose&&a(y)})}g(i,b),W()}export{ie as W,re as a,R as b};
