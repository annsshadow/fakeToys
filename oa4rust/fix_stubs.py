import re
import sys

def fix_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    original = content

    # Replace Option<Extension<Pool>> with Extension<Pool>
    content = content.replace('pool: Option<Extension<Pool>>,', 'pool: Extension<Pool>,')

    # Replace the match pool block
    old_block = '''    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };'''
    new_block = '''    let client = pool.get().await.map_err(|_| AppError::Internal)?;'''

    content = content.replace(old_block, new_block)

    if content != original:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Fixed: {filepath}")
    else:
        print(f"No changes: {filepath}")

if __name__ == '__main__':
    files = [
        r'D:\WORKSPACE\fakeToys\oa4rust\crates\attendance_assemble_control\src\lib.rs',
        r'D:\WORKSPACE\fakeToys\oa4rust\crates\general_assemble_control\src\lib.rs',
        r'D:\WORKSPACE\fakeToys\oa4rust\crates\file_assemble_control\src\lib.rs',
    ]
    for f in files:
        fix_file(f)
