
## VFIO 铏氭嫙璁惧


鏀寔鐨勮澶囩被鍨嬶細

  - KVM_DEV_TYPE_VFIO

姣忎釜 VM 鍙兘鍒涘缓涓€涓?VFIO 瀹炰緥銆傛墍鍒涘缓鐨勮澶囪窡韪?VM 姝ｅ湪浣跨敤鐨?VFIO 鏂囦欢锛坓roup 鎴?device锛夛紝浠ュ強閭ｄ簺瀵?VM 鐨勬纭€у拰鍔犻€熻嚦鍏抽噸瑕佺殑 group/device 鐗规€с€傞殢鐫€ group/device 琚?VM 鍚敤鎴栫鐢紝搴斿綋灏卞畠浠殑瀛樺湪鏇存柊 KVM銆傚綋鍚?KVM 娉ㄥ唽鏃讹紝KVM 浼氭寔鏈変竴涓 VFIO 鏂囦欢鐨勫紩鐢ㄣ€?
缁勶細
  KVM_DEV_VFIO_FILE
	alias: KVM_DEV_VFIO_GROUP

KVM_DEV_VFIO_FILE 灞炴€э細
  KVM_DEV_VFIO_FILE_ADD锛氬悜 VFIO-KVM 璁惧璺熻釜涓坊鍔犱竴涓?VFIO 鏂囦欢锛坓roup/device锛?
	kvm_device_attr.addr 鎸囧悜 VFIO 鏂囦欢鐨?int32_t 鏂囦欢鎻忚堪绗︺€?
  KVM_DEV_VFIO_FILE_DEL锛氫粠 VFIO-KVM 璁惧璺熻釜涓Щ闄や竴涓?VFIO 鏂囦欢锛坓roup/device锛?
	kvm_device_attr.addr 鎸囧悜 VFIO 鏂囦欢鐨?int32_t 鏂囦欢鎻忚堪绗︺€?
KVM_DEV_VFIO_GROUP锛堜粎闄愪簬澶勭悊 VFIO group fd 鐨勪紶缁?kvm 璁惧缁勶級锛?  KVM_DEV_VFIO_GROUP_ADD锛氫笌 KVM_DEV_VFIO_FILE_ADD 鐩稿悓锛屼絾浠呴拡瀵?group fd

  KVM_DEV_VFIO_GROUP_DEL锛氫笌 KVM_DEV_VFIO_FILE_DEL 鐩稿悓锛屼絾浠呴拡瀵?group fd

  KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE锛氶檮鍔犱竴涓鎴锋満鍙鐨?TCE 琛紝
	鐢?sPAPR KVM 鍒嗛厤銆?```

		struct kvm_vfio_spapr_tce {
			__s32	groupfd;
			__s32	tablefd;
		};

	鍏朵腑锛?
	- @groupfd 鏄?VFIO group 鐨勬枃浠舵弿杩扮锛?	- @tablefd 鏄€氳繃 KVM_CREATE_SPAPR_TCE 鍒嗛厤鐨?TCE 琛ㄧ殑鏂囦欢鎻忚堪绗︺€?
```
涓婇潰鐨?FILE/GROUP_ADD 鎿嶄綔搴斿綋鍦ㄩ€氳繃 VFIO_GROUP_GET_DEVICE_FD 璁块棶璁惧鏂囦欢鎻忚堪绗︿箣鍓嶈皟鐢紝浠ユ敮鎸侀偅浜涢渶瑕佸湪鍏?.open_device() 鍥炶皟涓缃?kvm 鎸囬拡鐨勯┍鍔ㄣ€傚浜庨€氳繃瀛楃璁惧 open 鑾峰緱璁惧鏂囦欢鎻忚堪绗︼紙骞堕€氳繃 VFIO_DEVICE_BIND_IOMMUFD 鑾峰緱璁惧璁块棶锛夌殑鎯呭喌涔熷悓鏍峰姝ゃ€傚浜庢绫绘枃浠舵弿杩扮锛屽簲鍦?VFIO_DEVICE_BIND_IOMMUFD 涔嬪墠璋冪敤 FILE_ADD锛屼互鏀寔鍓嶉潰鎻愬埌鐨勯偅浜涢┍鍔ㄣ€?