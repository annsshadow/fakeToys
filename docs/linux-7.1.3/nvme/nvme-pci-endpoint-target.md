
## NVMe PCI 绔偣鍔熻兘鐩爣


:Author: Damien Le Moal <dlemoal@kernel.org>

NVMe PCI 绔偣鍔熻兘鐩爣椹卞姩浣跨敤閰嶇疆浜?PCI 浼犺緭绫诲瀷鐨?NVMe fabrics 鐩爣
鎺у埗鍣紝瀹炵幇浜嗕竴涓?NVMe PCIe 鎺у埗鍣ㄣ€?
## 姒傝堪


NVMe PCI 绔偣鍔熻兘鐩爣椹卞姩鍏佽閫氳繃 PCIe 閾捐矾鏆撮湶涓€涓?NVMe 鐩爣鎺у埗鍣紝
浠庤€屽疄鐜颁竴涓被浼间簬鏅€?M.2 SSD 鐨?NVMe PCIe 璁惧銆傜洰鏍囨帶鍒跺櫒鐨勫垱寤烘柟寮忎笌
浣跨敤 NVMe over fabrics 鏃剁浉鍚岋細璇ユ帶鍒跺櫒琛ㄧず閫氳繃绔彛璁块棶鏌愪釜 NVMe 瀛愮郴缁熺殑
鎺ュ彛銆傜鍙ｄ紶杈撶被鍨嬪繀椤婚厤缃负 鈥減ci鈥濄€傚瓙绯荤粺鍙厤缃负浣跨敤甯歌鏂囦欢鎴栧潡璁惧
浣滀负鍛藉悕绌洪棿鍚庣锛屼篃鍙娇鐢?NVMe 鐩撮€氾紝鍚?PCI 涓绘満鏆撮湶涓€涓幇鏈夌殑鐗╃悊 NVMe
璁惧锛屾垨涓€涓?NVMe fabrics 涓绘満鎺у埗鍣紙渚嬪涓€涓?NVMe TCP 涓绘満鎺у埗鍣級銆?
NVMe PCI 绔偣鍔熻兘鐩爣椹卞姩灏藉彲鑳戒緷璧?NVMe 鐩爣鏍稿績浠ｇ爜鏉ヨВ鏋愬拰鎵ц鐢?PCIe
涓绘満鎻愪氦鐨?NVMe 鍛戒护銆傜劧鑰岋紝鍊熷姪 PCI 绔偣妗嗘灦 API 鍜?DMA API锛岃椹卞姩杩樿礋璐?绠＄悊閫氳繃 PCIe 閾捐矾杩涜鐨勬墍鏈夋暟鎹紶杈撱€傝繖鎰忓懗鐫€ NVMe PCI 绔偣鍔熻兘鐩爣椹卞姩
瀹炵幇浜嗚嫢骞?NVMe 鏁版嵁缁撴瀯绠＄悊涓庨儴鍒?NVMe 鍛戒护瑙ｆ瀽銆?
1) 璇ラ┍鍔ㄤ娇鐢?DMA锛堣嫢鏀寔锛夋垨 MMIO锛堝惁鍒欙級鏉ヤ粠鎻愪氦闃熷垪涓幏鍙?NVMe 鍛戒护銆?   鍙栧洖鐨勬瘡鏉″懡浠ら殢鍚庝娇鐢ㄤ竴涓伐浣滈」鎵ц锛屼互鍦ㄤ笉鍚?CPU 涓婂苟琛屾墽琛屽鏉″懡浠?   鏉ユ渶澶у寲鎬ц兘銆傝椹卞姩浣跨敤涓€涓伐浣滈」涓嶆柇杞鎵€鏈夋彁浜ら槦鍒楃殑 doorbell
   锛堥棬閾冿級锛屼互妫€娴嬫潵鑷?PCIe 涓绘満鐨勫懡浠ゆ彁浜ゃ€?
2) 璇ラ┍鍔ㄤ娇鐢?MMIO 灏嗗凡瀹屾垚鍛戒护鐨勫畬鎴愰槦鍒楁潯鐩鍒跺埌涓绘満鐨勫畬鎴愰槦鍒楋紝浠庤€?   灏嗗叾浼犺緭缁?PCIe 涓绘満銆傚湪灏嗗畬鎴愭潯鐩姇閫掑埌瀹屾垚闃熷垪鍚庯紝璇ラ┍鍔ㄤ娇鐢?PCI 绔偣
   妗嗘灦 API 鍚戜富鏈鸿Е鍙戜腑鏂紝浠ラ€氱煡鍛戒护瀹屾垚銆?
