## 杩滅▼澶勭悊鍣ㄦ鏋讹紙Remote Processor Framework锛?

## 绠€浠?

鐜颁唬 SoC 閫氬父鍦ㄩ潪瀵圭О澶氬鐞嗭紙AMP锛夐厤缃腑鍖呭惈寮傛瀯鐨勮繙绋嬪鐞嗗櫒璁惧锛岃繖浜涜澶囧彲鑳借繍琛屼笉鍚岀殑鎿嶄綔绯荤粺瀹炰緥锛屾棤璁烘槸 Linux 杩樻槸浠讳綍鍏跺畠椋庢牸鐨勫疄鏃舵搷浣滅郴缁熴€?
渚嬪锛孫MAP4 鎷ユ湁鍙屾牳 Cortex-A9銆佸弻鏍?Cortex-M3 浠ュ強涓€涓?C64x+ DSP銆傚湪鍏稿瀷閰嶇疆涓紝鍙屾牳 Cortex-A9 浠?SMP 閰嶇疆杩愯 Linux锛岃€屽叾瀹冧笁涓牳蹇冿紙涓や釜 M3 鏍稿績鍜屼竴涓?DSP锛夊悇鑷互 AMP 閰嶇疆杩愯鑷繁鐨?RTOS 瀹炰緥銆?
remoteproc 妗嗘灦鍏佽涓嶅悓鐨勫钩鍙?鏋舵瀯鎺у埗锛堜笂鐢点€佸姞杞藉浐浠躲€佹柇鐢碉級杩欎簺杩滅▼澶勭悊鍣紝鍚屾椂鎶借薄鎺夌‖浠跺樊寮傦紝鍥犳鏃犻渶閲嶅缂栧啓鏁翠釜椹卞姩銆傛澶栵紝璇ユ鏋惰繕浼氫负鏀寔杩欑閫氫俊鏂瑰紡鐨勮繙绋嬪鐞嗗櫒娣诲姞 rpmsg virtio 璁惧銆傝繖鏍凤紝鐗瑰畾浜庡钩鍙扮殑 remoteproc 椹卞姩鍙渶瑕佹彁渚涘皯閲忓簳灞傚鐞嗗嚱鏁帮紝鐒跺悗鎵€鏈?rpmsg 椹卞姩灏遍兘鑳芥甯稿伐浣滐紙鍏充簬鍩轰簬 virtio 鐨?rpmsg 鎬荤嚎鍙婂叾椹卞姩鐨勬洿澶氫俊鎭紝璇峰弬闃?Documentation/staging/rpmsg.rst锛夈€傜幇鍦ㄤ篃鍙互娉ㄥ唽鍏跺畠绫诲瀷鐨?virtio 璁惧銆傚浐浠跺彧闇€鍏竷瀹冧滑鏀寔鍝 virtio 璁惧锛岀劧鍚?remoteproc 灏变細娣诲姞杩欎簺璁惧銆傝繖浣垮緱浠ユ渶灏忕殑寮€鍙戞垚鏈紝灏嗙幇鏈夌殑 virtio 椹卞姩涓庤繙绋嬪鐞嗗櫒鍚庣澶嶇敤鎴愪负鍙兘銆?

## 鐢ㄦ埛 API


