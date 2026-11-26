const r=await fetch('http://127.0.0.1:8080/api/v1/config');
const j=await r.json(); const d=j.data||j;
const sk=[...(d.skills||[]),...(d.composite||[])].filter(x=>x.kind==='active'&&x.unlock_level<=3).map(x=>({id:x.id,name:x.name,dmg:x.base_damage,unlock:x.unlock_level}));
const eq=(d.equipment||[]).filter(x=>x.unlock_level<=3).map(x=>({id:x.id,slot:x.slot,bonus:x.base_bonus_pct,unlock:x.unlock_level}));
console.log('ACTIVE_SKILLS='+JSON.stringify(sk));
console.log('EQUIP='+JSON.stringify(eq));
console.log('TOPKEYS='+Object.keys(d).join(','));
