
## 鍐呭瓨鐑彃鎷?

## 鍐呭瓨鐑彃鎷斾簨浠堕€氱煡鍣?

鐑彃鎷斾簨浠朵細琚彂閫佸埌涓€涓€氱煡闃熷垪銆?
### 鍐呭瓨閫氱煡鍣?

鍦?`include/linux/memory.h` 涓畾涔変簡鍏绫诲瀷鐨勯€氱煡锛?
MEM_GOING_ONLINE
  鍦ㄦ柊鍐呭瓨鍙樺緱鍙敤涔嬪墠浜х敓锛屼互渚胯兘澶熻鍚勫瓙绯荤粺鍋氬ソ鍑嗗鏉ュ鐞嗗唴瀛樸€傛鏃堕〉鍒嗛厤鍣ㄤ粛鏃犳硶浠庢柊鍐呭瓨涓垎閰嶃€?
MEM_CANCEL_ONLINE
  褰?MEM_GOING_ONLINE 澶辫触鏃朵骇鐢熴€?
MEM_ONLINE
  褰撳唴瀛樻垚鍔熶笂绾挎椂浜х敓銆傚洖璋冨彲浠ヤ粠鏂板唴瀛樹腑鍒嗛厤椤点€?
MEM_GOING_OFFLINE
  鍦ㄥ紑濮嬪唴瀛樹笅绾胯繃绋嬫椂浜х敓銆傛鏃跺凡鏃犳硶鍐嶄粠璇ュ唴瀛樹腑鍒嗛厤锛屼絾閮ㄥ垎灏嗚涓嬬嚎鐨勫唴瀛樹粛鍦ㄤ娇鐢ㄤ腑銆傝鍥炶皟鍙敤浜庝粠鎸囧畾鐨勫唴瀛樺潡涓噴鏀炬煇涓瓙绯荤粺宸茬煡鐨勫唴瀛樸€?
MEM_CANCEL_OFFLINE
  褰?MEM_GOING_OFFLINE 澶辫触鏃朵骇鐢熴€傛垜浠皾璇曚笅绾跨殑鍐呭瓨鍧楅噸鏂板彲鐢ㄣ€?
MEM_OFFLINE
  鍦ㄤ笅绾垮唴瀛樺畬鎴愬悗浜х敓銆?
```

  hotplug_memory_notifier(callback_func, priority)

```
priority 鍊艰緝澶х殑鍥炶皟鍑芥暟浼氬湪 priority 鍊艰緝灏忕殑鍥炶皟鍑芥暟涔嬪墠琚皟鐢ㄣ€?
```

  int callback_func(
    struct notifier_block *self, unsigned long action, void *arg);

```
鍥炶皟鍑芥暟鐨勭涓€涓弬鏁帮紙self锛夋槸鎸囧悜閫氱煡閾句腑鏌愪釜鍧楃殑鎸囬拡锛岃鍧楁寚鍚戝洖璋冨嚱
鏁拌嚜韬€傜浜屼釜鍙傛暟锛坅ction锛夋槸涓婅堪浜嬩欢绫诲瀷涔嬩竴銆?
```
	struct memory_notify {
		unsigned long start_pfn;
		unsigned long nr_pages;
	}

```
- start_pfn 涓轰笂绾?涓嬬嚎鍐呭瓨鐨勮捣濮?pfn銆?- nr_pages 涓轰笂绾?涓嬬嚎鍐呭瓨鐨勯〉鏁般€?
鏈夊彲鑳藉湪鏈敹鍒?MEM_GOING_ONLINE 閫氱煡鐨勬儏鍐典笅灏辨敹鍒?MEM_CANCEL_ONLINE 閫氱煡锛?MEM_CANCEL_OFFLINE 涓?MEM_GOING_OFFLINE 涔熷悓鏍峰姝ゃ€傝繖浼氬湪鏌愪釜娑堣垂鑰呭け璐?鏃跺彂鐢燂紝鎰忓懗鐫€鎴戜滑涓柇浜嗚皟鐢ㄩ摼骞跺仠姝㈣皟鐢ㄩ€氱煡鍣ㄧ殑鍏朵綑娑堣垂鑰呫€傚洜姝わ紝
memory_notify 鐨勪娇鐢ㄨ€呬笉搴斿仛浠讳綍鍋囪锛屽苟搴斿噯澶囧ソ澶勭悊姝ょ被鎯呭喌銆?
鍥炶皟渚嬬▼搴旇繑鍥?`include/linux/notifier.h` 涓畾涔夌殑浠ヤ笅鍊间箣涓€锛?NOTIFY_DONE銆丯OTIFY_OK銆丯OTIFY_BAD銆丯OTIFY_STOP

