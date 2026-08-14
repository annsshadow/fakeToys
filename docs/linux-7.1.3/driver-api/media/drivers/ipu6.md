
## Intel IPU6 椹卞姩


Author: Bingbu Cao <bingbu.cao@intel.com>

## Overview


Intel IPU6 鏄?the sixth generation 鐨?Intel Image Processing Unit 浣跨敤 鍦?涓€浜?
Intel Chipsets 渚嬪 Tiger Lake, Jasper Lake, Alder Lake, Raptor Lake 鍜?
Meteor Lake. IPU6 consists 鐨?two 涓昏 绯荤粺: 杈撳叆 绯荤粺 (ISYS) 鍜?
Processing 绯荤粺 (PSYS). IPU6 鏄?visible 鍦?the PCI 鎬荤嚎 浣滀负 涓€涓?鍗曚釜 璁惧, 瀹?
鍙?涓?found 鐢?`lspci`:

`0000:00:05.0 Multimedia controller: Intel Corporation Device xxxx (rev xx)`

IPU6 鍏锋湁 涓€涓?16 MB BAR 鍦?PCI 閰嶇疆 Space 鐢ㄤ簬 MMIO 瀵勫瓨鍣?鍏?鏄?
visible 鐢ㄤ簬 椹卞姩.

## Buttress


The IPU6 鏄?connecting 鍒?the 绯荤粺 fabric 涓?Buttress 鍏?鏄?enabling host
椹卞姩 鍒?control the IPU6, 瀹?涔?allows IPU6 access the 绯荤粺 鍐呭瓨 鍒?
store 鍜?鍔犺浇 甯?pixel streams 鍜?浠讳綍 鍏朵粬 metadata.

Buttress mainly manages 鑻ュ共 绯荤粺 functionalities: 鐢垫簮绠＄悊,
涓柇 handling, 鍥轰欢 authentication 鍜?鍏ㄥ眬 timer sync.

### ISYS 鍜?PSYS 鐢垫簮 flow


IPU6 椹卞姩 initialize the ISYS 鍜?PSYS 鐢垫簮 up 鎴?down 璇锋眰 鐢?璁剧疆 the
Buttress frequency control 娉ㄥ唽 鐢ㄤ簬 ISYS 鍜?PSYS
(`IPU6_BUTTRESS_REG_IS_FREQ_CTL` 鍜?`IPU6_BUTTRESS_REG_PS_FREQ_CTL`) 鍦?
鍑芥暟:


Buttress forwards the 璇锋眰 鍒?Punit, 涔嬪悗 Punit execute the 鐢垫簮 up flow,
Buttress indicates 椹卞姩 璇?ISYS 鎴?PSYS 鏄?powered up 鐢?updating the 鐢垫簮
鐘舵€?瀵勫瓨鍣?

	  needs take place 涔嬪悗 PSYS 鐢垫簮 down 鐢变簬 纭欢 limitation.

### 涓柇


IPU6 涓柇 鍙?涓?generated 浣滀负 MSI 鎴?INTA, 涓柇 灏?涓?triggered 褰?
ISYS, PSYS, Buttress 浜嬩欢 鎴?閿欒 happen, 椹卞姩 鍙?get the 涓柇 cause
鐢?reading the 涓柇 鐘舵€?娉ㄥ唽 `BUTTRESS_REG_ISR_STATUS`, 椹卞姩
clears the irq 鐘舵€?鍜?鐒跺悗 calls 鐗瑰畾 ISYS 鎴?PSYS irq 澶勭悊绋嬪簭.


### 瀹夊叏 鍜?鍥轰欢 authentication


