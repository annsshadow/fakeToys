
## The PCI Express Advanced Error Reporting Driver Guide HOWTO


:浣滆€? - T. Long Nguyen <tom.l.nguyen@intel.com>
          - Yanmin Zhang <yanmin.zhang@intel.com>

:鐗堟潈: |copy| 2006 Intel Corporation

## Overview


### About this guide


鏈寚鍗楁弿杩?PCI Express锛圥CIe锛夐珮绾ч敊璇姤鍛婏紙Advanced Error Reporting锛孉ER锛夐┍鍔ㄧ殑鍩虹
鐭ヨ瘑锛屽苟鎻愪緵鏈夊叧濡備綍浣跨敤瀹冿紝浠ュ強濡備綍浣跨鐐硅澶囬┍鍔ㄧ鍚?PCIe AER 椹卞姩鐨勪俊鎭€?


### What is the PCIe AER Driver?


PCIe 閿欒淇″彿鍙互鍙戠敓鍦?PCIe 閾捐矾鏈韩涓婏紝涔熷彲浠ヤ唬琛ㄥ湪閾捐矾涓婂彂璧风殑浜嬪姟銆侾CIe 瀹氫箟浜嗕袱绉?
閿欒鎶ュ憡鑼冨紡锛氬熀绾胯兘鍔涳紙baseline capability锛夊拰楂樼骇閿欒鎶ュ憡鑳藉姏銆傛墍鏈?PCIe 缁勪欢閮藉繀椤?
鎻愪緵鍩虹嚎鑳藉姏锛屽畠瀹氫箟浜嗕竴缁勬渶灏忛敊璇姤鍛婅姹傘€傞珮绾ч敊璇姤鍛婅兘鍔涢€氳繃 PCIe 楂樼骇閿欒鎶ュ憡鎵╁睍
鑳藉姏缁撴瀯瀹炵幇锛屾彁渚涙洿鍋ュ．鐨勯敊璇姤鍛娿€?

PCIe AER 椹卞姩鎻愪緵浜嗘敮鎸?PCIe 楂樼骇閿欒鎶ュ憡鑳藉姏鐨勫熀纭€璁炬柦銆侾CIe AER 椹卞姩鎻愪緵涓変釜鍩烘湰鍔熻兘锛?

  - 濡傛灉鍙戠敓閿欒锛屾敹闆嗗叏闈㈢殑閿欒淇℃伅銆?
  - 鍚戠敤鎴锋姤鍛婇敊璇€?
  - 鎵ц閿欒鎭㈠鎿嶄綔銆?

AER 椹卞姩鍙寕鎺ュ埌鏀寔 PCIe AER 鑳藉姏鐨?Root Port 鍜?RCEC 涓娿€?


## User Guide


### Include the PCIe AER Root Driver into the Linux Kernel


PCIe AER 椹卞姩鏄竴涓€氳繃 PCIe Port Bus 椹卞姩鎸傛帴鐨?Root Port 鏈嶅姟椹卞姩銆傚鏋滅敤鎴锋兂浣跨敤瀹冿紝
蹇呴』缂栬瘧璇ラ┍鍔ㄣ€傚畠鐢?CONFIG_PCIEAER 鍚敤锛岃€?CONFIG_PCIEAER 渚濊禆浜?CONFIG_PCIEPORTBUS銆?

### Load PCIe AER Root Driver


鏌愪簺绯荤粺鐨勫浐浠朵腑甯︽湁 AER 鏀寔銆傚湪鍥轰欢澶勭悊 AER 鐨勫悓鏃跺惎鐢?Linux 鐨?AER 鏀寔浼氬鑷翠笉鍙娴?
鐨勮涓恒€傚洜姝わ紝闄ら潪鍥轰欢閫氳繃 ACPI _OSC 鏂规硶灏?AER 鎺у埗鏉冩巿浜堟搷浣滅郴缁燂紝鍚﹀垯 Linux 涓嶅鐞?
AER 浜嬩欢銆傛湁鍏?_OSC 鐢ㄦ硶鐨勮缁嗕俊鎭紝璇峰弬闃?PCI 鍥轰欢瑙勮寖銆?