NOTIFY_DONE 涓?NOTIFY_OK 瀵瑰悗缁鐞嗘病鏈夊奖鍝嶃€?
NOTIFY_BAD 鐢ㄤ綔瀵?MEM_GOING_ONLINE銆丮EM_GOING_OFFLINE銆丮EM_ONLINE 鎴?MEM_OFFLINE 鍔ㄤ綔鐨勫搷搴旓紝鐢ㄤ簬鍙栨秷鐑彃鎷斻€傚畠浼氬仠姝㈤€氱煡闃熷垪鐨勫悗缁鐞嗐€?
NOTIFY_STOP 鍋滄閫氱煡闃熷垪鐨勫悗缁鐞嗐€?
### NUMA 鑺傜偣閫氱煡鍣?

鍦?`include/linux/node.h` 涓畾涔変簡鍏绫诲瀷鐨勯€氱煡锛?
NODE_ADDING_FIRST_MEMORY
 鍦ㄨ鑺傜偣棣栨鏈夊唴瀛樺彲鐢ㄤ箣鍓嶄骇鐢熴€?
NODE_CANCEL_ADDING_FIRST_MEMORY
 褰?NODE_ADDING_FIRST_MEMORY 澶辫触鏃朵骇鐢熴€?
NODE_ADDED_FIRST_MEMORY
 褰撹鑺傜偣棣栨鏈夊唴瀛樺彲鐢ㄦ椂浜х敓銆?
NODE_REMOVING_LAST_MEMORY
 褰撹鑺傜偣鏈€鍚庡彲鐢ㄧ殑鍐呭瓨鍗冲皢琚笅绾挎椂浜х敓銆?
NODE_CANCEL_REMOVING_LAST_MEMORY
 褰?NODE_CANCEL_REMOVING_LAST_MEMORY 澶辫触鏃朵骇鐢熴€?
NODE_REMOVED_LAST_MEMORY
 褰撹鑺傜偣鏈€鍚庡彲鐢ㄧ殑鍐呭瓨宸茶涓嬬嚎鏃朵骇鐢熴€?
```

  hotplug_node_notifier(callback_func, priority)

```
priority 鍊艰緝澶х殑鍥炶皟鍑芥暟浼氬湪 priority 鍊艰緝灏忕殑鍥炶皟鍑芥暟涔嬪墠琚皟鐢ㄣ€?
```

  int callback_func(

    struct notifier_block *self, unsigned long action, void *arg);

```
鍥炶皟鍑芥暟鐨勭涓€涓弬鏁帮紙self锛夋槸鎸囧悜閫氱煡閾句腑鏌愪釜鍧楃殑鎸囬拡锛岃鍧楁寚鍚戝洖璋冨嚱
鏁拌嚜韬€傜浜屼釜鍙傛暟锛坅ction锛夋槸涓婅堪浜嬩欢绫诲瀷涔嬩竴銆?
```
        struct node_notify {
                int nid;
        }

```
- nid 涓烘垜浠娣诲姞鎴栫Щ闄ゅ唴瀛樼殑鑺傜偣銆?
鏈夊彲鑳藉湪鏈敹鍒?NODE_ADDING_FIRST_MEMORY 閫氱煡鐨勬儏鍐典笅灏辨敹鍒?NODE_CANCEL_ADDING_FIRST_MEMORY 閫氱煡锛孨ODE_CANCEL_REMOVING_LAST_MEMORY 涓?NODE_REMOVING_LAST_MEMORY 涔熷悓鏍峰姝ゃ€傝繖浼氬湪鏌愪釜娑堣垂鑰呭け璐ユ椂鍙戦€侊紝鎰忓懗鐫€鎴戜滑
涓柇浜嗚皟鐢ㄩ摼骞跺仠姝㈣皟鐢ㄩ€氱煡鍣ㄧ殑鍏朵綑娑堣垂鑰呫€傚洜姝わ紝node_notify 鐨勪娇鐢ㄨ€呬笉搴?鍋氫换浣曞亣璁撅紝骞跺簲鍑嗗濂藉鐞嗘绫绘儏鍐点€?
鍥炶皟渚嬬▼搴旇繑鍥?`include/linux/notifier.h` 涓畾涔夌殑浠ヤ笅鍊间箣涓€锛?NOTIFY_DONE銆丯OTIFY_OK銆丯OTIFY_BAD銆丯OTIFY_STOP

