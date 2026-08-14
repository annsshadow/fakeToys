## Linux Security Modules: General Security Hooks for Linux


:Author: Stephen Smalley
:Author: Timothy Fraser
:Author: Chris Vance


   鏈功涓弿杩扮殑 API 宸茬粡杩囨椂銆?
## Introduction


2001 骞?3 鏈堬紝缇庡浗鍥藉瀹夊叏灞€锛圢SA锛夊湪 2.5 Linux Kernel Summit 涓婂仛浜嗕竴涓叧浜?Security-Enhanced Linux锛圫ELinux锛夌殑婕旇銆係ELinux 鏄?Linux 鍐呮牳涓伒娲讳笖缁嗙矑搴﹂潪鑷富璁块棶鎺у埗鐨勪竴绉嶅疄鐜帮紝鏈€鍒濅綔涓哄畠鑷繁鐗瑰畾鐨勫唴鏍歌ˉ涓佸疄鐜般€傚叾浠栧嚑涓畨鍏ㄩ」鐩紙渚嬪 RSBAC銆丮edusa锛変篃涓?Linux 鍐呮牳寮€鍙戜簡鐏垫椿鐨勮闂帶鍒舵灦鏋勶紝骞朵笖鍚勭椤圭洰涓?Linux 寮€鍙戜簡鐗瑰畾鐨勮闂帶鍒舵ā鍨嬶紙渚嬪 LIDS銆丏TE銆丼ubDomain锛夈€傛瘡涓」鐩兘寮€鍙戝苟缁存姢浜嗚嚜宸辩殑鍐呮牳琛ヤ竵浠ユ敮鎸佸叾瀹夊叏闇€姹傘€?
浣滀负瀵?NSA 婕旇鐨勫洖搴旓紝Linus Torvalds 鍙戣〃浜嗕竴绯诲垪璇勮锛屾弿杩颁簡涓€涓粬鎰挎剰鑰冭檻绾冲叆涓荤嚎 Linux 鍐呮牳鐨勫畨鍏ㄦ鏋躲€備粬鎻忚堪浜嗕竴涓€氱敤妗嗘灦锛岃妗嗘灦灏嗘彁渚涗竴缁勫畨鍏ㄩ挬瀛愭潵鎺у埗瀵瑰唴鏍稿璞＄殑鎿嶄綔锛屽苟鍦ㄥ唴鏍告暟鎹粨鏋勪腑鎻愪緵涓€缁勪笉閫忔槑鐨勫畨鍏ㄥ瓧娈典互缁存姢瀹夊叏灞炴€с€傜劧鍚庯紝杩欎釜妗嗘灦鍙互琚彲鍔犺浇鍐呮牳妯″潡鐢ㄦ潵瀹炵幇浠讳綍鏈熸湜鐨勫畨鍏ㄦā鍨嬨€侺inus 杩樺缓璁皢 Linux capabilities 浠ｇ爜杩佺Щ鍒拌繖鏍风殑妯″潡涓€?
Linux Security Modules锛圠SM锛夐」鐩敱 WireX 鍙戣捣锛屾棬鍦ㄥ紑鍙戣繖鏍蜂竴涓鏋躲€侺SM 鏄涓畨鍏ㄩ」鐩紙鍖呮嫭 Immunix銆丼ELinux銆丼GI 鍜?Janus锛変互鍙婂涓釜浜猴紙鍖呮嫭 Greg Kroah-Hartman 鍜?James Morris锛夎仈鍚堝紑鍙戠殑鎴愭灉锛岀洰鐨勬槸寮€鍙戝疄鐜拌妗嗘灦鐨?Linux 鍐呮牳琛ヤ竵銆傝宸ヤ綔鍦?2003 骞?12 鏈堣绾冲叆涓荤嚎銆傛湰鎶€鏈姤鍛婃杩颁簡璇ユ鏋跺拰 capabilities 瀹夊叏妯″潡銆?
## LSM Framework


