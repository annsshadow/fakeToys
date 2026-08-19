import re

with open('Cargo.lock', 'r', encoding='utf-8') as f:
    content = f.read()

# Remove signature 0.1.0 package definition
content = re.sub(
    r'\[\[package\]\]\nname = "signature"\nversion = "0\.1\.0"\n.*?\n(?=\[\[package\]\]|$)',
    '',
    content,
    flags=re.DOTALL
)

# Remove signature 2.2.0 package definition
content = re.sub(
    r'\[\[package\]\]\nname = "signature"\nversion = "2\.2\.0"\n.*?\n(?=\[\[package\]\]|$)',
    '',
    content,
    flags=re.DOTALL
)

# Replace 'signature 0.1.0' with 'signature' in oa4rust dependencies
content = content.replace('"signature 0.1.0"', '"signature"')

# Replace 'signature 2.2.0' with 'signature' if present
content = content.replace('"signature 2.2.0"', '"signature"')

with open('Cargo.lock', 'w', encoding='utf-8') as f:
    f.write(content)

print('Done')
