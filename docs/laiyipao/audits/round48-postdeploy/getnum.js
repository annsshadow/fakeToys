// extract dotted path from double-encoded agent-browser eval JSON
const fs=require('fs');
let v=JSON.parse(fs.readFileSync(process.argv[2],'utf8'));
if(typeof v==='string') v=JSON.parse(v);
for(const k of process.argv[3].split('.')) v=v?.[k];
process.stdout.write(String(v??''));
