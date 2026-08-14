
## PCI Express 绔彛鎬荤嚎椹卞姩鎸囧崡 HOWTO


:Author: Tom L Nguyen tom.l.nguyen@intel.com 11/03/2004
:Copyright: |copy| 2004 Intel Corporation

## 鍏充簬鏈寚鍗?

鏈寚鍗椾粙缁?PCI Express 绔彛鎬荤嚎椹卞姩鐨勫熀纭€鐭ヨ瘑锛屽苟鎻愪緵濡備綍浣垮悇鏈嶅姟椹卞姩鍚?PCI Express 绔彛鎬荤嚎椹卞姩娉ㄥ唽/娉ㄩ攢鐨勪俊鎭€?

## 浠€涔堟槸 PCI Express 绔彛鎬荤嚎椹卞姩


PCI Express 绔彛鏄竴绉嶉€昏緫涓婄殑 PCI-PCI 妗ョ粨鏋勩€侾CI Express 绔彛鏈変袱绉嶇被鍨嬶細
Root Port锛堟牴绔彛锛夊拰 Switch Port锛堜氦鎹㈢鍙ｏ級銆俁oot Port 浠?PCI Express
Root Complex 鍙戣捣涓€鏉?PCI Express 閾捐矾锛岃€?Switch Port 灏?PCI Express 閾捐矾
杩炴帴鍒板唴閮ㄩ€昏緫 PCI 鎬荤嚎銆係witch Port 鐨?secondary 鎬荤嚎浠ｈ〃浜ゆ崲鏈虹殑鍐呴儴璺敱
閫昏緫锛岀О涓轰氦鎹㈡満鐨?Upstream Port锛堜笂娓哥鍙ｏ級銆備氦鎹㈡満鐨?Downstream Port
锛堜笅娓哥鍙ｏ級灏嗕氦鎹㈡満鐨勫唴閮ㄨ矾鐢辨€荤嚎妗ユ帴鍒颁唬琛ㄦ潵鑷?PCI Express 浜ゆ崲鏈虹殑涓嬫父
PCI Express 閾捐矾鐨勬€荤嚎銆?
涓€涓?PCI Express 绔彛鏍规嵁鍏剁鍙ｇ被鍨嬶紝鏈€澶氬彲鎻愪緵鍥涚涓嶅悓鐨勫姛鑳斤紝鏈枃妗ｇО
涔嬩负鏈嶅姟锛坰ervices锛夈€侾CI Express 绔彛鐨勬湇鍔″寘鎷師鐢熺儹鎻掓嫈鏀寔锛圚P锛夈€佺數婧?绠＄悊浜嬩欢鏀寔锛圥ME锛夈€侀珮绾ч敊璇笂鎶ユ敮鎸侊紙AER锛夊拰铏氭嫙閫氶亾鏀寔锛圴C锛夈€傝繖浜涙湇鍔?鍙互鐢变竴涓鏉傜殑椹卞姩缁熶竴澶勭悊锛屼篃鍙互鍒嗗埆鍒嗗竷骞剁敱鐩稿簲鐨勬湇鍔￠┍鍔ㄥ鐞嗐€?
## 涓轰綍浣跨敤 PCI Express 绔彛鎬荤嚎椹卞姩锛?

