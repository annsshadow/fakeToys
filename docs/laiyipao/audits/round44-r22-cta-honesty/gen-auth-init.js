let s='';process.stdin.on('data',d=>s+=d);process.stdin.on('end',()=>{
  try {
    const j=JSON.parse(s), d=j.data||j;
    if(!d.access_token){console.error('no access_token');process.exit(2);}
    const fs=require('fs');
    const esc=v=>JSON.stringify(String(v||''));
    const lines=[
      '// QA init: 本地回环，令牌由 /auth/guest 运行时生成，不入库真值。',
      "localStorage.setItem('lyp_token',"+esc(d.access_token)+');',
      "localStorage.setItem('lyp_refresh',"+esc(d.refresh_token||'')+');',
      "localStorage.setItem('lyp_guest',"+esc(d.guest_token||process.env.GT||'')+');',
      "localStorage.setItem('lyp_profile',"+esc(JSON.stringify({nickname:'QA',isGuest:true}))+');',
      "localStorage.setItem('lyp_battle_speed','8');"
    ];
    fs.writeFileSync(process.argv[2],lines.join('\n')+'\n');
    console.log('AUTH_INIT_BYTES='+fs.statSync(process.argv[2]).size);
  } catch(e){console.error('parse failed',e.message);process.exit(3);}
});
