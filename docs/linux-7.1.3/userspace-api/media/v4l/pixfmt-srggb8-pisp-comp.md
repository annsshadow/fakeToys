######## V4L2_PIX_FMT_PISP_COMP1_RGGB ('PC1R'), V4L2_PIX_FMT_PISP_COMP1_GRBG ('PC1G'), V4L2_PIX_FMT_PISP_COMP1_GBRG ('PC1g'), V4L2_PIX_FMT_PISP_COMP1_BGGR ('PC1B), V4L2_PIX_FMT_PISP_COMP1_MONO ('PC1M'), V4L2_PIX_FMT_PISP_COMP2_RGGB ('PC2R'), V4L2_PIX_FMT_PISP_COMP2_GRBG ('PC2G'), V4L2_PIX_FMT_PISP_COMP2_GBRG ('PC2g'), V4L2_PIX_FMT_PISP_COMP2_BGGR ('PC2B), V4L2_PIX_FMT_PISP_COMP2_MONO ('PC2M')


## Raspberry Pi PiSP 8 位压Bayer 格式


## 描述

Raspberry Pi ISP（PiSP）使用一组三种定速率压缩Bayer 格式。可以减去黑电平偏移以提升压缩效率；标称黑电平及偏移量必须通过带外（out-of-band）方式给出。每条扫描线都被填充8 像素宽的整数倍，8 个水平连续像素组成的块用 8 字节编码

模式 1 采用量化与基于差值的编码方案，可保留最12 个有效位。模2 是一种简单的类平方根压扩（companding）方案，包含 6 段分段线性（PWL）和弦，可保留最12 个有效位。模3 同时结合了压扩（4 段和弦）与差值方案，可保留最14 个有效位

本说明的其余部分适用于模1 和模3

每个 8 像素块被拆分为偶数相与奇数相，各4 个像素，由内存中连续位置32 位字分别独立编码。每32 位字的最2 位给出其“量化模式”

在量化模0 下，最低的 321 个量化等级为 FSD/4096 的整数倍，其余等级FSD/2048 的连续整数倍。量化模1 2 使用线性量化，步长分别FSD/1024 FSD/512。四个像素各自独立量化，并舍入到最近的等级。在量化模式 2 中，当中间两个样本的量化(q1,q2) 均处[384..511] 范围时，9 位表q1，随7 位表(q2 & 127)；否则，对于量化模式 0：一9 位字段编MIN(q1,q2)（必须处[0..511] 范围），一7 位字段编(q2-q1+64)（必须处[0..127] 范围）

每个外侧样本 (q0,q3) 使用一个基于其内侧相邻样本 q1 q2 7 位字段编码。在量化模式 2 中，当内侧样本的量化值处[448..511] 范围时，字段值为 (q0-384)；否则对于量化模0：外侧样本编码为 (q0-MAX(0,q1-64))。q3 同理基于 q2 编码。这些值都必须处于 [0..127] 范围。上述各字段分别2 位，按小端序打包，得到一个采LE 字节序的 32 位字

量化模式 3 具有.5 位”逃逸（escape）机制，当上述编码均无法容纳时使用。每个像素值被量化176 个等级中最接近的一个，其中最低的 95 个等级为 FSD/256 的整数倍，其余等级FSD/128 的整数倍（等级 175 表示非常接近 FSD 的值，解码时可能需要饱和算术）

每一对量化像(q0,q1) (q2,q3) 由一15 位字段联合编码：2816*(q0>>4) + 16*q1 + (q0&15)。三个字段分别为 255 位，按小端序{15,15,2} 顺序打包

一个压缩格式的软件解码器实现可`Raspberry Pi camera applications code base <https://github.com/raspberrypi/rpicam-apps/blob/main/image/dng.cpp>`_ 中找到
