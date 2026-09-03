const fs=require('fs');
const p='D:/WORKSPACE/fakeToys/oa4rust/crates/organization_assemble_control/src/lib.rs';
let c=fs.readFileSync(p,'utf8');
if(!c.includes('JAVA_BASE')){
  c=c.replace('pub fn router', 'pub const JAVA_BASE: &str = "/jaxrs/organization/assemble/control";\npub fn router');
  fs.writeFileSync(p,c);
  console.log('Added JAVA_BASE to organization_assemble_control');
} else {
  console.log('JAVA_BASE already exists');
}
