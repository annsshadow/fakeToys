
## ACPI 涓殑 MDIO 鎬荤嚎涓?PHY


MDIO 鎬荤嚎 [phy] 涓婄殑 PHY 浣跨敤 fwnode_mdiobus_register_phy() 杩涜鎺㈡祴鍜屾敞鍐屻€?
涔嬪悗锛屼负浜嗗皢杩欎簺 PHY 杩炴帴鍒板畠浠悇鑷殑 MAC锛屽繀椤诲紩鐢ㄦ敞鍐屽湪
MDIO 鎬荤嚎涓婄殑 PHY銆?
鏈枃妗ｄ粙缁嶄簡涓や釜 _DSD 灞炴€э紝鐢ㄤ簬灏?MDIO 鎬荤嚎 [dsd-properties-rules] 涓婄殑
PHY 杩炴帴鍒?MAC 灞傘€?
杩欎簺灞炴€ф槸鏍规嵁鈥滅敤浜?_DSD 鐨勮澶囧睘鎬?UUID鈥漑dsd-guide] 鏂囨。瀹氫箟鐨勶紝骞朵笖
鍖呭惈瀹冧滑鐨勮澶囨暟鎹弿杩扮锛圖evice Data Descriptors锛変腑蹇呴』浣跨敤
daffd814-6eba-4d8c-8a91-bc9bbf4aa301 UUID銆?
### phy-handle


瀵逛簬姣忎釜 MAC 鑺傜偣锛屼娇鐢ㄨ澶囧睘鎬?"phy-handle" 鏉ュ紩鐢?娉ㄥ唽鍦?MDIO 鎬荤嚎涓婄殑 PHY銆傚浜庨€氳繃 MDIO 鎬荤嚎灏?PHY 杩炴帴鍒?MAC 鐨?缃戠粶鎺ュ彛锛岃繖鏄己鍒舵€х殑銆?
鍦?MDIO 鎬荤嚎椹卞姩鍒濆鍖栨湡闂达紝浣跨敤璇ユ€荤嚎涓婄殑 PHY 閫氳繃濡備笅鎵€绀虹殑 _ADR 瀵硅薄
杩涜鎺㈡祴锛屽苟娉ㄥ唽鍒?MDIO 鎬荤嚎涓娿€?

      Scope(\_SB.MDI0)
      {
        Device(PHY1) {
          Name (_ADR, 0x1)
        } // end of PHY1

        Device(PHY2) {
          Name (_ADR, 0x2)
        } // end of PHY2
      }

涔嬪悗锛屽湪 MAC 椹卞姩鍒濆鍖栨湡闂达紝蹇呴』浠?MDIO 鎬荤嚎妫€绱㈠凡娉ㄥ唽鐨?PHY 璁惧銆備负姝わ紝
MAC 椹卞姩闇€瑕佸紩鐢ㄥ厛鍓嶆敞鍐岀殑 PHY锛岃繖浜涘紩鐢ㄤ綔涓鸿澶囧璞″紩鐢ㄦ彁渚?锛堜緥濡?\_SB.MDI0.PHY1锛夈€?
### phy-mode


"phy-mode" _DSD 灞炴€х敤浜庢弿杩颁笌 PHY 鐨勮繛鎺ャ€?phy-mode" 鐨勬湁鏁堝€煎湪
[ethernet-controller] 涓畾涔夈€?
### managed


鍙€夊睘鎬э紝鎸囧畾 PHY 绠＄悊绫诲瀷銆?managed" 鐨勬湁鏁堝€煎湪 [ethernet-controller] 涓?瀹氫箟銆?
### fixed-link


