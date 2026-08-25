## dGPU 固件刷写（firmware flashing

### IFWI

刷写 dGPU 集成的固件镜像（IFWI）受使用 PSP 来协调更新（Navi3x 或更新的 GPU）的 GPU 支持。对于受支持GPU，`amdgpu` 会导出一系列可用于刷写过程的 sysfs 文件
IFWI 刷写过程如下
1. 确保 IFWI 镜像适用于系统上dGPU2. IFWI 镜像“写入（Write）”到 sysfs 文件 `psp_vbflash`。这会将 IFWI 暂存（stage）到内存中3. `psp_vbflash` sysfs 文件“读取（Read）”以启动刷写过程4. 轮询（Poll）`psp_vbflash_status` sysfs 文件以确定刷写过程何时完成
### USB-C PD F/W

在支持刷写更新后USB-C PD 固件镜像GPU 上，该过程通过 `usbc_pd_fw` sysfs 文件完成
- 读取该文件将提供当前的固件版本- 将存储在 `/lib/firmware/amdgpu` 中的固件负载（firmware payload）的名称写入sysfs 文件，将启动刷写过程
存储`/lib/firmware/amdgpu` 中的固件负载可以任意命名，只要它`amdgpu` 所使用的其他现有二进制文件不冲突即可
### sysfs 文件
