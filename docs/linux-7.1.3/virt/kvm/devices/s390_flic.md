
## FLIC锛坒loating interrupt controller锛屾诞鍔ㄤ腑鏂帶鍒跺櫒锛?


FLIC 澶勭悊娴姩锛堥潪姣?CPU锛変腑鏂紝鍗?I/O銆佹湇鍔′互鍙婃煇浜涙満鍣ㄦ鏌ワ紙machine check锛変腑鏂€?
鎵€鏈変腑鏂兘瀛樺偍鍦ㄦ瘡 VM 鐨勬寕璧蜂腑鏂垪琛ㄤ腑銆侳LIC 瀵硅鍒楄〃鎵ц鎿嶄綔銆?

鍙兘瀹炰緥鍖栦竴涓?FLIC 瀹炰緥銆?

FLIC 鎻愪緵浠ヤ笅鏀寔锛?
- 娣诲姞涓柇锛圞VM_DEV_FLIC_ENQUEUE锛?
- 妫€鏌ュ綋鍓嶆寕璧风殑涓柇锛圞VM_FLIC_GET_ALL_IRQS锛?
- 娓呴櫎鎵€鏈夋寕璧风殑娴姩涓柇锛圞VM_DEV_FLIC_CLEAR_IRQS锛?
- 娓呴櫎涓€涓寕璧风殑娴姩 I/O 涓柇锛圞VM_DEV_FLIC_CLEAR_IO_IRQ锛?
- 涓哄鏈哄惎鐢?绂佺敤閫忔槑鐨勫紓姝ラ〉閿欒锛坅sync page faults锛?
- 娉ㄥ唽鍜屼慨鏀归€傞厤鍣ㄤ腑鏂簮锛圞VM_DEV_FLIC_ADAPTER_*锛?
- 淇敼 AIS锛坅dapter-interruption-suppression锛岄€傞厤鍣ㄤ腑鏂姂鍒讹級妯″紡鐘舵€侊紙KVM_DEV_FLIC_AISM锛?
- 鍦ㄦ寚瀹氶€傞厤鍣ㄤ笂娉ㄥ叆閫傞厤鍣ㄤ腑鏂紙KVM_DEV_FLIC_AIRQ_INJECT锛?
- 鑾峰彇/璁剧疆鎵€鏈?AIS 妯″紡鐘舵€侊紙KVM_DEV_FLIC_AISM_ALL锛?

缁勶細
  KVM_DEV_FLIC_ENQUEUE
    灏嗕竴涓紦鍐插尯鍜岄暱搴︿紶鍏ュ唴鏍革紝闅忓悗瀹冧滑琚敞鍏ュ埌鎸傝捣涓柇鍒楄〃涓€?
    attr->addr 鍖呭惈鎸囧悜缂撳啿鍖虹殑鎸囬拡锛宎ttr->attr 鍖呭惈缂撳啿鍖虹殑闀垮害銆?
    浠庣敤鎴风┖闂村鍒剁殑鏁版嵁缁撴瀯 kvm_s390_irq 鐨勬牸寮忓畾涔変簬 usr/include/linux/kvm.h銆?

  KVM_DEV_FLIC_GET_ALL_IRQS
    灏嗘墍鏈夋诞鍔ㄤ腑鏂鍒跺埌涓€涓敱鐢ㄦ埛绌洪棿鎻愪緵鐨勭紦鍐插尯涓€?
    褰撶紦鍐插尯澶皬鏃惰繑鍥?-ENOMEM锛岃繖鏄寚绀虹敤鎴风┖闂寸敤涓€涓洿澶х殑缂撳啿鍖洪噸璇曘€?

    -ENOBUFS 鍦ㄥ垎閰嶅唴鏍哥┖闂寸紦鍐插尯澶辫触鏃惰繑鍥炪€?

    -EFAULT 鍦ㄥ皢鏁版嵁澶嶅埗鍒扮敤鎴风┖闂村け璐ユ椂杩斿洖銆傛墍鏈変腑鏂繚鎸佹寕璧凤紝鍗充笉浼氳浠庡綋鍓?
    鎸傝捣涓柇鍒楄〃涓垹闄ゃ€俛ttr->addr 鍖呭惈鐢ㄦ埛绌洪棿缂撳啿鍖虹殑鍦板潃锛屾墍鏈変腑鏂暟鎹皢琚鍒?
    鍒拌缂撳啿鍖恒€俛ttr->attr 鍖呭惈缂撳啿鍖虹殑澶у皬锛堝瓧鑺傦級銆?

  KVM_DEV_FLIC_CLEAR_IRQS
    绠€鍗曞湴浠庡綋鍓嶆寕璧风殑娴姩涓柇鍒楄〃涓垹闄ゆ墍鏈夊厓绱犮€傛病鏈変腑鏂娉ㄥ叆鍒板鏈恒€?

  KVM_DEV_FLIC_CLEAR_IO_IRQ
    鍒犻櫎涓€涓紙濡傛灉瀛樺湪锛塈/O 涓柇锛岃涓柇閽堝鐢?attr->addr锛堝湴鍧€锛夊拰 attr->attr锛堥暱搴︼級
    鎵€鎸囧畾缂撳啿鍖轰紶鍏ョ殑瀛愮郴缁熸爣璇嗗瓧锛坰ubsystem identification word锛夋墍鏍囪瘑鐨勫瓙閫氶亾
    锛坰ubchannel锛夈€?

  KVM_DEV_FLIC_APF_ENABLE
    涓哄鏈哄惎鐢ㄥ紓姝ラ〉閿欒銆傚洜姝ゅ湪澶ч〉閿欒锛坢ajor page fault锛夋儏鍐典笅锛屽涓绘満琚厑璁稿紓姝?
    澶勭悊瀹冨苟缁х画杩愯瀹㈡満銆?

    -EINVAL 鍦ㄩ拡瀵?ucontrol VM 鐨?FLIC 璋冪敤鏃惰繑鍥炪€?

  KVM_DEV_FLIC_APF_DISABLE_WAIT
    涓哄鏈虹鐢ㄥ紓姝ラ〉閿欒锛屽苟绛夊緟鐩村埌宸茬粡鎸傝捣鐨勫紓姝ラ〉閿欒瀹屾垚銆傝繖瀵逛簬鍦ㄨ縼绉讳腑鏂垪琛?
    涔嬪墠涓烘瘡涓?init 涓柇瑙﹀彂涓€涓畬鎴愪腑鏂槸蹇呰鐨勩€?

    -EINVAL 鍦ㄩ拡瀵?ucontrol VM 鐨?FLIC 璋冪敤鏃惰繑鍥炪€?

  KVM_DEV_FLIC_ADAPTER_REGISTER
    娉ㄥ唽涓€涓?I/O 閫傞厤鍣ㄤ腑鏂簮銆傛帴鍙椾竴涓?kvm_s390_io_adapter
