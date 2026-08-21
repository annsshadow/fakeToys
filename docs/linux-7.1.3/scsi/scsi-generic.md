
## SCSI 通用（sg）驱

                                                        20020126

## 简

SCSI 通用驱动（sg）是sd、st sr（分别是磁盘、磁带与 CD-ROM）并列的四个“高层”SCSI 设备
驱动之一。Sg 比它的兄弟们更通用（但层级更低），往往用于那些不适合已有分类SCSI 设备因此，sg 被用于扫描仪、CD 刻录机以及数字方式读取音CD 等用途
这里不记录驱动的接口，而是提供版本信息以及指向文档与示例（URL）的指引
## sg 驱动的主要版

Linux 内核（lk）中存在 sg 的三个主要版本：
      - sg 版本 1（原始版），1992 年到 1999 年初（lk 2.2.5）。它基于 sg_header 接口结构      - sg 版本 2，从 2.2 系列lk 2.2.6 开始。它基于 sg_header 接口结构的扩展版本      - sg 版本 3，见lk 2.4 系列（以lk 2.5 系列）。它增加sg_io_hdr 接口结构
## sg 驱动文档


sg 驱动的最新文档保存在

- https://sg.danny.cz/sg/

这描述的lk 2.4 系列中的 sg 版本 3 驱动
lk 2.2 系列sg 版本 2 驱动的文档（大型版本）可在以下位置找
- https://sg.danny.cz/sg/p/scsi-generic_long.txt銆。
sg 驱动的原始文档（lk 2.2.6 之前）可LDP 归档中找
- https://tldp.org/HOWTO/archived/SCSI-Programming-HOWTO/index.html

sg 所属的 Linux SCSI 子系统的更一般描述可https://www.tldp.org/HOWTO/SCSI-2.4-HOWTO 找到
## 示例代码与工

sg 工具有两个软件包
    =========   ==========================================================
    sg3_utils   用于 lk 2.4 中的 sg 版本 3 驱动
    sg_utils   用于 lk 2.2 及更早版本中sg 版本 2（及原始版）驱动
    =========   ==========================================================

两个软件包都可在 lk 2.4 系列中工作。不过，sg3_utils 提供更多的能力。它们可在以下位置找到：
https://sg.danny.cz/sg/sg3_utils.html 涓?freecode.com

另一种方法是查看使用 sg 驱动的应用程序。这些包cdrecord、cdparanoia、SANE cdrdao
## Linux 内核版本sg 驱动版本的映

以下2.4 系列中具有新sg 驱动Linux 内核列表
     - lk 2.4.0 : sg 版本 3.1.17
     - lk 2.4.7 : sg 版本 3.1.19
     - lk 2.4.10 : sg 版本 3.1.20 [#]_
     - lk 2.4.17 : sg 版本 3.1.22

       接下来的六个 Linux 内核版本
作为参考，以下2.2 系列中具有新sg 驱动Linux 内核列表
     - lk 2.2.0 : 原始 sg 版本 [无版本号]
     - lk 2.2.6 : sg 版本 2.1.31
     - lk 2.2.8 : sg 版本 2.1.32
     - lk 2.2.10 : sg 版本 2.1.34 [SG_GET_VERSION_NUM ioctl 首次出现]
     - lk 2.2.14 : sg 版本 2.1.36
     - lk 2.2.16 : sg 版本 2.1.38
     - lk 2.2.17 : sg 版本 2.1.39
     - lk 2.2.20 : sg 版本 2.1.40

lk 2.5 开发系列目前包sg 版本 3.5.23，它在功能上等同lk 2.4.17 中的 sg 版本 3.1.22
Douglas Gilbert

2002 骞?1 鏈?26 鏃。
dgilbert@interlog.com
