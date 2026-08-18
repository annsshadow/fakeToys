
## TPH 鏀寔


:Copyright: 2024 Advanced Micro Devices, Inc.
:Authors: - Eric van Tassell <eric.vantassell@amd.com>
          - Wei Huang <wei.huang2@amd.com>


## 姒傝堪


TPH锛圱LP Processing Hints锛孴LP 澶勭悊鎻愮ず锛夋槸涓€椤?PCIe 鐗规€э紝瀹冨厑璁哥鐐硅澶囦负鎸囧悜鍐呭瓨绌洪棿鐨?璇锋眰鎻愪緵浼樺寲鎻愮ず銆傝繖浜涙彁绀轰互涓€绉嶇О涓鸿浆鍚戞爣绛撅紙Steering Tags锛孲Ts锛夌殑鏍煎紡宓屽叆鍒拌姹傛柟鐨?TLP
澶翠腑锛屼娇绯荤粺纭欢锛堝鏍瑰鍚堜綋 Root Complex锛夎兘澶熸洿濂藉湴涓鸿繖浜涜姹傜鐞嗗钩鍙拌祫婧愩€?
渚嬪锛屽湪鏀寔鍩轰簬 TPH 鐨勭洿鎺ユ暟鎹紦瀛樻敞鍏ョ殑骞冲彴涓婏紝绔偣璁惧鍙互鍦ㄥ叾 DMA 娴侀噺涓寘鍚€傚綋鐨?ST锛?浠ユ寚瀹氭暟鎹簲琚啓鍏ュ摢涓紦瀛樸€傝繖浣垮緱 CPU 鏍稿績鏈夋洿楂樼殑姒傜巼浠庣紦瀛樹腑鑾峰彇鏁版嵁锛屼粠鑰屽彲鑳芥彁鍗囨€ц兘骞?闄嶄綆鏁版嵁澶勭悊涓殑寤惰繜銆?

## 濡備綍浣跨敤 TPH


TPH 鍦?PCIe 涓〃鐜颁负涓€涓彲閫夌殑鎵╁睍鑳藉姏銆侺inux 鍐呮牳鍦ㄥ惎鍔ㄦ椂澶勭悊 TPH 鐨勫彂鐜帮紝浣嗚澶囬┍鍔ㄨ嫢瑕?浣跨敤 TPH锛屽垯闇€鑷璇锋眰鍚敤 TPH銆備竴鏃﹀惎鐢紝椹卞姩浣跨敤鎻愪緵鐨?API 鑾峰彇鐩爣鍐呭瓨鐨勮浆鍚戞爣绛撅紙Steering
Tag锛夛紝骞跺皢璇?ST 缂栫▼鍒拌澶囩殑 ST 琛ㄤ腑銆?
### 鍦?Linux 涓惎鐢?TPH 鏀寔


瑕佹敮鎸?TPH锛屽唴鏍稿繀椤诲惎鐢?CONFIG_PCIE_TPH 閫夐」鏉ユ瀯寤恒€?
### 绠＄悊 TPH


```

  int pcie_enable_tph(struct pci_dev *pdev, int mode);

```
姝ゅ嚱鏁颁负璁惧鍚敤鍏锋湁鐗瑰畾 ST 妯″紡鐨?TPH 鏀寔銆傚綋鍓嶆敮鎸佺殑妯″紡鍖呮嫭锛?
  - PCI_TPH_ST_NS_MODE - 鏃?ST 妯″紡
  - PCI_TPH_ST_IV_MODE - 涓柇鍚戦噺妯″紡
  - PCI_TPH_ST_DS_MODE - 璁惧鐗瑰畾妯″紡

`pcie_enable_tph()` 鍦ㄥ惎鐢ㄥ墠浼氭鏌ヨ澶囨槸鍚﹀疄闄呮敮鎸佹墍璇锋眰鐨勬ā寮忋€傝澶囬┍鍔ㄥ彲浠ユ牴鎹?`pcie_enable_tph()` 鐨勮繑鍥炲€煎垽鏂敮鎸佸摢绉?TPH 妯″紡锛屽苟鎹姝ｇ‘鍦板惎鐢ㄣ€?
```

  void pcie_disable_tph(struct pci_dev *pdev);

```
### 绠＄悊 ST


