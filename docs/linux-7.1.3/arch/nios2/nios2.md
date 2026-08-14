## Linux 在 Nios II 架构上


这是 Linux 向 Nios II（nios2）处理器的移植。

要编译 Nios II 目标，你需要一个支持通用系统调用 ABI 的 GCC 版本。有关如何为 Nios II 平台编译和启动软件的更多信息，请参见以下链接：
http://www.rocketboards.org/foswiki/Documentation/NiosIILinuxUserManual

供参考，请参见以下链接：
http://www.altera.com/literature/lit-nio2.jsp

## 什么是 Nios II？


Nios II 是专为 Altera 系列 FPGA 设计的 32 位嵌入式处理器架构。为了支持 Linux，Nios II 需要配置启用 MMU 和硬件乘法器。

## Nios II ABI


请参考《Nios II 处理器参考手册》中的“应用程序二进制接口”（Application Binary Interface）章节。