### AER error output


褰撴崟鑾峰埌 PCIe AER 閿欒鏃讹紝浼氬悜鎺у埗鍙拌緭鍑轰竴鏉￠敊璇秷鎭€傚鏋滄槸鍙籂姝ｉ敊璇紙correctable
error锛夛紝鍒欎綔涓鸿鍛婃秷鎭緭鍑恒€傚惁鍒欙紝浣滀负閿欒娑堟伅鎵撳嵃銆傚洜姝ょ敤鎴峰彲浠ラ€夋嫨涓嶅悓鐨勬棩蹇楃骇鍒潵
杩囨护鎺夊彲绾犳閿欒娑堟伅銆?

```

  0000:50:00.0: PCIe Bus Error: severity=Uncorrectable (Fatal), type=Transaction Layer, (Requester ID)
  0000:50:00.0:   device [8086:0329] error status/mask=00100000/00000000
  0000:50:00.0:    [20] UnsupReq               (First)
  0000:50:00.0:   TLP Header: 0x04000001 0x00200a03 0x05010000 0x00050100

```
鍦ㄧず渚嬩腑锛屸€淩equester ID鈥濇寚灏嗛敊璇秷鎭彂閫佺粰 Root Port 鐨勮澶囩殑 ID銆傚叾浠栧瓧娈佃鍙傞槄 PCIe
瑙勮寖銆?

鈥淭LP Header鈥濇槸寮曡捣閿欒鐨?TLP 鐨勫師濮嬪崄鍏繘鍒舵牸寮忕殑鍓嶇紑/澶撮儴銆傝灏?TLP Header 瑙ｇ爜涓哄彲璇?
褰㈠紡锛屽彲浠ヤ娇鐢?tlp-tool锛?

https://github.com/mmpg-x86/tlp-tool

```

  curl -L https://git.kernel.org/linus/2ca1c94ce0b6 | rtlp-tool --aer

```
### AER Ratelimits


鐢变簬姣忎釜浜嬪姟閮藉彲鑳戒骇鐢熼敊璇秷鎭紝鎴戜滑鍙兘浼氱湅鍒板ぇ閲忔姤鍛婄殑閿欒銆備负浜嗛槻姝㈠璇濈殑璁惧娣规病
鎺у埗鍙?鍋滄粸鎵ц锛屾秷鎭寜璁惧鍜岄敊璇被鍨嬶紙鍙籂姝?vs. 闈炶嚧鍛戒笉鍙籂姝ｏ級杩涜闄愭祦銆傝嚧鍛介敊璇?
锛堝寘鎷?DPC 閿欒锛変笉鍙楅€熺巼闄愬埗銆?

AER 浣跨敤榛樿鐨勯€熺巼闄愬埗锛欴EFAULT_RATELIMIT_BURST锛?0 涓簨浠讹級鍦?DEFAULT_RATELIMIT_INTERVAL
锛? 绉掞級鍐呫€?

閫熺巼闄愬埗浠?sysfs 灞炴€х殑褰㈠紡鏆撮湶锛屽苟涓斿彲閰嶇疆銆傝鍙傞槄
Documentation/ABI/testing/sysfs-bus-pci-devices-aer銆?

### AER Statistics / Counters


褰撴崟鑾峰埌 PCIe AER 閿欒鏃讹紝璁℃暟鍣?缁熻淇℃伅涔熶互 sysfs 灞炴€х殑褰㈠紡鏆撮湶锛岃褰曚簬
Documentation/ABI/testing/sysfs-bus-pci-devices-aer銆?

## Developer Guide


瑕佸惎鐢ㄩ敊璇仮澶嶏紝杞欢椹卞姩蹇呴』鎻愪緵鍥炶皟鍑芥暟銆?

涓轰簡鏇村ソ鍦扮悊瑙?AER锛屽紑鍙戣€呴渶瑕佷簡瑙?AER 鐨勫伐浣滃師鐞嗐€?

