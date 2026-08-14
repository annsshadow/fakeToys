## Freescale BookE32 的 KASLR


KASLR 一词代表内核地址空间布局随机化（Kernel Address Space Layout
Randomization）。

本文档试图解释 Freescale BookE32 的 KASLR 实现。KASLR 是一项安全特性，可阻止
依赖内核内部位置知识的漏洞利用尝试。

由于 CONFIG_RELOCATABLE 已经支持，我们需要做的是将内核映射或复制到适当位置并
重定位。Freescale Book-E 部件期望 lowmem 由固定的 TLB 项（TLB1）映射。TLB1 项
不适合在随机化区域中直接映射内核，因此我们选择将内核复制到适当位置并重新启动
以重定位。

熵来自 banner 和定时器基址，它们每次构建和启动都会改变。这不太安全，因此
引导加载程序还可以通过设备树中的 /chosen/kaslr-seed 节点传递熵。

我们将使用低内存的前 512M 来随机化内核映像。内存将被划分为 64M 的区域。我们
将使用熵的低 8 位来决定 64M 区域的索引。然后我们选择

```

    KERNELBASE

        |-->   64M   <--|
        |               |
        +---------------+    +----------------+---------------+
        |               |....|    |kernel|    |               |
        +---------------+    +----------------+---------------+
        |                         |
        |----->   offset    <-----|

                              kernstart_virt_addr

```
要启用 KASLR，设置 CONFIG_RANDOMIZE_BASE = y。如果启用了 KASLR 并且你想在运行时
禁用它，请在内核命令行中添加“nokaslr”。
