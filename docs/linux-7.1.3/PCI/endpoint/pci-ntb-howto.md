
## PCI 闈為€忔槑妗ワ紙NTB锛夌鐐瑰姛鑳斤紙EPF锛夌敤鎴锋寚鍗?

:Author: Kishon Vijay Abraham I <kishon@ti.com>

鏈枃妗ｆ槸涓€浠芥寚鍗楋紝甯姪鐢ㄦ埛浣跨敤 pci-epf-ntb 鍔熻兘椹卞姩涓?ntb_hw_epf 涓绘満椹卞姩鏉ュ疄鐜?NTB 鍔熻兘銆備笅闈㈢粰鍑轰簡
涓绘満渚т笌 EP 渚ч渶瑕侀伒寰殑姝ラ鍒楄〃銆傚叧浜庝娇鐢ㄥ彲閰嶇疆绔偣瀹炵幇 NTB 鐨勭‖浠堕厤缃笌鍐呴儴鏈哄埗锛岃鍙傝
`Documentation/PCI/endpoint/pci-ntb-function.rst`銆?
## 绔偣璁惧


### 绔偣鎺у埗鍣ㄨ澶?

瑕佸疄鐜?NTB 鍔熻兘锛岃嚦灏戦渶瑕佷袱涓鐐规帶鍒跺櫒璁惧銆?
```

	# ls /sys/class/pci_epc/
	2900000.pcie-ep  2910000.pcie-ep

```
```

	# ls /sys/kernel/config/pci_ep/controllers
	2900000.pcie-ep  2910000.pcie-ep


```
### 绔偣鍔熻兘椹卞姩


```

	# ls /sys/bus/pci-epf/drivers
	pci_epf_ntb   pci_epf_ntb

```
```

	# ls /sys/kernel/config/pci_ep/functions
	pci_epf_ntb   pci_epf_ntb


```
### 鍒涘缓 pci-epf-ntb 璁惧


鍙互浣跨敤 configfs 鍒涘缓 PCI 绔偣鍔熻兘璁惧銆傝鍒涘缓
```

	# mount -t configfs none /sys/kernel/config
	# cd /sys/kernel/config/pci_ep/
	# mkdir functions/pci_epf_ntb/func1

```
涓婇潰鐨?"mkdir func1" 鍒涘缓浜嗗皢琚?pci_epf_ntb 椹卞姩鎺㈡祴鐨?pci-epf-ntb 鍔熻兘璁惧銆?
PCI 绔偣妗嗘灦浼氱敤浠ヤ笅鍐呭濉厖璇ョ洰褰?```

	# ls functions/pci_epf_ntb/func1
	baseclass_code    deviceid          msi_interrupts    pci-epf-ntb.0
	progif_code       secondary         subsys_id         vendorid
	cache_line_size   interrupt_pin     msix_interrupts   primary
	revid             subclass_code     subsys_vendor_id

```
PCI 绔偣鍔熻兘椹卞姩浼氬湪璁惧缁戝畾鍒伴┍鍔ㄦ椂锛岀敤榛樿鍊煎～鍏呰繖浜涙潯鐩€俻ci-epf-ntb 椹卞姩浼氬～鍏?```

	# cat functions/pci_epf_ntb/func1/vendorid
	0xffff
	# cat functions/pci_epf_ntb/func1/interrupt_pin
	0x0001


```
### 閰嶇疆 pci-epf-ntb 璁惧


鐢ㄦ埛鍙互浣跨敤鍏?configfs 鏉＄洰閰嶇疆 pci-epf-ntb 璁惧銆備负浜嗘洿鏀?vendorid 涓?deviceid锛岃鎵ц浠ヤ笅
```

	# echo 0x104c > functions/pci_epf_ntb/func1/vendorid
	# echo 0xb00d > functions/pci_epf_ntb/func1/deviceid

```
PCI 绔偣妗嗘灦杩樹細鑷姩鍦ㄥ姛鑳藉睘鎬х洰褰曚腑鍒涘缓涓€涓瓙鐩綍銆傝瀛愮洰褰曚笌鍔熻兘璁惧鐨勫悕绉扮浉鍚岋紝骞剁敤浠ヤ笅
NTB 鐗瑰畾鐨勫唴瀹瑰～鍏?```

	# ls functions/pci_epf_ntb/func1/pci_epf_ntb.0/
	db_count    mw1         mw2         mw3         mw4         num_mws
	spad_count

```
```

	# echo 4 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/db_count
	# echo 128 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/spad_count
	# echo 2 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/num_mws
	# echo 0x100000 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/mw1
	# echo 0x100000 > functions/pci_epf_ntb/func1/pci_epf_ntb.0/mw2

```
### 灏?pci-epf-ntb 璁惧缁戝畾鍒?EP 鎺у埗鍣?

NTB 鍔熻兘璁惧搴旇繛鎺ュ埌杩炴帴鍒颁袱鍙颁富鏈虹殑涓や釜 PCI 绔偣鎺у埗鍣ㄣ€備娇鐢?NTB 鍔熻兘璁惧鍐呴儴鐨?'primary' 鍜?'secondary' 鏉＄洰锛屽皢涓€涓?PCI 绔偣鎺у埗鍣ㄨ繛鎺ュ埌 primary 鎺ュ彛锛屽皢鍙︿竴涓?PCI 绔偣鎺у埗鍣ㄨ繛鎺ュ埌 secondary
```

	# ln -s controllers/2900000.pcie-ep/ functions/pci-epf-ntb/func1/primary
	# ln -s controllers/2910000.pcie-ep/ functions/pci-epf-ntb/func1/secondary

```
瀹屾垚涓婅堪姝ラ鍚庯紝涓や釜 PCI 绔偣鎺у埗鍣ㄩ兘鍑嗗濂戒笌涓绘満寤虹珛閾捐矾銆?

### 鍚姩閾捐矾


涓轰簡璁╃鐐硅澶囦笌涓绘満寤虹珛閾捐矾锛宊start_ 瀛楁搴旇濉厖涓?'1'銆傚浜?NTB锛屼袱涓?PCI 绔偣鎺у埗鍣ㄩ兘
```

	# echo 1 > controllers/2900000.pcie-ep/start
	# echo 1 > controllers/2910000.pcie-ep/start


```
## RootComplex 璁惧


### lspci 杈撳嚭


娉ㄦ剰锛屾澶勫垪鍑虹殑璁惧瀵瑰簲浜庡～鍏呭湪浠ヤ笅浣嶇疆鐨勬暟鍊?```

	# lspci
	0000:00:00.0 PCI bridge: Texas Instruments Device b00d
	0000:01:00.0 RAM memory: Texas Instruments Device b00d


```
### 浣跨敤 ntb_hw_epf 璁惧


涓绘満渚ц蒋浠堕伒寰?Linux 涓爣鍑嗙殑 NTB 杞欢鏋舵瀯銆傛墍鏈夌幇鏈夌殑瀹㈡埛绔晶 NTB 瀹炵敤宸ュ叿锛屽 NTB Transport Client銆?NTB Netdev銆丯TB Ping Pong Test Client 鍜?NTB Tool Test Client锛岄兘鍙互涓?NTB 鍔熻兘璁惧涓€璧蜂娇鐢ㄣ€?
鍏充簬 NTB 鐨勬洿澶氫俊鎭紝璇峰弬瑙?[Non-Transparent Bridge <../../driver-api/ntb>](Non-Transparent Bridge <../../driver-api/ntb>)
