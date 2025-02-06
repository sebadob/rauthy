const M={"001":1,AD:1,AE:6,AF:6,AI:1,AL:1,AM:1,AN:1,AR:1,AT:1,AU:1,AX:1,AZ:1,BA:1,BE:1,BG:1,BH:6,BM:1,BN:1,BY:1,CH:1,CL:1,CM:1,CN:1,CR:1,CY:1,CZ:1,DE:1,DJ:6,DK:1,DZ:6,EC:1,EE:1,EG:6,ES:1,FI:1,FJ:1,FO:1,FR:1,GB:1,GE:1,GF:1,GP:1,GR:1,HR:1,HU:1,IE:1,IQ:6,IR:6,IS:1,IT:1,JO:6,KG:1,KW:6,KZ:1,LB:1,LI:1,LK:1,LT:1,LU:1,LV:1,LY:6,MC:1,MD:1,ME:1,MK:1,MN:1,MQ:1,MV:5,MY:1,NL:1,NO:1,NZ:1,OM:6,PL:1,QA:6,RE:1,RO:1,RS:1,RU:1,SD:6,SE:1,SI:1,SK:1,SM:1,SY:6,TJ:1,TM:1,TR:1,UA:1,UY:1,UZ:1,VA:1,VN:1,XK:1};function i(t,n,e){let a=t.calendar.toJulianDay(t),o=L(n),r=Math.ceil(a+1-o)%7;return r<0&&(r+=7),r}function y(t,n){return t.calendar.toJulianDay(t)-n.calendar.toJulianDay(n)}function h(t,n){return d(t)-d(n)}function d(t){return t.hour*36e5+t.minute*6e4+t.second*1e3+t.millisecond}let s=null;function E(){return s==null&&(s=new Intl.DateTimeFormat().resolvedOptions().timeZone),s}function m(t){return t.subtract({days:t.day-1})}function I(t,n,e){let a=i(t,n);return t.subtract({days:a})}const A=new Map;function D(t){if(Intl.Locale){let e=A.get(t);return e||(e=new Intl.Locale(t).maximize().region,e&&A.set(t,e)),e}let n=t.split("-")[1];return n==="u"?void 0:n}function L(t){let n=D(t);return n&&M[n]||0}const S={AF:[4,5],AE:[5,6],BH:[5,6],DZ:[5,6],EG:[5,6],IL:[5,6],IQ:[5,6],IR:[5,5],JO:[5,6],KW:[5,6],LY:[5,6],OM:[5,6],QA:[5,6],SA:[5,6],SD:[5,6],SY:[5,6],YE:[5,6]};function g(t,n){let e=t.calendar.toJulianDay(t),a=Math.ceil(e+1)%7;a<0&&(a+=7);let o=D(n),[r,u]=S[o]||[6,0];return a===r||a===u}function N(t){let n=Date.parse(t.split("T")[0]);if(!isNaN(n))return n/1e3}function R(t,n){let e=Date.parse(`${t}T${n}`);if(!isNaN(e))return e/1e3-new Date().getTimezoneOffset()}function T(t){return`${t.year}-${t.month>9?t.month:"0"+t.month}-${t.day>9?t.day:"0"+t.day}`}function $(t,n){const e=t.month;t=m(t),t=I(t,n);let a=[];const o=l=>{l.push({day:t.day,month:t.month,year:t.year,isWeekend:g(t,n)}),t=t.add({days:1})},r=()=>{let l={days:[]};for(let f=0;f<7;f++)o(l.days);a.push(l)};for(r();t.month===e;)r();let u=a[a.length-1],c=i(t,n);for(;c>0;)o(u.days),c=i(t,n);return a[a.length-1]=u,a}function p(t){const n=t||new Date;let e=n.getDate();e<10&&(e="0"+e);let a=n.getMonth()+1;return a<10&&(a="0"+a),`${n.getUTCFullYear()}-${a}-${e}`}function B(t){const n=new Date;let e=n.getHours();e<10&&(e="0"+e);let a=n.getMinutes();return a<10&&(a="0"+a),`${e}:${a}`}export{E as $,B as a,R as b,y as c,h as d,i as e,p as f,$ as g,T as h,N as u};