鍦ㄧ幇鏈夌殑 Linux 鍐呮牳涓紝Linux 璁惧椹卞姩妯″瀷锛圠inux Device Driver Model锛夊厑璁?涓€涓墿鐞嗚澶囦粎鐢卞崟涓┍鍔ㄥ鐞嗐€侾CI Express 绔彛鏄竴涓叿鏈夊涓笉鍚屾湇鍔＄殑
PCI-PCI 妗ヨ澶囥€備负淇濇寔绠€娲佺殑瑙ｅ喅鏂规锛屾瘡涓湇鍔″彲浠ユ湁鑷繁鐨勮蒋浠舵湇鍔￠┍鍔ㄣ€?杩欑鎯呭喌涓嬶紝澶氫釜鏈嶅姟椹卞姩浼氱珵浜夊悓涓€涓?PCI-PCI 妗ヨ澶囥€備緥濡傦紝濡傛灉 PCI Express
Root Port 鐨勫師鐢熺儹鎻掓嫈鏈嶅姟椹卞姩鍏堣鍔犺浇锛屽畠灏变細鍗犵敤涓€涓?PCI-PCI 妗?Root Port銆?鍥犳鍐呮牳涓嶄細涓鸿 Root Port 鍔犺浇鍏朵粬鏈嶅姟椹卞姩銆傛崲鍙ヨ瘽璇达紝浣跨敤褰撳墠鐨勯┍鍔ㄦā鍨嬶紝
涓嶅彲鑳借澶氫釜鏈嶅姟椹卞姩鍚屾椂鍔犺浇骞惰繍琛屽湪鍚屼竴涓?PCI-PCI 妗ヨ澶囦笂銆?
瑕佷娇澶氫釜鏈嶅姟椹卞姩鑳藉鍚屾椂杩愯锛屽氨闇€瑕佷竴涓?PCI Express 绔彛鎬荤嚎椹卞姩锛屽畠绠＄悊
鎵€鏈夊凡濉厖鐨?PCI Express 绔彛锛屽苟鎸夐渶灏嗘彁渚涚殑鎵€鏈夋湇鍔¤姹傚垎鍙戠粰鐩稿簲鐨勬湇鍔?椹卞姩銆備娇鐢?PCI Express 绔彛鎬荤嚎椹卞姩鐨勪竴浜涗富瑕佷紭鐐瑰涓嬶細

  - 鍏佽鍦?PCI-PCI 妗ョ鍙ｈ澶囦笂鍚屾椂杩愯澶氫釜鏈嶅姟椹卞姩銆?
  - 鍏佽浠ョ嫭绔嬬殑鍒嗛樁娈垫柟寮忓疄鐜版湇鍔￠┍鍔ㄣ€?
  - 鍏佽涓€涓湇鍔￠┍鍔ㄨ繍琛屽湪澶氫釜 PCI-PCI 妗ョ鍙ｈ澶囦笂銆?
  - 灏?PCI-PCI 妗ョ鍙ｈ澶囩殑璧勬簮绠＄悊骞跺垎鍙戠粰璇锋眰鐨勬湇鍔￠┍鍔ㄣ€?
## 閰嶇疆 PCI Express 绔彛鎬荤嚎椹卞姩涓庢湇鍔￠┍鍔ㄧ殑姣旇緝


### 灏?PCI Express 绔彛鎬荤嚎椹卞姩鏀寔缂栧叆鍐呮牳


鏄惁鍖呭惈 PCI Express 绔彛鎬荤嚎椹卞姩锛屽彇鍐充簬鍐呮牳閰嶇疆涓槸鍚﹀寘鍚?PCI Express
鏀寔銆傚綋鍐呮牳鍚敤浜?PCI Express 鏀寔鏃讹紝鍐呮牳浼氳嚜鍔ㄥ皢 PCI Express 绔彛鎬荤嚎
椹卞姩浣滀负鍐呮牳椹卞姩鍖呭惈杩涙潵銆?
### 鍚敤鏈嶅姟椹卞姩鏀寔


PCI 璁惧椹卞姩鍩轰簬 Linux 璁惧椹卞姩妯″瀷瀹炵幇銆傛墍鏈夋湇鍔￠┍鍔ㄩ兘鏄?PCI 璁惧椹卞姩銆?濡備笂鎵€杩帮紝涓€鏃﹀唴鏍稿姞杞戒簡 PCI Express 绔彛鎬荤嚎椹卞姩锛屽氨涓嶅彲鑳藉啀鍔犺浇浠讳綍鏈嶅姟
椹卞姩銆傝绗﹀悎 PCI Express 绔彛鎬荤嚎椹卞姩妯″瀷锛岄渶瑕佸鐜版湁鏈嶅姟椹卞姩鍋氫竴浜涙渶灏忕殑
鏀瑰姩锛屼笖杩欎簺鏀瑰姩涓嶄細褰卞搷鐜版湁鏈嶅姟椹卞姩鐨勫姛鑳姐€?
鏈嶅姟椹卞姩闇€瑕佷娇鐢ㄤ笅闈㈡墍绀虹殑涓や釜 API 灏嗗叾鏈嶅姟娉ㄥ唽鍒?PCI Express 绔彛鎬荤嚎椹卞姩
锛堝弬瑙?5.2.1 鍜?5.2.2 鑺傦級銆傞噸瑕佺殑鏄紝鏈嶅姟椹卞姩鍦ㄨ皟鐢ㄨ繖浜?API 涔嬪墠锛屽繀椤诲厛
鍒濆鍖?pcie_port_service_driver 鏁版嵁缁撴瀯锛岃缁撴瀯浣嶄簬澶存枃浠?/include/linux/pcieport_if.h 涓€傝嫢涓嶈繖鏍峰仛灏嗗鑷磋韩浠戒笉鍖归厤锛屼娇 PCI Express
绔彛鎬荤嚎椹卞姩鏃犳硶鍔犺浇璇ユ湇鍔￠┍鍔ㄣ€?
#### pcie_port_service_register


