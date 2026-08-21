# 链接完整性审计报告（link-integrity）

- 生成时间：2026-08-21T13:55:47.669185+00:00
- 审计范围：`docs/linux-7.1.3/`
- 扫描文件数：**3942**
- 总链接数：**3961**
- 外部 http(s) 链接：18（只计数，未探测）
- 纯锚点链接：7（跳过）
- 其他协议链接：0（跳过）
- 内部链接：3936（指向文件 3826，指向目录 5）
- **断链总数：105**
- **断链率：2.65%**（占全部链接）；占内部链接 2.67%
- 涉及断链的文件数：40

## Top 10 最常被断链指向的目标

| 次数 | 目标 |
|---:|---|
| 4 | `*` |
| 3 | `admin-guide/reporting-issues.rst` |
| 2 | `example-configurations/hb-interleave.rst` |
| 2 | `srctree/include/linux/printk.h` |
| 1 | `URL` |
| 1 | `url` |
| 1 | `../../arch` |
| 1 | `../../kbuild` |
| 1 | `boot-interrupts` |
| 1 | `part-name` |

## 断链按目录分布（按源文件一级子目录）

| 目录 | 断链数 |
|---|---:|
| `plans` | 25 |
| `driver-api` | 19 |
| `gpu` | 16 |
| `trace` | 13 |
| `userspace-api` | 5 |
| `translations` | 4 |
| `(root)` | 3 |
| `bpf` | 3 |
| `core-api` | 3 |
| `admin-guide` | 2 |
| `w1` | 2 |
| `arch` | 1 |
| `block` | 1 |
| `dev-tools` | 1 |
| `hwmon` | 1 |
| `input` | 1 |
| `netlink` | 1 |
| `networking` | 1 |
| `rust` | 1 |
| `usb` | 1 |
| `wmi` | 1 |

## 说明

- 相对路径以所在文件目录为基解析；`/` 开头按文档树根解析。
- 解析顺序：精确路径 → `<目标>.md` → `<目标>/index.md`。
- 带 `#anchor` 的目标只验证文件部分；URL 编码（如 `%20`）已解码。
- 明细见 `link-integrity-report.json`。