鍒?鍦板潃 the IPU6 鍥轰欢 瀹夊叏 concerns, the IPU6 鍥轰欢 needs 鍒?
undergo 涓€涓?authentication 杩涚▼ 涔嬪墠 瀹冩槸 allowed 鍒?executed 鍦?the IPU6
鍐呴儴 processors. The IPU6 椹卞姩 灏?work 涓?Converged 瀹夊叏 Engine
(CSE) 鍒?complete authentication 杩涚▼. The CSE 鏄?responsible 鐨?
authenticating the IPU6 鍥轰欢. The authenticated 鍥轰欢 binary 鏄?copied
杩涘叆 涓€涓?isolated 鍐呭瓨 region. 鍥轰欢 authentication 杩涚▼ 鏄?implemented
鐢?CSE 浠ヤ笅 涓€涓?IPC handshake 涓?the IPU6 椹卞姩. 瀛樺湪 涓€浜?Buttress
瀵勫瓨鍣?浣跨敤 鐢?the CSE 鍜?the IPU6 椹卞姩 鍒?communicate 涓?姣忎釜 鍏朵粬 閫氳繃
IPC.


### 鍏ㄥ眬 timer sync


The IPU6 椹卞姩 initiates 涓€涓?Hammock Harbor synchronization flow 姣忎釜 time 瀹?
starts 鐩告満 鎿嶄綔. The IPU6 灏?synchronizes 涓€涓?鍐呴儴 counter 鍦?the
Buttress 涓?涓€涓?copy 鐨?the SoC time, 姝?counter maintains the up-to-date time
鐩村埌 鐩告満 鎿嶄綔 鏄?stopped. The IPU6 椹卞姩 鍙?浣跨敤 姝?time counter 鍒?
calibrate the timestamp 鍩轰簬 the timestamp 鍦?鍝嶅簲 浜嬩欢 鏉ヨ嚜 鍥轰欢.


## DMA 鍜?MMU


The IPU6 鍏锋湁 鍏?own scalar processor 浣曞 the 鍥轰欢 杩愯 鍦?鍜?涓€涓?鍐呴儴
32-浣?铏氭嫙 鍦板潃 space. The IPU6 鍏锋湁 MMU 鍦板潃 translation 纭欢 鍒?
鍏佽 璇?scalar processors 鍒?access the 鍐呴儴 鍐呭瓨 鍜?澶栭儴 绯荤粺
鍐呭瓨 through IPU6 铏氭嫙 鍦板潃. The 鍦板潃 translation 鏄?鍩轰簬 two
levels 鐨?椤?lookup 琛?stored 鍦?绯荤粺 鍐呭瓨 鍏?鏄?maintained 鐢?the
IPU6 椹卞姩. The IPU6 椹卞姩 sets the level-1 椤?琛?base 鍦板潃 鍒?MMU
娉ㄥ唽 鍜?allows MMU 鍒?perform 椤?琛?lookups.

The IPU6 椹卞姩 exports 鍏?own DMA 鎿嶄綔. The IPU6 椹卞姩 灏?鏇存柊 the
椤?琛?鏉＄洰 鐢ㄤ簬 姣忎釜 DMA 鎿嶄綔 鍜?invalidate the MMU TLB 涔嬪悗 姣忎釜
unmap 鍜?free.

## 鍥轰欢 鏂囦欢 鏍煎紡