```
  int rproc_boot(struct rproc *rproc)

```
鍚姩涓€涓繙绋嬪鐞嗗櫒锛堝嵆鍔犺浇鍏跺浐浠躲€佷负鍏朵笂鐢碘€︹€︼級銆?
濡傛灉璇ヨ繙绋嬪鐞嗗櫒宸茬粡涓婄數锛岃鍑芥暟浼氱珛鍗筹紙鎴愬姛锛夎繑鍥炪€?
鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鍊笺€傛敞鎰忥細瑕佷娇鐢ㄨ鍑芥暟锛屼綘搴旇宸茬粡鎷ユ湁涓€涓湁鏁堢殑 rproc 鍙ユ焺銆傛湁鍑犵骞插噣鐨勬柟寮忓彲浠ヨ幏寰楀畠锛坉evres銆乸data銆乺emoteproc_rpmsg.c 鐨勫仛娉曪紝鎴栬€呭鏋滆繖绉嶆柟寮忓彉寰楁櫘閬嶏紝鎴戜滑涔熷彲鑳戒細鑰冭檻浣跨敤 dev_archdata锛夈€?
```
  int rproc_shutdown(struct rproc *rproc)

```
鍏抽棴涓€涓繙绋嬪鐞嗗櫒锛堝厛鍓嶇敱 rproc_boot() 鍚姩锛夈€傚鏋?@rproc 浠嶈鍏跺畠鐢ㄦ埛浣跨敤锛岄偅涔堣鍑芥暟鍙細閫掑噺鐢垫簮寮曠敤璁℃暟骞堕€€鍑猴紝鑰屼笉浼氱湡姝ｇ粰璁惧鏂數銆?
鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鍊笺€傛瘡涓€娆″ rproc_boot() 鐨勮皟鐢ㄩ兘蹇呴』锛堟渶缁堬級浼撮殢涓€娆″ rproc_shutdown() 鐨勮皟鐢ㄣ€傚啑浣欏湴璋冪敤 rproc_shutdown() 鏄竴涓?bug銆?

```
  we're not decrementing the rproc's refcount, only the power refcount.
  which means that the @rproc handle stays valid even after
  rproc_shutdown() returns, and users can still use it with a subsequent
  rproc_boot(), if needed.

```
```
  struct rproc *rproc_get_by_phandle(phandle phandle)

```
浣跨敤璁惧鏍?phandle 鏌ユ壘涓€涓?rproc 鍙ユ焺銆傛垚鍔熸椂杩斿洖 rproc 鍙ユ焺锛屽け璐ユ椂杩斿洖 NULL銆傝鍑芥暟浼氶€掑杩滅▼澶勭悊鍣ㄧ殑寮曠敤璁℃暟锛屽洜姝ゅ綋涓嶅啀闇€瑕?rproc 鏃讹紝鍔″繀浣跨敤 rproc_put() 灏嗗叾閫掑噺鍥炲幓銆?

## 鍏稿瀷鐢ㄦ硶


```
  #include <linux/remoteproc.h>

  /* in case we were given a valid 'rproc' handle */
  int dummy_rproc_example(struct rproc *my_rproc)
  {
	int ret;

	/* let's power on and boot our remote processor */
	ret = rproc_boot(my_rproc);
	if (ret) {
		/*
		 * something went wrong. handle it and leave.
		 */
	}

	/*
	 * our remote processor is now powered on... give it some work
	 */

	/* let's shut it down now */
	rproc_shutdown(my_rproc);
  }

```
## 渚涘疄鐜拌€呬娇鐢ㄧ殑 API


```
  struct rproc *rproc_alloc(struct device *dev, const char *name,
				const struct rproc_ops *ops,
				const char *firmware, int len)

```
鍒嗛厤涓€涓柊鐨勮繙绋嬪鐞嗗櫒鍙ユ焺锛屼絾鏆備笉娉ㄥ唽瀹冦€傚繀闇€鐨勫弬鏁版湁锛氬簳灞傝澶囥€佽杩滅▼澶勭悊鍣ㄧ殑鍚嶇О銆佺壒瀹氫簬骞冲彴鐨勬搷浣滃鐞嗗嚱鏁般€佺敤浜庡惎鍔ㄨ rproc 鐨勫浐浠跺悕绉帮紝浠ュ強鍒嗛厤璇?rproc 鐨勯┍鍔ㄦ墍闇€鐨勭鏈夋暟鎹暱搴︼紙浠ュ瓧鑺傝锛夈€?
璇ュ嚱鏁板簲鐢?rproc 瀹炵幇鍦ㄨ繙绋嬪鐞嗗櫒鍒濆鍖栨湡闂翠娇鐢ㄣ€?
浣跨敤璇ュ嚱鏁板垱寤?rproc 鍙ユ焺涔嬪悗锛屽湪鍑嗗灏辩华鏃讹紝瀹炵幇鑰呭簲璋冪敤 rproc_add() 鏉ュ畬鎴愯繙绋嬪鐞嗗櫒鐨勬敞鍐屻€?
鎴愬姛鏃惰繑鍥炴柊鐨?rproc锛屽け璐ユ椂杩斿洖 NULL銆?

  **never** 鍗充娇璇?rproc 灏氭湭娉ㄥ唽锛屼篃缁濅笉鑳界洿鎺ラ噴鏀?@rproc銆傜浉鍙嶏紝褰撲綘闇€瑕佸洖閫€ rproc_alloc() 鏃讹紝搴斾娇鐢?rproc_free()銆?
