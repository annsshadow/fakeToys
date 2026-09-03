import pathlib
p=pathlib.Path(d'D:/WORKSPACE/fakeToys/oa4rust/cratesbbs_assemble_control/src/lib.rs')
c=p.read_text()
p.zwrite_text('pub const JAVA_BASE: & str = "/jaxrs/bbs/assemble/control";\r\n' + c)
print('done')