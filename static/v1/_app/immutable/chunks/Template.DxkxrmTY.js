import"./disclose-version.BDr9Qe-U.js";import{g as i,j as p,a6 as f,p as m,o as l,a as u}from"./index-client.C1uYfxHd.js";import{p as c}from"./props.OoymhrXq.js";import{p as g}from"./proxy.bZKuG6wn.js";import{f as y}from"./ThemeSwitch.B8t8VQsu.js";const d="false";let s=f(d==="true");function n(){return{get(){return i(s)},set(r){p(s,g(r))}}}function v(r,o){m(o,!0);let e=c(o,"value",15);l(async()=>{if(n().get()){let t=await y(`/auth/v1/template/${o.id}`);t.error?console.error(t.error):t.text&&a(t.text)}else{let t=document.getElementById(o.id);t&&a(t.innerHTML)}});function a(t){typeof e()=="boolean"?e(t==="true"):typeof e()=="string"?e(t):e(JSON.parse(t))}u()}export{v as T,n as u};