```

  int pcie_port_service_register(struct pcie_port_service_driver *new)

```
璇?API 鍙栦唬浜?Linux 椹卞姩妯″瀷鐨?pci_register_driver API銆傛湇鍔￠┍鍔ㄥ簲濮嬬粓鍦ㄦā鍧?鍒濆鍖栵紙module init锛夋椂璋冪敤 pcie_port_service_register銆傛敞鎰忥紝鏈嶅姟椹卞姩鍔犺浇鍚庯紝
璇稿 pci_enable_device(dev) 鍜?pci_set_master(dev) 涔嬬被鐨勮皟鐢ㄤ笉鍐嶅繀瑕侊紝鍥犱负
杩欎簺璋冪敤鐢?PCI 绔彛鎬荤嚎椹卞姩鎵ц銆?
#### pcie_port_service_unregister


```

  void pcie_port_service_unregister(struct pcie_port_service_driver *new)

```
pcie_port_service_unregister 鍙栦唬 Linux 椹卞姩妯″瀷鐨?pci_unregister_driver銆傚畠
鍦ㄦā鍧楅€€鍑烘椂鎬绘槸鐢辨湇鍔￠┍鍔ㄨ皟鐢ㄣ€?
#### 绀轰緥浠ｇ爜


涓嬮潰鏄敤浜庡垵濮嬪寲绔彛鏈嶅姟椹卞姩鏁版嵁缁撴瀯鐨勭ず渚嬫湇鍔￠┍鍔ㄤ唬鐮併€?
```

  static struct pcie_port_service_id service_id[] = { {
    .vendor = PCI_ANY_ID,
    .device = PCI_ANY_ID,
    .port_type = PCIE_RC_PORT,
    .service_type = PCIE_PORT_SERVICE_AER,
    }, { /* end: all zeroes */ }
  };

  static struct pcie_port_service_driver root_aerdrv = {
    .name		= (char *)device_name,
    .id_table	= service_id,

    .probe		= aerdrv_load,
    .remove		= aerdrv_unload,

    .suspend	= aerdrv_suspend,
    .resume		= aerdrv_resume,
  };

```

涓嬮潰鏄敞鍐?娉ㄩ攢鏈嶅姟椹卞姩鐨勭ず渚嬩唬鐮併€?
```

  static int __init aerdrv_service_init(void)
  {
    int retval = 0;

    retval = pcie_port_service_register(&root_aerdrv);
    if (!retval) {
      /*
      * FIX ME
      */
    }
    return retval;
  }

  static void __exit aerdrv_service_exit(void)
  {
    pcie_port_service_unregister(&root_aerdrv);
  }

  module_init(aerdrv_service_init);
  module_exit(aerdrv_service_exit);

```

## 鍙兘鐨勮祫婧愬啿绐?

鐢变簬鍏佽 PCI-PCI 妗ョ鍙ｈ澶囩殑鎵€鏈夋湇鍔￠┍鍔ㄥ悓鏃惰繍琛岋紝涓嬮潰鍒楀嚭鍑犵鍙兘鐨勮祫婧?鍐茬獊鍙婂缓璁殑瑙ｅ喅鏂规銆?
### MSI 涓?MSI-X 鍚戦噺璧勬簮