NOTIFY_DONE 涓?NOTIFY_OK 瀵瑰悗缁鐞嗘病鏈夊奖鍝嶃€?
NOTIFY_BAD 鐢ㄤ綔瀵?NODE_ADDING_FIRST_MEMORY銆丯ODE_REMOVING_LAST_MEMORY銆?NODE_ADDED_FIRST_MEMORY 鎴?NODE_REMOVED_LAST_MEMORY 鍔ㄤ綔鐨勫搷搴旓紝鐢ㄤ簬鍙栨秷鐑彃鎷斻€?瀹冧細鍋滄閫氱煡闃熷垪鐨勫悗缁鐞嗐€?
NOTIFY_STOP 鍋滄閫氱煡闃熷垪鐨勫悗缁鐞嗐€?
璇锋敞鎰忥紝瀵逛簬 NODE_ADDED_FIRST_MEMORY / NODE_REMOVED_FIRST_MEMORY 鎴戜滑涓嶅簲澶辫触锛?鍥犱负姝ゆ椂 memory_hotplug 浠ｇ爜宸叉棤娉曞洖婊氥€?
## 閿佺殑鍐呴儴鏈哄埗


褰撴坊鍔?绉婚櫎浣跨敤鍐呭瓨鍧楄澶囷紙鍗虫櫘閫?RAM锛夌殑鍐呭瓨鏃讹紝搴旀寔鏈?device_hotplug_lock锛屼互锛?
- 涓庝笂绾?涓嬬嚎璇锋眰锛堜緥濡傞€氳繃 sysfs锛変繚鎸佸悓姝ャ€傝繖鏍凤紝鍐呭瓨鍧楄澶囧彧鏈夊湪鍐呭瓨琚?  瀹屽叏娣诲姞鍚庯紝鎵嶈兘琚敤鎴风┖闂磋闂紙.online/.state 灞炴€э級銆傝€屽湪绉婚櫎鍐呭瓨鏃讹紝
  鎴戜滑鐭ラ亾娌℃湁浜哄湪鍏抽敭鍖烘涓€?- 涓?CPU 鐑彃鎷斿強绫讳技鎿嶄綔淇濇寔鍚屾锛堜緥濡備笌 ACPI 鍜?PPC 鐩稿叧锛夈€?
鐗瑰埆鍦帮紝鍦ㄦ坊鍔犲唴瀛樿€岀敤鎴风┖闂磋瘯鍥炬瘮棰勬湡鏇村揩鍦板皢璇ュ唴瀛樹笂绾挎椂锛屽瓨鍦ㄤ竴绉嶅彲鑳界殑
閿佸弽杞紝浣跨敤 device_hotplug_lock 鍙伩鍏嶈闂锛?
- device_online() 浼氬厛鑾峰彇 device_lock()锛岄殢鍚庤幏鍙?mem_hotplug_lock
- add_memory_resource() 浼氬厛鑾峰彇 mem_hotplug_lock锛岄殢鍚庤幏鍙?device_lock()
  锛堝湪鍒涘缓璁惧鏈熼棿锛屼簬 bus_add_device() 涓級銆?
鐢变簬璇ヨ澶囧湪瀵圭敤鎴风┖闂村彲瑙佷箣鍚庢墠浼氳幏鍙?device_lock()锛屽洜姝ゅ彲鑳藉鑷撮攣鍙嶈浆銆?
鍐呭瓨鐨勪笂绾?涓嬬嚎搴旈€氳繃 device_online()/device_offline() 瀹屾垚 鈥斺€?浠ョ‘淇濆叾涓?缁?sysfs 鍙戣捣鐨勬搷浣滄纭悓姝ャ€傚缓璁寔鏈?device_hotplug_lock锛堜緥濡備互淇濇姢
online_type锛夈€?
褰撴坊鍔?绉婚櫎/涓婄嚎/涓嬬嚎鍐呭瓨锛屾垨娣诲姞/绉婚櫎寮傛瀯/璁惧鍐呭瓨鏃讹紝鎴戜滑搴斿缁堜互鍐欐ā寮?鎸佹湁 mem_hotplug_lock锛屼互涓茶鍖栧唴瀛樼儹鎻掓嫈锛堜緥濡傚鍏ㄥ眬/zone 鍙橀噺鐨勮闂級銆?
姝ゅ锛宮em_hotplug_lock锛堜笌 device_hotplug_lock 涓嶅悓锛夊湪璇绘ā寮忎笅鍏佽涓€涓浉褰?楂樻晥鐨?get_online_mems/put_online_mems 瀹炵幇锛屽洜姝よ闂唴瀛樼殑浠ｇ爜鍙€熸闃叉
璇ュ唴瀛樻秷澶便€?