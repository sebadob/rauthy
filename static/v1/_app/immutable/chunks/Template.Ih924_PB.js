import"./disclose-version.BP4T4xdV.js";import{a5 as a,p as i,a as m}from"./index-client.hxMyB_b4.js";import{p as n}from"./props.DNAkfByl.js";import{u as s}from"./is_dev.svelte.CWG7yUVe.js";import{f as p}from"./fetch.Bq59EV_b.js";function f(l,r){m(r,!0);let e=n(r,"value",15);a(async()=>{if(s().get()){let t=await p(`/auth/v1/template/${r.id}`);t.error?console.error(t.error):t.text&&o(t.text)}else{let t=document.getElementById(r.id);t&&o(t.innerHTML)}});function o(t){typeof e()=="boolean"?e(t==="true"):typeof e()=="string"?e(t):e(JSON.parse(t))}i()}export{f as T};
