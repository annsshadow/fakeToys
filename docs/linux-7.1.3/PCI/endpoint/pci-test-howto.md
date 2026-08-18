
## PCI 娴嬭瘯鐢ㄦ埛鎸囧崡


:Author: Kishon Vijay Abraham I <kishon@ti.com>

鏈枃妗ｆ槸涓€浠芥寚鍗楋紝甯姪鐢ㄦ埛浣跨敤 pci-epf-test 鍔熻兘椹卞姩涓?pci_endpoint_test 涓绘満椹卞姩鏉ユ祴璇?PCI銆備笅闈㈢粰鍑哄湪涓绘満渚т笌 EP 渚ч渶瑕侀伒寰殑姝ラ鍒楄〃銆?
## 绔偣璁惧


### 绔偣鎺у埗鍣ㄨ澶?

```

	# ls /sys/class/pci_epc/
	  51000000.pcie_ep

```
```

	# ls /sys/kernel/config/pci_ep/controllers
	  51000000.pcie_ep


```

### 绔偣鍔熻兘椹卞姩


```

	# ls /sys/bus/pci-epf/drivers
	  pci_epf_test

```
```

	# ls /sys/kernel/config/pci_ep/functions
	  pci_epf_test


```

### 鍒涘缓 pci-epf-test 璁惧


鍙互浣跨敤 configfs 鍒涘缓 PCI 绔偣鍔熻兘璁惧銆傝鍒涘缓璁惧锛屾墽琛屼互涓嬪懡浠わ細

```

	# mount -t configfs none /sys/kernel/config
	# cd /sys/kernel/config/pci_ep/
	# mkdir functions/pci_epf_test/func1

```

涓婇潰鐨?"mkdir func1" 灏嗗垱寤?pci-epf-test 鍔熻兘璁惧锛岃璁惧浼氳 pci_epf_test 椹卞姩鎺㈡祴鍒般€?
PCI 绔偣妗嗘灦浼氬湪璇ョ洰褰曚笅濉厖浠ヤ笅鍐呭锛?
```

	# ls functions/pci_epf_test/func1
	  baseclass_code	interrupt_pin	progif_code	subsys_id
	  cache_line_size	msi_interrupts	revid		subsys_vendorid
	  deviceid          	msi_interrupts	subclass_code	vendorid

```

褰撹澶囩粦瀹氬埌椹卞姩鏃讹紝PCI 绔偣鍔熻兘椹卞姩浼氱敤榛樿鍊煎～鍏呰繖浜涙潯鐩€俻ci-epf-test 椹卞姩浼氱敤绫讳技浠ヤ笅鐨勫€煎～鍏呰繖浜涙潯鐩細

```

	# cat functions/pci_epf_test/func1/vendorid
	  0xffff
	# cat functions/pci_epf_test/func1/interrupt_pin
	  0x0001


```

### 閰嶇疆 pci-epf-test 璁惧


鐢ㄦ埛鍙互浣跨敤 configfs 鏉＄洰閰嶇疆 pci-epf-test 璁惧銆傝淇敼鍔熻兘鎵€浣跨敤鐨?vendorid 涓?MSI 涓柇鏁伴噺锛屾墽琛屼互涓嬪懡浠わ細

```

	# echo 0x104c > functions/pci_epf_test/func1/vendorid
	# echo 0xb500 > functions/pci_epf_test/func1/deviceid
	# echo 32 > functions/pci_epf_test/func1/msi_interrupts
	# echo 2048 > functions/pci_epf_test/func1/msix_interrupts

```
```

	# grep . functions/pci_epf_test/func1/pci_epf_test.0/bar?_size
	  functions/pci_epf_test/func1/pci_epf_test.0/bar0_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar1_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar2_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar3_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar4_size:131072
	  functions/pci_epf_test/func1/pci_epf_test.0/bar5_size:1048576

```
```

	# echo 1048576 > functions/pci_epf_test/func1/pci_epf_test.0/bar1_size

```

