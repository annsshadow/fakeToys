
## 浣跨敤 CONFIGFS 閰嶇疆 PCI Endpoint

:Author: Kishon Vijay Abraham I <kishon@ti.com>

PCI Endpoint Core 閫氳繃 configfs 鍏ュ彛锛坧ci_ep锛夋潵閰嶇疆 PCI endpoint function锛?骞跺皢鍏朵笌 endpoint controller 缁戝畾銆傦紙鍏充簬閰嶇疆 PCI Endpoint Function 鐨勫叾瀹?鏈哄埗锛岃鍙傞槄 [^1^]銆傦級

## 鎸傝浇 configfs

PCI Endpoint Core 灞備細鍦ㄥ凡鎸傝浇鐨?configfs 涓垱寤?pci_ep 鐩綍
```

	mount -t configfs none /sys/kernel/config

```
## 鐩綍缁撴瀯

pci_ep configfs 鍦ㄥ叾鏍圭洰褰曚笅鏈変袱涓洰褰曪細controllers 鍜?functions銆傜郴缁熶腑
瀛樺湪鐨勬瘡涓?EPC 璁惧閮戒細鍦?**controllers** 鐩綍涓嬫湁涓€椤癸紝绯荤粺涓瓨鍦ㄧ殑姣忎釜
EPF 椹卞姩閮戒細鍦?**functions** 鐩綍涓嬫湁涓€椤广€?```

	/sys/kernel/config/pci_ep/
		.. controllers/
		.. functions/

```
## 鍒涘缓 EPF 璁惧

姣忎釜宸叉敞鍐岀殑 EPF 椹卞姩閮戒細鍒楀湪 controllers 鐩綍涓嬨€備笌 EPF 椹卞姩瀵瑰簲鐨勯」鐢?EPF
鏍稿績鍒涘缓銆?```

	/sys/kernel/config/pci_ep/functions/
		.. <EPF Driver1>/
			... <EPF Device 11>/
			... <EPF Device 21>/
			... <EPF Device 31>/
		.. <EPF Driver2>/
			... <EPF Device 12>/
			... <EPF Device 22>/

```
涓轰簡鍒涘缓鐢?<EPF Driver> 鎺㈡祴鐨勭被鍨嬬殑 <EPF device>锛岀敤鎴峰繀椤诲湪 <EPF DriverN>
鍐呭垱寤轰竴涓洰褰曘€?
姣忎釜 <EPF device> 鐩綍閮藉寘鍚互涓嬪彲鐢ㄤ簬閰嶇疆 endpoint function 鏍囧噯閰嶇疆澶寸殑
椤广€傦紙杩欎簺椤瑰湪鍒涘缓浠讳綍鏂扮殑 <EPF Device> 鏃剁敱妗嗘灦鍒涘缓锛?```

		.. <EPF Driver1>/
			... <EPF Device 11>/
				... vendorid
				... deviceid
				... revid
				... progif_code
				... subclass_code
				... baseclass_code
				... cache_line_size
				... subsys_vendor_id
				... subsys_id
				... interrupt_pin
			        ... <Symlink EPF Device 31>/
                                ... primary/
			                ... <Symlink EPC Device1>/
                                ... secondary/
			                ... <Symlink EPC Device2>/

```
濡傛灉涓€涓?EPF 璁惧闇€瑕佸叧鑱?2 涓?EPC锛堜緥濡傞潪閫忔槑妗ョ殑鎯呭喌锛夛紝鍒欏簲灏嗚繛鎺ュ埌涓?锛坧rimary锛夋帴鍙ｇ殑 endpoint controller 鐨勭鍙烽摼鎺ユ坊鍔犲埌 'primary' 鐩綍涓紝灏?杩炴帴鍒颁粠锛坰econdary锛夋帴鍙ｇ殑 endpoint controller 鐨勭鍙烽摼鎺ユ坊鍔犲埌 'secondary'
鐩綍涓€?
<EPF Device> 鐩綍鍙互鍖呭惈鎸囧悜鍏跺畠 <EPF Device> 鐨勭鍙烽摼鎺ュ垪琛紙<Symlink EPF
Device 31>锛夈€傝繖浜涚鍙烽摼鎺ュ簲鐢辩敤鎴峰垱寤猴紝鐢ㄤ簬琛ㄧず缁戝畾鍒扮墿鐞嗗姛鑳界殑铏氭嫙鍔熻兘銆傚湪
涓婅堪鐩綍缁撴瀯涓紝<EPF Device 11> 鏄墿鐞嗗姛鑳斤紝<EPF Device 31> 鏄櫄鎷熷姛鑳姐€備竴涓?EPF 璁惧涓€鏃﹂摼鎺ュ埌鍙︿竴涓?EPF 璁惧锛屽氨涓嶈兘鍐嶉摼鎺ュ埌 EPC 璁惧銆?
## EPC 璁惧

姣忎釜宸叉敞鍐岀殑 EPC 璁惧閮戒細鍒楀湪 controllers 鐩綍涓嬨€備笌 EPC 璁惧瀵瑰簲鐨勯」鐢?EPC
鏍稿績鍒涘缓銆?```

	/sys/kernel/config/pci_ep/controllers/
		.. <EPC Device1>/
			... <Symlink EPF Device11>/
			... <Symlink EPF Device12>/
			... start
		.. <EPC Device2>/
			... <Symlink EPF Device21>/
			... <Symlink EPF Device22>/
			... start

```
<EPC Device> 鐩綍浼氬寘鍚竴涓寚鍚?<EPF Device> 鐨勭鍙烽摼鎺ュ垪琛ㄣ€傝繖浜涚鍙烽摼鎺ュ簲
鐢辩敤鎴峰垱寤猴紝鐢ㄤ簬琛ㄧず endpoint 璁惧涓殑鍔熻兘銆傚彧鏈夎〃绀虹墿鐞嗗姛鑳界殑 <EPF Device>
鎵嶈兘閾炬帴鍒?EPC 璁惧銆?
<EPC Device> 鐩綍杩樹細鏈変竴涓?**start** 瀛楁銆備竴鏃﹀悜璇ュ瓧娈靛啓鍏?"1"锛宔ndpoint
璁惧灏卞噯澶囧ソ涓庝富鏈哄缓绔嬮摼璺€傝繖閫氬父鏄湪鎵€鏈?EPF 璁惧鍒涘缓骞堕摼鎺ュ埌 EPC 璁惧涔嬪悗
杩涜鐨勩€?```

			 | controllers/
				| <Directory: EPC name>/
					| <Symbolic Link: Function>
					| start
			 | functions/
				| <Directory: EPF driver>/
					| <Directory: EPF device>/
						| vendorid
						| deviceid
						| revid
						| progif_code
						| subclass_code
						| baseclass_code
						| cache_line_size
						| subsys_vendor_id
						| subsys_id
						| interrupt_pin
						| function

```
[^1^] Documentation/PCI/endpoint/pci-endpoint.rst
