const fs=require('fs');
const fixes=[
  ['organization_assemble_control','/jaxrs/organization/assemble/control'],
  ['processplatform_service_processing','/jaxrs/processplatform/service/processing'],
  ['jpush_assemble_control','/jaxrs/jpush/assemble/control'],
  ['query_service_processing','/jaxrs/query/service/processing'],
  ['portal_assemble_designer','/jaxrs/portal/assemble/designer'],
  ['portal_assemble_surface','/jaxrs/portal/assemble/surface'],
  ['file_assemble_control','/jaxrs/file/assemble/control'],
  ['cms_assemble_control','/jaxrs/cms/assemble/control'],
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
