
## Linux 鐨?Macintosh HFS 鏂囦欢绯荤粺


HFS 浠ｈ〃 `Hierarchical File System`锛堝垎灞傛枃浠剁郴缁燂級锛屾槸 Mac Plus 鍙婃墍鏈夊悗缁?Macintosh 鏈哄瀷鎵€浣跨敤鐨勬枃浠剁郴缁熴€傛洿鏃╃殑 Macintosh 鏈哄瀷浣跨敤 MFS锛坄Macintosh File
System`锛孧acintosh 鏂囦欢绯荤粺锛夛紝璇ユ牸寮忎笉鍙楁敮鎸侊紱MacOS 8.1 鍙婃洿鏂扮増鏈敮鎸佷竴绉?鍚嶄负 HFS+ 鐨勬枃浠剁郴缁燂紝瀹冧笌 HFS 绫讳技浣嗗湪澶氫釜鏂归潰杩涜浜嗘墿灞曘€傝浠?Linux 璁块棶
姝ょ被鏂囦欢绯荤粺锛岃浣跨敤 hfsplus 鏂囦欢绯荤粺椹卞姩銆?
## 鎸傝浇閫夐」


鎸傝浇 HFS 鏂囦欢绯荤粺鏃讹紝鎺ュ彈浠ヤ笅閫夐」锛?
  creator=cccc, type=cccc
	鎸囧畾鐢?MacOS finder 鏄剧ず鐨?creator/type 鍊硷紝鐢ㄤ簬鍒涘缓鏂版枃浠躲€?	榛樿鍊硷細'????'銆?
  uid=n, gid=n
  	鎸囧畾鎷ユ湁鏂囦欢绯荤粺涓墍鏈夋枃浠剁殑鐢ㄦ埛/缁勩€?	榛樿鍊硷細鎸傝浇杩涚▼鐨勭敤鎴?缁?id銆?
  dir_umask=n, file_umask=n, umask=n
	鎸囧畾鐢ㄤ簬鎵€鏈夋枃浠躲€佹墍鏈夌洰褰曟垨鎵€鏈夋枃浠朵笌鐩綍鐨?umask銆?	榛樿鍊间负鎸傝浇杩涚▼鐨?umask銆?
  session=n
  	閫夋嫨瑕佷綔涓?HFS 鏂囦欢绯荤粺鎸傝浇鐨?CDROM 浼氳瘽銆傞粯璁や氦鐢?CDROM 椹卞姩
	鏉ュ喅瀹氥€傝閫夐」鍦ㄥ簳灞傝澶囦笉鏄?CDROM 鏃朵細澶辫触銆?
  part=n
  	浠庤澶囦腑閫夋嫨绗?n 涓垎鍖恒€傝繖鍙 CDROM 鏈夋剰涔夛紝鍥犱负 CDROM 鏃犳硶鍦?	Linux 涓嬭鍒嗗尯銆傚浜庣鐩樿澶囷紝閫氱敤鐨勫垎鍖鸿В鏋愪唬鐮佷細鏇挎垜浠畬鎴愭浜嬨€?	榛樿瀹屽叏涓嶈В鏋愬垎鍖鸿〃銆?
  quiet
  	蹇界暐鏃犳晥鐨勬寕杞介€夐」锛岃€屼笉鏄姤閿欍€?

## 鍐欏叆 HFS 鏂囦欢绯荤粺


HFS 骞堕潪 UNIX 鏂囦欢绯荤粺锛屽洜姝ゅ畠涓嶅叿澶囦綘鎵€鏈熸湜鐨勫父瑙佺壒鎬э細

 - 浣犳棤娉曚慨鏀规枃浠剁殑 set-uid銆乻et-gid銆乻ticky 鎴栧彲鎵ц浣嶏紝涔熸棤娉曚慨鏀瑰叾 uid
   鍜?gid銆? - 浣犳棤娉曞垱寤虹‖閾炬帴鎴栫鍙烽摼鎺ャ€佽澶囨枃浠躲€乻ocket 鎴?FIFO銆?
涓嶈繃 HFS 鍏锋湁姣忎釜鏂囦欢澶氫釜 fork 鐨勬蹇点€傝繖浜涢潪鏍囧噯鐨?fork 鍦ㄥ父瑙勬枃浠剁郴缁熷懡鍚?绌洪棿涓琛ㄧず涓洪殣钘忕殑闄勫姞鏂囦欢锛岃繖澶氬皯鏈変簺 hack 鐨勫懗閬擄紝骞朵娇寰楀叾璇箟鏄惧緱鏈変簺
濂囨€細

 - 浣犳棤娉曞垱寤恒€佸垹闄ゆ垨閲嶅懡鍚嶆枃浠剁殑璧勬簮 fork 鎴?Finder 鐨勫厓鏁版嵁銆? - 涓嶈繃瀹冧滑浼氶殢鐩稿簲鐨勬暟鎹?fork 鎴栫洰褰曚竴璧疯鍒涘缓锛堜娇鐢ㄩ粯璁ゅ€硷級銆佸垹闄ゅ拰閲嶅懡鍚嶃€? - 灏嗘枃浠跺鍒跺埌鍙︿竴绉嶆枃浠剁郴缁熸椂浼氫涪澶遍偅浜涘 MacOS 姝ｅ父宸ヤ綔蹇呬笉鍙皯鐨勫睘鎬с€?

## 鍒涘缓 HFS 鏂囦欢绯荤粺


Robert Leslie 鐨?hfsutils 杞欢鍖呬腑鍖呭惈涓€涓悕涓?hformat 鐨勭▼搴忥紝鍙敤浜庡垱寤?HFS 鏂囦欢绯荤粺銆傝瑙?<https://www.mars.org/home/rob/proj/hfs/>銆?

## 鑷磋阿


HFS 椹卞姩鐢?Paul H. Hargrove锛坔argrove@sccm.Stanford.EDU锛夌紪鍐欍€俁oman Zippel
锛坮oman@ardistech.com锛夐噸鍐欎簡浠ｇ爜鐨勫ぇ閮ㄥ垎锛屽苟寮曞叆浜嗘簮鑷?Brad Boyer 鐨?hfsplus
椹卞姩鐨?btree 渚嬬▼銆?