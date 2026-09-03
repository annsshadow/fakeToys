const fs=require('fs');
const fixes=[
  ['calendar_assemble_control','/jaxrs/calendar_assemble_control'],
  ['component_assemble_control','/jaxrs/component_assemble_control'],
  ['organization_assemble_authentication','/jaxrs/organization_assemble_authentication'],
  ['organization_assemble_personal','/jaxrs/organization_assemble_personal'],
  ['processplatform_service_processing','/jaxrs/processplatform_service_processing'],
  ['ai_assemble_control','/jaxrs/ai_assemble_control'],
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