瑕嗙洊榛樿鐨?BAR 澶у皬鍙兘鍦ㄥ皢 pci-epf-test 璁惧缁戝畾鍒?PCI 绔偣鎺у埗鍣ㄩ┍鍔ㄤ箣鍓嶈繘琛屻€?
娉ㄦ剰锛氭煇浜涚鐐规帶鍒跺櫒鍙兘鍏锋湁鍥哄畾澶у皬鎴栦繚鐣欑殑 BAR锛涘浜庤繖绫绘帶鍒跺櫒锛宑onfigfs 涓搴旂殑 BAR 澶у皬灏嗚蹇界暐銆?

### 灏?pci-epf-test 璁惧缁戝畾鍒?EP 鎺у埗鍣?

涓轰簡璁╃鐐瑰姛鑳借澶囧彲鐢紝蹇呴』灏嗗叾缁戝畾鍒?PCI 绔偣鎺у埗鍣ㄩ┍鍔ㄣ€備娇鐢?configfs 缁戝畾璇ュ姛鑳斤細

```

	# ln -s functions/pci_epf_test/func1 controllers/51000000.pcie_ep/

```

瀹屾垚涓婅堪姝ラ鍚庯紝PCI 绔偣鍗冲彲鍑嗗涓庝富鏈哄缓绔嬮摼璺€?

### 鍚姩閾捐矾


绔偣璁惧瑕佷笌涓绘満寤虹珛閾捐矾锛岄渶鍚?start 灞炴€у啓鍏?1锛?
```

	# echo 1 > controllers/51000000.pcie_ep/start


```

## RootComplex 璁惧


### lspci 杈撳嚭


璇锋敞鎰忥紝姝ゅ鍒楀嚭鐨勮澶囧搴斾簬鍓嶆枃閰嶇疆涓～鍏呯殑鍊硷細

```

	00:00.0 PCI bridge: Texas Instruments Device 8888 (rev 01)
	01:00.0 Unassigned class [ff00]: Texas Instruments Device b500


```

### 浣跨敤绔偣娴嬭瘯鍔熻兘璁惧


tools/testing/selftests/pci_endpoint 涓姞鍏ョ殑 Kselftest 鍙敤浜庤繍琛屾墍鏈夐粯璁ょ殑 PCI 绔偣娴嬭瘯銆傝鏋勫缓 PCI 绔偣鐨?Kselftest锛屾墽琛岋細

```

	# cd <kernel-dir>
	# make -C tools/testing/selftests/pci_endpoint

```
```

	# cd <kernel-dir>
	# make -C tools/testing/selftests/pci_endpoint INSTALL_PATH=/usr/bin install

```

娴嬭瘯绋嬪簭灏嗕綅浜?<rootfs>/usr/bin/ 鐩綍涓嬨€?
#### Kselftest 杈撳嚭

