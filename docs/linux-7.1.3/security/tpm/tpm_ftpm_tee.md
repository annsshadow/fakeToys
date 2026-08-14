## 固件 TPM 驱动


本文档描述固件可信平台模块（fTPM）设备驱动。

## 简介


该驱动是运行于 ARM TrustZone 环境中固件的垫片（shim）。驱动允许程序以与硬件 TPM 交互相同的方式与 TPM 交互。

## 设计


驱动充当一层薄薄的传递层，在固件实现的 TPM 之间转发命令。驱动本身不包含太多逻辑，更像是固件与内核/用户空间之间的哑管道（dumb pipe）。

固件本身基于以下论文：
https://www.microsoft.com/en-us/research/wp-content/uploads/2017/06/ftpm1.pdf

驱动加载时会向用户空间暴露 `/dev/tpmX` 字符设备，使用户空间能够通过该设备与固件 TPM 通信。
