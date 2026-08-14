# Linux Kernel 瀛︿範璺緞

> 浠?3934 绡囧唴鏍告枃妗ｄ腑绮鹃€夊嚭鐨勯槄璇昏矾绾垮浘銆?> 鎸夋璺緞闃呰锛屽彲鍦ㄥ悎鐞嗘椂闂村唴寤虹珛瀵瑰唴鏍告暣浣撴灦鏋勭殑蹇冩櫤妯″瀷銆?
---

## 濡備綍浣跨敤鏈寚鍗?
- **蹇呰** 鏍囪浜嗗缓绔嬪熀纭€璁ょ煡涓嶅彲鎴栫己鐨勬枃妗?- **閫夎** 鍙寜鍏磋叮鍜岄渶姹傞€夋嫨闃呰
- 姣忛樁娈靛缓璁寜椤哄簭闃呰锛屼絾闃舵涔嬮棿鍙互璺宠繃涓嶉渶瑕佺殑棰嗗煙
- 閾炬帴鎸囧悜 `docs/` 涓嬬殑 Markdown 鐗堟湰锛堢敱 `tools/docs/rst-to-md.py` 杞崲鐢熸垚锛?
---

## 闃舵 1锛歄rientation锛堝繀璇伙紝绾?2 灏忔椂锛?
鐩爣锛氫簡瑙ｅ唴鏍告槸浠€涔堛€佹枃妗ｅ湪鍝噷銆佺ぞ鍖哄浣曡繍浣溿€?
| # | 鏂囨。 | 璇存槑 |
|---|------|------|
| 1 | `README` | 鎸夎鑹诧紙寮€鍙戣€?鐮旂┒鑰?瀹夊叏涓撳/绯荤粺绠＄悊鍛橈級缁欎綘鐨勫叆鍙?|
| 2 | `Documentation/index.md` | 鏂囨。鍦板浘锛屼簡瑙ｆ暣涓枃妗ｆ爲鐨勭粍缁囩粨鏋?|
| 3 | `Documentation/admin-guide/README.md` | "鍐呮牳鏄粈涔?鈥斺€旂‖浠舵敮鎸併€佹瀯寤哄畨瑁呫€佸揩閫熸瑙?|
| 4 | `Documentation/process/howto.md` | 濡備綍鎴愪负鍐呮牳寮€鍙戣€呪€斺€斿伐鍏枫€侀偖浠跺垪琛ㄣ€佺ぞ鍖鸿鑼?|
| 5 | `Documentation/process/development-process.md` | 鍐呮牳绀惧尯濡備綍杩愪綔鈥斺€斿彂甯冨懆鏈熴€佸悎骞剁獥鍙ｃ€佽ˉ涓佺敓鍛藉懆鏈?|

---

## 闃舵 2锛氭灦鏋勫熀纭€锛堝繀璇伙紝绾?4-6 灏忔椂锛?
鐩爣锛氬缓绔嬪鍐呮牳鎵ц妯″瀷鐨勬牳蹇冪洿瑙夈€傝繖鏄渶閲嶈鐨勯樁娈点€?
| # | 鏂囨。 | 璇存槑 |
|---|------|------|
| 6 | `Documentation/kernel-hacking/hacking.md` | 猸?**鏈€閲嶈鐨勫崟绡囨枃妗?*锛欳PU 鎵ц涓婁笅鏂囷紙鐢ㄦ埛/杞腑鏂?纭腑鏂?绌洪棽锛夈€乣current` 鎸囬拡銆佽皟搴︾偣銆佸熀纭€閿?|
| 7 | `Documentation/core-api/index.md` | 鍐呮牳 API 鍒嗙被鍥捐氨鈥斺€斿揩閫熸壂涓€閬嶏紝鐭ラ亾鏈変粈涔堟帴鍙ｅ彲鐢?|
| 8 | `Documentation/mm/index.md` | 鍐呭瓨绠＄悊锛氫粠鐗╃悊鍐呭瓨鍒伴〉琛ㄣ€乻lab銆乿malloc |
| 9 | `Documentation/scheduler/index.md` | 杩涚▼璋冨害锛欳FS 璁捐 + 褰撳墠 EEVDF 璋冨害鍣?|
| 10 | `Documentation/locking/index.md` | 鍚屾鍘熻锛氳嚜鏃嬮攣銆佷簰鏂ラ攣銆丷CU 閿佺殑鍒嗙被涓庝娇鐢ㄥ満鏅?|
| 11 | `Documentation/RCU/index.md` | RCU 鏈哄埗锛氳-澶嶅埗-鏇存柊鐨勬牳蹇冩€濇兂涓庡疄鐜?|
| 12 | `Documentation/core-api/irq/index.md` | 涓柇澶勭悊锛欼RQ 鍩熴€?affinity銆?managed IRQ |

**闃呰寤鸿锛?*
- 鍏堣 `hacking.md`锛堢 6 绡囷級锛屽畠浼氬湪 30 鍒嗛挓鍐呯粰浣?鍐呮牳鏄€庝箞璺戣捣鏉?鐨勬暣浣撴劅瑙?- 鐒跺悗璇?`core-api/index.md`锛堢 7 绡囷級寤虹珛 API 鍦板浘
- 鍏朵綑 5 绡囨寜浣犵殑鍏磋叮椤哄簭闃呰

---

## 闃舵 3锛氬瓙绯荤粺閫夎锛堥€夎锛屾寜闇€娣卞叆锛?
鐩爣锛氭寜鍏磋叮娣卞叆浜嗚В鍏蜂綋瀛愮郴缁熴€?
| # | 鏂囨。 | 璇存槑 |
|---|------|------|
| 13 | `Documentation/filesystems/index.md` | 铏氭嫙鏂囦欢绯荤粺锛圴FS锛夛細superblock/inode/dentry 妯″瀷銆佽矾寰勬煡鎵俱€佹寕杞藉懡鍚嶇┖闂?|
| 14 | `Documentation/networking/index.md` | 缃戠粶鏍堬細`sk_buff` 鐢熷懡鍛ㄦ湡銆丯API銆乣netdevice` 妯″瀷銆佸崗璁爤 |
| 15 | `Documentation/driver-api/index.md` | 椹卞姩妯″瀷锛歬object/device/driver/bus 灞傛缁撴瀯 |
| 16 | `Documentation/power/index.md` | 鐢垫簮绠＄悊锛氳繍琛屾椂 PM銆佺郴缁熸寕璧?鎭㈠ |
| 17 | `Documentation/security/index.md` | 瀹夊叏鏋舵瀯锛歀SM 妗嗘灦銆佸唴鏍歌嚜闃插尽鏈哄埗銆佸嚟璇佺鐞?|
| 18 | `Documentation/trace/index.md` | 鍙娴嬫€э細tracepoint銆乫trace銆乲probes |