3) 瀵逛簬浠讳綍甯︽湁鏁版嵁缂撳啿鍖虹殑鍛戒护锛孨VMe PCI 绔偣鐩爣椹卞姩瑙ｆ瀽鍛戒护鐨?PRP 鎴?   SGL 鍒楄〃锛屼互鍒涘缓涓€缁勮〃绀哄懡浠ゆ暟鎹紦鍐插尯鍦ㄤ富鏈轰笂鏄犲皠鐨?PCI 鍦板潃娈靛垪琛ㄣ€?   鍛戒护鏁版嵁缂撳啿鍖洪€氳繃璇ョ粍 PCI 鍦板潃娈典娇鐢?DMA锛堣嫢鏀寔锛夊湪 PCIe 閾捐矾涓婁紶杈撱€?   鑻ヤ笉鏀寔 DMA锛屽垯浣跨敤 MMIO锛岃繖浼氬鑷存€ц兘浣庝笅銆傚浜庡啓鍛戒护锛屽懡浠ゆ暟鎹紦鍐插尯
   鍦ㄦ墽琛屽懡浠ゅ墠锛堜娇鐢ㄧ洰鏍囨牳蹇冧唬鐮侊級浠庝富鏈轰紶杈撳埌鏈湴鍐呭瓨缂撳啿鍖恒€傚浜庤鍛戒护锛?   浼氬垎閰嶄竴涓湰鍦板唴瀛樼紦鍐插尯鏉ユ墽琛屽懡浠わ紝鍛戒护瀹屾垚鍚庤缂撳啿鍖哄唴瀹硅浼犺緭缁欎富鏈恒€?
### 鎺у埗鍣ㄨ兘鍔?

閫氳繃 BAR 0 瀵勫瓨鍣ㄥ悜 PCIe 涓绘満鏆撮湶鐨?NVMe 鑳藉姏锛屽嚑涔庝笌鐩爣鏍稿績浠ｇ爜瀹炵幇鐨?NVMe 鐩爣鎺у埗鍣ㄨ兘鍔涚浉鍚屻€備絾鏈変竴浜涗緥澶栥€?
1) NVMe PCI 绔偣鐩爣椹卞姩濮嬬粓灏嗘帶鍒跺櫒鑳藉姏 CQR 浣嶇疆浣嶏紝浠ヨ姹傗€滆姹傝繛缁槦鍒椻€濄€?   杩欐槸涓轰簡渚夸簬灏嗛槦鍒楃殑 PCI 鍦板潃鑼冨洿鏄犲皠鍒版湰鍦?CPU 鍦板潃绌洪棿銆?
2) doorbell stride锛堥棬閾冩骞咃紝DSTRB锛夊缁堣涓?4B

3) 鐢变簬 PCI 绔偣妗嗘灦娌℃湁鎻愪緵澶勭悊 PCI 灞傜骇澶嶄綅鐨勬柟娉曪紝鎺у埗鍣ㄨ兘鍔?NSSR 浣?   锛圢VM Subsystem Reset Supported锛孨VM 瀛愮郴缁熷浣嶆敮鎸侊級濮嬬粓琚竻闆躲€?
4) 鍚姩鍒嗗尯鏀寔锛圔PS锛夈€佹寔涔呭唴瀛樺尯鍩熸敮鎸侊紙PMRS锛変互鍙婃帶鍒跺櫒鍐呭瓨缂撳啿鍖烘敮鎸?   锛圕MBS锛夎兘鍔涗粠涓嶆姤鍛娿€?
### 鏀寔鐨勭壒鎬?

NVMe PCI 绔偣鐩爣椹卞姩瀹炵幇浜嗗 PRP 鍜?SGL 鐨勬敮鎸併€傝椹卞姩杩樺疄鐜颁簡 IRQ 鍚戦噺
鍚堝苟涓庢彁浜ら槦鍒椾徊瑁佺獊鍙戙€?
闃熷垪鐨勬渶澶ф暟閲忎互鍙婃渶澶ф暟鎹紶杈撳ぇ灏忥紙MDTS锛夊彲鍦ㄥ惎鍔ㄦ帶鍒跺櫒鍓嶉€氳繃 configfs
閰嶇疆銆備负閬垮厤鎵ц鍛戒护鏃舵湰鍦板唴瀛樺崰鐢ㄨ繃澶氾紝MDTS 榛樿涓?512 KB锛屽苟琚檺鍒朵负
鏈€澶?2 MB锛堜汉涓轰笂闄愶級銆?
### 鎵€闇€鐨勬渶灏?PCI 鍦板潃鏄犲皠绐楀彛鏁伴噺


