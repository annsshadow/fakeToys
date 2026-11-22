var fs=require("fs");
var j=JSON.parse(fs.readFileSync(process.argv[2],"utf8"));
var d=j.data||j;
var s=d.skills||[];
var lv=d.levels||[];
var one=s.filter(function(x){return x.id===1||x.skill_id===1})[0]||s[0];
console.log(JSON.stringify({skills:s.length,levels:lv.length,skill1:one?{id:one.id||one.skill_id,name:one.name,base_damage:one.base_damage}:null}));