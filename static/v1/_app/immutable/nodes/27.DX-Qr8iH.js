import{a as t,b as $,c as _,t as P}from"../chunks/disclose-version.C0dpEWi_.js";import{p as W,o as Y,a0 as z,_ as m,a as A,j as o,g as c,Z as b,n as U,$ as X,a2 as B,a3 as Z,a4 as F,a1 as q}from"../chunks/index-client.LkyNpeLM.js";import{h as D,s as I}from"../chunks/render.D21ACvZ4.js";import{i as k}from"../chunks/if.BQqAvi9S.js";import{p}from"../chunks/proxy.bVSoDcUp.js";import{g as J,W as K,X as Q,Y as S}from"../chunks/dataFetching.C4rEqQY1.js";import{W as V}from"../chunks/WebauthnRequest.CDgCRlgS.js";import{s as aa}from"../chunks/snippet.C6oh0jnA.js";import{L as ea}from"../chunks/LangSelector.PrFnGVsI.js";import{B as oa}from"../chunks/Button.BDI_bIrm.js";import{u as sa}from"../chunks/i18n.svelte.DPkpATkG.js";var ta=$("<div><h2>Cookies disabled</h2> <p>You need to enable Cookies.<br> Without them, a safe login cannot be executed.</p></div>"),ra=$("<noscript></noscript> <!>",1);function ia(j,g){W(g,!0);let x=b(!1),u=b(!1);Y(()=>{o(u,p(navigator.cookieEnabled)),o(x,!0)});var s=ra(),n=z(m(s),2);{var y=h=>{var a=_(),C=m(a);{var e=i=>{var l=_(),E=m(l);aa(E,()=>g.children??U),t(i,l)},r=i=>{var l=ta();t(i,l)};k(C,i=>{c(u)?i(e):i(r,!1)})}t(h,a)};k(n,h=>{c(x)&&h(y)})}t(j,s),A()}var na=$('<div class="btn flex-col svelte-1dkpwyo"><!></div>'),ca=$('<div class="error svelte-1dkpwyo"> </div>'),la=$("<!> <!>",1);function va(j,g){W(g,!0);let x=sa(),u=b(!1),s=b(""),n=b(void 0);Y(async()=>{const a=J();if(a.error){o(s,`${a.error}: ${a.error_description}`);return}let C={state:a.state,code:a.code,pkce_verifier:K(),xsrf_token:Q()},e=await S(C);if(e.status===202)window.location.replace(e.headers.get("location"));else if(e.status===200)o(s,""),o(n,p(await e.json()));else if(e.status===204)window.location.replace("/auth/v1/account");else if(e.status===403){let r=await e.json();console.error(r),o(s,p(r.message))}else if(e.status===406)o(s,p(x.authorize.clientForceMfa)),o(u,!0);else{let r=await e.text();o(s,`HTTP ${e.status}: ${r}`)}});function y(a){o(s,p(a||"ERROR")),o(n,void 0)}function h(a){a&&window.location.replace(a.loc)}D(a=>{X.title="Callback"}),ia(j,{children:(a,C)=>{var e=la(),r=m(e);{var i=v=>{V(v,{onSuccess:h,onError:y,get data(){return c(n)},set data(O){o(n,p(O))}})},l=v=>{var O=_(),G=m(O);{var H=d=>{var f=na(),T=B(f);oa(T,{$$events:{click:()=>window.location.href="/auth/v1/account"},children:(L,R)=>{Z();var w=P("ACCOUNT LOGIN");t(L,w)},$$slots:{default:!0}}),F(f),t(d,f)},M=d=>{var f=_(),T=m(f);{var L=R=>{var w=ca(),N=B(w,!0);F(w),q(()=>I(N,c(s))),t(R,w)};k(T,R=>{c(s)&&R(L)},!0)}t(d,f)};k(G,d=>{c(u)?d(H):d(M,!1)},!0)}t(v,O)};k(r,v=>{c(n)?v(i):v(l,!1)})}var E=z(r,2);ea(E,{absolute:!0}),t(a,e)},$$slots:{default:!0}}),A()}export{va as component};
