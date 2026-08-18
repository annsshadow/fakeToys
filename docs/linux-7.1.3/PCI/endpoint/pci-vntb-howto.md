
## PCI 闈為€忔槑妗ワ紙NTB锛夌鐐瑰姛鑳斤紙EPF锛夌敤鎴锋寚鍗?

:浣滆€? Frank Li <Frank.Li@nxp.com>

鏈枃妗ｆ槸涓€浠芥寚鍗楋紝甯姪鐢ㄦ埛浣跨敤 pci-epf-vntb 鍔熻兘椹卞姩鍜?ntb_hw_epf 涓绘満椹卞姩
鏉ュ疄鐜?NTB 鍔熻兘銆備笅闈㈢粰鍑轰簡鍦ㄤ富鏈轰晶鍜?EP 渚ч渶瑕侀伒寰殑姝ラ鍒楄〃銆傛湁鍏充娇鐢?鍙厤缃鐐圭殑 NTB 鐨勭‖浠堕厤缃笌鍐呴儴鏈哄埗锛岃鍙傝
Documentation/PCI/endpoint/pci-vntb-function.rst銆?
## 绔偣璁惧


### 绔偣鎺у埗鍣ㄨ澶?

```

        # ls /sys/class/pci_epc/
          5f010000.pcie_ep

```
```

        # ls /sys/kernel/config/pci_ep/controllers
          5f010000.pcie_ep

```
### 绔偣鍔熻兘椹卞姩


```

	# ls /sys/bus/pci-epf/drivers
	pci_epf_ntb  pci_epf_test  pci_epf_vntb

```
```

	# ls /sys/kernel/config/pci_ep/functions
	pci_epf_ntb  pci_epf_test  pci_epf_vntb


```
### 鍒涘缓 pci-epf-vntb 璁惧


PCI 绔偣鍔熻兘璁惧鍙互浣跨敤 configfs 鍒涘缓銆傝鍒涘缓
```

	# mount -t configfs none /sys/kernel/config
	# cd /sys/kernel/config/pci_ep/
	# mkdir functions/pci_epf_vntb/func1

```
涓婇潰鐨?"mkdir func1" 鍒涘缓浜嗗皢鐢?pci_epf_vntb 椹卞姩鎺㈡祴鐨?pci-epf-vntb 鍔熻兘璁惧銆?
PCI 绔偣妗嗘灦浼氫娇鐢ㄤ互涓嬪唴瀹瑰～鍏呰鐩綍
```

	# ls functions/pci_epf_vntb/func1
	baseclass_code    deviceid          msi_interrupts    pci-epf-vntb.0
	progif_code       secondary         subsys_id         vendorid
	cache_line_size   interrupt_pin     msix_interrupts   primary
	revid             subclass_code     subsys_vendor_id

```
PCI 绔偣鍔熻兘椹卞姩鍦ㄨ澶囩粦瀹氬埌椹卞姩鏃讹紝浼氱敤榛樿鍊煎～鍏呰繖浜涙潯鐩€俻ci-epf-vntb
椹卞姩浼氬～鍏?```

	# cat functions/pci_epf_vntb/func1/vendorid
	0xffff
	# cat functions/pci_epf_vntb/func1/interrupt_pin
	0x0001


```
### 閰嶇疆 pci-epf-vntb 璁惧