"fixed-link" 鐢?MAC 绔彛鐨勪竴涓粎鏁版嵁锛坉ata-only锛夊瓙鑺傜偣鎻忚堪锛岃瀛愯妭鐐归€氳繃
鍒嗗眰鏁版嵁鎵╁睍锛圲UID dbb8e3e6-5886-4ba6-8795-1319f52a966b锛屼緷鎹?[dsd-guide]
鈥淿DSD 瀹炵幇鎸囧崡鈥濇枃妗ｏ級閾炬帴鍒?_DSD 鍖呬腑銆傝瀛愯妭鐐瑰簲鍖呭惈涓€涓繀闇€灞炴€?锛?speed"锛変互鍙婂彲鑳界殑鍙€夊睘鎬р€斺€斿弬鏁板強鍏跺€肩殑瀹屾暣鍒楄〃鍦?[ethernet-controller]
涓寚瀹氥€?
浠ヤ笅 ASL 绀轰緥璇存槑浜嗚繖浜涘睘鎬х殑鐢ㄦ硶銆?
### MDIO 鑺傜偣鐨?DSDT 鏉＄洰


MDIO 鎬荤嚎鏈変竴涓?SoC 缁勪欢锛圡DIO 鎺у埗鍣級鍜屼竴涓钩鍙扮粍浠讹紙MDIO 鎬荤嚎涓婄殑 PHY锛夈€?
a) 纭呯粍浠?### 璇ヨ妭鐐规弿杩?MDIO 鎺у埗鍣紝MDI0



	Scope(_SB)
	{
	  Device(MDI0) {
	    Name(_HID, "NXP0006")
	    Name(_CCA, 1)
	    Name(_UID, 0)
	    Name(_CRS, ResourceTemplate() {
	      Memory32Fixed(ReadWrite, MDI0_BASE, MDI_LEN)
	      Interrupt(ResourceConsumer, Level, ActiveHigh, Shared)
	       {
		 MDI0_IT
	       }
	    }) // end of _CRS for MDI0
	  } // end of MDI0
	}

b) 骞冲彴缁勪欢
### PHY1 鍜?PHY2 鑺傜偣琛ㄧず杩炴帴鍒?MDIO 鎬荤嚎 MDI0 鐨?PHY



	Scope(\_SB.MDI0)
	{
	  Device(PHY1) {
	    Name (_ADR, 0x1)
	  } // end of PHY1

	  Device(PHY2) {
	    Name (_ADR, 0x2)
	  } // end of PHY2
	}

### 琛ㄧず MAC 鑺傜偣鐨?DSDT 鏉＄洰


浠ヤ笅鏄紩鐢ㄤ簡 PHY 鑺傜偣鐨?MAC 鑺傜偣銆?### phy-mode 鍜?phy-handle 鐨勪娇鐢ㄥ鍓嶆墍杩般€?


	Scope(\_SB.MCE0.PR17)
	{
	  Name (_DSD, Package () {
	     ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package (2) {"phy-mode", "rgmii-id"},
		     Package (2) {"phy-handle", \_SB.MDI0.PHY1}
	      }
	   })
	}

	Scope(\_SB.MCE0.PR18)
	{
	  Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package (2) {"phy-mode", "rgmii-id"},
		    Package (2) {"phy-handle", \_SB.MDI0.PHY2}}
	    }
	  })
	}

### 鎸囧畾浜?"managed" 灞炴€х殑 MAC 鑺傜偣绀轰緥銆?


	Scope(\_SB.PP21.ETH0)
	{
	  Name (_DSD, Package () {
	     ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package () {"phy-mode", "sgmii"},
		     Package () {"managed", "in-band-status"}
		 }
	   })
	}

### 甯︽湁 "fixed-link" 瀛愯妭鐐圭殑 MAC 鑺傜偣绀轰緥銆?


	Scope(\_SB.PP21.ETH1)
	{
	  Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package () {"phy-mode", "sgmii"},
		 },
	    ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		 Package () {
		     Package () {"fixed-link", "LNK0"}
		 }
	  })
	  Name (LNK0, Package(){ // Data-only subnode of port
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package () {"speed", 1000},
		     Package () {"full-duplex", 1}
		 }
	  })
	}

## 鍙傝€?

[phy] Documentation/networking/phy.rst

[dsd-properties-rules]
    Documentation/firmware-guide/acpi/DSD-properties-rules.rst

[ethernet-controller]
    Documentation/devicetree/bindings/net/ethernet-controller.yaml

[dsd-guide] DSD Guide.
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc, 寮曠敤鏃ユ湡
    2021-11-30銆?