```
  void rproc_free(struct rproc *rproc)

```
閲婃斁涓€涓敱 rproc_alloc 鍒嗛厤鐨?rproc 鍙ユ焺銆?
璇ュ嚱鏁版湰璐ㄤ笂鏄€氳繃閫掑噺 rproc 鐨勫紩鐢ㄨ鏁版潵鍥為€€ rproc_alloc()銆傚畠涓嶄細鐩存帴閲婃斁 rproc锛涘彧鏈夊綋瀵?rproc 娌℃湁鍏跺畠寮曠敤銆佷笖鍏跺紩鐢ㄨ鏁扮幇鍦ㄩ檷涓洪浂鏃讹紝鎵嶄細鐪熸閲婃斁銆?
```
  int rproc_add(struct rproc *rproc)

```
鍦ㄩ€氳繃 rproc_alloc() 鍒嗛厤涔嬪悗锛屽悜 remoteproc 妗嗘灦娉ㄥ唽 @rproc銆?
褰撴帰娴嬪埌涓€涓柊鐨勮繙绋嬪鐞嗗櫒璁惧鏃讹紝鐢辩壒瀹氫簬骞冲彴鐨?rproc 瀹炵幇璋冪敤銆?
鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炵浉搴旂殑閿欒鐮併€傛敞鎰忥細璇ュ嚱鏁颁細鍚姩涓€涓紓姝ョ殑鍥轰欢鍔犺浇涓婁笅鏂囷紝瀹冨皢鏌ユ壘璇?rproc 鐨勫浐浠舵墍鏀寔鐨?virtio 璁惧銆?
濡傛灉鎵惧埌锛岃繖浜?virtio 璁惧灏嗚鍒涘缓骞舵坊鍔狅紝鍥犳浣滀负娉ㄥ唽璇ヨ繙绋嬪鐞嗗櫒鐨勭粨鏋滐紝鍙兘浼氭湁棰濆鐨?virtio 椹卞姩琚帰娴嬪埌銆?
```
  int rproc_del(struct rproc *rproc)

```
鍥為€€ rproc_add()銆?
褰撶壒瀹氫簬骞冲彴鐨?rproc 瀹炵幇鍐冲畾绉婚櫎璇?rproc 璁惧鏃讹紝搴斿綋璋冪敤姝ゅ嚱鏁般€傚畠搴斿綋浠呭湪鍏堝墠瀵?rproc_add() 鐨勮皟鐢ㄥ凡鎴愬姛瀹屾垚鏃舵墠琚皟鐢ㄣ€?
鍦?rproc_del() 杩斿洖涔嬪悗锛孈rproc 浠嶇劧鏈夋晥锛屽叾鏈€鍚庣殑寮曠敤璁℃暟搴斿綋閫氳繃璋冪敤 rproc_free() 鏉ラ€掑噺銆?
鎴愬姛鏃惰繑鍥?0锛屽鏋?@rproc 鏃犳晥鍒欒繑鍥?-EINVAL銆?
```
  void rproc_report_crash(struct rproc *rproc, enum rproc_crash_type type)

```
鎶ュ憡 remoteproc 涓彂鐢熶簡涓€娆″穿婧冦€?
姣忔鐗瑰畾浜庡钩鍙扮殑 rproc 瀹炵幇妫€娴嬪埌涓€娆″穿婧冩椂锛岄兘蹇呴』璋冪敤姝ゅ嚱鏁般€傚畠涓嶅簲琚潪 remoteproc 椹卞姩璋冪敤銆傝鍑芥暟鍙互鍦ㄥ師瀛?涓柇涓婁笅鏂囦腑璋冪敤銆?

