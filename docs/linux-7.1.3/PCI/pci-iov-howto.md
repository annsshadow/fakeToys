
## PCI Express I/O 铏氭嫙鍖栦娇鐢ㄦ寚鍗楋紙Howto锛?

:Copyright: |copy| 2009 Intel Corporation
:Authors: - Yu Zhao <yu.zhao@intel.com>
          - Donald Dutile <ddutile@redhat.com>

## 姒傝堪


### 浠€涔堟槸 SR-IOV

鍗曟牴 I/O 铏氭嫙鍖栵紙Single Root I/O Virtualization锛孲R-IOV锛夋槸 PCI Express 鐨勪竴椤规墿灞曡兘鍔涳紝瀹冧娇涓€涓墿鐞嗚澶囪〃鐜颁负澶氫釜铏氭嫙璁惧銆傜墿鐞嗚澶囩О涓虹墿鐞嗗姛鑳斤紙Physical Function锛孭F锛夛紝鑰岃櫄鎷熻澶囩О涓鸿櫄鎷熷姛鑳斤紙Virtual Function锛孷F锛夈€俈F 鐨勫垎閰嶅彲鐢?PF 閫氳繃灏佽鍦ㄨ鑳藉姏涓殑瀵勫瓨鍣ㄥ姩鎬佹帶鍒躲€傞粯璁ゆ儏鍐典笅锛屾鐗规€ф湭鍚敤锛孭F 琛ㄧ幇涓轰紶缁?PCIe 璁惧銆備竴鏃﹀紑鍚紝姣忎釜 VF 鐨?PCI 閰嶇疆绌洪棿鍙敱鍏惰嚜韬殑鎬荤嚎锛圔us锛夈€佽澶囷紙Device锛変笌鍔熻兘锛團unction锛夊彿锛堝嵆璺敱 ID锛孯outing ID锛夎闂€傛瘡涓?VF 杩樻嫢鏈?PCI 鍐呭瓨绌洪棿锛圥CI Memory Space锛夛紝鐢ㄤ簬鏄犲皠鍏跺瘎瀛樺櫒闆嗐€俈F 璁惧椹卞姩浣滅敤浜庤瀵勫瓨鍣ㄩ泦锛屼粠鑰屼娇鍏惰兘姝ｅ父宸ヤ綔骞惰〃鐜颁负鐪熷疄瀛樺湪鐨?PCI 璁惧銆?
## 鐢ㄦ埛鎸囧崡


### 濡備綍鍚敤 SR-IOV 鑳藉姏

鍚敤 SR-IOV 鏈夊绉嶆柟娉曘€?绗竴绉嶆柟娉曚腑锛岃澶囬┍鍔紙PF 椹卞姩锛夐€氳繃 SR-IOV 鏍稿績鎻愪緵鐨?API 鏉ユ帶鍒惰鑳藉姏鐨勫紑鍚笌鍏抽棴銆傚鏋滅‖浠跺叿澶?SR-IOV 鑳藉姏锛屽姞杞藉叾 PF 椹卞姩灏嗗惎鐢ㄥ畠浠ュ強涓庤 PF 鍏宠仈鐨勬墍鏈?VF銆傛煇浜?PF 椹卞姩闇€瑕佷竴涓ā鍧楀弬鏁版潵璁惧畾瑕佸惎鐢ㄧ殑 VF 鏁伴噺銆?绗簩绉嶆柟娉曚腑锛屽悜 sysfs 鏂囦欢 sriov_numvfs 鍐欏叆锛屽皢鍚敤鎴栧叧闂笌鏌愪釜 PCIe PF 鍏宠仈鐨?VF銆傝鏂规硶瀹炵幇鐨勬槸閫?PF 鐨?VF 鍚敤/鍏抽棴鍊硷紝鑰岀涓€绉嶆柟娉曚綔鐢ㄤ簬鍚屼竴璁惧鐨勬墍鏈?PF銆傛澶栵紝PCI SRIOV 鏍稿績灞備細纭繚鍚敤/鍏抽棴鎿嶄綔鍚堟硶锛屼互鍑忓皯澶氫釜椹卞姩瀵圭浉鍚屾鏌ョ殑閲嶅瀹炵幇锛屼緥濡傦紝鍚敤 VF 鏃舵鏌?numvfs == 0锛岀‘淇?numvfs <= totalvfs銆?绗簩绉嶆柟娉曟槸闈㈠悜鏂扮殑/鏈潵鐨?VF 璁惧鎵€鎺ㄨ崘鐨勬柟娉曘€?
### 濡備綍浣跨敤铏氭嫙鍔熻兘锛圴F锛?
VF 鍦ㄥ唴鏍镐腑琚綋浣滅儹鎻掓嫈鐨?PCI 璁惧澶勭悊锛屽洜姝ゅ簲鑳藉儚鐪熷疄 PCI 璁惧涓€鏍峰伐浣溿€俈F 闇€瑕佷笌鍏舵櫘閫?PCI 璁惧鐩稿悓鐨勮澶囬┍鍔ㄣ€?
## 寮€鍙戣€呮寚鍗?