The IPU6 鍥轰欢 鏄?鍦?Code Partition Directory (CPD) 鏂囦欢 鏍煎紡. The CPD
鍥轰欢 鍖呭惈 涓€涓?CPD header, 鑻ュ共 CPD 鏉＄洰 鍜?components. The CPD
component 鍖呭惈 3 鏉＄洰 - manifest, metadata 鍜?妯″潡 鏁版嵁. Manifest 鍜?
metadata 鏄?瀹氫箟 鐢?CSE 鍜?浣跨敤 鐢?CSE 鐢ㄤ簬 authentication. 妯″潡 鏁版嵁 鏄?
鐗瑰畾 鍒?IPU6 鍏?holds the binary 鏁版嵁 鐨?鍥轰欢 called package
directory. The IPU6 椹卞姩 (`ipu6-cpd.c` 鐗瑰埆鏄? parses 鍜?validates
the CPD 鍥轰欢 鏂囦欢 鍜?gets the package directory binary 鏁版嵁 鐨?the IPU6
鍥轰欢, copies 瀹?鍒?鐗瑰畾 DMA 缂撳啿鍖?鍜?sets 鍏?base 鍦板潃 鍒?Buttress
`FW_SOURCE_BASE` 娉ㄥ唽. Finally the CSE 灏?鎵ц authentication 鐢ㄤ簬 姝?
鍥轰欢 binary.


## Syscom 鎺ュ彛


The IPU6 椹卞姩 communicates 涓?鍥轰欢 閫氳繃 the Syscom ABI. Syscom 鏄?涓€涓?
inter-processor communication mechanism 涔嬮棿 the IPU scalar processors 鍜?
the CPU. 瀛樺湪 涓€涓?鏁板瓧 鐨?resources shared 涔嬮棿 鍥轰欢 鍜?杞欢.
涓€涓?绯荤粺 鍐呭瓨 region 浣曞 the message queues reside, 鍥轰欢 鍙?access the
鍐呭瓨 region 閫氳繃 the IPU MMU. The Syscom queues 鏄?FIFO fixed depth queues
涓?涓€涓?configurable 鏁板瓧 鐨?tokens (messages). 瀛樺湪 涔?閫氱敤 IPU6 MMIO
瀵勫瓨鍣?浣曞 the 闃熷垪 璇诲彇 鍜?鍐欏叆 indices reside. 杞欢 鍜?鍥轰欢
鍑芥暟 浣滀负 producer 鍜?consumer 鐨?tokens 鍦?the queues 鍜?鏇存柊 the 鍐欏叆
鍜?璇诲彇 indices separately 褰?sending 鎴?receiving 姣忎釜 message.

The IPU6 椹卞姩 蹇呴』 prepare 鍜?configure the 鏁板瓧 鐨?杈撳叆 鍜?杈撳嚭
queues, configure the count 鐨?tokens 姣?闃熷垪 鍜?the 澶у皬 鐨?姣?token 涔嬪墠
initiating 鍜?starting the communication 涓?鍥轰欢. 鍥轰欢 鍜?杞欢
蹇呴』 浣跨敤 鐩稿悓 configurations. The IPU6 Buttress 鍏锋湁 涓€涓?鏁板瓧 鐨?鍥轰欢 boot
鍙傛暟 瀵勫瓨鍣?鍏?鍙?涓?浣跨敤 鍒?store the 鍦板潃 鐨?閰嶇疆 鍜?
initialise the Syscom 鐘舵€? 鐒跺悗 椹卞姩 鍙?璇锋眰 鍥轰欢 鍒?鍚姩 鍜?杩愯 閫氳繃
璁剧疆 the scalar processor control 鐘舵€?娉ㄥ唽.

## 杈撳叆 绯荤粺


IPU6 杈撳叆 绯荤粺 consists 鐨?MIPI D-PHY 鍜?鑻ュ共 CSI-2 receivers.  瀹?鍙?
capture image pixel 鏁版嵁 鏉ヨ嚜 鐩告満 浼犳劅鍣?鎴?鍏朵粬 MIPI CSI-2 杈撳嚭 璁惧.

### D-PHYs 鍜?CSI-2 ports lane 鏄犲皠


The IPU6 integrates 涓嶅悓 D-PHY IPs 鍦?涓嶅悓 SoCs, 鍦?Tiger Lake 鍜?
Alder Lake, IPU6 integrates MCD10 D-PHY, IPU6SE 鍦?Jasper Lake integrates JSL
D-PHY 鍜?IPU6EP 鍦?Meteor Lake integrates 涓€涓?Synopsys DWC D-PHY. 瀛樺湪 涓€涓?
adaptional layer 涔嬮棿 D-PHY 鍜?CSI-2 receiver 鎺у埗鍣?鍏?鍖呭惈 绔彛
閰嶇疆, PHY wrapper 鎴?绉佹湁 test interfaces 鐢ㄤ簬 D-PHY. 瀛樺湪 3
D-PHY 椹卞姩 `ipu6-isys-mcd-phy.c`, `ipu6-isys-jsl-phy.c` 鍜?
`ipu6-isys-dwc-phy.c` program the 涓婃枃 3 D-PHYs 鍦?IPU6.

涓嶅悓 IPU6 versions 鍏锋湁 涓嶅悓 D-PHY lanes mappings, 鍦?Tiger Lake,
瀛樺湪 12 鏁版嵁 lanes 鍜?8 clock lanes, IPU6 鏀寔 鏈€澶?8 CSI-2 ports,
鍙傝 the PPI mmapping 鍦?`ipu6-isys-mcd-phy.c` 鐢ㄤ簬 鏇村 information. 鍦?Jasper
Lake 鍜?Alder Lake, D-PHY 鍏锋湁 8 鏁版嵁 lanes 鍜?4 clock lanes, the IPU6 supports
鏈€澶?4 CSI-2 ports. 鐢ㄤ簬 Meteor Lake, D-PHY 鍏锋湁 12 鏁版嵁 lanes 鍜?6 clock
lanes 鍥犳 IPU6 鏀寔 鏈€澶?6 CSI-2 ports.

	  lanes. 渚嬪, 鐢ㄤ簬 CSI-2 绔彛 0 鍜?1, CSI-2 绔彛 0 鏀寔
	  鏈€澶?4 鏁版嵁 lanes, CSI-2 绔彛 1 鏀寔 鏈€澶?2 鏁版嵁 lanes, CSI-2
	  绔彛 0 涓?2 鏁版嵁 lanes 鍙?work together 涓?CSI-2 绔彛 1 涓?2
	  鏁版嵁 lanes. 鑻?trying 鍒?浣跨敤 CSI-2 绔彛 0 涓?4 lanes, CSI-2 绔彛 1
	  灏?涓?涓?鍙敤 浣滀负 the 4 鏁版嵁 lanes 鏄?shared 鐢?CSI-2 绔彛 0
	  鍜?1. The 鐩稿悓 applies 鍒?CSI ports 2/3, 4/5 鍜?7/8.

### ISYS 鍥轰欢 ABIs


The IPU6 鍥轰欢 implements 涓€涓?绯诲垪 鐨?ABIs 鐢ㄤ簬 杞欢 access. 涓€鑸€岃█,
杞欢 firstly prepares the 娴?閰嶇疆 ``缁撴瀯浣?
ipu6_fw_isys_娴乢cfg_鏁版嵁_abi`` 鍜?sends the 閰嶇疆 鍒?鍥轰欢 閫氳繃
sending `STREAM_OPEN` 鍛戒护. 娴?閰嶇疆 鍖呭惈 杈撳叆 pins 鍜?
杈撳嚭 pins, 杈撳叆 pin `struct ipu6_fw_isys_input_pin_info_abi` defines the
resolution 鍜?鏁版嵁 绫诲瀷 鐨?杈撳叆 source, 杈撳嚭 pin ``缁撴瀯浣?
ipu6_fw_isys_杈撳嚭_pin_info_abi`` defines the 杈撳嚭 resolution, stride 鍜?
甯?鏍煎紡, 绛?

涓€鏃?the 椹卞姩 gets the 涓柇 鏉ヨ嚜 鍥轰欢 璇?indicates 娴?鎵撳紑
successfully, the 椹卞姩 灏?send the `STREAM_START` 鍜?`STREAM_CAPTURE`
鍛戒护 鍒?璇锋眰 鍥轰欢 鍒?鍚姩 capturing image frames. `STREAM_CAPTURE`
鍛戒护 queues the 缂撳啿鍖?鍒?鍥轰欢 涓?``缁撴瀯浣?
ipu6_fw_isys_甯buff_set``, 杞欢 鐒跺悗 waits 鐢ㄤ簬 the 涓柇 鍜?
鍝嶅簲 鏉ヨ嚜 鍥轰欢, `PIN_DATA_READY` means 涓€涓?缂撳啿鍖?鏄?ready 鍦?涓€涓?鐗瑰畾
杈撳嚭 pin 鍜?鐒跺悗 杞欢 鍙?return the 缂撳啿鍖?鍒?鐢ㄦ埛.

	  capture 鐢?IPU6 ISYS 椹卞姩.
