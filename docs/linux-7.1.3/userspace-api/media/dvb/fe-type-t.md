
######## 前端类型


由于历史原因，前端类型以传输中所用调制方式的类型命名。前端类型由 fe_type_t 类型
给出，定义如下：



    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4


    - .. row 1

       - fe_type

       - 描述

       - DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> 等价的类型

    - .. row 2

       - .. _FE-QPSK:

	  `FE_QPSK`

       - 用于 DVB-S 标准

       - `SYS_DVBS`

    - .. row 3

       - .. _FE-QAM:

	  `FE_QAM`

       - 用于 DVB-C 附录 A 标准

       - `SYS_DVBC_ANNEX_A`

    - .. row 4

       - .. _FE-OFDM:

	  `FE_OFDM`

       - 用于 DVB-T 标准

       - `SYS_DVBT`

    - .. row 5

       - .. _FE-ATSC:

	  `FE_ATSC`

       - 用于 ATSC 标准（地面）或美国使用的 DVB-C 附录 B（有线）

       - `SYS_ATSC`（地面）或 `SYS_DVBC_ANNEX_B`（有线）


较新的格式如 DVB-S2、ISDB-T、ISDB-S 和 DVB-T2 未在上面描述，因为它们通过新的
FE_GET_PROPERTY/FE_GET_SET_PROPERTY <FE_GET_PROPERTY> ioctl，使用
DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> 参数得到支持。

在过去，结构体 `dvb_frontend_info` 曾经包含 `fe_type_t` 字段以指示传输系统，填充为
`FE_QPSK`、`FE_QAM`、`FE_OFDM` 或 `FE_ATSC` 之一。虽然为了保持向后兼容仍会填充该字段，
但其使用已被弃用，因为它只能报告一个传输系统，而某些设备支持多个传输系统。请改用
DTV_ENUM_DELSYS <DTV-ENUM-DELSYS>。

在支持多个传输系统的设备上，结构体 **`dvb_frontend_info`** 中的 `fe_type_t` 被填充为
当前标准，该标准由最后一次使用 DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> 属性的
FE_SET_PROPERTY <FE_GET_PROPERTY> 调用所选择。
