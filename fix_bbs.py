import pathlib
p=pathlib.Path(d'D:/WORKSPACE/fakeToys/oa4rust/crates/bbs_assemble_control/src/lib.rs')
c=p.read_text()
p.write_text('pub const JAVA_BASE: & str = "/jaxrs/bbs/assemble/control";\r\n' + c)
print('done')