澶у鏁?PCI 绔偣鎺у埗鍣ㄦ彁渚涚殑鏄犲皠绐楀彛鏁伴噺鏈夐檺锛岀敤浜庡皢 PCI 鍦板潃鑼冨洿鏄犲皠鍒?鏈湴 CPU 鍐呭瓨鍦板潃銆侼VMe PCI 绔偣鐩爣鎺у埗鍣ㄥ皢鏄犲皠绐楀彛鐢ㄤ簬浠ヤ笅鐩殑銆?
1) 涓€涓敤浜庤Е鍙?MSI 鎴?MSI-X 涓柇鐨勫唴瀛樼獥鍙?2) 涓€涓敤浜?MMIO 浼犺緭鐨勫唴瀛樼獥鍙?3) 姣忎釜瀹屾垚闃熷垪涓€涓唴瀛樼獥鍙?
鑰冭檻鍒?NVMe PCI 绔偣鐩爣椹卞姩鎿嶄綔鐨勯珮搴﹀紓姝ユ€ц川锛屼笂杩板唴瀛樼獥鍙ｄ竴鑸笉浼氳
鍚屾椂浣跨敤锛屼絾杩欑鎯呭喌鍙兘鍙戠敓銆傚洜姝わ紝鍙敮鎸佺殑瀹夊叏瀹屾垚闃熷垪鏈€澶ф暟閲忕瓑浜?PCI
绔偣鎺у埗鍣ㄧ殑鍐呭瓨鏄犲皠绐楀彛鎬绘暟鍑忓幓浜屻€備緥濡傦紝瀵逛簬涓€涓湁 32 涓彲鐢ㄥ嚭绔欏唴瀛樼獥鍙?鐨勭鐐?PCI 鎺у埗鍣紝鏈€澶氬彲瀹夊叏鍦拌繍琛?30 涓畬鎴愰槦鍒楋紝鑰屼笉浼氭湁鍥犲唴瀛樼獥鍙ｄ笉瓒?瀵艰嚧 PCI 鍦板潃鏄犲皠閿欒鐨勯闄┿€?
### 闃熷垪瀵圭殑鏈€澶ф暟閲?

鍦?NVMe PCI 绔偣鐩爣椹卞姩缁戝畾鍒?PCI 绔偣鎺у埗鍣ㄦ椂锛屼細鍒嗛厤 BAR 0锛屽叾绌洪棿
瓒充互瀹圭撼绠＄悊闃熷垪鍜屽涓?I/O 闃熷垪銆傚彲鏀寔鐨?I/O 闃熷垪瀵圭殑鏈€澶ф暟閲忓彈鑻ュ共鍥犵礌
闄愬埗銆?
1) NVMe 鐩爣鏍稿績浠ｇ爜灏?I/O 闃熷垪鐨勬渶澶ф暟閲忛檺鍒朵负鍦ㄧ嚎 CPU 鐨勬暟閲忋€?2) 鍖呮嫭绠＄悊闃熷垪鍦ㄥ唴鐨勯槦鍒楀鎬绘暟锛屼笉鑳借秴杩囧彲鐢ㄧ殑 MSI-X 鎴?MSI 鍚戦噺鏁伴噺銆?3) 瀹屾垚闃熷垪鐨勬€绘暟涓嶈兘瓒呰繃 PCI 鏄犲皠绐楀彛鎬绘暟鍑?2锛堣涓婃枃锛夈€?
NVMe 绔偣鍔熻兘椹卞姩鍏佽閫氳繃 configfs 閰嶇疆闃熷垪瀵圭殑鏈€澶ф暟閲忋€?
### 闄愬埗涓庡 NVMe 瑙勮寖鐨勪笉鍚堣


涓?NVMe 鐩爣鏍稿績浠ｇ爜绫讳技锛孨VMe PCI 绔偣鐩爣椹卞姩涓嶆敮鎸佸涓彁浜ら槦鍒楀叡鐢?鍚屼竴涓畬鎴愰槦鍒椼€傛墍鏈夋彁浜ら槦鍒楀繀椤绘寚瀹氫竴涓敮涓€鐨勫畬鎴愰槦鍒椼€?