PCIe 閿欒鍒嗕负涓ょ被锛氬彲绾犳閿欒鍜屼笉鍙籂姝ｉ敊璇€傝繖绉嶅垎绫诲熀浜庤繖浜涢敊璇殑褰卞搷锛屽彲鑳藉鑷存€ц兘
涓嬮檷鎴栧姛鑳藉け鏁堛€?

鍙籂姝ｉ敊璇鎺ュ彛鐨勫姛鑳芥病鏈変换浣曞奖鍝嶃€侾CIe 鍗忚鍙互鍦ㄤ笉闇€瑕佷换浣曡蒋浠跺共棰勬垨浠讳綍鏁版嵁涓㈠け鐨?
鎯呭喌涓嬫仮澶嶃€傝繖浜涢敊璇敱纭欢妫€娴嬪苟绾犳銆?

涓庡彲绾犳閿欒涓嶅悓锛屼笉鍙籂姝ｉ敊璇細褰卞搷鎺ュ彛鐨勫姛鑳姐€備笉鍙籂姝ｉ敊璇彲鑳藉鑷寸壒瀹氫簨鍔℃垨鐗瑰畾 PCIe
閾捐矾涓嶅彲闈犮€傛牴鎹繖浜涢敊璇姸鍐碉紝涓嶅彲绾犳閿欒杩涗竴姝ュ垎涓洪潪鑷村懡閿欒锛坣on-fatal error锛夊拰鑷村懡
閿欒锛坒atal error锛夈€傞潪鑷村懡閿欒瀵艰嚧鐗瑰畾浜嬪姟涓嶅彲闈狅紝浣?PCIe 閾捐矾鏈韩瀹屽叏姝ｅ父銆傚彟涓€鏂归潰锛?
鑷村懡閿欒瀵艰嚧閾捐矾涓嶅彲闈犮€?

褰撳惎鐢?PCIe 閿欒鎶ュ憡鏃讹紝璁惧鎹曡幏鍒伴敊璇悗浼氳嚜鍔ㄥ悜涓婇潰鐨?Root Port 鍙戦€佷竴鏉￠敊璇秷鎭€俁oot
Port 鍦ㄦ敹鍒伴敊璇姤鍛婃秷鎭悗锛屼細鍦ㄥ叾 AER 鑳藉姏缁撴瀯涓唴閮ㄥ鐞嗗苟璁板綍璇ラ敊璇秷鎭€傝璁板綍鐨勯敊璇?
淇℃伅鍖呮嫭灏嗛敊璇姤鍛婁唬鐞嗙殑 Requester ID 瀛樺叆閿欒婧愯瘑鍒瘎瀛樺櫒锛屽苟鐩稿簲鍦拌缃?Root Error
Status 瀵勫瓨鍣ㄧ殑閿欒浣嶃€傚鏋滃湪 Root Error Command 瀵勫瓨鍣ㄤ腑鍚敤浜?AER 閿欒鎶ュ憡锛孯oot Port
鍦ㄦ娴嬪埌閿欒鏃朵細鐢熸垚涓€涓腑鏂€?

娉ㄦ剰锛屼笂杩伴敊璇笌 PCIe 灞傜骇缁撴瀯鍜岄摼璺湁鍏炽€傝繖浜涢敊璇笉鍖呮嫭浠讳綍璁惧鐗瑰畾鐨勯敊璇紝鍥犱负璁惧鐗瑰畾
閿欒浠嶄細鐩存帴鍙戦€佺粰璁惧椹卞姩銆?

### Provide callbacks


#### PCI error-recovery callbacks


PCIe AER Root 椹卞姩鍦ㄦ墽琛岄敊璇仮澶嶆搷浣滄椂锛屼娇鐢ㄩ敊璇洖璋冩潵涓庢墍娑夊強灞傜骇缁撴瀯涓殑涓嬫父璁惧椹卞姩
鍗忚皟銆?

