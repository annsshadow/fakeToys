
## 平台（Platform）驱动


有一些驱动专注于为已经包含在主板上、且既不使用 USB 也不使用 PCI 总线的功能提供支持。这些驱动被称为平台驱动，在嵌入式设备上非常流行。

当前支持的平台驱动（不包括 staging 驱动）列于下表

=================  ============================================================
驱动               名称
=================  ============================================================
am437x-vpfe        TI AM437x VPFE
aspeed-video       Aspeed AST2400 和 AST2500
atmel-isc          ATMEL 图像传感器控制器（ISC）
atmel-isi          ATMEL 图像传感器接口（ISI）
cafe_ccic          Marvell 88ALP01（Cafe）CMOS 摄像头控制器
cdns-csi2rx        Cadence MIPI-CSI2 RX 控制器
cdns-csi2tx        Cadence MIPI-CSI2 TX 控制器
coda-vpu           Chips&Media Coda 多标准编解码器 IP
dm355_ccdc         TI DM355 CCDC 视频捕获
dm644x_ccdc        TI DM6446 CCDC 视频捕获
exynos-fimc-is     EXYNOS4x12 FIMC-IS（成像子系统）
exynos-fimc-lite   EXYNOS FIMC-LITE 摄像头接口
exynos-gsc         Samsung Exynos G-Scaler
exy                Samsung S5P/EXYNOS4 SoC 系列摄像头子系统
imx-pxp            i.MX 像素流水线（PXP）
isdf               TI DM365 ISIF 视频捕获
mmp_camera         Marvell Armada 610 集成摄像头控制器
mtk_jpeg           Mediatek JPEG 编解码器
mtk-mdp            Mediatek MDP
mtk-vcodec-dec     Mediatek 视频编解码器
mtk-vpu            Mediatek 视频处理单元
mx2_emmaprp        MX2 eMMa-PrP
omap3-isp          OMAP 3 摄像头
omap-vout          OMAP2/OMAP3 V4L2-显示
pxa_camera         PXA27x 快速捕获接口
qcom-camss         Qualcomm V4L2 摄像头子系统
rcar-csi2          R-Car MIPI CSI-2 接收器
rcar_drif          Renesas 数字无线电接口（DRIF）
rcar-fcp           Renesas 帧压缩处理器
rcar_fdp1          Renesas Fine Display 处理器
rcar-jpu           Renesas JPEG 处理单元
rcar-vin           R-Car 视频输入（VIN）
renesas-ceu        Renesas 捕获引擎单元（CEU）
rockchip-rga       Rockchip 光栅 2D 图形加速单元
s3c-camif          Samsung S3C24XX/S3C64XX SoC 摄像头接口
s5p-csis           S5P/EXYNOS MIPI-CSI2 接收器（MIPI-CSIS）
s5p-fimc           S5P/EXYNOS4 FIMC/CAMIF 摄像头接口
s5p-g2d            Samsung S5P 和 EXYNOS4 G2D 2D 图形加速器
s5p-jpeg           Samsung S5P/Exynos3250/Exynos4 JPEG 编解码器
s5p-mfc            Samsung S5P MFC 视频编解码器
sh_veu             SuperH VEU mem2mem 视频处理
sh_vou             SuperH VOU 视频输出
stm32-dcmi         STM32 数字摄像头内存接口（DCMI）
stm32-dma2d        STM32 Chrom-Art 加速单元
sun4i-csi          Allwinner A10 CMOS 传感器接口支持
sun6i-csi          Allwinner V3s 摄像头传感器接口
sun8i-di           Allwinner 去隔行
sun8i-rotate       Allwinner DE2 旋转
ti-cal             TI 内存到内存多媒体设备
ti-csc             TI DVB 平台设备
ti-vpe             TI VPE（视频处理引擎）
venus-enc          Qualcomm Venus V4L2 编码器/解码器
via-camera         VIAFB 摄像头控制器
video-mux          视频多路复用器
vpif_display       TI DaVinci VPIF V4L2-显示
vpif_capture       TI DaVinci VPIF 视频捕获
vsp1               Renesas VSP1 视频处理引擎
xilinx-tpg         Xilinx 视频测试图案生成器
xilinx-video       Xilinx 视频 IP（实验性）
xilinx-vtc         Xilinx 视频时序控制器
=================  ============================================================

### MMC/SDIO DVB 适配器


=======  ===========================================
驱动    名称
=======  ===========================================
smssdio  Siano SMS1xxx 基于 MDTV 的 SDIO 接口
=======  ===========================================
