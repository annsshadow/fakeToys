import re

filepath = 'oa4rust/crates/processplatform_assemble_bam/src/lib.rs'
with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
    content = f.read()

# Strategy: replace all lines that contain garbled GBK characters
# These are comments with known patterns of garbled text
# We'll replace them with clean English equivalents

lines = content.split('\n')
cleaned = []
for line in lines:
    # Check if line contains garbled characters (high codepoint range from GBK mojibake)
    has_garbled = any(ord(c) > 0x4e00 and c not in 'OA协同平台BAM配置模块路由获取状态返回当前运行只读查询列表详情' for c in line)
    # More reliable: check for specific garbled patterns
    garbled_markers = ['鈹', '娴', '鍛', '鏃', '绂', '绛', '宸', '浠', '璺', '缁', '呴', '厤', '湁']
    is_garbled = any(m in line for m in garbled_markers)

    if is_garbled and line.strip().startswith('//'):
        # Replace with clean equivalent
        cleaned.append('  // [comment cleaned - encoding fix]')
    elif is_garbled and line.strip().startswith('///'):
        cleaned.append('  /// [doc comment - encoding fix]')
    else:
        cleaned.append(line)

result = '\n'.join(cleaned)

with open(filepath, 'w', encoding='utf-8') as f:
    f.write(result)

print(f'Fixed garbled comments. File written.')
print(f'Original lines: {len(lines)}, Cleaned lines: {len(cleaned)}')
