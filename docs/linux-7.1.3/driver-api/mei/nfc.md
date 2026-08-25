### MEI NFC

本文介绍 Intel 管理引擎（MEI）总线后挂接的 NFC 设备支持，说MEI 客户端总线如何NFC 芯片暴露phy 设备，并Linux NFC 子系统的 Microread、PN544 驱动绑定的协议栈结构



部分 Intel 8 系列9 系列芯片组支持连接在 Intel 管理引擎（Management Engine）控制器后面NFC 设备
MEI 客户端总线NFC 芯片作为 NFC phy 设备暴露出来，并支持Linux NFC 子系统里Microread NXP PN544 NFC 设备驱动进行绑定

   :alt: MEI NFC digraph
   :caption: **MEI NFC** 协议

   digraph NFC {
    cl_nfc -> me_cl_nfc;
    "drivers/nfc/mei_phy" -> cl_nfc [lhead=bus];
    "drivers/nfc/microread/mei" -> cl_nfc;
    "drivers/nfc/microread/mei" -> "drivers/nfc/mei_phy";
    "drivers/nfc/pn544/mei" -> cl_nfc;
    "drivers/nfc/pn544/mei" -> "drivers/nfc/mei_phy";
    "net/nfc" -> "drivers/nfc/microread/mei";
    "net/nfc" -> "drivers/nfc/pn544/mei";
    "neard" -> "net/nfc";
    cl_nfc [label="mei/bus(nfc)"];
    me_cl_nfc [label="me fw (nfc)"];
   }

