
## POWER9 eXternal Interrupt Virtualization Engine (XIVE Gen1)


鏀寔鐨勮澶囩被鍨嬶細
  - KVM_DEV_TYPE_XIVE     POWER9 XIVE 涓柇鎺у埗鍣?绗?1 浠?
璇ヨ澶囧厖褰撹櫄鎷熸満鐨勭粓绔帶鍒跺櫒銆傚畠鎻愪緵 KVM 鎺ュ彛锛岀敤浜庡湪搴曞眰 POWER9 XIVE 涓柇鎺у埗鍣ㄤ腑閰嶇疆铏氭嫙鏈虹殑鍚勭涓柇婧愩€?
鍙兘瀹炰緥鍖栦竴涓?XIVE 瀹炰緥銆傚鎴锋満 XIVE 璁惧闇€瑕?POWER9 涓绘満锛屽苟涓斿鎴锋満鎿嶄綔绯荤粺搴斿綋鏀寔 XIVE 鍘熺敓鍒╃敤锛坣ative exploitation锛変腑鏂ā寮忋€傚惁鍒欙紝瀹冨簲褰撹繍琛屽湪绉颁负 XICS锛圥OWER7/8锛夌殑閬楃暀涓柇妯″紡涓嬨€?
- 璁惧鏄犲皠

  KVM 璁惧鏆撮湶浜?XIVE 纭欢涓婄敤浜庝腑鏂鐞嗙殑涓嶅悓 MMIO 鍖洪棿銆傝繖浜涘尯闂撮€氳繃鑷畾涔夌殑 VM 缂洪〉锛坒ault锛夊鐞嗙▼搴忥紝浠?VMA 鐨勫舰寮忔毚闇茬粰瀹㈡埛鏈恒€?
  1. 绾跨▼涓柇绠＄悊鍖猴紙TIMA锛?
  姣忎釜绾跨▼閮芥湁涓€涓叧鑱旂殑绾跨▼涓柇绠＄悊涓婁笅鏂囷紝鐢变竴缁勫瘎瀛樺櫒缁勬垚銆傝繖浜涘瘎瀛樺櫒璁╃嚎绋嬭兘澶熷鐞嗕紭鍏堢骇绠＄悊鍜屼腑鏂‘璁ゃ€傚叾涓渶閲嶈鐨勬湁锛?
      - Interrupt Pending Buffer     (IPB)
      - Current Processor Priority   (CPPR)
      - Notification Source Register (NSR)

  瀹冧滑浠ュ洓涓笉鍚岀殑椤垫毚闇茬粰杞欢锛屾瘡椤垫彁渚涗竴绉嶅叿鏈変笉鍚岀壒鏉冪骇鐨勮鍥俱€傜涓€椤电敤浜庣墿鐞嗙嚎绋嬩笂涓嬫枃锛岀浜岄〉鐢ㄤ簬绠＄悊绋嬪簭锛坔ypervisor锛夈€傚彧鏈夌涓夐〉锛堟搷浣滅郴缁燂級鍜岀鍥涢〉锛堢敤鎴风骇锛夎鏆撮湶缁欏鎴锋満銆?
  2. 浜嬩欢鐘舵€佺紦鍐插尯锛圗SB锛?
  姣忎釜婧愰兘鍏宠仈涓€涓簨浠剁姸鎬佺紦鍐插尯锛圗SB锛夛紝瀹冨搴斾竴瀵瑰伓鏁?濂囨暟椤碉紝鎻愪緵鐢ㄤ簬绠＄悊璇ユ簮鐨勫懡浠わ細渚嬪瑙﹀彂锛坱rigger锛夈€丒OI銆佸叧闂婧愮瓑銆?
  3. 璁惧鐩撮€?
  褰撹澶囪鐩撮€氬埌瀹㈡埛鏈烘椂锛屾簮涓柇鏉ヨ嚜涓嶅悓鐨勭‖浠舵帶鍒跺櫒锛圥HB4锛夛紝鏆撮湶缁欏鎴锋満鐨?ESB 椤靛簲褰撻€傚簲杩欑鍙樺寲銆?
  褰撹澶囩殑纭欢涓柇琚槧灏勮繘鎴栬В闄ゆ槧灏勫嚭瀹㈡埛鏈?IRQ 鍙风┖闂存椂锛屼細璋冪敤 passthru_irq 杈呭姪鍑芥暟 kvmppc_xive_set_mapped() 鍜?kvmppc_xive_clr_mapped()銆侹VM 璁惧鎵╁睍浜嗚繖浜涜緟鍔╁嚱鏁帮紝浠ユ竻闄ゆ琚槧灏勭殑瀹㈡埛鏈?IRQ 鍙峰搴旂殑 ESB 椤碉紝鐒跺悗璁?VM 缂洪〉澶勭悊绋嬪簭閲嶆柊濉厖銆傝澶勭悊绋嬪簭浼氭彃鍏ヤ笌琚洿閫氳澶囩殑纭欢涓柇鐩稿搴旂殑 ESB 椤碉紱鑻ヨ澶囧凡琚Щ闄わ紝鍒欐彃鍏ュ垵濮嬬殑 IPI ESB 椤点€?
  ESB 閲嶆槧灏勫瀹㈡埛鏈哄拰鎿嶄綔绯荤粺鐨勮澶囬┍鍔ㄦ槸瀹屽叏閫忔槑鐨勩€傛墍鏈夊鐞嗛兘鍦?VFIO 浠ュ強 KVM-PPC 涓殑涓婅堪杈呭姪鍑芥暟鍐呭畬鎴愩€?
