import{w as ea,x as Q,y as _e,F as aa,I as $e,aH as ta,aN as sa,aM as oa,A as ia,ar as ra,Y as la,aY as na,d as Ie,W as ie,V as va,ac as ca,a2 as m,a4 as u,a1 as y,g as l,p as J,o as Ee,j as R,aZ as da,n as re,a0 as _,a as ee,Z as G,_ as le,h as ae,a3 as ua}from"./index-client.LkyNpeLM.js";import{l as Te,k as ma,w as Ce,d as K,s as I,e as j}from"./render.D21ACvZ4.js";import{d as Se,e as fa,a as k,b as C,t as Ae,c as Ne,n as pa}from"./disclose-version.C0dpEWi_.js";import{e as De,i as ba}from"./OptionSelect.CHnFxA-i.js";import{s as M,r as Re,l as xa,t as Ue,f as ha,a as wa}from"./Loading.CZYfBo5y.js";import{p as E,s as ne,b as ka,a as ve}from"./props.BDzDHVdA.js";import{x as je,v as ga,a4 as ya,O as Me,a5 as _a,a6 as $a,a7 as Ia}from"./dataFetching.C4rEqQY1.js";import{r as ce}from"./legacy-client.yBTdRgtG.js";import{i as L}from"./if.BQqAvi9S.js";import{p as Y}from"./proxy.bVSoDcUp.js";import{s as de}from"./snippet.C6oh0jnA.js";import{b as Oe}from"./this.BhcZdidn.js";import{w as Ea}from"./index.DFcE2Cdu.js";import{i as Xe}from"./tweened._kLXMt8j.js";import{I as Ta}from"./Input.BWHohn37.js";import{I as Ca,a as Sa}from"./CheckIcon.D95u0ggr.js";import{u as Aa}from"./i18n.svelte.DPkpATkG.js";function He(e,t,o,a,i){var s=e,n="",c;ea(()=>{if(n===(n=t()??"")){Q&&_e();return}c!==void 0&&(la(c),c=void 0),n!==""&&(c=aa(()=>{if(Q){$e.data;for(var v=_e(),f=v;v!==null&&(v.nodeType!==8||v.data!=="");)f=v,v=ta(v);if(v===null)throw sa(),oa;Se($e,f),s=ia(v);return}var $=n+"",h=fa($);Se(ra(h),h.lastChild),s.before(h)}))})}function Na(e,t,o=t){var a=na();Te(e,"input",i=>{var s=i?e.defaultValue:e.value;if(s=ue(e)?me(s):s,o(s),a&&s!==(s=t())){var n=e.selectionStart,c=e.selectionEnd;e.value=s??"",c!==null&&(e.selectionStart=n,e.selectionEnd=Math.min(c,e.value.length))}}),(Q&&e.defaultValue!==e.value||Ie(t)==null&&e.value)&&o(ue(e)?me(e.value):e.value),ie(()=>{var i=t();ue(e)&&i===me(e.value)||e.type==="date"&&!i&&!e.value||i!==e.value&&(e.value=i??"")})}function Da(e,t,o=t){Te(e,"change",a=>{var i=a?e.defaultChecked:e.checked;o(i)}),(Q&&e.defaultChecked!==e.checked||Ie(t)==null)&&o(e.checked),ie(()=>{var a=t();e.checked=!!a})}function ue(e){var t=e.type;return t==="number"||t==="range"}function me(e){return e===""?null:+e}function Ra(e,t,o=t){var a=e==="x",i=()=>Ce(()=>{s=!0,clearTimeout(n),n=setTimeout(c,100),o(window[a?"scrollX":"scrollY"])});addEventListener("scroll",i,{passive:!0});var s=!1,n,c=()=>{s=!1},v=!0;ie(()=>{var f=t();v?v=!1:!s&&f!=null&&(s=!0,clearTimeout(n),scrollTo(window.scrollX,f),n=setTimeout(c,100))}),va(i),ca(()=>{removeEventListener("scroll",i)})}function Ua(e,t){ma(window,["resize"],()=>Ce(()=>t(window[e])))}var ja=(e,t,o)=>t(l(o)),Ma=C('<button class="btn font-label svelte-n2aryx"> </button>'),Oa=C('<div class="bar noselect svelte-n2aryx"></div>');function Xa(e,t){let o=E(t,"labels",19,()=>[]),a=E(t,"selected",15);var i=Oa();De(i,21,o,ba,(s,n)=>{var c=Ma();c.__click=[ja,a,n];var v=m(c,!0);u(c),y(()=>{M(c,"data-selected",a()===l(n)),I(v,l(n))}),j("keypress",c,()=>a(l(n))),k(s,c)}),u(i),k(e,i)}K(["click"]);function fe(e,t,o,a){if(typeof o=="number"||Xe(o)){const i=a-o,s=(o-t)/(e.dt||1/60),n=e.opts.stiffness*i,c=e.opts.damping*s,v=(n-c)*e.inv_mass,f=(s+v)*e.dt;return Math.abs(f)<e.opts.precision&&Math.abs(i)<e.opts.precision?a:(e.settled=!1,Xe(o)?new Date(o.getTime()+f):o+f)}else{if(Array.isArray(o))return o.map((i,s)=>fe(e,t[s],o[s],a[s]));if(typeof o=="object"){const i={};for(const s in o)i[s]=fe(e,t[s],o[s],a[s]);return i}else throw new Error(`Cannot spring ${typeof o} values`)}}function pe(e,t={}){const o=Ea(e),{stiffness:a=.15,damping:i=.8,precision:s=.01}=t;let n,c,v,f=e,$=e,h=1,N=0,A=!1;function d(b,p={}){$=b;const x=v={};return e==null||p.hard||r.stiffness>=1&&r.damping>=1?(A=!0,n=Re.now(),f=b,o.set(e=$),Promise.resolve()):(p.soft&&(N=1/((p.soft===!0?.5:+p.soft)*60),h=0),c||(n=Re.now(),A=!1,c=xa(w=>{if(A)return A=!1,c=null,!1;h=Math.min(h+N,1);const D={inv_mass:h,opts:r,settled:!0,dt:(w-n)*60/1e3},O=fe(D,f,e,$);return n=w,f=e,o.set(e=O),D.settled&&(c=null),!D.settled})),new Promise(w=>{c.promise.then(()=>{x===v&&w()})}))}const r={set:d,update:(b,p)=>d(b($,e),p),subscribe:o.subscribe,stiffness:a,damping:i,precision:s};return r}var Ha=C('<div class="tooltip svelte-op12yu"><!></div>'),Pa=C('<div role="none"><!> <!></div>');function be(e,t){J(t,!0);const o=ne(),a=()=>ve(l(f),"$coords",o);let i=E(t,"xOffset",3,10),s=E(t,"yOffset",3,10),n=E(t,"text",3,""),c=E(t,"html",3,""),v=G(!1),f=G(void 0),$;Ee(()=>{ka(R(f,Y(pe({x:(window==null?void 0:window.innerWidth)/2,y:window.innerHeight/2},{stiffness:.1,damping:.6}))),"$coords",o)}),da(()=>{clearTimeout($)});function h(){clearTimeout($),R(v,!0)}function N(){$=setTimeout(()=>{R(v,!1)},100)}function A(x){l(f).set({x:x.clientX+i(),y:x.clientY+s()})}var d=Pa();d.__mouseover=h,d.__mouseout=N,d.__mousemove=A;var r=m(d);de(r,()=>t.children??re);var b=_(r,2);{var p=x=>{var w=Ha(),D=m(w);{var O=S=>{var g=Ae();y(()=>I(g,n())),k(S,g)},Z=S=>{var g=Ne(),U=le(g);{var q=P=>{var V=Ne(),z=le(V);He(z,c),k(P,V)};L(U,P=>{c()&&P(q)},!0)}k(S,g)};L(D,S=>{n()?S(O):S(Z,!1)})}u(w),y(()=>M(w,"style",`top: ${`${a().y}px`??""}; left: ${`${a().x}px`??""}`)),Ue(7,w,()=>ha,()=>({delay:400,duration:200})),k(x,w)};L(b,x=>{l(v)&&x(p)})}u(d),j("focus",d,h),j("blur",d,N),k(e,d),ee()}K(["mouseover","mouseout","mousemove"]);var Ya=C('<div role="switch" tabindex="0" class="outer svelte-1mo9pov"><div class="inner svelte-1mo9pov"></div></div>');function Va(e,t){J(t,!0);const o=ne(),a=()=>ve(n,"$margin",o);let i=E(t,"selected",15,!1);const s=11;let n=pe(0,{stiffness:.15,damping:.5});ce(()=>{n.set(i()?s:0)});function c(){i(!i())}var v=Ya();v.__click=c;var f=m(v);u(v),y(()=>{M(v,"aria-checked",i()),je(v,"selectedOuter",i()),M(f,"style",`margin-left: ${`${a()}px`??""}`),je(f,"selected",i())}),j("keypress",v,c),k(e,v),ee()}K(["click"]);var Wa=pa('<svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5"></path></svg>');function Pe(e,t){let o=E(t,"color",3,"var(--col-text)"),a=E(t,"opacity",3,.9),i=E(t,"width",3,20);var s=Wa();M(s,"stroke-width",2),y(()=>{M(s,"width",i()),M(s,"color",o()),M(s,"opacity",a())}),k(e,s)}var Ga=C('<div class="body svelte-1xshgki"><!></div>'),La=C('<div class="container svelte-1xshgki"><div class="containerHeader svelte-1xshgki"><div role="button" tabindex="0" class="expand svelte-1xshgki"><div><!></div></div> <div class="header svelte-1xshgki"><!></div></div> <!></div>');function Ye(e,t){J(t,!0);const o=ne(),a=()=>ve(h,"$rotate",o);let i=E(t,"idx",11,0),s=E(t,"show",15),n=E(t,"expandedCallback",3,()=>{}),c=G(!1),v=G(void 0),f=G(void 0),$=i()%2===0?"2px solid var(--col-acnt)":"2px solid var(--col-acnta)";const h=pe(0,{stiffness:.1,damping:.4});function N(){!s()&&n()&&n()(),s(!s())}function A(){const g=l(v).offsetTop+l(f).scrollHeight+100;window.innerHeight+window.scrollY-g<0&&window.scroll({top:g,behavior:"smooth"})}ce(()=>{s()?h.set(90):h.set(0)}),ce(()=>{s()&&l(f)&&setTimeout(()=>{A()},100)});var d=La(),r=m(d),b=m(r);b.__click=N;var p=m(b),x=m(p),w=ae(()=>l(c)?"var(--col-act2a)":"var(--col-act2)");Pe(x,{get color(){return l(w)}}),u(p),u(b);var D=_(b,2),O=m(D);de(O,()=>t.header??re),u(D),u(r);var Z=_(r,2);{var S=g=>{var U=Ga(),q=m(U);de(q,()=>t.body??re),u(U),Oe(U,P=>R(f,P),()=>l(f)),Ue(7,U,()=>wa,()=>({duration:200})),k(g,U)};L(Z,g=>{s()&&g(S)})}u(d),Oe(d,g=>R(v,g),()=>l(v)),y(()=>{ga(d,"border-left",s()?$:"none"),M(p,"style",`rotate: ${a()??""}deg`)}),j("mouseenter",b,()=>R(c,!0)),j("mouseleave",b,()=>R(c,!1)),j("keypress",b,N),k(e,d),ee()}K(["click"]);var Za=C('<div class="device-header svelte-oumbix"><div class="device-head font-mono svelte-oumbix"> </div></div>'),qa=(e,t,o)=>t(l(o).id),za=C('<div role="button" tabindex="0" class="icon-btn-input svelte-oumbix"><!></div>'),Ba=(e,t,o)=>t(l(o).id),Fa=C('<div role="button" tabindex="0" class="icon-btn-value svelte-oumbix"><!></div>'),Ka=C('<div class="unit svelte-oumbix"><div class="label font-label svelte-oumbix"> </div> <div class="row svelte-oumbix"><div class="value svelte-oumbix"> </div> <!></div></div>'),Qa=C('<div class="device svelte-oumbix"><div class="unit svelte-oumbix"><div class="label font-label svelte-oumbix"> </div> <div class="value font-mono svelte-oumbix"> </div></div> <div class="row svelte-oumbix"><!> <!></div> <div class="unit svelte-oumbix"><div class="label font-label svelte-oumbix"> </div> <div class="value svelte-oumbix"> </div></div> <div class="unit svelte-oumbix"><div class="label font-label svelte-oumbix"> </div> <div class="value svelte-oumbix"> </div></div> <!> <div class="unit svelte-oumbix"><div class="label font-label svelte-oumbix"> </div> <div class="value svelte-oumbix"> </div></div></div>'),Ja=C('<div class="head svelte-oumbix"> </div> <div class="devices svelte-oumbix"></div>',1);function et(e,t){J(t,!0);let o=E(t,"userId",3,""),a=Aa(),i=G(Y([])),s=Y({}),n=Y({});Ee(()=>{c()});async function c(){let d=await ya(o()),r=await d.json();if(d.ok){for(let b of r)n[b.id]=b.name;R(i,Y(r))}else console.error("error fetching devices: "+r.message)}async function v(d){let r=n[d];if($a.test(r))s[d]="";else{s[d]=(a==null?void 0:a.common.invalidInput)||"Invalid Input";return}let b={device_id:d,name:r},p=await _a(o(),b);if(p.ok){s[d]="";for(let x of l(i))if(x.id===d){x.name=r;break}R(i,Y([...l(i)])),n[d]=r}else{let x=await p.json();console.error(x)}}async function f(d){let r={device_id:d},b=await Ia(o(),r);if(b.ok){for(let p of l(i))if(p.id===d){p.refresh_exp=void 0;break}R(i,Y([...l(i)]))}else{let p=await b.json();console.error(p)}}var $=Ja(),h=le($),N=m(h,!0);u(h);var A=_(h,2);De(A,21,()=>l(i),d=>d.id,(d,r)=>{Ye(d,{header:b=>{var p=Za(),x=m(p),w=m(x,!0);u(x),u(p),y(()=>I(w,l(r).name)),k(b,p)},body:b=>{var p=Qa(),x=m(p),w=m(x),D=m(w,!0);y(()=>I(D,(a==null?void 0:a.account.deviceId.toUpperCase())||"ID")),u(w);var O=_(w,2),Z=m(O,!0);u(O),u(x);var S=_(x,2),g=m(S),U=ae(()=>(a==null?void 0:a.account.deviceName.toUpperCase())||"Name");Ta(g,{autocomplete:"off",get placeholder(){return l(U)},get value(){return n[l(r).id]},set value(T){n[l(r).id]=T},get error(){return s[l(r).id]},set error(T){s[l(r).id]=T},$$events:{enter:()=>v(l(r).id)},children:(T,W)=>{ua();var X=Ae();y(()=>I(X,(a==null?void 0:a.account.deviceName.toUpperCase())||"NAME")),k(T,X)},$$slots:{default:!0}});var q=_(g,2);{var P=T=>{var W=ae(()=>(a==null?void 0:a.common.save)||"Save");be(T,{get text(){return l(W)},children:(X,ye)=>{var H=za();H.__click=[qa,v,r];var B=m(H);Ca(B,{color:"var(--col-ok)",width:24}),u(H),j("keypress",H,()=>v(l(r).id)),k(X,H)},$$slots:{default:!0}})};L(q,T=>{n[l(r).id]!=l(r).name&&T(P)})}u(S);var V=_(S,2),z=m(V),Ve=m(z,!0);y(()=>I(Ve,(a==null?void 0:a.account.regDate.toUpperCase())||"REGISTRATION DATE")),u(z);var xe=_(z,2),We=m(xe,!0);u(xe),u(V);var te=_(V,2),se=m(te),Ge=m(se,!0);y(()=>I(Ge,(a==null?void 0:a.account.accessExp.toUpperCase())||"ACCESS EXPIRES")),u(se);var he=_(se,2),Le=m(he,!0);y(()=>I(Le,Me(l(r).access_exp))),u(he),u(te);var we=_(te,2);{var Ze=T=>{var W=Ka(),X=m(W),ye=m(X,!0);y(()=>I(ye,(a==null?void 0:a.account.accessRenew.toUpperCase())||"ACCESS RENEW UNTIL")),u(X);var H=_(X,2),B=m(H),Be=m(B,!0);y(()=>I(Be,Me(l(r).refresh_exp))),u(B);var Fe=_(B,2),Ke=ae(()=>(a==null?void 0:a.account.accessRenewDelete)||"Delete the possibility to renew");be(Fe,{get text(){return l(Ke)},children:(Qe,at)=>{var F=Fa();F.__click=[Ba,f,r];var Je=m(F);Sa(Je,{color:"var(--col-err)",width:24}),u(F),j("keypress",F,()=>f(l(r).id)),k(Qe,F)},$$slots:{default:!0}}),u(H),u(W),k(T,W)};L(we,T=>{l(r).refresh_exp&&T(Ze)})}var ke=_(we,2),oe=m(ke),qe=m(oe,!0);y(()=>I(qe,(a==null?void 0:a.account.regIp.toUpperCase())||"REGISTRATION FROM IP")),u(oe);var ge=_(oe,2),ze=m(ge,!0);u(ge),u(ke),u(p),y(()=>{I(Z,l(r).id),I(We,l(r).created),I(ze,l(r).peer_ip)}),k(b,p)},$$slots:{header:!0,body:!0}})}),u(A),y(()=>I(N,(a==null?void 0:a.account.devicesDesc)||"Devices linked to this account")),k(e,$),ee()}K(["click"]);export{et as D,Ye as E,Pe as I,Va as S,Xa as T,be as a,Na as b,Ua as c,Ra as d,Da as e,He as h};