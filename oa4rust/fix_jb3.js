const fs=require('fs');
const fixes=[
  ['organization_assemble_personal','/jaxrs/organization/assemble/personal'],
  ['organization_assemble_authentication','/jaxrs/organization/assemble/authentication'],
  ['processplatform_service_processing','/jaxrs/processplatform/service/processing'],
];
let fixed=0;
for(const [crate,jb] of fixes){
  const p=`D:/WORKSPACE/fakeToys/oa4rust/crates/${crate}/src/lib.rs`;
  if(!fs.existsSync(p)) continue;
  let c=fs.readFileSync(p,'utf8');
  if(!c.includes('JAVA_BASE')) continue;
  c=c.replace(/pub const JAVA_BASE: &str = "[^"]+"/, `pub const JAVA_BASE: &str = "${jb}"`);
  fs.writeFileSync(p,c);
  console.log('Fixed: '+crate+' -> '+jb);
  fixed++;
}
console.log('Total fixed: '+fixed);
