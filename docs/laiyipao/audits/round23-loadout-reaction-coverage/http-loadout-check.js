const {execFileSync}=require('child_process');
const http=require('http');
function req(method,path,token,body){return new Promise((resolve)=>{
  const data=body?JSON.stringify(body):null;
  const opts={host:'127.0.0.1',port:8080,path,method,headers:{'Content-Type':'application/json'}};
  if(token)opts.headers['Authorization']='Bearer '+token;
  if(data)opts.headers['Content-Length']=Buffer.byteLength(data);
  const r=http.request(opts,(res)=>{let s='';res.on('data',d=>s+=d);res.on('end',()=>resolve(s))});
  r.on('error',e=>resolve('ERR:'+e.message));if(data)r.write(data);r.end();
});}
(async()=>{for(const uid of [10,127,11,1]){
  const gt=execFileSync('docker',['exec','lyp-qa-pg','psql','-U','postgres','-d','laiyipao','-t','-A','-c','SELECT guest_token FROM users WHERE id='+uid],{encoding:'utf8'}).trim();
  const ar=await req('POST','/api/v1/auth/guest',null,{guest_token:gt});
  let token='';try{const j=JSON.parse(ar);token=(j.data&&(j.data.access_token||j.data.token))||j.access_token||''}catch(e){}
  const lo=await req('GET','/api/v1/me/loadout',token);
  console.log('UID='+uid+' LOADOUT='+lo);
}})();
