const fs=require("fs");
const p=process.argv[2];
const lines=fs.readFileSync(p,"utf8").trim().split("\n").map(x=>JSON.parse(JSON.parse(x)));
const seen=new Map();
for(const s of lines){if(!s.hasBrief||!s.head)continue;const h=s.head;if(!seen.has(h))seen.set(h,s);}
for(const [h,s] of seen){
  console.log(h+" | waveNow="+s.waveNow+" | rows="+s.rows.map(r=>r.text).join(",")+" | ai="+s.aiConcl+" | unknown="+s.unknownTags+" | overflow="+s.docOverflowX+" | undef="+s.hasUndefined+"/"+s.hasNaN+" | cards="+s.cardCount);
}
console.log("TOTAL="+lines.length+" UNIQUE_HEADS="+seen.size);