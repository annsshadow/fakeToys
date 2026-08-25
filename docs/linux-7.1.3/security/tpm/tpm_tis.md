
## TPM FIFO 接口驱动


TCG PTP 规范定义了两种接口类型：FIFO CRB。前者基于有序的读写操作，后者基于包含一个完整命令或响应的缓冲区
FIFO（First-In-First-Out，先进先出）接口被依赖于 tpm_tis_core 的驱动所使用。最Linux 只有一个名tpm_tis 的驱动，它覆盖内存映射（MMIO）接口，但后来被扩展为覆TCG 标准支持的其他物理接口
由于上述历史原因，最初的 MMIO 驱动被称tpm_tis，FIFO 驱动的框架被命名tpm_tis_core。tpm_tis 中的后缀"tis"来自 TPM Interface Specification（TPM 接口规范），TPM 1.x 芯片的硬件接口规范
通信基于一块由 TPM 芯片通过硬件总线或内存映射共享的 20 KiB 缓冲区（取决于物理接线方式）。该缓冲区进一步被划分为五个等大小4 KiB 缓冲区，它们提供等价的寄存器集合，用CPU TPM 之间的通信。这些通信端点TCG 术语称为 localities（局部域）
当内核想要向 TPM 芯片发送命令时，它首先通过设置 TPM_ACCESS 寄存器中requestUse 位来保留 locality 0。当访问被授予时，该位由芯片清除。一旦完成通信，内核写TPM_ACCESS.activeLocality 位。这通知芯片locality 已被释放
待处理的 localities 由芯片按优先级从高到低依次处理，一次一个：

- Locality 0 优先级最低- Locality 5 优先级最高
关于 localities 的目的和含义的进一步信息，可在 TCG PC Client Platform TPM Profile 规范3.2 节中找到
## 参考资

TCG PC Client Platform TPM Profile (PTP) Specification
https://trustedcomputinggroup.org/resource/pc-client-platform-tpm-profile-ptp-specification/