杞悜鏍囩锛圫teering Tags锛夋槸骞冲彴鐗瑰畾鐨勩€侾CIe 瑙勮寖骞舵湭瑙勫畾 ST 鏉ヨ嚜浣曞銆傜浉鍙嶏紝PCI 鍥轰欢瑙勮寖
瀹氫箟浜嗕竴涓?ACPI _DSM 鏂规硶锛堝弬瑙?`Revised _DSM for Cache Locality TPH Features ECN
<https://members.pcisig.com/wg/PCI-SIG/document/15470>`_锛夛紝鐢ㄤ簬妫€绱㈠叿鏈夊悇绉嶅睘鎬х殑鐩爣鍐呭瓨鐨?ST銆傛湰瀹炵幇鏀寔鐨勫氨鏄鏂规硶銆?
瑕佹绱笌鐗瑰畾 CPU 鍏宠仈鐨勭洰鏍囧唴瀛樼殑杞悜鏍囩锛屼娇鐢?```

  int pcie_tph_get_cpu_st(struct pci_dev *pdev, enum tph_mem_type type,
                          unsigned int cpu, u16 *tag);

```
`type` 鍙傛暟鐢ㄤ簬鎸囧畾鐩爣鍐呭瓨鐨勭被鍨嬶紝鍙互鏄槗澶辨€э紙volatile锛夋垨鎸佷箙鎬э紙persistent锛夈€?`cpu` 鍙傛暟鎸囧畾鍐呭瓨鎵€鍏宠仈鐨?CPU銆?
妫€绱㈠埌 ST 鍊煎悗锛岃澶囬┍鍔ㄥ彲浠ヤ娇鐢ㄤ互涓嬪嚱鏁?```

  int pcie_tph_set_st_entry(struct pci_dev *pdev, unsigned int index,
                            u16 tag);

```
`index` 鍙傛暟鏄?ST 鏍囩灏嗚鍐欏叆鐨?ST 琛ㄦ潯鐩储寮曘€俙pcie_tph_set_st_entry()` 浼氱‘瀹?ST 琛ㄧ殑
姝ｇ‘浣嶇疆锛堟棤璁烘槸鍦?MSI-X 琛ㄤ腑杩樻槸鍦?TPH 鎵╁睍鑳藉姏绌洪棿涓級锛屽苟灏嗚浆鍚戞爣绛惧啓鍏ョ敱 `index` 鍙傛暟
鎸囧悜鐨?ST 鏉＄洰銆?
濡備綍浣跨敤杩欎簺 TPH 鍑芥暟瀹屽叏鐢遍┍鍔ㄥ喅瀹氥€備緥濡傦紝缃戠粶璁惧鐨勯┍鍔ㄥ彲浠ュ湪 RX/TX 闃熷垪鐨勪腑鏂翰鍜屾€?鍙戠敓鏀瑰彉鏃讹紝浣跨敤涓婅堪 TPH API 鏉ユ洿鏂拌浆鍚戞爣绛俱€備笅闈㈡槸涓€涓腑鏂翰鍜屾€ч€氱煡鍣ㄧ殑绀轰緥浠ｇ爜锛?

    static void irq_affinity_notified(struct irq_affinity_notify *notify,
                                      const cpumask_t *mask)
    {
         struct drv_irq *irq;
         unsigned int cpu_id;
         u16 tag;

         irq = container_of(notify, struct drv_irq, affinity_notify);
         cpumask_copy(irq->cpu_mask, mask);

         /** 閫夋嫨涓€涓悎閫傜殑 CPU 浣滀负鐩爣 - 杩欓噷浠呬綔绀轰緥 **/
         cpu_id = cpumask_first(irq->cpu_mask);

         if (pcie_tph_get_cpu_st(irq->pdev, TPH_MEM_TYPE_VM, cpu_id,
                                 &tag))
             return;

         if (pcie_tph_set_st_entry(irq->pdev, irq->msix_nr, tag))
             return;
    }

### 绯荤粺鑼冨洿鍐呯鐢?TPH


鏈変竴涓彲鐢ㄧ殑鍐呮牳鍛戒护琛岄€夐」鏉ユ帶鍒?TPH 鐗规€э細
    - "notph"锛歍PH 灏嗗鎵€鏈夌鐐硅澶囩鐢ㄣ€?