鏁版嵁缁撴瀯 pci_driver 鏈変竴涓寚閽?err_handler锛屾寚鍚?pci_error_handlers锛屽悗鑰呯敱鍑犱釜鍥炶皟鍑芥暟
鎸囬拡缁勬垚銆傞櫎浜?PCIe 鐗瑰畾鐨勯儴鍒嗗锛堣涓嬫枃锛夛紝AER 椹卞姩閬靛惊 pci-error-recovery.rst 涓畾涔夌殑
瑙勫垯銆傛湁鍏冲洖璋冪殑璇︾粏瀹氫箟锛岃鍙傞槄 pci-error-recovery.rst銆?

浠ヤ笅鍚勮妭璇存槑浜嗕綍鏃惰皟鐢ㄩ敊璇洖璋冨嚱鏁般€?

#### Correctable errors


鍙籂姝ｉ敊璇鎺ュ彛鐨勫姛鑳芥病鏈変换浣曞奖鍝嶃€侾CIe 鍗忚鍙互鍦ㄤ笉闇€瑕佷换浣曡蒋浠跺共棰勬垨浠讳綍鏁版嵁涓㈠け鐨?
鎯呭喌涓嬫仮澶嶃€傝繖浜涢敊璇笉闇€瑕佷换浣曟仮澶嶆搷浣溿€侫ER 椹卞姩鐩稿簲鍦版竻闄よ澶囩殑鍙籂姝ｉ敊璇姸鎬佸瘎瀛樺櫒锛屽苟
璁板綍杩欎簺閿欒銆?

#### Uncorrectable (non-fatal and fatal) errors


AER 椹卞姩鎵ц涓€娆?Secondary Bus Reset锛堟绾ф€荤嚎澶嶄綅锛変互浠庝笉鍙籂姝ｉ敊璇腑鎭㈠銆傚浣嶅簲鐢ㄤ簬
鍙戣捣璁惧涔嬩笂鐨勭鍙ｏ細濡傛灉鍙戣捣璁惧鏄竴涓鐐癸紙Endpoint锛夛紝鍒欏彧澶嶄綅璇ョ鐐广€傚彟涓€鏂归潰锛屽鏋滃彂璧?
璁惧鏈変粠灞炶澶囷紝閭ｄ簺璁惧涔熶細鍏ㄩ儴鍙楀埌澶嶄綅褰卞搷銆?

濡傛灉鍙戣捣璁惧鏄竴涓?Root Complex Integrated Endpoint锛堟牴澶嶅悎浣撻泦鎴愮鐐癸級锛屽垯娌℃湁鍙互搴旂敤
Secondary Bus Reset 鐨勭鍙ｄ箣涓娿€傚湪杩欑鎯呭喌涓嬶紝AER 椹卞姩鏀逛负搴旂敤 Function Level Reset锛堝姛鑳?
绾у浣嶏級銆?

濡傛灉閿欒娑堟伅鎸囩ず闈炶嚧鍛介敊璇紝鍒欎笉闇€瑕佸湪涓婃父鎵ц澶嶄綅銆侫ER 椹卞姩鍚戞煇涓眰绾х粨鏋勪腑鍏宠仈鐨勬墍鏈?
椹卞姩璋冪敤 error_detected(dev, pci_channel_io_normal)
```

  Endpoint <==> Downstream Port B <==> Upstream Port A <==> Root Port

```
濡傛灉 Upstream Port A 鎹曡幏浜嗕竴涓?AER 閿欒锛屽垯璇ュ眰绾х粨鏋勭敱 Downstream Port B 鍜?Endpoint 缁勬垚銆?

