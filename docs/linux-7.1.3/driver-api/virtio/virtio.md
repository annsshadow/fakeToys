


## Linux 涓婄殑 Virtio


## 绠€浠?

Virtio 鏄竴涓紑鏀炬爣鍑嗭紝瀹氫箟浜嗕笉鍚岀被鍨嬮┍鍔ㄤ笌璁惧涔嬮棿鐨勯€氫俊鍗忚锛岃鍙傝
virtio 瑙勮寖鐨勭 5 绔狅紙鈥滆澶囩被鍨嬧€濓級锛坄[^1^]`_锛夈€傚畠鏈€鍒濇槸浣滀负鐢辩鐞嗙▼搴?瀹炵幇鐨勫崐铏氭嫙鍖栵紙paravirtualized锛夎澶囩殑鏍囧噯寮€鍙戠殑锛屼絾涔熷彲鐢ㄤ簬灏嗕换浣曞吋瀹圭殑
璁惧锛堢湡瀹炵殑鎴栦豢鐪熺殑锛変笌椹卞姩鎺ュ彛杩炴帴銆?
鍑轰簬璇存槑鐩殑锛屾湰鏂囨。灏嗚仛鐒︿簬涓€涓父瑙佹儏鍐碉細杩愯鍦ㄨ櫄鎷熸満涓殑 Linux 鍐呮牳锛?浣跨敤绠＄悊绋嬪簭鎻愪緵鐨勫崐铏氭嫙鍖栬澶囷紝绠＄悊绋嬪簭閫氳繃 PCI 绛夋爣鍑嗘満鍒跺皢瀹冧滑鏆撮湶涓?virtio 璁惧銆?
## 璁惧 - 椹卞姩閫氫俊锛歷irtqueue


灏界 virtio 璁惧瀹為檯涓婃槸绠＄悊绋嬪簭涓殑涓€涓娊璞″眰锛屼絾瀹冧滑琚毚闇茬粰瀹㈡埛鏈猴紝
灏卞ソ鍍忓畠浠槸浣跨敤鐗瑰畾浼犺緭鏂规硶鈥斺€擯CI銆丮MIO 鎴?CCW鈥斺€旂殑鐗╃悊璁惧锛岃繖鐙珛浜?璁惧鏈韩銆倂irtio 瑙勮寖璇︾粏瀹氫箟浜嗚繖浜涗紶杈撴柟娉曪紝鍖呮嫭璁惧鍙戠幇銆佽兘鍔涗笌涓柇澶勭悊銆?
瀹㈡埛鏈烘搷浣滅郴缁熶腑鐨勯┍鍔ㄤ笌绠＄悊绋嬪簭涓殑璁惧涔嬮棿鐨勯€氫俊鏄€氳繃鍏变韩鍐呭瓨锛堣繖姝ｆ槸
virtio 璁惧濡傛楂樻晥鐨勫師鍥狅級瀹屾垚鐨勶紝浣跨敤绉颁负 virtqueue 鐨勪笓鐢ㄦ暟鎹粨鏋勶紝瀹冧滑
瀹為檯涓婃槸缂撳啿鍖烘弿杩扮鐨勭幆褰㈢紦鍐插尯锛坮ing buffer锛塠#f1]_锛岀被浼间簬缃戠粶璁惧涓?浣跨敤鐨勯偅浜涳細

    :identifiers: struct vring_desc

鎻忚堪绗︽寚鍚戠殑鎵€鏈夌紦鍐插尯閮界敱瀹㈡埛鏈哄垎閰嶏紝骞剁敱涓绘満鐢ㄤ簬璇诲彇鎴栧啓鍏ワ紝浣嗕笉鑳藉悓鏃?鐢ㄤ簬涓よ€呫€?
鏈夊叧 virtqueue 鐨勫弬鑰冨畾涔夛紝璇峰弬鑰?virtio 瑙勮寖鐨勭 2.5 鑺傦紙鈥淰irtqueues鈥濓級
锛坄[^1^]`_锛夛紝浠ュ強鍗氬鏂囩珷鈥淰irtqueues and virtio ring: How the data travels鈥?锛坄[^2^]`_锛夛紝浜嗚В涓绘満璁惧涓庡鎴锋満椹卞姩濡備綍閫氫俊鐨勫浘瑙ｆ瑙堛€?
`vring_virtqueue` 缁撴瀯浣撳缓妯′簡涓€涓?virtqueue锛屽寘鎷幆褰㈢紦鍐插尯涓庣鐞嗘暟鎹€?宓屽叆璇ョ粨鏋勪綋涓殑鏄?`virtqueue` 缁撴瀯浣擄紝瀹冩槸鏈€缁堣 virtio 椹卞姩浣跨敤鐨勬暟鎹?缁撴瀯锛?
    :identifiers: struct virtqueue

璇ョ粨鏋勪綋鎸囧悜鐨勫洖璋冨嚱鏁板湪璁惧娑堣垂浜嗛┍鍔ㄦ彁渚涚殑缂撳啿鍖烘椂琚Е鍙戙€傛洿鍏蜂綋鍦拌锛?瑙﹀彂灏嗘槸绠＄悊绋嬪簭鍙戝嚭鐨勪腑鏂紙鍙傝 vring_interrupt()锛夈€備腑鏂姹傚鐞嗙▼搴忓湪
virtqueue 璁剧疆杩囩▼锛堜紶杈撶浉鍏筹級鏈熼棿涓?virtqueue 娉ㄥ唽銆?
    :identifiers: vring_interrupt


## 璁惧鍙戠幇涓庢帰娴?

鍦ㄥ唴鏍镐腑锛寁irtio 鏍稿績鍖呭惈 virtio 鎬荤嚎椹卞姩浠ュ強浼犺緭鐩稿叧鐨勯┍鍔紝濡?`virtio-pci`
鍜?`virtio-mmio`銆傜劧鍚庤繕鏈夐拡瀵圭壒瀹氳澶囩被鍨嬬殑鍚勪釜 virtio 椹卞姩锛屽畠浠敞鍐屽埌
virtio 鎬荤嚎椹卞姩銆?
鍐呮牳濡備綍鎵惧埌骞堕厤缃?virtio 璁惧鍙栧喅浜庣鐞嗙▼搴忓浣曞畾涔夊畠銆備互 `QEMU virtio-console
<https://gitlab.com/qemu-project/qemu/-/blob/master/hw/char/virtio-console.c>`__
璁惧涓轰緥銆傚綋浣跨敤 PCI 浣滀负浼犺緭鏂规硶鏃讹紝璁惧灏嗕互鍘傚晢 0x1af4锛圧ed Hat, Inc.锛?鍜岃澶?id 0x1003锛坴irtio console锛夊嚭鐜板湪 PCI 鎬荤嚎涓婏紝濡傝鑼冧腑鎵€瀹氫箟锛屽洜姝?鍐呮牳浼氬儚瀵瑰緟浠讳綍鍏朵粬 PCI 璁惧涓€鏍锋娴嬪畠銆?
鍦?PCI 鏋氫妇杩囩▼涓紝濡傛灉鍙戠幇鏌愪釜璁惧鍖归厤 virtio-pci 椹卞姩锛堟牴鎹?virtio-pci
璁惧琛紝浠讳綍 PCI
```

	/* Qumranet donated their vendor ID for devices 0x1000 thru 0x10FF. */
	static const struct pci_device_id virtio_pci_id_table[] = {
		{ PCI_DEVICE(PCI_VENDOR_ID_REDHAT_QUMRANET, PCI_ANY_ID) },
		{ 0 }
	};

```
閭ｄ箞 virtio-pci 椹卞姩浼氳鎺㈡祴锛屽苟涓斿鏋滄帰娴嬮『鍒╋紝
```

	static int virtio_pci_probe(struct pci_dev *pci_dev,
				    const struct pci_device_id *id)
	{
		...

		if (force_legacy) {
			rc = virtio_pci_legacy_probe(vp_dev);
			/* Also try modern mode if we can't map BAR0 (no IO space). */
			if (rc == -ENODEV || rc == -ENOMEM)
				rc = virtio_pci_modern_probe(vp_dev);
			if (rc)
				goto err_probe;
		} else {
			rc = virtio_pci_modern_probe(vp_dev);
			if (rc == -ENODEV)
				rc = virtio_pci_legacy_probe(vp_dev);
			if (rc)
				goto err_probe;
		}

		...

		rc = register_virtio_device(&vp_dev->vdev);

```
褰撹澶囨敞鍐屽埌 virtio 鎬荤嚎鏃讹紝鍐呮牳灏嗗湪鎬荤嚎涓婂鎵捐兘澶熷鐞嗚璁惧鐨勯┍鍔紝骞惰皟鐢?璇ラ┍鍔ㄧ殑 `probe` 鏂规硶銆?
姝ゆ椂锛寁irtqueue 灏嗛€氳繃璋冪敤鐩稿簲鐨?`virtio_find` 杈呭姪鍑芥暟鏉ュ垎閰嶅拰閰嶇疆锛屼緥濡?virtio_find_single_vq() 鎴?virtio_find_vqs()锛屽畠浠渶缁堜細璋冪敤涓€涓紶杈撶浉鍏崇殑
`find_vqs` 鏂规硶銆?

## 鍙傝€?

_`[^1^]` Virtio Spec v1.2:
https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html


_`[^2^]` Virtqueues and virtio ring: How the data travels
https://www.redhat.com/en/blog/virtqueues-and-virtio-ring-how-data-travels
