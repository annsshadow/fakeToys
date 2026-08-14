### MEI NFC

鏈枃浠嬬粛 Intel 绠＄悊寮曟搸锛圡EI锛夋€荤嚎鍚庢寕鎺ョ殑 NFC 璁惧鏀寔锛岃鏄?MEI 瀹㈡埛绔€荤嚎濡備綍灏?NFC 鑺墖鏆撮湶涓?phy 璁惧锛屽苟涓?Linux NFC 瀛愮郴缁熺殑 Microread銆丳N544 椹卞姩缁戝畾鐨勫崗璁爤缁撴瀯銆?



閮ㄥ垎 Intel 8 绯诲垪鍜?9 绯诲垪鑺墖缁勬敮鎸佽繛鎺ュ湪 Intel 绠＄悊寮曟搸锛圡anagement Engine锛夋帶鍒跺櫒鍚庨潰鐨?NFC 璁惧銆?
MEI 瀹㈡埛绔€荤嚎灏?NFC 鑺墖浣滀负 NFC phy 璁惧鏆撮湶鍑烘潵锛屽苟鏀寔涓?Linux NFC 瀛愮郴缁熼噷鐨?Microread 鍜?NXP PN544 NFC 璁惧椹卞姩杩涜缁戝畾銆?

   :alt: MEI NFC digraph
   :caption: **MEI NFC** 鍗忚鏍?

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

