const fs=require("node:fs");const d=JSON.parse(fs.readFileSync(process.argv[2],"utf8"));console.log(JSON.stringify({rating_weights:d.rating_weights,version:d.version,keys:Object.keys(d).length}));