涓€鏃﹀湪璁惧涓婂惎鐢ㄤ簡 MSI 鎴?MSI-X 涓柇锛岃澶囧氨浼氫繚鎸佽妯″紡锛岀洿鍒板啀娆¤绂佺敤銆?鐢变簬鍚屼竴 PCI-PCI 妗ョ鍙ｇ殑鏈嶅姟椹卞姩鍏变韩鍚屼竴涓墿鐞嗚澶囷紝濡傛灉鏌愪釜鏈嶅姟椹卞姩鍚敤
鎴栫鐢?MSI/MSI-X 妯″紡锛屽彲鑳戒細瀵艰嚧涓嶅彲棰勬湡鐨勮涓恒€?
涓洪伩鍏嶈繖绉嶆儏鍐碉紝鎵€鏈夋湇鍔￠┍鍔ㄩ兘涓嶅厑璁稿湪鍏惰澶囦笂鍒囨崲涓柇妯″紡銆侾CI Express 绔彛
鎬荤嚎椹卞姩璐熻矗纭畾涓柇妯″紡锛屼笖杩欏鏈嶅姟椹卞姩搴旀槸閫忔槑鐨勩€傛湇鍔￠┍鍔ㄥ彧闇€浜嗚В鍒嗛厤缁?struct pcie_device 鐨?irq 瀛楁鐨勫悜閲?IRQ锛岃瀛楁鍦?PCI Express 绔彛鎬荤嚎椹卞姩
鎺㈡祴姣忎釜鏈嶅姟椹卞姩鏃朵紶鍏ャ€傛湇鍔￠┍鍔ㄥ簲浣跨敤 (struct pcie_device*)dev->irq 鏉ヨ皟鐢?request_irq/free_irq銆傛澶栵紝涓柇妯″紡瀛樺偍鍦?struct pcie_device 鐨?interrupt_mode
瀛楁涓€?
### PCI 鍐呭瓨/IO 鏄犲皠鍖哄煙


鐢ㄤ簬 PCI Express 鐢垫簮绠＄悊锛圥ME锛夈€侀珮绾ч敊璇笂鎶ワ紙AER锛夈€佺儹鎻掓嫈锛圚P锛夊拰铏氭嫙
閫氶亾锛圴C锛夌殑鏈嶅姟椹卞姩浼氳闂?PCI Express 绔彛涓婄殑 PCI 閰嶇疆绌洪棿銆傚湪鎵€鏈夋儏鍐典笅锛?鎵€璁块棶鐨勫瘎瀛樺櫒褰兼鐙珛銆傛湰鏂囧亣璁炬墍鏈夋湇鍔￠┍鍔ㄩ兘浼氳〃鐜拌壇濂斤紝涓嶄細瑕嗙洊鍏朵粬鏈嶅姟
椹卞姩鐨勯厤缃缃€?
### PCI 閰嶇疆瀵勫瓨鍣?

姣忎釜鏈嶅姟椹卞姩閮藉湪鍏惰嚜韬殑鑳藉姏缁撴瀯锛坈apability structure锛変笂鎵ц PCI 閰嶇疆鎿嶄綔锛?浣?PCI Express 鑳藉姏缁撴瀯闄ゅ锛屽畠琚寘鎷湇鍔￠┍鍔ㄥ湪鍐呯殑璁稿椹卞姩鍏变韩銆俁MW 鑳藉姏
璁块棶鍣紙pcie_capability_clear_and_set_word()銆乸cie_capability_set_word() 鍜?pcie_capability_clear_word()锛変細淇濇姢涓€缁勯€夊畾鐨?PCI Express 鑳藉姏瀵勫瓨鍣細

- Link Control Register
- Root Control Register
- Link Control 2 Register

瀵硅繖浜涘瘎瀛樺櫒鐨勪换浣曟洿鏀归兘搴斾娇鐢?RMW 璁块棶鍣ㄨ繘琛岋紝浠ラ伩鍏嶅洜骞跺彂鏇存柊鑰屼骇鐢熼棶棰樸€?鏈夊叧鍙椾繚鎶ゅ瘎瀛樺櫒鐨勬渶鏂板垪琛紝璇峰弬闃?pcie_capability_clear_and_set_word()銆?