**闃呰寤鸿锛?*
- 姣忎釜瀛愮郴缁熼€氬父鍙渶瑕佽 `index.md` + 1-2 绡囨牳蹇冭璁℃枃妗?- 涓嶉渶瑕佹寜椤哄簭璇伙紝鏍规嵁浣犵殑鍏磋叮鎴栧伐浣滈渶姹傞€夋嫨

---

## 闃舵 4锛氬紑鍙戝疄璺碉紙蹇呰锛岀害 2 灏忔椂锛?
鐩爣锛氫簡瑙ｅ浣曞悜鍐呮牳鎻愪氦浠ｇ爜銆侀伒瀹堢殑瑙勮寖銆佸彲鐢ㄧ殑宸ュ叿銆?
| # | 鏂囨。 | 璇存槑 |
|---|------|------|
| 19 | `Documentation/process/coding-style.rst` | 鍐呮牳缂栫爜瑙勮寖鈥斺€旂缉杩涖€佸懡鍚嶃€佹敞閲娿€佺┖鏍间笌鍒惰〃绗?|
| 20 | `Documentation/process/submitting-patches.rst` | 琛ヤ竵鎻愪氦娴佺▼锛歡it 鏍煎紡銆乣git send-email`銆乣Signed-off-by`銆乧hangelog |
| 21 | `Documentation/kbuild/index.md` | 鏋勫缓绯荤粺锛氶《灞?Makefile銆並config 璇硶銆佹ā鍧楁瀯寤?|
| 22 | `Documentation/dev-tools/index.md` | 寮€鍙戝伐鍏凤細checkpatch銆並Unit 鍗曞厓娴嬭瘯銆佽皟璇曞伐鍏?|

---

## 闃舵 5锛氭墿灞曡祫婧愶紙閫夎锛?
鐩爣锛氭壘鍒版洿娣卞叆鐨勫閮ㄥ涔犳潗鏂欍€?
| # | 鏂囨。 | 璇存槑 |
|---|------|------|
| 23 | `Documentation/process/kernel-docs.md` | 澶栭儴涔︾睄鍜岃鏂囨帹鑽愶紙Linux Device Drivers銆丩WN 鏂囩珷绛夛級 |
| 24 | `Documentation/admin-guide/index.md` | 绯荤粺绠＄悊鍛樻帴鍙ｏ細`/proc/sys` 鍙皟鍙傛暟銆佸惎鍔ㄥ弬鏁?|
| 25 | `Documentation/userspace-api/index.md` | 鐢ㄦ埛绌洪棿 API锛氱郴缁熻皟鐢ㄣ€佸畨鍏ㄦ帴鍙ｃ€乣/dev` 璁惧 |

---

## 蹇€熷弬鑰冿細鎸夌洰鏍囬€夋嫨

| 浣犵殑鐩爣 | 寤鸿璺緞 |
|----------|----------|
| 蹇€熶簡瑙ｅ唴鏍稿叏璨?| 闃舵 1 + 闃舵 2锛堢 6 绡囧繀璇伙級 |
| 鍐欑涓€涓唴鏍告ā鍧?| 闃舵 1 鈫?闃舵 2 鈫?闃舵 4 鈫?`Documentation/driver-api/index.md` |
| 鐞嗚В鍐呭瓨绠＄悊 | 闃舵 1 鈫?闃舵 2 鈫?`Documentation/mm/index.md` 鈫?`Documentation/vm/index.md` |
| 鐞嗚В鏂囦欢绯荤粺 | 闃舵 1 鈫?闃舵 2 鈫?`Documentation/filesystems/index.md` |
| 鐞嗚В缃戠粶鏍?| 闃舵 1 鈫?闃舵 2 鈫?`Documentation/networking/index.md` |
| 鍑嗗鎻愪氦琛ヤ竵 | 闃舵 1 鈫?闃舵 4锛堢 19-20 绡囷級 |

---

## 澶囨敞

- 鎵€鏈?Markdown 鏂囦欢鐢?`tools/docs/rst-to-md.py` 浠?Sphinx `.rst` 婧愭枃浠惰嚜鍔ㄨ浆鎹㈢敓鎴?- 濡傛灉鍙戠幇杞崲璐ㄩ噺闂锛屽彲浠ユ煡鐪嬪師濮?`.rst` 鏂囦欢鎴栦慨澶嶈浆鎹㈣剼鏈?- 鏂囨。鍐呭闅忓唴鏍哥増鏈洿鏂帮紝鏈寚鍗楀熀浜?v7.1.3锛?Baby Opossum Posse"锛?