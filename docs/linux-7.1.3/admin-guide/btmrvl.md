## btmrvl 椹卞姩


鎵€鏈夊懡浠ら兘閫氳繃 debugfs 鎺ュ彛浣跨敤銆?
## 璁剧疆/鑾峰彇椹卞姩閰嶇疆


璺緞锛?debug/btmrvl/config/

gpiogap=[n], hscfgcmd
```
	bit 8:0  -- Gap
	bit 16:8 -- GPIO

	鍏朵腑 GPIO 鏄敤浜庡敜閱掍富鏈虹殑 GPIO 寮曡剼缂栧彿銆?	鍙互鏄换鎰忔湁鏁?GPIO 寮曡剼鍙凤紙渚嬪 0-7锛夋垨 0xff锛堟鏃舵敼鐢?SDIO 鎺ュ彛
	鏉ュ敜閱掞級銆?
	鍏朵腑 Gap 鏄敜閱掍俊鍙蜂笌鍞ら啋浜嬩欢涔嬮棿鐨勯棿闅旓紙鍗曚綅涓烘绉掞級锛屾垨涓?0xff
	琛ㄧず鐗规畩鐨勫涓荤潯鐪狅紙host sleep锛夎缃€?
	鐢ㄦ硶::

		# 浣跨敤 SDIO 鎺ュ彛鍞ら啋涓绘満骞跺皢 GAP 璁句负 0x80锛?		echo 0xff80 > /debug/btmrvl/config/gpiogap
		echo 1 > /debug/btmrvl/config/hscfgcmd

		# 浣跨敤 GPIO 寮曡剼 #3 鍞ら啋涓绘満骞跺皢 GAP 璁句负 0xff锛?		echo 0x03ff >  /debug/btmrvl/config/gpiogap
		echo 1 > /debug/btmrvl/config/hscfgcmd

```
psmode=[n], pscmd
	杩欎簺鍛戒护鐢ㄤ簬鍚敤/绂佺敤鑷姩鐫＄湢妯″紡

```

			1 	-- 鍚敤鑷姩鐫＄湢妯″紡
			0 	-- 绂佺敤鑷姩鐫＄湢妯″紡

	鐢ㄦ硶::

		# 鍚敤鑷姩鐫＄湢妯″紡
		echo 1 > /debug/btmrvl/config/psmode
		echo 1 > /debug/btmrvl/config/pscmd

		# 绂佺敤鑷姩鐫＄湢妯″紡
		echo 0 > /debug/btmrvl/config/psmode
		echo 1 > /debug/btmrvl/config/pscmd


```
hsmode=[n], hscmd
	杩欎簺鍛戒护鐢ㄤ簬鍚敤瀹夸富鐫＄湢鎴栧敜閱掑浐浠?
```

			1	-- 鍚敤瀹夸富鐫＄湢
			0	-- 鍞ら啋鍥轰欢

	鐢ㄦ硶::

		# 鍚敤瀹夸富鐫＄湢
		echo 1 > /debug/btmrvl/config/hsmode
		echo 1 > /debug/btmrvl/config/hscmd

		# 鍞ら啋鍥轰欢
		echo 0 > /debug/btmrvl/config/hsmode
		echo 1 > /debug/btmrvl/config/hscmd


```
## 鑾峰彇椹卞姩鐘舵€?

璺緞锛?debug/btmrvl/status/

```

	cat /debug/btmrvl/status/<args>

```
鍏朵腑 args 涓猴細

curpsmode
	璇ュ懡浠ゆ樉绀哄綋鍓嶇殑鑷姩鐫＄湢鐘舵€併€?
psstate
	璇ュ懡浠ゆ樉绀虹數婧愯妭鐪佺姸鎬併€?
hsstate
	璇ュ懡浠ゆ樉绀哄涓荤潯鐪犵姸鎬併€?
txdnldrdy
	璇ュ懡浠ゆ樉绀?Tx 涓嬭浇灏辩华鏍囧織鐨勫€笺€?
## 鍙戝嚭鍘熷 HCI 鍛戒护


浣跨敤 hcitool 鍙戝嚭鍘熷 HCI 鍛戒护锛岃鍙傞槄 hcitool 鎵嬪唽

```

	Hcitool cmd <ogf> <ocf> [Parameters]

```
```

	hcitool cmd 0x3f 0x5b 0xf5 0x01 0x00    --鍚敤鍏ㄩ儴鎺ュ彛
	hcitool cmd 0x3f 0x5b 0xf5 0x01 0x01    --鍚敤 Wlan 鎺ュ彛
	hcitool cmd 0x3f 0x5b 0xf5 0x01 0x02    --鍚敤 BT 鎺ュ彛
	hcitool cmd 0x3f 0x5b 0xf5 0x00 0x00    --绂佺敤鍏ㄩ儴鎺ュ彛
	hcitool cmd 0x3f 0x5b 0xf5 0x00 0x01    --绂佺敤 Wlan 鎺ュ彛
	hcitool cmd 0x3f 0x5b 0xf5 0x00 0x02    --绂佺敤 BT 鎺ュ彛

```
## SD8688 鍥轰欢


闀滃儚鏂囦欢锛?
- /lib/firmware/sd8688_helper.bin
- /lib/firmware/sd8688.bin


杩欎簺闀滃儚鍙互浠庝互涓嬪湴鍧€涓嬭浇锛?
git.infradead.org/users/dwmw2/linux-firmware.git/libertas/
