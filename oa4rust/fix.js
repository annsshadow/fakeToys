const fs=require('fs');
const p='D:/WORKSPACE/fakeToys/oa4rust/crates/processplatform_assemble_surface/src/lib.rs';
let c=fs.readFileSync(p,'utf8');
c=c.replace('pub mod routes;', 'pub const JAVA_BASE: &str = "/jaxrs/processplatform/assemble/surface";\npub mod routes;');
fs.writeFileSync(p,c);
console.log('Done');