## 鐢ㄦ埛鎸囧崡


鏈妭鎻忚堪纭欢闇€姹傦紝浠ュ強濡備綍鎼缓涓€涓?NVMe PCI 绔偣鐩爣璁惧銆?
### 鍐呮牳闇€姹?

鍐呮牳蹇呴』缂栬瘧鏃跺惎鐢ㄩ厤缃€夐」 CONFIG_PCI_ENDPOINT銆丆ONFIG_PCI_ENDPOINT_CONFIGFS
鍜?CONFIG_NVME_TARGET_PCI_EPF銆侰ONFIG_PCI銆丆ONFIG_BLK_DEV_NVME 鍜?CONFIG_NVME_TARGET 涔熷繀椤诲惎鐢紙杩欐樉鐒讹級銆?
闄ゆ浠ュ锛岃繕搴旇嚦灏戜负鎵€鐢ㄧ殑绔偣纭欢鎻愪緵鑷冲皯涓€涓?PCI 绔偣鎺у埗鍣ㄩ┍鍔ㄣ€?
涓轰究浜庢祴璇曪紝杩樺缓璁惎鐢?null-blk 椹卞姩锛圕ONFIG_BLK_DEV_NULL_BLK锛夈€傝繖鏍峰嵆鍙?浣跨敤涓€涓互 null_blk 鍧楄澶囦綔涓哄瓙绯荤粺鍛藉悕绌洪棿鐨勭畝鍗曟惌寤恒€?
### 纭欢闇€姹?

瑕佷娇鐢?NVMe PCI 绔偣鐩爣椹卞姩锛岃嚦灏戦渶瑕佷竴涓鐐规帶鍒跺櫒璁惧銆?
```

       # ls /sys/class/pci_epc/
        a40000000.pcie-ep

```
```

       # ls /sys/kernel/config/pci_ep/controllers
        a40000000.pcie-ep

```
绔偣鏉垮崱褰撶劧涔熷繀椤婚€氳繃涓€鏍?RX-TX 淇″彿浜ゅ弶鐨?PCI 绾跨紗杩炴帴鍒颁富鏈恒€傚鏋滄墍鐢?鐨勪富鏈?PCI 鎻掓Ы涓嶅叿澶囧嵆鎻掑嵆鐢ㄨ兘鍔涳紝鍒欏簲鍦ㄩ厤缃?NVMe PCI 绔偣璁惧鏃跺叧闂富鏈?鐢垫簮銆?
### NVMe 绔偣璁惧


鍒涘缓涓€涓?NVMe 绔偣璁惧鏄竴涓袱姝ヨ繃绋嬨€傞鍏堬紝蹇呴』瀹氫箟涓€涓?NVMe 鐩爣瀛愮郴缁熷拰
绔彛銆傚叾娆★紝蹇呴』鎼缓 NVMe PCI 绔偣璁惧锛屽苟灏嗗叾缁戝畾鍒版墍鍒涘缓鐨勫瓙绯荤粺鍜岀鍙ｃ€?
### 鍒涘缓 NVMe 瀛愮郴缁熶笌绔彛


