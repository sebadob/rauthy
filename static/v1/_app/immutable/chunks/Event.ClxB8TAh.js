import"./disclose-version.BDr9Qe-U.js";import{p as Z,v as z,t as n,c as B,r as I,s as h,a as G,w as H,u as F,f as v,Q as j,O as C,ap as q}from"./runtime.BONl2e88.js";import{s as u}from"./render.AqJWcVDb.js";import{i as b}from"./if.DS90EFHs.js";import{a as s,t as y,d as E,e as K}from"./template.B7ldVCvc.js";import{b as J,t as Q}from"./index.BJiFQ8Di.js";import{o as W}from"./index-client.Bjp_k6xO.js";import{z as L,O as X}from"./helpers.CQGRRstl.js";import{u as ee}from"./language.svelte.5-Elpi0x.js";import{A as te}from"./A.UEWOjmpJ.js";let se={common:{account:"Account",back:"Zur\xFCck",search:"Suchen",searchOptions:"Suchoptionen",until:"Bis"},error:{needsAdminRole:"Um Zugriff zu erhalten ist die Rolle <b>rauthy_admin</b> notwendig.",noAdmin:`F\xFCr Rauthy Admin Accounts ist <b>MFA Pflicht.</b><br>
            Im <b>Account</b> kann ein Passkey hinterlegt und MFA aktiviert werden.<br>
            Danach muss ein Logout und neuer Login folgen`},nav:{apiKeys:"API Keys",attributes:"Attribute",blacklist:"Blacklist",clients:"Clients",config:"Konfiguration",docs:"Dokumentation",events:"Events",groups:"Gruppen",providers:"Provider",roles:"Rollen",scopes:"Scopes",sessions:"Sessions",users:"Benutzer"}},ae={common:{account:"Account",back:"Back",search:"Search",searchOptions:"Search Options",until:"Until"},error:{needsAdminRole:`You are not assigned to the <b>rauthy_admin</b> role.<br/>
            You do not have access to the admin panel.`,noAdmin:`A Rauthy admin account must have <b>MFA enabled.</b><br>
            Please navigate to your <b>account</b> and activate MFA.<br>
            Afterward, you need to do a logout and log back in.`},nav:{apiKeys:"API Keys",attributes:"Attributes",blacklist:"Blacklist",clients:"Clients",config:"Config",docs:"Docs",events:"Events",groups:"Groups",providers:"Providers",roles:"Roles",scopes:"Scopes",sessions:"Sessions",users:"Users"}},O;function T(){return O}function ne(){switch(ee()){case"de":O=se;break;default:O=ae;break}}const x=(k,e=j)=>{var f=K(),l=v(f);{var o=d=>{var r=oe(),R=B(r,!0);I(r),n(()=>u(R,e())),s(d,r)};b(l,d=>{e()&&d(o)})}s(k,f)};var oe=y('<div class="ip font-mono svelte-9ykd41"> </div>'),re=y(" <!>",1),ie=y(" <!>",1),ce=y(" <!>",1),ve=y('<div role="contentinfo" class="event svelte-9ykd41"><div class="ts svelte-9ykd41"> </div> <!> <!></div>');function ue(k,e){Z(e,!0);let f=T(),l=H(!1);W(()=>{new Date().getTime()-1e3*30<e.event.timestamp&&(setTimeout(()=>{z(l,!0)},100),setTimeout(()=>{z(l,!1)},2500))});var o=ve();const d=C(()=>`2px solid ${X(e.event.level)}`);n(()=>J(o,"border-left",F(d)));var r=B(o),R=B(r,!0);n(()=>u(R,L(e.event.timestamp/1e3))),I(r);var D=h(r,2);{var U=t=>{var a=E();n(()=>u(a,e.event.typ)),s(t,a)};b(D,t=>{e.event.typ!=="RauthyHealthy"&&e.event.typ!=="RauthyUnhealthy"&&e.event.typ!=="NewRauthyVersion"&&t(U)})}var _=h(D,2);{var N=t=>{var a=re(),g=v(a),w=h(g);x(w,()=>e.event.ip),n(()=>u(g,`${`: ${e.event.data}`??""} `)),s(t,a)},V=t=>{var a=K(),g=v(a);{var w=i=>{var A=C(()=>e.event.text||"");te(i,{get href(){return F(A)},target:"_blank",children:(S,M)=>{q();var $=E("NewRauthyVersion");s(S,$)},$$slots:{default:!0}})},Y=i=>{var A=K(),S=v(A);{var M=c=>{var m=ie(),p=v(m);n(()=>u(p,`${f.common.until??""}
        ${(e.event.data&&L(e.event.data))??""} `));var P=h(p);x(P,()=>e.event.ip),s(c,m)},$=c=>{var m=ce(),p=v(m),P=h(p);x(P,()=>e.event.ip),n(()=>u(p,`${e.event.text??""} `)),s(c,m)};b(S,c=>{e.event.typ==="IpBlacklisted"?c(M):c($,!1)},!0)}s(i,A)};b(g,i=>{e.event.typ==="NewRauthyVersion"?i(w):i(Y,!1)},!0)}s(t,a)};b(_,t=>{e.event.typ==="InvalidLogins"?t(N):t(V,!1)})}I(o),n(()=>Q(o,"highlight",F(l))),s(k,o),G()}export{ue as E,ne as i,T as u};
