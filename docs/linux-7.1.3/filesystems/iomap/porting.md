
..
        维护作者理智的笨拙风格说明        请尽量将句子放在独立的行上开始，这样句子的变化就不会diff         串色（bleed colors）        标题装饰sphinx.rst 中有文档说明
## 移植你的文件系统


   :local:

## 为何转换

将文件系统转换为 iomap 有几个理由：

 1. 经典Linux I/O 路径效率并不高。页缓存（pagecache）操作一次只锁定一个基页，
    然后调用文件系统来只为该页返回一个映射。直I/O 操作一次只构建一个文件块
    大小I/O 请求。这对于 ext2 这类直接/间接映射的文件系统工作得足够好，但对    XFS 这类基于 extent（区间）的文件系统则非常低效
 2. folio（large folios）仅通过 iomap 支持；没有计划将旧的 buffer_head 路径
    转换为使用它们
 3. 对类内存设备（memory-like devices）上存储的直接访问（fsdax）仅通过 iomap
    支持
 4. 降低各文件系统维护者的维护开销。iomap 自己处理常见的页缓存相关操作，例    folio 的分配、实例化（instantiating）、锁定和解锁。使iomap 的文件系    不需要实->write_begin()>write_end() direct_IO 这些
    address_space_operations銆。
## 我该如何转换一个文件系统？


首先，从你的源码中添`#include <linux/iomap.h>`，并向你文件系统Kconfig 选项
添加 `select FS_IOMAP`。构建内核，针对你的文件系统支持的多种配置，`-g all` 选项
运行 fstests，以建立哪些测试通过、哪些失败的基线
推荐的方法是首先实现 `->iomap_begin`（如有必要再`->iomap_end`），以允iomap
获得文件范围的只读映射。大多数情况下，这是将现有的 `get_block()` 函数针对只读
映射做相对平凡的转换。`FS_IOC_FIEMAP` 是一个很好的首要目标，因为支持它并实现它简单，然后可以从用户态确extent 映射迭代是否正确。如FIEMAP 返回了正确的信息这是一个好迹象，表明其它只读映射操作也会做正确的事情
接下来，修改文件系统`get_block(create = false)` 实现，使用新`->iomap_begin`
实现来为选定的读操作映射文件空间。通过一个调试旋钮隐藏为选定调用路径开iomap
映射函数的能力。有必要编写一些代码来`iomap` 结构填充基于 bufferhead 的映信息，但新函数可以在不需要实现任iomap API 的情况下被测试
一旦只读函数这样工作起来，逐个将每个高层文件操作转换为使用 iomap 原生 API，而不经由 `get_block()`。一次做一个，回归应当不言自明。你**确实**有一fstests 的回测试基线，对吧？建议先转换交换文件（swap file）激活、`SEEK_DATA` `SEEK_HOLE`然后再处I/O 路径。此时一个可能的复杂性会是转换带缓冲的读 I/O 路径，因bufferheads。带缓冲的读 I/O 路径还不需要转换，不过直接 I/O 读路径应当在此阶转换
此时，你应该审视你的 `->iomap_begin` 函数。如果它基于 `flags` 参数的分派在大量
代码块之间切换，你应该考虑将它拆分为具有更小、更内聚函数的每操作 iomap ops。XFS
就是一个很好的例子
接下来要做的是在 `->iomap_begin`/`->iomap_end` 方法中实`get_blocks(create == true)` 功能。强烈建议为写操作创建单独的映射函数iomap ops然后将直I/O 写路径转换为 iomap，并开始认真地在文件系统上运行启用DIO fsx这将暴露出新的写映射实现引入的大量数据完整性边界情况缺陷
现在，将任何剩余的文操作转换为调iomap 函数。这将使整个文件系统使用新的映射
函数，并且在此步骤之后它们应该基本被调试并正确工作
此时，带缓冲的读和写路径很可能仍然需要转换。映射函数应该都正确工作，所以唯一需做的就是重写所有与 bufferheads 接口的代之以iomap folios 接口。首先更容易的是
将普通文I/O（没有任何花哨特性，fscrypt、fsverity、压缩或 data=journaling转换为使iomap。其中一些花哨特性（fscrypt 和压缩）iomap 中尚未实现。对于使页缓存的符号链接和目录的未日志（unjournalled）文件系统，你也可以尝试将它们的处理
转换iomap
剩下的部分留作读者练习，因为它对每个文件系统都不同。如果你遇到问题，请发邮件给
`get_maintainers.pl` 中的人员和邮件列表寻求帮助