## 瀹炵幇鍥炶皟


杩欎簺鍥炶皟搴旂敱鐗瑰畾浜庡钩鍙扮殑 remoteproc 鎻愪緵
```
  /**
   * struct rproc_ops - platform-specific device handlers
   * @start:	power on the device and boot it
   * @stop:	power off the device
   * @kick:	kick a virtqueue (virtqueue id given as a parameter)
   */
  struct rproc_ops {
	int (*start)(struct rproc *rproc);
	int (*stop)(struct rproc *rproc);
	void (*kick)(struct rproc *rproc, int vqid);
  };

```
姣忎竴涓?remoteproc 瀹炵幇鑷冲皯搴斿綋鎻愪緵 ->start 鍜?->stop 澶勭悊鍑芥暟銆傚鏋滆繕甯屾湜鏈?rpmsg/virtio 鍔熻兘锛岄偅涔堜篃搴斿綋鎻愪緵 ->kick 澶勭悊鍑芥暟銆?
->start() 澶勭悊鍑芥暟鎺ュ彈涓€涓?rproc 鍙ユ焺锛屽苟搴斿綋涓鸿澶囦笂鐢靛苟鍚姩瀹冿紙浣跨敤 rproc->priv 鏉ヨ闂壒瀹氫簬骞冲彴鐨勭鏈夋暟鎹級銆傚惎鍔ㄥ湴鍧€锛堝鏋滈渶瑕佺殑璇濓級鍙互鍦?rproc->bootaddr 涓壘鍒帮紙remoteproc 鏍稿績灏?ELF 鍏ュ彛鐐规斁鍦ㄩ偅閲岋級銆傛垚鍔熸椂搴斿綋杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇爜銆?
->stop() 澶勭悊鍑芥暟鎺ュ彈涓€涓?rproc 鍙ユ焺骞朵负璁惧鏂數銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖鐩稿簲鐨勯敊璇爜銆?
->kick() 澶勭悊鍑芥暟鎺ュ彈涓€涓?rproc 鍙ユ焺锛屼互鍙婃斁缃簡鏂版秷鎭殑 virtqueue 绱㈠紩銆傚疄鐜板簲褰撲腑鏂繙绋嬪鐞嗗櫒锛岃瀹冪煡閬撹嚜宸辨湁寰呭鐞嗙殑娑堟伅銆傞€氱煡杩滅▼澶勭悊鍣ㄥ叿浣撹鏌ョ湅鍝釜 virtqueue 绱㈠紩鏄彲閫夌殑锛氶亶鍘嗙幇鏈夌殑 virtqueue 骞跺湪 used 鐜腑鏌ユ壘鏂扮殑缂撳啿鍖烘槸瀹规槗鐨勶紙涓斾唬浠蜂笉楂橈級銆?

## 浜岃繘鍒跺浐浠剁粨鏋?

