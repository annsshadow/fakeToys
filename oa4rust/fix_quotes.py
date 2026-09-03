import pathlib
root = pathlib.Path(r'D:\WORKSPACE\fakeToys\oa4rust\crates')
fixed = 0
for lib in root.glob('*/*.rs'):
    b = lib.read_bytes()
    old = b.replace(b'pub const JAVA_BASE: &str = ''', b'pub const JAVA_BASE: &str = \"')
    if old != b:
        lib.write_bytes(old)
        fixed += 1
print(f'Fixed {fixed} files')
