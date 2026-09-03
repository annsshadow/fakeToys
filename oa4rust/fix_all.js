const fs=require('fs');
const crates = [
  ['ai_assemble_control', '/jaxrs/ai/assemble/control'],
  ['attendance_assemble_control', '/jaxrs/attendance/assemble/control'],
  ['calendar_assemble_control', '/jaxrs/calendar/assemble/control'],
  ['component_assemble_control', '/jaxrs/component/assemble/control'],
  ['correlation_service_processing', '/jaxrs/correlation/service/processing'],
  ['general_assemble_control', '/jaxrs/general/assemble/control'],
  ['hotpic_assemble_control', '/jaxrs/hotpic/assemble/control'],
  ['jpush_assemble_control', '/jaxrs/jpush/assemble/control'],
  ['meeting_assemble_control', '/jaxrs/meeting/assemble/control'],
  ['message_assemble_communicate', '/jaxrs/message/assemble/communicate'],
  ['mind_assemble_control', '/jaxrs/mind/assemble/control'],
  ['organization_assemble_authentication', '/jaxrs/organization/assemble/authentication'],
  ['organization_assemble_personal', '/jaxrs/organization/assemble/personal'],
  ['portal_assemble_designer', '/jaxrs/portal/assemble/designer'],
  ['portal_assemble_surface', '/jaxrs/portal/assemble/surface'],
  ['processplatform_assemble_bam', '/jaxrs/processplatform/assemble/bam'],
  ['processplatform_assemble_designer', '/jaxrs/processplatform/assemble/designer'],
  ['query_assemble_designer', '/jaxrs/query/assemble/designer'],
  ['query_service_processing', '/jaxrs/query/service/processing'],
];
let fixed = 0;
for (const [crate, jb] of crates) {
  const p = `D:/WORKSPACE/fakeToys/oa4rust/crates/${crate}/src/lib.rs`;
  if (!fs.existsSync(p)) continue;
  let c = fs.readFileSync(p, 'utf8');
  if (c.includes('JAVA_BASE')) continue;
  c = c.replace('pub mod routes;', `pub const JAVA_BASE: &str = "${jb}";\npub mod routes;`);
  fs.writeFileSync(p, c);
  console.log('Fixed: ' + crate);
  fixed++;
}
console.log('Total fixed: ' + fixed);
