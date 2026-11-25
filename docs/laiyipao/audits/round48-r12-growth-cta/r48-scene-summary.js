const fs=require('fs');
const d='docs/laiyipao/audits/round48-r12-growth-cta/data/';
function load(p){let v=JSON.parse(fs.readFileSync(d+p,'utf8'));if(typeof v==='string')v=JSON.parse(v);return v;}
function act(o,k){return (o[k]||[]).filter(x=>x.active).map(x=>x.text).join('|');}
const one=process.argv.indexOf('one');
const names=one>=0?[process.argv[one+1]]:['element','equipment','reaction'];
for(const n of names){
 const b=load(n+'-before.json');
 const a=fs.existsSync(d+n+'-after.json')?load(n+'-after.json'):null;
 console.log(n+' BEFORE hash='+b.hash+' cta='+b.ctaText+' prio='+b.priorityText+' tabs='+act(b,'tabDefs')+' gear='+act(b,'gearChips'));
 if(a){
  const mv=fs.existsSync(d+n+'-move.txt')?fs.readFileSync(d+n+'-move.txt','utf8').trim():'';
  const dn=fs.existsSync(d+n+'-down.txt')?fs.readFileSync(d+n+'-down.txt','utf8').trim():'';
  const up=fs.existsSync(d+n+'-up.txt')?fs.readFileSync(d+n+'-up.txt','utf8').trim():'';
  console.log(n+' AFTER  hash='+a.hash+' cta='+a.ctaText+' tabs='+act(a,'tabDefs')+' gear='+act(a,'gearChips')+' inner='+a.innerW+'/'+a.docW+' click=['+mv+'|'+dn+'|'+up+']');
 }
}
