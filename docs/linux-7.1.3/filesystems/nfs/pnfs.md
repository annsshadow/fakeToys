## pnfs 涓殑寮曠敤璁℃暟


杩欓噷鏈夊嚑涓浉浜掑叧鑱旂殑缂撳瓨銆傛垜浠湁甯冨眬锛坙ayout锛夛紝涓€涓竷灞€鍙互寮曠敤澶氫釜璁惧锛坉evice锛夛紝姣忎釜璁惧鍙堝彲浠ュ紩鐢ㄥ涓暟鎹湇鍔″櫒锛坉ata server锛夈€傛瘡涓暟鎹湇鍔″櫒鍙互琚涓澶囧紩鐢ㄣ€傛瘡涓澶囧張鍙互琚涓竷灞€寮曠敤銆備负浜嗚杩欎竴鍒囦繚鎸佹竻鏅帮紝鎴戜滑闇€瑕佽繘琛屽紩鐢ㄨ鏁般€?

## struct pnfs_layout_hdr


绾夸笂鍛戒护 LAYOUTGET 瀵瑰簲浜?struct pnfs_layout_segment锛岄€氬父浠ュ彉閲忓悕 lseg 鏉ユ寚浠ｃ€傛瘡涓?nfs_inode 鍙互鍦?nfsi->layout 涓寔鏈変竴涓寚鍚戣繖浜涘竷灞€娈电紦瀛樼殑鎸囬拡锛屽叾绫诲瀷涓?struct pnfs_layout_hdr銆?
鎴戜滑涓烘寚鍚戝畠鐨?inode 寮曠敤璇ュご閮紝璺ㄨ秺姣忎釜寮曠敤瀹冪殑鏈畬鎴?RPC 璋冪敤锛圠AYOUTGET銆丩AYOUTRETURN銆丩AYOUTCOMMIT锛夛紝骞朵负鍏朵腑鍖呭惈鐨勬瘡涓?lseg 寮曠敤瀹冦€?
姣忎釜澶撮儴锛堝綋闈炵┖鏃讹級杩樹細琚斁鍏ヤ竴涓笌 struct nfs_client锛坈l_layouts锛夊叧鑱旂殑閾捐〃涓€傛斁鍏ヨ閾捐〃骞朵笉浼氬鍔犲紩鐢ㄨ鏁帮紝鍥犱负甯冨眬鐢变娇鍏剁暀鍦ㄩ摼琛ㄤ腑鐨?lseg 缁存寔鐫€銆?
## deviceid_cache


lseg 寮曠敤 device id锛岃繖浜?id 鎸?nfs_client 鍜屽竷灞€椹卞姩绫诲瀷鏉ヨВ鏋愩€俤evice id 淇濆瓨鍦ㄤ竴涓?RCU 缂撳瓨锛坰truct nfs4_deviceid_cache锛変腑銆傝缂撳瓨鏈韩鍦ㄦ瘡涓寕杞斤紙mount锛夋湡闂磋寮曠敤銆傝繖浜涙潯鐩紙struct nfs4_deviceid锛夋湰韬湪姣忎釜寮曠敤瀹冧滑鐨?lseg 鐨勭敓鍛藉懆鏈熷唴琚寔鏈夈€?
浣跨敤 RCU 鏄洜涓?deviceid 鍩烘湰涓婃槸涓€涓竴娆″啓鍏ャ€佸娆¤鍙栵紙write once, read many锛夌殑鏁版嵁缁撴瀯銆?2 涓《锛坆ucket锛夌殑 hlist 澶у皬闇€瑕佹洿濂界殑鐞嗙敱锛屼絾閴翠簬姣忎釜鏂囦欢绯荤粺鍙互鏈夊涓?deviceid锛岃€屾瘡涓?nfs_client 鍙堝彲浠ユ湁澶氫釜鏂囦欢绯荤粺锛岃繖浼间箮鏄悎鐞嗙殑銆?
鍝堝笇浠ｇ爜鏄粠 nfsd 浠ｇ爜搴撳鍒惰繃鏉ョ殑銆傚叧浜庡搱甯屽強璇ョ畻娉曞悇绉嶅彉浣撶殑璁ㄨ鍙互鍦?`杩欓噷銆?<http://groups.google.com/group/comp.lang.c/browse_thread/thread/9522965e2b8d3809>`_ 鎵惧埌

## 鏁版嵁鏈嶅姟鍣ㄧ紦瀛?

鏂囦欢椹卞姩锛坒ile driver锛夎澶囧紩鐢ㄦ暟鎹湇鍔″櫒锛岃繖浜涙暟鎹湇鍔″櫒淇濆瓨鍦ㄤ竴涓ā鍧楃骇缂撳瓨涓€傚叾寮曠敤鍦ㄦ寚鍚戝畠鐨?deviceid 鐨勭敓鍛藉懆鏈熷唴琚寔鏈夈€?
## lseg


lseg 缁存姢涓€涓笌 NFS_LSEG_VALID 浣嶅搴旂殑棰濆寮曠敤锛岃浣嶄娇鍏剁暀鍦?pnfs_layout_hdr 鐨勯摼琛ㄤ腑銆傚綋鏈€鍚庝竴涓?lseg 浠?pnfs_layout_hdr 鐨勯摼琛ㄤ腑绉婚櫎鏃讹紝浼氳缃?NFS_LAYOUT_DESTROYED 浣嶏紝闃绘鍐嶅姞鍏ヤ换浣曟柊鐨?lseg銆?
## 甯冨眬椹卞姩锛坙ayout drivers锛?

PNFS 浣跨敤浜嗘墍璋撶殑甯冨眬椹卞姩銆係TD 瀹氫箟浜?4 绉嶅熀鏈竷灞€绫诲瀷锛?files"銆?objects"銆?blocks" 鍜?"flexfiles"銆傚浜庢瘡绉嶇被鍨嬶紝閮芥湁涓€涓竷灞€椹卞姩锛屽甫鏈変竴涓敱 nfs-client 鐨?pnfs-core 璋冪敤鐨勯€氱敤鍑芥暟鍚戦噺琛紝鐢ㄤ簬瀹炵幇涓嶅悓鐨勫竷灞€绫诲瀷銆?
Files 甯冨眬椹卞姩浠ｇ爜浣嶄簬锛歠s/nfs/filelayout/.. 鐩綍
Blocks 甯冨眬椹卞姩浠ｇ爜浣嶄簬锛歠s/nfs/blocklayout/.. 鐩綍
Flexfiles 甯冨眬椹卞姩浠ｇ爜浣嶄簬锛歠s/nfs/flexfilelayout/.. 鐩綍

## blocks 甯冨眬璁剧疆


TODO锛氳褰?blocks 甯冨眬椹卞姩鐨勮缃渶姹?