const fs=require('fs');
const fixes=[
  ['processplatform_service_processing','/jaxrs/processplatform/service/processing'],
];
for(const [crate,jb] of fixes){
  const p=`D:/WORKSPACE/fakeToys/oa4rust/crates/${crate}/src/lib.rs`;
  if(!fs.existsSync(p)) continue;
  let c=fs.readFileSync(p,'utf8');
  if(c.includes('JAVA_BASE')) continue;
  c=c.replace('pub fn router', 'pub const JAVA_BASE: &str = "${jb}";\npub fn router');
  fs.writeFileSync(p,c);
  console.log('Fixed: '+crate);
}
console.log('Done');