鐩墠 remoteproc 鏀寔 ELF32 鍜?ELF64 鍥轰欢浜岃繘鍒舵枃浠躲€備笉杩囷紝鎴戜滑寰堝彲鑳戒細甯屾湜鐢ㄨ妗嗘灦鏀寔鐨勫叾瀹冨钩鍙?璁惧灏嗗熀浜庝笉鍚岀殑浜岃繘鍒舵牸寮忋€?
褰撹繖浜涚敤渚嬪嚭鐜版椂锛屾垜浠繀椤诲皢浜岃繘鍒舵牸寮忎笌妗嗘灦鏍稿績瑙ｈ€︼紝浠ヤ究鍦ㄤ笉閲嶅閫氱敤浠ｇ爜鐨勬儏鍐典笅鏀寔澶氱浜岃繘鍒舵牸寮忋€?
褰撳浐浠惰瑙ｆ瀽鏃讹紝瀹冪殑鍚勪釜娈典細鏍规嵁鎸囧畾鐨勮澶囧湴鍧€锛堝鏋滆繙绋嬪鐞嗗櫒鐩存帴璁块棶鍐呭瓨锛屽垯鍙兘鏄墿鐞嗗湴鍧€锛夎鍔犺浇鍒板唴瀛樹腑銆?
闄や簡鏍囧噯鐨?ELF 娈典箣澶栵紝澶у鏁拌繙绋嬪鐞嗗櫒杩樹細鍖呭惈涓€涓垜浠О涔嬩负鈥滆祫婧愯〃锛坮esource table锛夆€濈殑鐗规畩娈点€?
璧勬簮琛ㄥ寘鍚繙绋嬪鐞嗗櫒鍦ㄤ笂鐢典箣鍓嶆墍闇€鐨勭郴缁熻祫婧愶紝渚嬪鍒嗛厤鐗╃悊涓婅繛缁殑鍐呭瓨锛屾垨瀵规煇浜涚墖涓婂璁捐繘琛?iommu 鏄犲皠銆俁emotecore 鍙湁鍦ㄨ祫婧愯〃鐨勬墍鏈夎姹傞兘婊¤冻涔嬪悗鎵嶄細缁欒澶囦笂鐢点€?
闄や簡绯荤粺璧勬簮涔嬪锛岃祫婧愯〃杩樺彲鑳藉寘鍚敤浜庡叕甯冭繙绋嬪鐞嗗櫒鎵€鏀寔鐨勭壒鎬у拰閰嶇疆鐨勮祫婧愭潯鐩紝渚嬪璺熻釜缂撳啿鍖猴紙trace buffer锛変互鍙婂彈鏀寔鐨?virtio 璁惧锛堝強鍏堕厤缃級銆?
```
  /**
   * struct resource_table - firmware resource table header
   * @ver: version number
   * @num: number of resource entries
   * @reserved: reserved (must be zero)
   * @offset: array of offsets pointing at the various resource entries
   *
   * The header of the resource table, as expressed by this structure,
   * contains a version number (should we need to change this format in the
   * future), the number of available resource entries, and their offsets
   * in the table.
   */
  struct resource_table {
	u32 ver;
	u32 num;
	u32 reserved[2];
	u32 offset[0];
  } __packed;

```
绱ф帴鍦ㄨ澶撮儴涔嬪悗鐨勬槸璧勬簮鏉＄洰鏈韩锛?```
  /**
   * struct fw_rsc_hdr - firmware resource entry header
   * @type: resource type
   * @data: resource data
   *
   * Every resource entry begins with a 'struct fw_rsc_hdr' header providing
   * its @type. The content of the entry itself will immediately follow
   * this header, and it should be parsed according to the resource type.
   */
  struct fw_rsc_hdr {
	u32 type;
	u8 data[0];
  } __packed;

```
鏈変簺璧勬簮鏉＄洰浠呬粎鏄€氬憡锛屽憡鐭ヤ富鏈烘煇浜涚壒瀹氱殑 remoteproc 閰嶇疆銆傚叾瀹冩潯鐩垯瑕佹眰涓绘満鍋氭煇浜涗簨鎯咃紙渚嬪鍒嗛厤涓€涓郴缁熻祫婧愶級銆傛湁鏃惰繕闇€瑕佸崗鍟嗭細鍥轰欢璇锋眰涓€涓祫婧愶紝涓€鏃﹀垎閰嶅畬鎴愶紝涓绘満搴斿綋灏嗗叾缁嗚妭锛堜緥濡傚凡鍒嗛厤鍐呭瓨鍖哄煙鐨勫湴鍧€锛夊弽棣堝洖鍘汇€?
```
  /**
   * enum fw_resource_type - types of resource entries
   *
   * @RSC_CARVEOUT:   request for allocation of a physically contiguous
   *		    memory region.
   * @RSC_DEVMEM:     request to iommu_map a memory-based peripheral.
   * @RSC_TRACE:	    announces the availability of a trace buffer into which
   *		    the remote processor will be writing logs.
   * @RSC_VDEV:       declare support for a virtio device, and serve as its
   *		    virtio header.
   * @RSC_LAST:       just keep this one at the end
   * @RSC_VENDOR_START:	start of the vendor specific resource types range
   * @RSC_VENDOR_END:	end of the vendor specific resource types range
   *
   * Please note that these values are used as indices to the rproc_handle_rsc
   * lookup table, so please keep them sane. Moreover, @RSC_LAST is used to
   * check the validity of an index before the lookup table is accessed, so
   * please update it as needed.
   */
  enum fw_resource_type {
	RSC_CARVEOUT		= 0,
	RSC_DEVMEM		= 1,
	RSC_TRACE		= 2,
	RSC_VDEV		= 3,
	RSC_LAST		= 4,
	RSC_VENDOR_START	= 128,
	RSC_VENDOR_END		= 512,
  };

```
鍏充簬鐗瑰畾璧勬簮绫诲瀷鐨勬洿澶氱粏鑺傦紝璇峰弬闃?include/linux/remoteproc.h 涓叾涓撻棬鐨勭粨鏋勩€?
鎴戜滑涔熼鏈熷湪灏嗘潵浼氬嚭鐜扮壒瀹氫簬骞冲彴鐨勮祫婧愭潯鐩€傚綋杩欑鎯呭喌鍙戠敓鏃讹紝鎴戜滑鍙互杞绘澗鍦版坊鍔犱竴涓柊鐨?RSC_PLATFORM 绫诲瀷锛屽苟灏嗚繖浜涜祫婧愪氦缁欑壒瀹氫簬骞冲彴鐨?rproc 椹卞姩鍘诲鐞嗐€?