- 缁勶細

1. KVM_DEV_XIVE_GRP_CTRL
     鎻愪緵瀵硅澶囩殑鍏ㄥ眬鎺у埗

  灞炴€э細
    1.1 KVM_DEV_XIVE_RESET锛堝彧鍐欙級
    澶嶄綅涓柇鎺у埗鍣ㄤ腑鍏充簬婧愬拰浜嬩欢闃熷垪鐨勯厤缃€備緵 kexec 鍜?kdump 浣跨敤銆?
    閿欒锛氭棤

    1.2 KVM_DEV_XIVE_EQ_SYNC锛堝彧鍐欙級
    鍚屾鎵€鏈夋簮鍜岄槦鍒楋紝骞跺皢 EQ 椤垫爣璁颁负鑴忋€傝繖鏄负浜嗗湪杩佺Щ铏氭嫙鏈烘椂纭繚鎹曡幏鍒颁竴鑷寸殑鍐呭瓨鐘舵€併€?
    閿欒锛氭棤

    1.3 KVM_DEV_XIVE_NR_SERVERS锛堝彧鍐欙級
    kvm_device_attr.addr 鎸囧悜涓€涓?__u32 鍊硷紝璇ュ€间负涓柇鏈嶅姟鍣ㄧ紪鍙风殑鏁伴噺锛堝嵆鍙兘鐨勬渶澶?vcpu id 鍔犱竴锛夈€?
    閿欒锛?
      =======  ==========================================
      -EINVAL  Value greater than KVM_MAX_VCPU_IDS.
      -EFAULT  Invalid user pointer for attr->addr.
      -EBUSY   A vCPU is already connected to the device.
      =======  ==========================================

2. KVM_DEV_XIVE_GRP_SOURCE锛堝彧鍐欙級
     鍦?XIVE 璁惧涓垵濮嬪寲涓€涓柊鐨勬簮骞跺皢鍏跺睆钄斤紙mask锛夈€?
  灞炴€э細
    涓柇婧愮紪鍙? (64-bit)

```
    bits:     | 63   ....  2 |   1   |   0
    values:   |    unused    | level | type

  - type:  0:MSI 1:LSI
  - level: assertion level in case of an LSI.

  Errors:

    =======  ==========================================
    -E2BIG   Interrupt source number is out of range
    -ENOMEM  Could not create a new source block
    -EFAULT  Invalid user pointer for attr->addr.
    -ENXIO   Could not allocate underlying HW interrupt
    =======  ==========================================

```
3. KVM_DEV_XIVE_GRP_SOURCE_CONFIG锛堝彧鍐欙級
     閰嶇疆婧愮殑瀹氬悜锛坱argeting锛?
  灞炴€э細
    涓柇婧愮紪鍙? (64-bit)

```
    bits:     | 63   ....  33 |  32  | 31 .. 3 |  2 .. 0
    values:   |    eisn       | mask |  server | priority

  - priority: 0-7 interrupt priority level
  - server: CPU number chosen to handle the interrupt
  - mask: mask flag (unused)
  - eisn: Effective Interrupt Source Number

  Errors:

    =======  =======================================================
    -ENOENT  Unknown source number
    -EINVAL  Not initialized source number
    -EINVAL  Invalid priority
    -EINVAL  Invalid CPU number.
    -EFAULT  Invalid user pointer for attr->addr.
    -ENXIO   CPU event queues not configured or configuration of the
	     underlying HW interrupt failed
    -EBUSY   No CPU available to serve interrupt
    =======  =======================================================

```
4. KVM_DEV_XIVE_GRP_EQ_CONFIG锛堣鍐欙級
     閰嶇疆鏌愪釜 CPU 鐨勪簨浠堕槦鍒?
  灞炴€э細
    EQ 鎻忚堪绗︽爣璇嗙 (64-bit)