### SR-IOV API


鍚敤 SR-IOV 鑳藉姏锛?
```

	int pci_enable_sriov(struct pci_dev *dev, int nr_virtfn);

```
`nr_virtfn` 涓鸿鍚敤鐨?VF 鏁伴噺銆?
```

	echo 'nr_virtfn' > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_numvfs

```
绂佺敤 SR-IOV 鑳藉姏锛?
```

	void pci_disable_sriov(struct pci_dev *dev);

```
```

	echo  0 > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_numvfs

```
瑕佸惎鐢ㄤ富鏈轰笂鐢卞吋瀹归┍鍔ㄨ嚜鍔ㄦ帰娴?VF锛堥粯璁よ涓猴級锛岃鍦ㄥ惎鐢?SR-IOV 鑳藉姏涔嬪墠杩愯浠ヤ笅鍛戒护銆?```

	echo 1 > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_drivers_autoprobe

```
瑕佺鐢ㄤ富鏈轰笂鐢卞吋瀹归┍鍔ㄨ嚜鍔ㄦ帰娴?VF锛岃鍦ㄥ惎鐢?SR-IOV 鑳藉姏涔嬪墠杩愯浠ヤ笅鍛戒护銆傛洿鏂版鏉＄洰涓嶄細褰卞搷宸茶鎺㈡祴鍒扮殑 VF銆?```

	echo  0 > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_drivers_autoprobe

```
### 浣跨敤绀轰緥


浠ヤ笅浠ｇ爜鐗囨婕旂ず浜?SR-IOV API 鐨勭敤娉曘€?```

	static int dev_probe(struct pci_dev *dev, const struct pci_device_id *id)
	{
		pci_enable_sriov(dev, NR_VIRTFN);

		...

		return 0;
	}

	static void dev_remove(struct pci_dev *dev)
	{
		pci_disable_sriov(dev);

		...
	}

	static int dev_suspend(struct device *dev)
	{
		...

		return 0;
	}

	static int dev_resume(struct device *dev)
	{
		...

		return 0;
	}

	static void dev_shutdown(struct pci_dev *dev)
	{
		...
	}

	static int dev_sriov_configure(struct pci_dev *dev, int numvfs)
	{
		if (numvfs > 0) {
			...
			pci_enable_sriov(dev, numvfs);
			...
			return numvfs;
		}
		if (numvfs == 0) {
			....
			pci_disable_sriov(dev);
			...
			return 0;
		}
	}

	static struct pci_driver dev_driver = {
		.name =		"SR-IOV Physical Function driver",
		.id_table =	dev_id_table,
		.probe =	dev_probe,
		.remove =	dev_remove,
		.driver.pm =	&dev_pm_ops,
		.shutdown =	dev_shutdown,
		.sriov_configure = dev_sriov_configure,
	};

```
