## 閰嶉瀛愮郴缁?

閰嶉瀛愮郴缁熷厑璁哥郴缁熺鐞嗗憳涓虹敤鎴峰拰/鎴栫粍璁剧疆宸茬敤绌洪棿涓庡凡鐢?inode 鏁伴噺锛坕node
鏄笌姣忎釜鏂囦欢鎴栫洰褰曠浉鍏宠仈鐨勬枃浠剁郴缁熺粨鏋勶級鐨勯檺鍒躲€傚浜庡凡鐢ㄧ┖闂村拰宸茬敤 inode
鏁伴噺锛屽疄闄呬笂鍚勬湁涓や釜闄愬埗銆傜涓€涓О涓鸿蒋闄愬埗锛坰oftlimit锛夛紝绗簩涓О涓虹‖闄愬埗
锛坔ardlimit锛夈€傜敤鎴锋案杩滀笉鑳借秴杩囦换浣曡祫婧愮殑纭檺鍒讹紙闄ら潪鍏舵嫢鏈?CAP_SYS_RESOURCE
鑳藉姏锛夈€傚厑璁哥敤鎴峰湪鏈夐檺鏃堕棿鍐呰秴杩囪蒋闄愬埗銆傝鏈熼檺绉颁负鈥滃闄愭湡鈥濓紙grace
period锛夋垨鈥滃闄愭椂闂粹€濓紙grace time锛夈€傚闄愭椂闂寸粨鏉熷悗锛岀敤鎴峰皢鏃犳硶鍒嗛厤鏇村
绌洪棿/inode锛岀洿鍒伴噴鏀捐冻澶熷鐨勮祫婧愪娇鍏朵綆浜庤蒋闄愬埗銆?
閰嶉闄愬埗锛堜互鍙婂闄愭椂闂寸殑闀跨煭锛夐拡瀵规瘡涓枃浠剁郴缁熻繘琛岀嫭绔嬭缃€?
鏈夊叧閰嶉璁捐鐨勬洿澶氱粏鑺傦紝璇峰弬闃?quota-tools 杞欢鍖呬腑鐨勬枃妗?(https://sourceforge.net/projects/linuxquota)銆?
## 閰嶉 netlink 鎺ュ彛

褰撶敤鎴疯秴杩囪蒋闄愬埗銆佸闄愭椂闂磋€楀敖鎴栬揪鍒扮‖闄愬埗鏃讹紝閰嶉瀛愮郴缁熶紶缁熶笂浼氬悜瀵艰嚧瓒呴檺鐨?杩涚▼鎵€鍦ㄧ殑鎺у埗缁堢鎵撳嵃涓€鏉℃秷鎭€傝繖绉嶆柟娉曠殑缂虹偣鏄細褰撶敤鎴蜂娇鐢ㄥ浘褰㈡闈㈡椂锛岄€氬父
鏃犳硶鐪嬪埌璇ユ秷鎭€傚洜姝よ璁′簡閰嶉 netlink 鎺ュ彛锛岀敤浜庡皢涓婅堪浜嬩欢鐨勪俊鎭紶閫掔粰鐢ㄦ埛鎬併€?鍦ㄧ敤鎴锋€侊紝杩欎簺淇℃伅鍙敱搴旂敤绋嬪簭鎹曡幏骞剁浉搴斿鐞嗐€?
璇ユ帴鍙ｄ娇鐢ㄩ€氱敤 netlink 妗嗘灦锛堟湁鍏宠灞傜殑鏇村缁嗚妭锛岃鍙傝
https://lwn.net/Articles/208755/ 鍜?http://www.infradead.org/~tgr/libnl/锛夈€?閰嶉閫氱敤 netlink 鎺ュ彛鐨勫悕绉颁负 "VFS_DQUOT"銆備笅鏂瑰父閲忕殑瀹氫箟浣嶄簬 <linux/quota.h>銆?鐢变簬閰嶉 netlink 鍗忚涓嶆劅鐭ュ懡鍚嶇┖闂达紝閰嶉 netlink 娑堟伅浠呭湪鍒濆缃戠粶鍛藉悕绌洪棿涓彂閫併€?
鐩墠锛岃鎺ュ彛浠呮敮鎸佷竴绉嶆秷鎭被鍨?QUOTA_NL_C_WARNING銆傝鍛戒护鐢ㄤ簬鍙戦€佸叧浜庝笂杩颁换涓€
浜嬩欢鐨勯€氱煡銆傛瘡鏉℃秷鎭湁鍏釜灞炴€с€傝繖浜涘睘鎬у涓嬶紙鍙傛暟绫诲瀷鍦ㄦ嫭鍙峰唴锛夛細

        QUOTA_NL_A_QTYPE (u32)
   - 琚秴杩囩殑閰嶉绫诲瀷锛圲SRQUOTA銆丟RPQUOTA 涔嬩竴锛?        QUOTA_NL_A_EXCESS_ID (u64)
   - 瓒呰繃闄愬埗鐨勭敤鎴风殑 UID/GID锛堝彇鍐充簬閰嶉绫诲瀷锛?        QUOTA_NL_A_CAUSED_ID (u64)
   - 瀵艰嚧璇ヤ簨浠剁殑鐢ㄦ埛鐨?UID
        QUOTA_NL_A_WARNING (u32)
   - 鍝闄愬埗琚秴杩囷細

		QUOTA_NL_IHARDWARN
		    inode 纭檺鍒?		QUOTA_NL_ISOFTLONGWARN
		    inode 杞檺鍒惰秴杩囩粰瀹氬闄?		    鍛ㄦ湡鐨勬椂闂存洿闀?		QUOTA_NL_ISOFTWARN
		    inode 杞檺鍒?		QUOTA_NL_BHARDWARN
		    绌洪棿锛堝潡锛夌‖闄愬埗
		QUOTA_NL_BSOFTLONGWARN
		    绌洪棿锛堝潡锛夎蒋闄愬埗瓒呰繃
		    缁欏畾瀹介檺鍛ㄦ湡鐨勬椂闂存洿闀裤€?		QUOTA_NL_BSOFTWARN
		    绌洪棿锛堝潡锛夎蒋闄愬埗

   - 褰撶敤鎴峰仠姝㈣秴杩囨煇涓€闄愬埗鏃讹紝涔熶负璇ヤ簨浠跺畾涔変簡鍥涗釜璀﹀憡锛?
		QUOTA_NL_IHARDBELOW
		    inode 纭檺鍒?		QUOTA_NL_ISOFTBELOW
		    inode 杞檺鍒?		QUOTA_NL_BHARDBELOW
		    绌洪棿锛堝潡锛夌‖闄愬埗
		QUOTA_NL_BSOFTBELOW
		    绌洪棿锛堝潡锛夎蒋闄愬埗

        QUOTA_NL_A_DEV_MAJOR (u32)
   - 鍙楀奖鍝嶆枃浠剁郴缁熸墍鍦ㄨ澶囩殑涓昏澶囧彿
        QUOTA_NL_A_DEV_MINOR (u32)
   - 鍙楀奖鍝嶆枃浠剁郴缁熸墍鍦ㄨ澶囩殑娆¤澶囧彿