```
    bits:     | 63   ....  32 | 31 .. 3 |  2 .. 0
    values:   |    unused     |  server | priority

  The kvm_device_attr.addr points to::

    struct kvm_ppc_xive_eq {
	__u32 flags;
	__u32 qshift;
	__u64 qaddr;
	__u32 qtoggle;
	__u32 qindex;
	__u8  pad[40];
    };

  - flags: queue flags
      KVM_XIVE_EQ_ALWAYS_NOTIFY (required)
	forces notification without using the coalescing mechanism
	provided by the XIVE END ESBs.
  - qshift: queue size (power of 2)
  - qaddr: real address of queue
  - qtoggle: current queue toggle bit
  - qindex: current queue index
  - pad: reserved for future use

  Errors:

    =======  =========================================
    -ENOENT  Invalid CPU number
    -EINVAL  Invalid priority
    -EINVAL  Invalid flags
    -EINVAL  Invalid queue size
    -EINVAL  Invalid queue address
    -EFAULT  Invalid user pointer for attr->addr.
    -EIO     Configuration of the underlying HW failed
    =======  =========================================

```
5. KVM_DEV_XIVE_GRP_SOURCE_SYNC锛堝彧鍐欙級
     鍚屾璇ユ簮浠ュ埛鏂颁簨浠堕€氱煡

  灞炴€э細
    涓柇婧愮紪鍙? (64-bit)

  閿欒锛?
    =======  =============================
    -ENOENT  Unknown source number
    -EINVAL  Not initialized source number
    =======  =============================

- VCPU 鐘舵€?
  XIVE 涓柇鎺у埗鍣紙IC锛夊湪绉颁负 NVT 鐨勫唴閮ㄧ粨鏋勪腑缁存姢 VP 鐨勪腑鏂姸鎬併€傚綋鏌愪釜 VP 鏈璋冨害鍒扮‖浠跺鐞嗗櫒绾跨▼涓婃椂锛岃嫢璇?VP 鏄煇涓簨浠堕€氱煡鐨勭洰鏍囷紝纭欢灏卞彲浠ユ洿鏂拌繖涓€缁撴瀯銆?
  瀵逛簬杩佺Щ鑰岃█锛屾崟鑾?NVT 涓紦瀛樼殑 IPB 寰堥噸瑕侊紝鍥犱负瀹冨悎鎴愪簡寰呭鐞嗕腑鏂殑浼樺厛绾с€傛垜浠繕浼氬鎹曡幏涓€浜涘唴瀹逛互鎶ュ憡璋冭瘯淇℃伅銆?
```
    bits:     |  63  ....  32  |  31  ....  0  |
    values:   |   TIMA word0   |   TIMA word1  |
    bits:     | 127       ..........       64  |
    values:   |            unused              |

```
- 杩佺Щ锛?
  浣跨敤 XIVE 鍘熺敓鍒╃敤妯″紡淇濆瓨铏氭嫙鏈虹姸鎬佹椂锛屽簲褰撻伒寰竴涓壒瀹氱殑椤哄簭銆傚綋铏氭嫙鏈哄仠姝㈡椂锛?
  1. 灞忚斀锛坢ask锛夋墍鏈夋簮锛圥Q=01锛変互鍋滄浜嬩欢娴併€?
  2. 鐢?KVM 鎺у埗 KVM_DEV_XIVE_EQ_SYNC 鍚屾 XIVE 璁惧锛屼互鍒锋柊鎵€鏈夊湪閫旂殑浜嬩欢閫氱煡骞剁ǔ瀹?EQ銆傚湪姝ら樁娈碉紝EQ 椤佃鏍囪涓鸿剰锛屼互纭繚瀹冧滑鍦ㄨ縼绉诲簭鍒椾腑琚紶杈撱€?
  3. 鎹曡幏婧愮殑瀹氬悜鐘舵€併€丒Q 閰嶇疆浠ュ強绾跨▼涓柇涓婁笅鏂囧瘎瀛樺櫒鐨勭姸鎬併€?
  鎭㈠杩囩▼绫讳技锛?
  1. 鎭㈠ EQ 閰嶇疆锛屽洜涓哄畾鍚戯紙targeting锛変緷璧栦簬瀹冦€?  2. 鎭㈠瀹氬悜
  3. 鎭㈠绾跨▼涓柇涓婁笅鏂?  4. 鎭㈠婧愮姸鎬?  5. 璁?vCPU 杩愯
