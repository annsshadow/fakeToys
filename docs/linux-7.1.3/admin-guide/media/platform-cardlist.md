
## 平台（Platform）驱

有一些驱动专注于为已经包含在主板上、且既不使用 USB 也不使用 PCI 总线的功能提供支持。这些驱动被称为平台驱动，在嵌入式设备上非常流行
当前支持的平台驱动（不包staging 驱动）列于下
=================  ============================================================
驱动               名称
=================  ============================================================
am437x-vpfe        TI AM437x VPFE
aspeed-video       Aspeed AST2400 鍜?AST2500
atmel-isc          ATMEL 图像传感器控制器（ISCatmel-isi          ATMEL 图像传感器接口（ISIcafe_ccic          Marvell 88ALP01（Cafe）CMOS 摄像头控制器
cdns-csi2rx        Cadence MIPI-CSI2 RX 控制cdns-csi2tx        Cadence MIPI-CSI2 TX 控制coda-vpu           Chips&Media Coda 多标准编解码IP
dm355_ccdc         TI DM355 CCDC 视频捕获
dm644x_ccdc        TI DM6446 CCDC 视频捕获
exynos-fimc-is     EXYNOS4x12 FIMC-IS（成像子系统exynos-fimc-lite   EXYNOS FIMC-LITE 摄像头接exynos-gsc         Samsung Exynos G-Scaler
exy                Samsung S5P/EXYNOS4 SoC 系列摄像头子系统
imx-pxp            i.MX 像素流水线（PXPisdf               TI DM365 ISIF 视频捕获
mmp_camera         Marvell Armada 610 集成摄像头控制器
mtk_jpeg           Mediatek JPEG 编解码器
mtk-mdp            Mediatek MDP
mtk-vcodec-dec     Mediatek 视频编解码器
mtk-vpu            Mediatek 视频处理单元
mx2_emmaprp        MX2 eMMa-PrP
omap3-isp          OMAP 3 摄像omap-vout          OMAP2/OMAP3 V4L2-显示
pxa_camera         PXA27x 快速捕获接qcom-camss         Qualcomm V4L2 摄像头子系统
rcar-csi2          R-Car MIPI CSI-2 接收rcar_drif          Renesas 数字无线电接口（DRIFrcar-fcp           Renesas 帧压缩处理器
rcar_fdp1          Renesas Fine Display 澶勭悊鍣?rcar-jpu           Renesas JPEG 澶勭悊鍗曞厓
rcar-vin           R-Car 视频输入（VINrenesas-ceu        Renesas 捕获引擎单元（CEUrockchip-rga       Rockchip 光栅 2D 图形加速单s3c-camif          Samsung S3C24XX/S3C64XX SoC 摄像头接s5p-csis           S5P/EXYNOS MIPI-CSI2 接收器（MIPI-CSISs5p-fimc           S5P/EXYNOS4 FIMC/CAMIF 摄像头接s5p-g2d            Samsung S5P EXYNOS4 G2D 2D 图形加速器
s5p-jpeg           Samsung S5P/Exynos3250/Exynos4 JPEG 编解码器
s5p-mfc            Samsung S5P MFC 视频编解码器
sh_veu             SuperH VEU mem2mem 视频处理
sh_vou             SuperH VOU 视频输出
stm32-dcmi         STM32 数字摄像头内存接口（DCMIstm32-dma2d        STM32 Chrom-Art 加速单sun4i-csi          Allwinner A10 CMOS 传感器接口支sun6i-csi          Allwinner V3s 摄像头传感器接口
sun8i-di           Allwinner 去隔sun8i-rotate       Allwinner DE2 旋转
ti-cal             TI 内存到内存多媒体设备
ti-csc             TI DVB 平台设备
ti-vpe             TI VPE（视频处理引擎）
venus-enc          Qualcomm Venus V4L2 编码解码via-camera         VIAFB 摄像头控制器
video-mux          视频多路复用vpif_display       TI DaVinci VPIF V4L2-显示
vpif_capture       TI DaVinci VPIF 视频捕获
vsp1               Renesas VSP1 视频处理引擎
xilinx-tpg         Xilinx 视频测试图案生成xilinx-video       Xilinx 视频 IP（实验性）
xilinx-vtc         Xilinx 视频时序控制=================  ============================================================

### MMC/SDIO DVB 閫傞厤鍣。

=======  ===========================================
驱动    名称
=======  ===========================================
smssdio  Siano SMS1xxx 基于 MDTV SDIO 接口
=======  ===========================================
