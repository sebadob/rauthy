import{a as K,b as P}from"./disclose-version.C0dpEWi_.js";import{aq as de,w as pe,x as A,A as z,y as ce,T as xe,au as Q,H as he,z as U,B,I as O,av as I,C as W,F as Z,G as me,v as $,aw as H,ax as _e,ay as be,az as ge,Y as we,ar as ye,aA as Se,aB as Ae,X as ke,aC as D,aD as F,m as Ce,N as E,aE as ee,aF as Oe,aG as qe,aH as Ge,V as ae,a1 as te,a2 as Ie,a4 as ne,g as M}from"./index-client.LkyNpeLM.js";import{l as Me,s as Ne}from"./render.D21ACvZ4.js";import{v as Te}from"./dataFetching.C4rEqQY1.js";import{i as ze}from"./proxy.bVSoDcUp.js";import{p as L}from"./props.BDzDHVdA.js";function le(a,e){return e}function Be(a,e,t,n){for(var l=[],s=e.length,o=0;o<s;o++)_e(e[o].e,l,!0);var c=s>0&&l.length===0&&t!==null;if(c){var d=t.parentNode;be(d),d.append(t),n.clear(),S(a,e[0].prev,e[s-1].next)}ge(l,()=>{for(var h=0;h<s;h++){var p=e[h];c||(n.delete(p.k),S(a,p.prev,p.next)),we(p.e,!c)}})}function se(a,e,t,n,l,s=null){var o=a,c={flags:e,items:new Map,first:null},d=(e&ee)!==0;if(d){var h=a;o=A?z(ye(h)):h.appendChild(de())}A&&ce();var p=null,g=!1;pe(()=>{var w=t(),r=xe(w)?w:w==null?[]:Q(w),f=r.length;if(g&&f===0)return;g=f===0;let v=!1;if(A){var k=o.data===he;k!==(f===0)&&(o=U(),z(o),B(!1),v=!0)}if(A){for(var x=null,m,_=0;_<f;_++){if(O.nodeType===8&&O.data===Se){o=O,v=!0,B(!1);break}var b=r[_],i=n(b,_);m=re(O,c,x,null,b,i,_,l,e),c.items.set(i,m),x=m}f>0&&z(U())}if(!A){var u=Ae;De(r,c,o,l,e,(u.f&I)!==0,n)}s!==null&&(f===0?p?W(p):p=Z(()=>s(o)):p!==null&&me(p,()=>{p=null})),v&&B(!0),t()}),A&&(o=O)}function De(a,e,t,n,l,s,o,c){var V,X,Y,j;var d=(l&Oe)!==0,h=(l&(D|F))!==0,p=a.length,g=e.items,w=e.first,r=w,f,v=null,k,x=[],m=[],_,b,i,u;if(d)for(u=0;u<p;u+=1)_=a[u],b=o(_,u),i=g.get(b),i!==void 0&&((V=i.a)==null||V.measure(),(k??(k=new Set)).add(i));for(u=0;u<p;u+=1){if(_=a[u],b=o(_,u),i=g.get(b),i===void 0){var ue=r?r.e.nodes_start:t;v=re(ue,e,v,v===null?e.first:v.next,_,b,u,n,l),g.set(b,v),x=[],m=[],r=v.next;continue}if(h&&Fe(i,_,u,l),i.e.f&I&&(W(i.e),d&&((X=i.a)==null||X.unfix(),(k??(k=new Set)).delete(i))),i!==r){if(f!==void 0&&f.has(i)){if(x.length<m.length){var G=m[0],y;v=G.prev;var R=x[0],N=x[x.length-1];for(y=0;y<x.length;y+=1)ie(x[y],G,t);for(y=0;y<m.length;y+=1)f.delete(m[y]);S(e,R.prev,N.next),S(e,v,R),S(e,N,G),r=G,v=N,u-=1,x=[],m=[]}else f.delete(i),ie(i,r,t),S(e,i.prev,i.next),S(e,i,v===null?e.first:v.next),S(e,v,i),v=i;continue}for(x=[],m=[];r!==null&&r.k!==b;)(s||!(r.e.f&I))&&(f??(f=new Set)).add(r),m.push(r),r=r.next;if(r===null)continue;i=r}x.push(i),v=i,r=i.next}if(r!==null||f!==void 0){for(var C=f===void 0?[]:Q(f);r!==null;)(s||!(r.e.f&I))&&C.push(r),r=r.next;var T=C.length;if(T>0){var ve=l&ee&&p===0?t:null;if(d){for(u=0;u<T;u+=1)(Y=C[u].a)==null||Y.measure();for(u=0;u<T;u+=1)(j=C[u].a)==null||j.fix()}Be(e,C,ve,g)}}d&&ke(()=>{var J;if(k!==void 0)for(i of k)(J=i.a)==null||J.apply()}),$.first=e.first&&e.first.e,$.last=v&&v.e}function Fe(a,e,t,n){n&D&&H(a.v,e),n&F?H(a.i,t):a.i=t}function re(a,e,t,n,l,s,o,c,d,h){var p=(d&D)!==0,g=(d&qe)===0,w=p?g?Ce(l):E(l):l,r=d&F?E(o):o,f={i:r,v:w,k:s,a:null,e:null,prev:t,next:n};try{return f.e=Z(()=>c(a,w,r),A),f.e.prev=t&&t.e,f.e.next=n&&n.e,t===null?e.first=f:(t.next=f,t.e.next=f.e),n!==null&&(n.prev=f,n.e.prev=f.e),f}finally{}}function ie(a,e,t){for(var n=a.next?a.next.e.nodes_start:t,l=e?e.e.nodes_start:t,s=a.e.nodes_start;s!==n;){var o=Ge(s);l.before(s),s=o}}function S(a,e,t){e===null?a.first=t:(e.next=t,e.e.next=t&&t.e),t!==null&&(t.prev=e,t.e.prev=e&&e.e)}function oe(a,e,t){if(a.multiple)return Re(a,e);for(var n of a.options){var l=q(n);if(ze(l,e)){n.selected=!0;return}}(!t||e!==void 0)&&(a.selectedIndex=-1)}function Le(a,e){ae(()=>{var t=new MutationObserver(()=>{var n=a.__value;oe(a,n)});return t.observe(a,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),()=>{t.disconnect()}})}function fe(a,e,t=e){var n=!0;Me(a,"change",l=>{var s=l?"[selected]":":checked",o;if(a.multiple)o=[].map.call(a.querySelectorAll(s),q);else{var c=a.querySelector(s)??a.querySelector("option:not([disabled])");o=c&&q(c)}t(o)}),ae(()=>{var l=e();if(oe(a,l,n),n&&l===void 0){var s=a.querySelector(":checked");s!==null&&(l=q(s),t(l))}a.__value=l,n=!1}),Le(a)}function Re(a,e){for(var t of a.options)t.selected=~e.indexOf(q(t))}function q(a){return"__value"in a?a.__value:a.value}var Ve=P('<option class="opt svelte-ysgm66"> </option>'),Xe=P('<select class="svelte-ysgm66"></select>');function Ye(a,e){let t=L(e,"value",15),n=L(e,"options",19,()=>[]),l=L(e,"width",3,"inherit");var s=Xe();se(s,21,n,le,(o,c)=>{var d=Ve(),h={},p=Ie(d,!0);ne(d),te(()=>{h!==(h=M(c))&&(d.value=(d.__value=M(c))==null?"":M(c)),Ne(p,M(c))}),K(o,d)}),ne(s),te(()=>Te(s,"width",l())),fe(s,t),K(a,s)}export{Ye as O,fe as b,se as e,le as i};