鐢ㄦ埛鍙互浣跨敤鍏?configfs 鏉＄洰閰嶇疆 pci-epf-vntb 璁惧銆備负浜嗘洿鏀?vendorid 鍜?deviceid锛岃鎵ц浠ヤ笅
```

	# echo 0x1957 > functions/pci_epf_vntb/func1/vendorid
	# echo 0x0809 > functions/pci_epf_vntb/func1/deviceid

```
PCI 绔偣妗嗘灦杩樹細鍦ㄥ姛鑳藉睘鎬х洰褰曚腑鑷姩鍒涘缓涓€涓瓙鐩綍銆傝瀛愮洰褰曚笌鍔熻兘璁惧鐨?鍚嶇О鐩稿悓锛屽苟濉厖鏈変互涓?NTB 鐗瑰畾鐨?```

	# ls functions/pci_epf_vntb/func1/pci_epf_vntb.0/
	ctrl_bar  db_count  mw1_bar  mw2_bar  mw3_bar  mw4_bar	spad_count
	db_bar	  mw1	    mw2      mw3      mw4      num_mws	vbus_number
	vntb_vid  vntb_pid

```
```

	# echo 4 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/db_count
	# echo 128 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/spad_count
	# echo 1 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/num_mws
	# echo 0x100000 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/mw1

```
榛樿鎯呭喌涓嬶紝姣忎釜鏋勯€狅紙construct锛変細鎸夐渶骞舵寜椤哄簭鍒嗛厤涓€涓?BAR銆傚鏋滃钩鍙伴渶瑕?鐗瑰畾鐨?BAR 璁剧疆锛屽彲浠ヤ娇鐢ㄧ浉鍏崇殑 `XYZ_bar` 鏉＄洰灏?BAR 鍒嗛厤缁欐瘡涓瀯閫犮€?
```

	# echo 0x1957 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/vntb_vid
	# echo 0x080A > functions/pci_epf_vntb/func1/pci_epf_vntb.0/vntb_pid
	# echo 0x10 > functions/pci_epf_vntb/func1/pci_epf_vntb.0/vbus_number

```
### 灏?pci-epf-vntb 璁惧缁戝畾鍒?EP 鎺у埗鍣?

NTB 鍔熻兘璁惧搴旈檮鐫€鍒拌繛鎺ュ埌涓绘満鐨?PCI 绔偣鎺у埗鍣ㄣ€?
	# ln -s controllers/5f010000.pcie_ep functions/pci_epf_vntb/func1/primary

瀹屾垚涓婅堪姝ラ鍚庯紝PCI 绔偣鎺у埗鍣ㄥ凡鍑嗗濂戒笌涓绘満寤虹珛閾捐矾銆?
### 鍚姩閾捐矾


涓轰簡璁╃鐐硅澶囦笌涓绘満寤虹珛閾捐矾锛屽簲灏?_start_ 瀛楁濉厖涓?'1'銆傚浜?NTB锛屼袱涓?PCI 绔偣鎺у埗鍣ㄩ兘闇€瑕?```

	# echo 1 > controllers/5f010000.pcie_ep/start

```
## 鏍瑰鍚堜綋锛圧ootComplex锛夎澶?

### 涓绘満渚х殑 lspci 杈撳嚭


娉ㄦ剰锛屾澶勫垪鍑虹殑璁惧瀵瑰簲浜庡湪浠ヤ笅浣嶇疆濉厖鐨勫€?```

	# lspci
        00:00.0 PCI bridge: Freescale Semiconductor Inc Device 0000 (rev 01)
        01:00.0 RAM memory: Freescale Semiconductor Inc Device 0809

```
## 绔偣璁惧 / 铏氭嫙 PCI 鎬荤嚎


### EP 渚?/ 铏氭嫙 PCI 鎬荤嚎鐨?lspci 杈撳嚭


娉ㄦ剰锛屾澶勫垪鍑虹殑璁惧瀵瑰簲浜庡湪浠ヤ笅浣嶇疆濉厖鐨勫€?```

        # lspci
        10:00.0 Unassigned class [ffff]: Dawicontrol Computersysteme GmbH Device 1234 (rev ff)

```
### 浣跨敤 ntb_hw_epf 璁惧


涓绘満渚ц蒋浠堕伒寰?Linux 涓爣鍑嗙殑 NTB 杞欢鏋舵瀯銆傛墍鏈夌幇鏈夌殑瀹㈡埛绔?NTB 瀹炵敤宸ュ叿锛?濡?NTB Transport Client銆丯TB Netdev銆丯TB Ping Pong Test Client 鍜?NTB Tool
Test Client锛岄兘鍙互涓?NTB 鍔熻兘璁惧涓€璧蜂娇鐢ㄣ€?
鏈夊叧 NTB 鐨勬洿澶氫俊鎭紝璇峰弬瑙?[Non-Transparent Bridge <../../driver-api/ntb>](Non-Transparent Bridge <../../driver-api/ntb>)
