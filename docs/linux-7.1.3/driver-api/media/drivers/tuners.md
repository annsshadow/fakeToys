
## 调谐器（Tuner）驱动

本文介绍 V4L2 媒体子系统下调谐器（Tuner）驱动的编程接口，说明不同风格（LG、Philips、Temic、ALPS 等）调谐器的频段切换字节差异，以及主要调谐器制造商的型号标识规则，供驱动开发者参考。



### 简单的调谐器编程


有几种不同风格的调谐器编程 API。
它们的主要区别在于频段切换字节。

- L= LG_API       (VHF_LO=0x01, VHF_HI=0x02, UHF=0x08, radio=0x04)
- P= PHILIPS_API  (VHF_LO=0xA0, VHF_HI=0x90, UHF=0x30, radio=0x04)
- T= TEMIC_API    (VHF_LO=0x02, VHF_HI=0x04, UHF=0x01)
- A= ALPS_API     (VHF_LO=0x14, VHF_HI=0x12, UHF=0x11)
- M= PHILIPS_MK3  (VHF_LO=0x01, VHF_HI=0x02, UHF=0x04, radio=0x19)

### 调谐器制造商


- Samsung 调谐器标识：（例如 TCPM9091PD27）


 TCP [ABCJLMNQ] 90[^89^][^125^] [DP] [ACD] 27 [ABCD]
 [ABCJLMNQ]:
   A= BG+DK
   B= BG
   C= I+DK
   J= NTSC-Japan
   L= Secam LL
   M= BG+I+DK
   N= NTSC
   Q= BG+I+DK+LL
 [^89^]: ?
 [^125^]:
   2: 无 FM
   5: 带 FM
 [DP]:
   D= NTSC
   P= PAL
 [ACD]:
   A= F 连接器
   C= Phono 连接器
   D= Din 插座
 [ABCD]:
   3 线/I2C 调谐，2 频段/3 频段

这些调谐器与 PHILIPS_API 兼容。

Philips 调谐器标识：（例如 FM1216MF）


  F[IRMQ]12[^1345^]6{MF|ME|MP}
  F[IRMQ]:
   FI12x6: 调谐器系列
   FR12x6: 调谐器 + 收音机 IF
   FM12x6: 调谐器 + FM
   FQ12x6: 特殊
   FMR12x6: 特殊
   TD15xx: 数字调谐器 ATSC
  12[^1345^]6:
   1216: PAL BG
   1236: NTSC
   1246: PAL I
   1256: Pal DK
  {MF|ME|MP}
   MF: BG LL 带 Secam（Multi France）
   ME: BG DK I LL   (Multi Europe)
   MP: BG DK I      (Multi PAL)
   MR: BG DK M (?)
   MG: BG DKI M (?)
  MK2 系列 PHILIPS_API，大多数调谐器与此兼容！
  MK3 系列于 2002 年引入，使用 PHILIPS_MK3_API

Temic 调谐器标识：（例如 4006FH5）


   4[^01^][^0136^][^269^]F[HYNR]5
    40x2: 调谐器 (5V/33V)，TEMIC_API。
    40x6: 调谐器 5V
    41xx: 调谐器 compact
    40x9: 调谐器+FM compact
   [^0136^]
    xx0x: PAL BG
    xx1x: Pal DK, Secam LL
    xx3x: NTSC
    xx6x: PAL I
   F[HYNR]5
    FH5: Pal BG
    FY5: 其他
    FN5: 多标准
    FR5: 带 FM 收音机
   3X xxxx: 带有特定连接器的订单号
  注意：只有 40x2 系列使用 TEMIC_API，所有更新的调谐器都使用 PHILIPS_API。

LG Innotek 调谐器：

- TPI8NSR11 : NTSC J/M    (TPI8NSR01 w/FM)  (P,210/497)
- TPI8PSB11 : PAL B/G     (TPI8PSB01 w/FM)  (P,170/450)
- TAPC-I701 : PAL I       (TAPC-I001 w/FM)  (P,170/450)
- TPI8PSB12 : PAL D/K+B/G (TPI8PSB02 w/FM)  (P,170/450)
- TAPC-H701P: NTSC_JP     (TAPC-H001P w/FM) (L,170/450)
- TAPC-G701P: PAL B/G     (TAPC-G001P w/FM) (L,170/450)
- TAPC-W701P: PAL I       (TAPC-W001P w/FM) (L,170/450)
- TAPC-Q703P: PAL D/K     (TAPC-Q001P w/FM) (L,170/450)
- TAPC-Q704P: PAL D/K+I   (L,170/450)
- TAPC-G702P: PAL D/K+B/G (L,170/450)

- TADC-H002F: NTSC (L,175/410?; 2-B, C-W+11, W+12-69)
- TADC-M201D: PAL D/K+B/G+I (L,143/425)  (声音控制在 I2C 地址 0xc8)
- TADC-T003F: NTSC 台湾  (L,175/410?; 2-B, C-W+11, W+12-69)

后缀：
  - P= 标准 phono 母座
  - D= IEC 母座
  - F= F 连接器

其他调谐器：

- TCL2002MB-1 : PAL BG + DK       =TUNER_LG_PAL_NEW_TAPC
- TCL2002MB-1F: PAL BG + DK w/FM  =PHILIPS_PAL
- TCL2002MI-2 : PAL I		= ??

ALPS 调谐器：

- 大多数与 LG_API 兼容
- TSCH6 使用 ALPS_API（TSCH5 ?）
- TSBE1 有额外的 API 05,02,08 控制字节=0xCB 来源:[#f1]_

