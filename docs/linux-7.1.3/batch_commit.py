#!/usr/bin/env python3
"""
分批提交脚本
"""

import os
import subprocess
from collections import defaultdict

def get_modified_files():
    """获取所有修改的文件"""
    result = subprocess.run(['git', 'status', '--short'], capture_output=True, text=True)
    files = []
    for line in result.stdout.strip().split('\n'):
        if line.startswith(' M'):
            file_path = line[2:].strip()
            files.append(file_path)
    return files

def group_files_by_directory(files):
    """按目录分组文件"""
    groups = defaultdict(list)
    for file_path in files:
        parts = file_path.split('/')
        if len(parts) > 1:
            directory = parts[0]
        else:
            directory = 'root'
        groups[directory].append(file_path)
    return groups

def commit_batch(directory, files, batch_number):
    """提交一批文件"""
    print(f"\n批次 {batch_number}: 提交 {directory} 目录 ({len(files)} 个文件)")
    
    # 添加文件
    for file_path in files:
        subprocess.run(['git', 'add', file_path], check=True)
    
    # 提交
    commit_message = f"docs: 整理 {directory} 目录文档 (批次 {batch_number})"
    subprocess.run(['git', 'commit', '-m', commit_message], check=True)
    
    print(f"[OK] 已提交 {len(files)} 个文件")

def main():
    print("开始分批提交...")
    
    # 获取修改的文件
    files = get_modified_files()
    print(f"共找到 {len(files)} 个修改的文件")
    
    # 按目录分组
    groups = group_files_by_directory(files)
    
    # 定义提交批次
    batches = [
        ('translations', 1),
        ('userspace-api', 2),
        ('admin-guide', 3),
        ('driver-api', 4),
        ('hwmon', 5),
        ('networking', 6),
        ('arch', 7),
    ]
    
    batch_number = 1
    committed_count = 0
    
    # 提交指定批次
    for directory, _ in batches:
        if directory in groups:
            commit_batch(directory, groups[directory], batch_number)
            committed_count += len(groups[directory])
            batch_number += 1
    
    # 提交剩余文件
    remaining_files = []
    for directory, files in groups.items():
        if directory not in [b[0] for b in batches]:
            remaining_files.extend(files)
    
    if remaining_files:
        # 按目录分组提交剩余文件
        remaining_groups = group_files_by_directory(remaining_files)
        for directory, files in remaining_groups.items():
            commit_batch(directory, files, batch_number)
            committed_count += len(files)
            batch_number += 1
    
    print(f"\n{'='*60}")
    print("提交完成")
    print(f"{'='*60}")
    print(f"总提交文件数: {committed_count}")
    print(f"总批次数: {batch_number - 1}")

if __name__ == "__main__":
    main()
