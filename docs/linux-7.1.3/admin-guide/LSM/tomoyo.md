## TOMOYO


## 什么是 TOMOYO

TOMOYO Linux 内核的一个基于名称的 MAC 扩展（LSM 模块）
基于 LiveCD 的教程可在以下位置获取：

https://tomoyo.sourceforge.net/1.8/ubuntu12.04-live.html
https://tomoyo.sourceforge.net/1.8/centos6-live.html

尽管这些教程使用的是LSM 版本TOMOYO，它们仍有助于你了解 TOMOYO 是什么
## 如何启用 TOMOYO

使用 `CONFIG_SECURITY_TOMOYO=y` 构建内核，并在内核命令行上传`security=tomoyo`
详情请参https://tomoyo.sourceforge.net/2.6/
## 文档在哪里？


用户 <-> 内核接口文档位于
https://tomoyo.sourceforge.net/2.6/policy-specification/index.html 銆。
我们为研讨会和专题讨论会准备的材料位https://sourceforge.net/projects/tomoyo/files/docs/ 。以下列表从三个方面选取
什么是 TOMOYO  TOMOYO Linux Overview
    https://sourceforge.net/projects/tomoyo/files/docs/lca2009-takeda.pdf
  TOMOYO Linux: pragmatic and manageable security for Linux
    https://sourceforge.net/projects/tomoyo/files/docs/freedomhectaipei-tomoyo.pdf
  TOMOYO Linux: A Practical Method to Understand and Protect Your Own Linux Box
    https://sourceforge.net/projects/tomoyo/files/docs/PacSec2007-en-no-demo.pdf

TOMOYO 能做什么？
  Deep inside TOMOYO Linux
    https://sourceforge.net/projects/tomoyo/files/docs/lca2009-kumaneko.pdf
  “基于路径名的访问控制”在安全中的作用    https://sourceforge.net/projects/tomoyo/files/docs/lfj2008-bof.pdf

TOMOYO 的历史？
  Realities of Mainlining
    https://sourceforge.net/projects/tomoyo/files/docs/lfj2008.pdf