濡備綍閰嶇疆 NVMe 鐩爣瀛愮郴缁熷拰绔彛鐨勮缁嗕俊鎭笉鍦ㄦ湰鏂囨。鑼冨洿鍐呫€備笅鏂囦粎鎻愪緵涓€涓?绠€鍗曠ず渚嬶紝灞曠ず涓€涓湁鍗曚釜鐢?null_blk 璁惧浣滀负鍚庣鐨勫懡鍚嶇┖闂寸殑绔彛鍜屽瓙绯荤粺銆?
```

       # mount -t configfs none /sys/kernel/config

```
鎺ヤ笅鏉ワ紝鍒涘缓涓€涓?null_blk 璁惧锛堥粯璁よ缃細缁欏嚭涓€涓?250 GB 鐨勮澶囷級锛?```

        # modprobe null_blk
        # ls /dev/nullb0
        /dev/nullb0

```
```

        # modprobe nvmet_pci_epf
        # lsmod | grep nvmet
        nvmet_pci_epf          32768  0
        nvmet                 118784  1 nvmet_pci_epf
        nvme_core             131072  2 nvmet_pci_epf,nvmet

```
鐜板湪锛屽垱寤轰竴涓瓙绯荤粺鍜岀鍙ｏ紝鎴戜滑灏嗗湪鎼缓 NVMe PCI 绔偣鐩爣璁惧鏃朵娇鐢ㄥ畠浠?鏉ュ垱寤轰竴涓?PCI 鐩爣鎺у埗鍣ㄣ€傚湪姝わ細
```

        # cd /sys/kernel/config/nvmet/subsystems
        # mkdir nvmepf.0.nqn
        # echo -n "Linux-pci-epf" > nvmepf.0.nqn/attr_model
        # echo "0x1b96" > nvmepf.0.nqn/attr_vendor_id
        # echo "0x1b96" > nvmepf.0.nqn/attr_subsys_vendor_id
        # echo 1 > nvmepf.0.nqn/attr_allow_any_host
        # echo 4 > nvmepf.0.nqn/attr_qid_max

```
鎺ヤ笅鏉ワ紝浣跨敤 null_blk 鍧楄澶囧垱寤哄苟鍚敤瀛愮郴缁熷懡鍚嶇┖闂达細
```

        # mkdir nvmepf.0.nqn/namespaces/1
        # echo -n "/dev/nullb0" > nvmepf.0.nqn/namespaces/1/device_path
        # echo 1 > "nvmepf.0.nqn/namespaces/1/enable"

```
```

        # cd /sys/kernel/config/nvmet/ports
        # mkdir 1
        # echo -n "pci" > 1/addr_trtype
        # ln -s /sys/kernel/config/nvmet/subsystems/nvmepf.0.nqn \
                /sys/kernel/config/nvmet/ports/1/subsystems/nvmepf.0.nqn

```
### 鍒涘缓 NVMe PCI 绔偣璁惧


鍦?NVMe 鐩爣瀛愮郴缁熷拰绔彛鍑嗗灏辩华鍚庯紝鐜板湪鍗冲彲鍒涘缓骞跺惎鐢?NVMe PCI 绔偣璁惧銆?NVMe PCI 绔偣鐩爣椹卞姩锛?```

        # ls /sys/kernel/config/pci_ep/functions
        nvmet_pci_epf

```
```

        # cd /sys/kernel/config/pci_ep/functions/nvmet_pci_epf
        # mkdir nvmepf.0
        # ls nvmepf.0/
        baseclass_code    msix_interrupts   secondary
        cache_line_size   nvme              subclass_code
        deviceid          primary           subsys_id
        interrupt_pin     progif_code       subsys_vendor_id
        msi_interrupts    revid             vendorid

```
浣跨敤浠绘剰璁惧 ID 閰嶇疆璇ュ姛鑳斤紙璁惧鐨勫巶鍟?ID 灏嗚嚜鍔ㄨ涓轰笌 NVMe 鐩爣瀛愮郴缁?鍘傚晢 ID 鐩稿悓鐨勫€硷級锛?```

        # cd /sys/kernel/config/pci_ep/functions/nvmet_pci_epf
        # echo 0xBEEF > nvmepf.0/deviceid
        # echo 32 > nvmepf.0/msix_interrupts

```
濡傛灉鎵€鐢ㄧ殑 PCI 绔偣鎺у埗鍣ㄤ笉鏀寔 MSI-X锛屽垯鍙娇鐢?MSI锛?```

        # echo 32 > nvmepf.0/msi_interrupts

```
鎺ヤ笅鏉ワ紝灏嗘垜浠殑绔偣璁惧涓庝箣鍓嶅垱寤虹殑鐩爣瀛愮郴缁熷拰绔彛缁戝畾锛?```

        # echo 1 > nvmepf.0/nvme/portid
        # echo "nvmepf.0.nqn" > nvmepf.0/nvme/subsysnqn

```
闅忓悗鍗冲彲灏嗚绔偣鍔熻兘缁戝畾鍒扮鐐规帶鍒跺櫒骞讹細
```

        # cd /sys/kernel/config/pci_ep
        # ln -s functions/nvmet_pci_epf/nvmepf.0 controllers/a40000000.pcie-ep/
        # echo 1 > controllers/a40000000.pcie-ep/start

```
鍦ㄧ鐐规満鍣ㄤ笂锛屽唴鏍告秷鎭細鏄剧ず NVMe 鐩爣璁惧鍜岀鐐硅澶囪鍒涘缓骞惰繛鎺ユ椂鐨勪俊鎭€?

        null_blk: disk nullb0 created
        null_blk: module loaded
        nvmet: adding nsid 1 to subsystem nvmepf.0.nqn
        nvmet_pci_epf nvmet_pci_epf.0: PCI endpoint controller supports MSI-X, 32 vectors
        nvmet: Created nvm controller 1 for subsystem nvmepf.0.nqn for NQN nqn.2014-08.org.nvmexpress:uuid:2ab90791-2246-4fbb-961d-4c3d5a5a0176.
        nvmet_pci_epf nvmet_pci_epf.0: New PCI ctrl "nvmepf.0.nqn", 4 I/O queues, mdts 524288 B