```

	# pci_endpoint_test
	TAP version 13
	1..16
	# Starting 16 tests from 9 test cases.
	#  RUN           pci_ep_bar.BAR0.BAR_TEST ...
	#            OK  pci_ep_bar.BAR0.BAR_TEST
	ok 1 pci_ep_bar.BAR0.BAR_TEST
	#  RUN           pci_ep_bar.BAR1.BAR_TEST ...
	#            OK  pci_ep_bar.BAR1.BAR_TEST
	ok 2 pci_ep_bar.BAR1.BAR_TEST
	#  RUN           pci_ep_bar.BAR2.BAR_TEST ...
	#            OK  pci_ep_bar.BAR2.BAR_TEST
	ok 3 pci_ep_bar.BAR2.BAR_TEST
	#  RUN           pci_ep_bar.BAR3.BAR_TEST ...
	#            OK  pci_ep_bar.BAR3.BAR_TEST
	ok 4 pci_ep_bar.BAR3.BAR_TEST
	#  RUN           pci_ep_bar.BAR4.BAR_TEST ...
	#            OK  pci_ep_bar.BAR4.BAR_TEST
	ok 5 pci_ep_bar.BAR4.BAR_TEST
	#  RUN           pci_ep_bar.BAR5.BAR_TEST ...
	#            OK  pci_ep_bar.BAR5.BAR_TEST
	ok 6 pci_ep_bar.BAR5.BAR_TEST
	#  RUN           pci_ep_basic.CONSECUTIVE_BAR_TEST ...
	#            OK  pci_ep_basic.CONSECUTIVE_BAR_TEST
	ok 7 pci_ep_basic.CONSECUTIVE_BAR_TEST
	#  RUN           pci_ep_basic.LEGACY_IRQ_TEST ...
	#            OK  pci_ep_basic.LEGACY_IRQ_TEST
	ok 8 pci_ep_basic.LEGACY_IRQ_TEST
	#  RUN           pci_ep_basic.MSI_TEST ...
	#            OK  pci_ep_basic.MSI_TEST
	ok 9 pci_ep_basic.MSI_TEST
	#  RUN           pci_ep_basic.MSIX_TEST ...
	#            OK  pci_ep_basic.MSIX_TEST
	ok 10 pci_ep_basic.MSIX_TEST
	#  RUN           pci_ep_data_transfer.memcpy.READ_TEST ...
	#            OK  pci_ep_data_transfer.memcpy.READ_TEST
	ok 11 pci_ep_data_transfer.memcpy.READ_TEST
	#  RUN           pci_ep_data_transfer.memcpy.WRITE_TEST ...
	#            OK  pci_ep_data_transfer.memcpy.WRITE_TEST
	ok 12 pci_ep_data_transfer.memcpy.WRITE_TEST
	#  RUN           pci_ep_data_transfer.memcpy.COPY_TEST ...
	#            OK  pci_ep_data_transfer.memcpy.COPY_TEST
	ok 13 pci_ep_data_transfer.memcpy.COPY_TEST
	#  RUN           pci_ep_data_transfer.dma.READ_TEST ...
	#            OK  pci_ep_data_transfer.dma.READ_TEST
	ok 14 pci_ep_data_transfer.dma.READ_TEST
	#  RUN           pci_ep_data_transfer.dma.WRITE_TEST ...
	#            OK  pci_ep_data_transfer.dma.WRITE_TEST
	ok 15 pci_ep_data_transfer.dma.WRITE_TEST
	#  RUN           pci_ep_data_transfer.dma.COPY_TEST ...
	#            OK  pci_ep_data_transfer.dma.COPY_TEST
	ok 16 pci_ep_data_transfer.dma.COPY_TEST
	# PASSED: 16 / 16 tests passed.
	# Totals: pass:16 fail:0 xfail:0 xpass:0 skip:0 error:0


```

瀵逛簬澶у鏁版敮鎸?DMA 鐨勭鐐规帶鍒跺櫒锛屾祴璇曠敤渚?16锛坧ci_ep_data_transfer.dma.COPY_TEST锛変細鍥犵己灏戝熀浜?DMA 鐨?MEMCPY 鑰屽け璐ャ€傚浜庤繖绫绘帶鍒跺櫒锛屽缓璁娇鐢ㄤ互涓嬪懡浠よ烦杩囪娴嬭瘯鐢ㄤ緥锛?
```

	# pci_endpoint_test -f pci_ep_bar -f pci_ep_basic -v memcpy -T COPY_TEST -v dma

```

#### Kselftest EP Doorbell


濡傛灉绔偣 MSI 鎺у埗鍣ㄧ敤浜?doorbell锛堥棬閾冿級鐢ㄤ緥锛岃杩愯浠ヤ笅鍛戒护杩涜娴嬭瘯锛?
	# pci_endpoint_test -f pcie_ep_doorbell

	# Starting 1 tests from 1 test cases.
	#  RUN           pcie_ep_doorbell.DOORBELL_TEST ...
	#            OK  pcie_ep_doorbell.DOORBELL_TEST
	ok 1 pcie_ep_doorbell.DOORBELL_TEST
	# PASSED: 1 / 1 tests passed.
	# Totals: pass:1 fail:0 xfail:0 xpass:0 skip:0 error:0
