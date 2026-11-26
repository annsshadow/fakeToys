const fs=require("fs"),path=require("path");
const root=require("child_process").execSync("git rev-parse --show-toplevel").toString().trim();
const d=path.join(root,"docs/laiyipao/audits/round48-r12-growth-cta");
let files=[];
(function walk(p){for(const e of fs.readdirSync(p,{withFileTypes:true})){const q=path.join(p,e.name);if(e.isDirectory())walk(q);else if(/\.(md|js|sh|json|txt)$/.test(e.name))files.push(q);}})(d);
const pats=[/eyJ[A-Za-z0-9_-]{10,}/,/Bearer\s+[A-Za-z0-9._-]{20,}/,/postgres:\/\/[^:\s]+:[^@\s]+@/,/C:\\Users\\[A-Za-z0-9_.-]+/];
let hits=[];
for(const f of files){const s=fs.readFileSync(f,"utf8");for(const p of pats){if(p.test(s))hits.push(path.relative(root,f)+":"+String(p));}}
console.log("SCANNED="+files.length+" SECRET_HITS="+hits.length);
for(const h of hits)console.log("HIT="+h);