```

	struct kvm_s390_io_adapter {
		__u32 id;
		__u8 isc;
		__u8 maskable;
		__u8 swap;
		__u8 flags;
	};

   id contains the unique id for the adapter, isc the I/O interruption subclass
   to use, maskable whether this adapter may be masked (interrupts turned off),
   swap whether the indicators need to be byte swapped, and flags contains
   further characteristics of the adapter.

   Currently defined values for 'flags' are:

   - KVM_S390_ADAPTER_SUPPRESSIBLE: adapter is subject to AIS
     (adapter-interrupt-suppression) facility. This flag only has an effect if
     the AIS capability is enabled.

   Unknown flag values are ignored.


  KVM_DEV_FLIC_ADAPTER_MODIFY
    Modifies attributes of an existing I/O adapter interrupt source. Takes
    a kvm_s390_io_adapter_req specifying the adapter and the operation::

	struct kvm_s390_io_adapter_req {
		__u32 id;
		__u8 type;
		__u8 mask;
		__u16 pad0;
		__u64 addr;
	};

    id specifies the adapter and type the operation. The supported operations
    are:

    KVM_S390_IO_ADAPTER_MASK
      mask or unmask the adapter, as specified in mask

    KVM_S390_IO_ADAPTER_MAP
      This is now a no-op. The mapping is purely done by the irq route.
    KVM_S390_IO_ADAPTER_UNMAP
      This is now a no-op. The mapping is purely done by the irq route.

  KVM_DEV_FLIC_AISM
    modify the adapter-interruption-suppression mode for a given isc if the
    AIS capability is enabled. Takes a kvm_s390_ais_req describing::

	struct kvm_s390_ais_req {
		__u8 isc;
		__u16 mode;
	};

    isc contains the target I/O interruption subclass, mode the target
    adapter-interruption-suppression mode. The following modes are
    currently supported:

    - KVM_S390_AIS_MODE_ALL: ALL-Interruptions Mode, i.e. airq injection
      is always allowed;
    - KVM_S390_AIS_MODE_SINGLE: SINGLE-Interruption Mode, i.e. airq
      injection is only allowed once and the following adapter interrupts
      will be suppressed until the mode is set again to ALL-Interruptions
      or SINGLE-Interruption mode.

  KVM_DEV_FLIC_AIRQ_INJECT
    Inject adapter interrupts on a specified adapter.
    attr->attr contains the unique id for the adapter, which allows for
    adapter-specific checks and actions.
    For adapters subject to AIS, handle the airq injection suppression for
    an isc according to the adapter-interruption-suppression mode on condition
    that the AIS capability is enabled.

  KVM_DEV_FLIC_AISM_ALL
    Gets or sets the adapter-interruption-suppression mode for all ISCs. Takes
    a kvm_s390_ais_all describing::

	struct kvm_s390_ais_all {
	       __u8 simm; /* Single-Interruption-Mode mask */
	       __u8 nimm; /* No-Interruption-Mode mask *
	};

    simm contains Single-Interruption-Mode mask for all ISCs, nimm contains
    No-Interruption-Mode mask for all ISCs. Each bit in simm and nimm corresponds
    to an ISC (MSB0 bit 0 to ISC 0 and so on). The combination of simm bit and
    nimm bit presents AIS mode for a ISC.

    KVM_DEV_FLIC_AISM_ALL is indicated by KVM_CAP_S390_AIS_MIGRATION.

```
娉ㄦ剰锛氬湪 FLIC 涓婃墽琛岀殑甯︽湁鏈煡缁勬垨灞炴€х殑 KVM_SET_DEVICE_ATTR/KVM_GET_DEVICE_ATTR 璁惧 ioctl 浼氱粰鍑洪敊璇爜 EINVAL锛堣€屼笉鏄?API 鏂囨。涓瀹氱殑 ENXIO锛夈€傛棤娉曞熀浜庝娇鐢ㄥ皾璇曟墍浜х敓鐨勯敊璇爜鏉ユ帹鏂煇涓?FLIC 鎿嶄綔涓嶅彲鐢ㄣ€?

  鎸囧畾浜嗛浂 schid銆?