LSM 妗嗘灦鎻愪緵浜嗕竴涓€氱敤鐨勫唴鏍告鏋朵互鏀寔瀹夊叏妯″潡銆傚叿浣撹€岃█锛孡SM 妗嗘灦涓昏鍏虫敞鏀寔璁块棶鎺у埗妯″潡锛屽敖绠℃湭鏉ョ殑寮€鍙戝彲鑳戒細瑙ｅ喅鍏朵粬瀹夊叏闇€姹傦紝渚嬪娌欑銆傛鏋舵湰韬笉鎻愪緵浠讳綍棰濆鐨勫畨鍏ㄦ€э紱瀹冧粎浠呮彁渚涙敮鎸佸畨鍏ㄦā鍧楃殑鍩虹璁炬柦銆侺SM 妗嗘灦鏄彲閫夌殑锛岄渶瑕佸惎鐢?`CONFIG_SECURITY`銆俢apabilities 閫昏緫琚疄鐜颁负涓€涓畨鍏ㄦā鍧椼€?杩欎釜 capabilities 妯″潡鍦?`LSM Capabilities Module`_ 涓繘涓€姝ヨ璁恒€?
LSM 妗嗘灦鍦ㄥ唴鏍告暟鎹粨鏋勪腑鍖呭惈瀹夊叏瀛楁锛屽苟鍦ㄥ唴鏍镐唬鐮佺殑鍏抽敭鐐硅皟鐢ㄩ挬瀛愬嚱鏁帮紝浠ョ鐞嗗畨鍏ㄥ瓧娈靛苟鎵ц璁块棶鎺у埗銆?瀹冭繕娣诲姞浜嗙敤浜庢敞鍐屽畨鍏ㄦā鍧楃殑鍑芥暟銆?鎺ュ彛 `/sys/kernel/security/lsm` 鎶ュ憡绯荤粺涓婂浜庢椿鍔ㄧ姸鎬佺殑瀹夊叏妯″潡浠ラ€楀彿鍒嗛殧鐨勫垪琛ㄣ€?
LSM 瀹夊叏瀛楁鍙槸 `void*` 鎸囬拡銆?杩欎簺鏁版嵁琚О涓?blob锛屽畠鍙敱妗嗘灦绠＄悊锛屼篃鍙敱浣跨敤瀹冪殑鍚勪釜瀹夊叏妯″潡绠＄悊銆?琚涓畨鍏ㄦā鍧椾娇鐢ㄧ殑瀹夊叏 blob 閫氬父鐢辨鏋剁鐞嗐€?瀵逛簬杩涚▼鍜?绋嬪簭鎵ц鐨勫畨鍏ㄤ俊鎭紝瀹夊叏瀛楁鍖呭惈鍦?`struct task_struct <task_struct>` 鍜?`struct cred <cred>` 涓€?瀵逛簬鏂囦欢绯荤粺
鐨勫畨鍏ㄤ俊鎭紝瀹夊叏瀛楁鍖呭惈鍦?:c:type:`struct super_block <super_block>` 涓€傚浜庣閬撱€佹枃浠跺拰濂楁帴瀛楃殑瀹夊叏
淇℃伅锛屽畨鍏ㄥ瓧娈靛寘鍚湪 :c:type:`struct inode <inode>` 鍜?`struct file <file>` 涓€?瀵逛簬 System V IPC 鐨勫畨鍏ㄤ俊鎭紝
瀹夊叏瀛楁琚坊鍔犲埌 :c:type:`struct kern_ipc_perm <kern_ipc_perm>` 鍜?:c:type:`struct msg_msg <msg_msg>` 涓紱姝ゅ锛?c:type:`struct msg_msg <msg_msg>`銆乻truct msg_queue 鍜?struct shmid_kernel 鐨勫畾涔?琚Щ鍔ㄥ埌澶存枃浠朵腑锛堝垎鍒负 `include/linux/msg.h` 鍜?`include/linux/shm.h`锛夛紝浠ュ厑璁稿畨鍏ㄦā鍧椾娇鐢ㄨ繖浜涘畾涔夈€?
瀵逛簬鏁版嵁鍖呭拰
缃戠粶璁惧鐨勫畨鍏ㄤ俊鎭紝瀹夊叏瀛楁琚坊鍔犲埌 `struct sk_buff <sk_buff>` 鍜?`struct scm_cookie <scm_cookie>` 涓€?涓庡叾浠栧畨鍏ㄦā鍧楁暟鎹笉鍚岋紝杩欓噷浣跨敤鐨勬暟鎹槸涓€涓?32 浣嶆暣鏁般€傚畨鍏ㄦā鍧楅渶瑕佹妸杩欎簺鍊兼槧灏勬垨浠ュ叾浠栨柟寮忎笌鐪熷疄鐨勫畨鍏ㄥ睘鎬у叧鑱旇捣鏉ャ€?
LSM 閽╁瓙缁存姢鍦ㄥ垪琛ㄤ腑銆傛瘡涓挬瀛愮淮鎶や竴涓垪琛紝閽╁瓙鎸?CONFIG_LSM 鎸囧畾鐨勯『搴忚皟鐢ㄣ€?姣忎釜閽╁瓙鐨勮缁嗘枃妗ｅ寘鍚湪 `security/security.c` 婧愭枃浠朵腑銆?
LSM 妗嗘灦鎻愪緵浜嗗閫氱敤瀹夊叏妯″潡鍫嗗彔鐨勮繎浼兼敮鎸併€傚畠瀹氫箟浜?security_add_hooks()锛屾瘡涓畨鍏ㄦā鍧楀悜瀹冧紶閫掍竴涓?`struct security_hooks_list <security_hooks_list>`锛岃繖浜涜娣诲姞鍒板垪琛ㄤ腑銆?LSM 妗嗘灦涓嶆彁渚涚Щ闄ゅ凡娉ㄥ唽閽╁瓙鐨勬満鍒躲€係ELinux 瀹夊叏妯″潡瀹炵幇浜嗕竴绉嶇Щ闄よ嚜韬殑鏂规硶锛屼絾璇ョ壒鎬у凡琚純鐢ㄣ€?
閽╁瓙鍙互鐪嬩綔鍒嗕负涓ゅぇ绫伙細鐢ㄤ簬绠＄悊瀹夊叏瀛楁鐨勯挬瀛愬拰鐢ㄤ簬鎵ц璁块棶鎺у埗鐨勯挬瀛愩€傜涓€绫婚挬瀛愮殑渚嬪瓙鍖呮嫭 security_inode_alloc() 鍜?security_inode_free()
杩欎簺閽╁瓙鐢ㄤ簬涓?inode 瀵硅薄鍒嗛厤鍜岄噴鏀惧畨鍏ㄧ粨鏋勩€?绗簩绫婚挬瀛愮殑涓€涓緥瀛愭槸 security_inode_permission() 閽╁瓙銆?璇ラ挬瀛愬湪璁块棶 inode 鏃舵鏌ユ潈闄愩€?
## LSM Capabilities Module


POSIX.1e capabilities 閫昏緫浣滀负瀛樺偍鍦?`security/commoncap.c` 鏂囦欢涓殑瀹夊叏妯″潡缁存姢銆俢apabilities 妯″潡浣跨敤 `lsm_info` 鎻忚堪鐨?order 瀛楁灏嗗叾鏍囪瘑涓鸿娉ㄥ唽鐨勭涓€涓畨鍏ㄦā鍧椼€?涓庡叾浠栨ā鍧椾笉鍚岋紝capabilities 瀹夊叏妯″潡涓嶄娇鐢ㄩ€氱敤瀹夊叏 blob銆傚師鍥犳槸鍘嗗彶鎬х殑锛屽熀浜庡紑閿€銆佸鏉傛€у拰鎬ц兘鏂归潰鐨勮€冭檻銆?