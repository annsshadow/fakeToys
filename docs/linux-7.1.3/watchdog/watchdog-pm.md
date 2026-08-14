## Linux 看门狗定时器电源管理指南


Last reviewed: 17-Dec-2018

Wolfram Sang <wsa+renesas@sang-engineering.com>

### 简介


本文档规定了关于看门狗设备及其电源管理处理的相关规则，以确保 Linux 系统行为的一致性。


### 恢复时 Ping


在恢复（resume）时，看门狗定时器应被重置为其选定值，以便用户空间有足够时间恢复。[^1^] [^2^]

[^1^] https://patchwork.kernel.org/patch/10252209/

[^2^] https://patchwork.kernel.org/patch/10711625/