### PCI 鏍瑰鍚堜綋涓绘満


鍚姩 PCI 涓绘満浼氳Е鍙?PCIe 閾捐矾鐨勫垵濮嬪寲锛圥CI 绔偣椹卞姩鍙兘浠ュ唴鏍告秷鎭彁绀猴級銆?褰撲富鏈?NVMe 椹卞姩鍚敤绔偣鏃讹紝绔偣涓婄殑鍐呮牳娑堟伅涔熶細缁欏嚭鎻愮ず锛?```

        nvmet_pci_epf nvmet_pci_epf.0: Enabling controller

```
鍦ㄤ富鏈轰竴渚э紝NVMe PCI 绔偣鍔熻兘鐩爣璁惧涓猴細
```

        # lspci -n
        0000:01:00.0 0108: 1b96:beef

```
```

        # lsblk
        NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
        nvme0n1     259:0    0   250G  0 disk

```
璇?NVMe 绔偣鍧楄澶囦箣鍚庡彲鍍忎换浣曞叾浠栧父瑙?NVMe 鍛藉悕绌洪棿鍧楄澶囦竴鏍蜂娇鐢ㄣ€?鍛戒护琛屽伐鍏?**nvme** 鍙敤浜庤幏鍙栨洿澶氫俊鎭細
```

        # nvme id-ctrl /dev/nvme0
        NVME Identify Controller:
        vid       : 0x1b96
        ssvid     : 0x1b96
        sn        : 94993c85650ef7bcd625
        mn        : Linux-pci-epf
        fr        : 6.13.0-r
        rab       : 6
        ieee      : 000000
        cmic      : 0xb
        mdts      : 7
        cntlid    : 0x1
        ver       : 0x20100
        ...


```
## 绔偣缁戝畾


NVMe PCI 绔偣鐩爣椹卞姩浣跨敤 PCI 绔偣 configfs 璁惧灞炴€х殑鏂瑰紡濡備笅銆?
================   ===========================================================
vendorid           蹇界暐锛堜娇鐢?NVMe 鐩爣瀛愮郴缁熺殑鍘傚晢 ID锛?deviceid           浠绘剰鍊煎潎鍙紙渚嬪 PCI_ANY_ID锛?revid              涓嶅叧蹇?progif_code        蹇呴』涓?0x02锛圢VM Express锛?baseclass_code     蹇呴』涓?0x01锛圥CI_BASE_CLASS_STORAGE锛?subclass_code      蹇呴』涓?0x08锛圢on-Volatile Memory controller锛?cache_line_size    涓嶅叧蹇?subsys_vendor_id   蹇界暐锛堜娇鐢?NVMe 鐩爣瀛愮郴缁熺殑瀛愮郴缁熷巶鍟?ID锛?subsys_id          浠绘剰鍊煎潎鍙紙渚嬪 PCI_ANY_ID锛?msi_interrupts     鑷冲皯绛変簬鏈熸湜鐨勯槦鍒楀鏁伴噺
msix_interrupts    鑷冲皯绛変簬鏈熸湜鐨勯槦鍒楀鏁伴噺
interrupt_pin      鍦ㄤ笉鏀寔 MSI 鍜?MSI-X 鏃朵娇鐢ㄧ殑 IRQ PIN
================   ===========================================================

NVMe PCI 绔偣鐩爣鍔熻兘鍦ㄥ姛鑳界洰褰曠殑 **nvme** 瀛愮洰褰曚笅杩樻湁涓€浜涚壒瀹氱殑鍙厤缃?瀛楁銆傝繖浜涘瓧娈靛涓嬨€?
================   ===========================================================
mdts_kb            鏈€澶ф暟鎹紶杈撳ぇ灏忥紝鍗曚綅涓?KiB锛堥粯璁わ細512锛?portid             瑕佷娇鐢ㄧ殑鐩爣绔彛 ID
subsysnqn          瑕佷娇鐢ㄧ殑鐩爣瀛愮郴缁?NQN
================   ===========================================================