## Virtio 涓?remoteproc


鍥轰欢搴斿綋鍚?remoteproc 鎻愪緵瀹冩墍鏀寔鐨?virtio 璁惧鍙婂叾閰嶇疆鐨勪俊鎭細涓€涓?RSC_VDEV 璧勬簮鏉＄洰搴斿綋鎸囧畾 virtio 璁惧 id锛堝 virtio_ids.h 涓級銆乿irtio 鐗规€с€乿irtio 閰嶇疆绌洪棿銆乿ring 淇℃伅绛夈€?
褰撲竴涓柊鐨勮繙绋嬪鐞嗗櫒琚敞鍐屾椂锛宺emoteproc 妗嗘灦浼氭煡鎵惧畠鐨勮祫婧愯〃锛屽苟娉ㄥ唽瀹冩墍鏀寔鐨?virtio 璁惧銆備竴涓浐浠跺彲浠ユ敮鎸佷换鎰忔暟閲忕殑 virtio 璁惧锛屼笖鍙互鏄换鎰忕被鍨嬶紙濡傛灉闇€瑕侊紝鍗曚釜杩滅▼澶勭悊鍣ㄤ篃鍙互杞绘澗鍦伴€氳繃杩欑鏂瑰紡鏀寔澶氫釜 rpmsg virtio 璁惧锛夈€?
褰撶劧锛孯SC_VDEV 璧勬簮鏉＄洰浠呰冻浠ョ敤浜?virtio 璁惧鐨勯潤鎬佸垎閰嶃€傚姩鎬佸垎閰嶄篃鍙互閫氳繃 rpmsg 鎬荤嚎瀹炵幇锛堢被浼间簬鎴戜滑宸茬粡瀵?rpmsg 閫氶亾杩涜鐨勫姩鎬佸垎閰嶏紱鏇村淇℃伅璇峰弬闃?rpmsg.txt锛夈€?