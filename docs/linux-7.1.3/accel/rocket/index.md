## accel/rocket Rockchip NPU 驱动


accel/rocket 驱动支持部分 Rockchip SoC（如 RK3588）内部的神经处理单元（NPU）。
Rockchip 称之为 RKNN，有时也称为 RKNPU。

硬件在 RK3588 TRM 的第 36 章中描述。

该驱动仅负责硬件的上下电、为设备分配并映射缓冲区，以及向前端单元提交任务。
其余工作均在用户空间完成，即作为 Mesa3D 项目一部分的 Gallium 驱动（同样称为
rocket）。

当前支持的硬件：

- RK3588
