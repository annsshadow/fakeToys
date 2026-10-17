# 格式一致性检查报告（format-consistency）

- 生成时间：2026-08-21T13:34:03.552636+00:00
- 扫描根目录：`D:/WORKSPACE/fakeToys/docs/linux-7.1.3`
- 文件总数：3942　耗时：1.16s　脚本：`format_consistency.py`

## 1. YAML Frontmatter

| 指标 | 数量 | 占比 |
| --- | --- | --- |
| 有 frontmatter | 4 | 0.1% |
| 无 frontmatter | 3938 | 99.9% |

出现过的字段及频次（Top 10）：

- `title`: 4
- `type`: 4
- `status`: 4
- `date`: 4
- `origin`: 4
- `deepened`: 2

## 2. Toctree 残留与目录形式

- `.. toctree::` RST 指令残留：**0** 个文件（共 0 处）
- 含 `toctree` 关键字（含正文提及，如 doc-guide/sphinx.md）：3 个文件
- 目录形式分布：
  - Markdown 列表目录（连续 ≥2 个列表链接项）：**326**（8.3%）
  - 仅单个列表链接项：133
  - 无目录结构：**3483**（88.4%）

## 3. 标题层级问题

- 层级跳跃文件（任一相邻标题跳级，如 h1→h3、h2→h4）：**156**（4.0%）
- 其中 h1 直接到 h3 及更深：**35**
- 重复 h1 文件：**67**（1.7%）

## 示例文件（各 5 个）

### toctree 指令残留
-（无）

### Markdown 列表目录
- `accel/qaic/index.md`
- `accounting/index.md`
- `admin-guide/abi.md`
- `admin-guide/acpi/index.md`
- `admin-guide/aoe/index.md`

### 无目录结构
- `_TRANSLATE_PROTOCOL.md`
- `_w2_test_copy.md`
- `accel/amdxdna/amdnpu.md`
- `accel/introduction.md`
- `accel/qaic/aic080.md`

### 标题层级跳跃
- `admin-guide/bug-hunting.md`
- `admin-guide/cgroup-v1/cgroups.md`
- `admin-guide/device-mapper/cache-policies.md`
- `admin-guide/device-mapper/dm-raid.md`
- `admin-guide/hw-vuln/attack_vector_controls.md`

### 重复 h1
- `_w2_test_copy.md`
- `admin-guide/cgroup-v1/cgroups.md`
- `admin-guide/cgroup-v1/cpusets.md`
- `admin-guide/cgroup-v1/freezer-subsystem.md`
- `admin-guide/device-mapper/dm-pcache.md`

> 方法说明：仅识别 ATX 标题（`#` 前缀）；忽略代码块与 frontmatter 内内容；
> 目录判定为连续 ≥2 个含 `(...)` 链接目标的列表项（部分文件因编码问题缺失 `]`，
> 故采用宽松匹配）。全量明细见 `format-consistency-report.json`。