椹卞姩鍙互杩斿洖 PCI_ERS_RESULT_CAN_RECOVER銆丳CI_ERS_RESULT_DISCONNECT 鎴?
PCI_ERS_RESULT_NEED_RESET锛屽叿浣撳彇鍐充簬瀹冩槸鍚﹀彲浠ュ湪涓嶅浣嶇殑鎯呭喌涓嬫仮澶嶃€佽涓鸿澶囦笉鍙仮澶嶏紝鎴?
闇€瑕佸浣嶆墠鑳芥仮澶嶃€傚鏋滄墍鏈夊彈褰卞搷鐨勯┍鍔ㄩ兘鍚屾剰鍙互鍦ㄤ笉澶嶄綅鐨勬儏鍐典笅鎭㈠锛屽垯璺宠繃澶嶄綅銆傚彧瑕佹湁涓€
涓┍鍔ㄨ姹傚浣嶏紝灏变細瑕嗙洊鎵€鏈夊叾浠栭┍鍔ㄣ€?

濡傛灉閿欒娑堟伅鎸囩ず鑷村懡閿欒锛屽唴鏍稿皢鍚戞煇涓眰绾х粨鏋勪腑鐨勬墍鏈夐┍鍔ㄥ箍鎾?error_detected(dev,
pci_channel_io_frozen)銆傜劧鍚庯紝蹇呴』鍦ㄤ笂娓告墽琛屽浣嶃€傚鏋?error_detected 杩斿洖
PCI_ERS_RESULT_CAN_RECOVER 琛ㄧず鍙互鍦ㄤ笉澶嶄綅鐨勬儏鍐典笅鎭㈠锛岄敊璇鐞嗗皢杩涘叆 mmio_enabled锛屼絾
涔嬪悗浠嶄細鎵ц澶嶄綅銆?

鎹㈠彞璇濊锛屽浜庨潪鑷村懡閿欒锛岄┍鍔ㄥ彲浠ラ€夋嫨杩涜澶嶄綅銆備絾瀵逛簬鑷村懡閿欒锛屽熀浜庨摼璺笉鍙潬鐨勫亣璁撅紝瀹冧滑
涓嶈兘閫夋嫨涓嶈繘琛屽浣嶃€?

### Frequently Asked Questions


闂細
  濡傛灉 PCIe 璁惧椹卞姩娌℃湁鎻愪緵閿欒鎭㈠澶勭悊绋嬪簭锛坧ci_driver->err_handler 绛変簬 NULL锛夛紝浼?
  鍙戠敓浠€涔堬紵

绛旓細
  涓庤椹卞姩鍏宠仈鐨勮澶囧皢鏃犳硶琚仮澶嶃€傚唴鏍稿皢鎵撳嵃鍑轰俊鎭€ф秷鎭潵璇嗗埆涓嶅彲鎭㈠鐨勮澶囥€?


## Software error injection


璋冭瘯 PCIe AER 閿欒鎭㈠浠ｇ爜鐩稿綋鍥伴毦锛屽洜涓哄緢闅捐Е鍙戠湡瀹炵殑纭欢閿欒銆傚彲浠ヤ娇鐢ㄥ熀浜庤蒋浠剁殑閿欒娉ㄥ叆
鏉ヤ吉閫犲悇绉?PCIe 閿欒銆?

棣栧厛浣犲簲鍦ㄥ唴鏍搁厤缃腑鍚敤 PCIe AER 杞欢閿欒娉ㄥ叆锛屽嵆浣犵殑 .config 涓簲鍖呭惈浠ヤ笅椤广€?

CONFIG_PCIEAER_INJECT=y or CONFIG_PCIEAER_INJECT=m

鐢ㄦ柊鍐呮牳閲嶅惎鎴栨彃鍏ユā鍧楀悗锛屽簲鍒涘缓涓€涓悕涓?/dev/aer_inject 鐨勮澶囨枃浠躲€?

鐒跺悗锛屼綘闇€瑕佷竴涓悕涓?aer-inject 鐨勭敤鎴风┖闂村伐鍏凤紝鍙粠浠ヤ笅鍦板潃鑾峰彇锛?

    https://github.com/intel/aer-inject.git

鏈夊叧 aer-inject 鐨勬洿澶氫俊鎭彲鍦ㄥ叾婧愪唬鐮佷腑鐨勬枃妗ｆ壘鍒般€?
