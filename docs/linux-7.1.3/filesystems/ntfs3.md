
## NTFS3

## 姒傝堪涓庡姛鑳?
NTFS3 鏄竴涓姛鑳藉畬澶囩殑 NTFS 璇诲啓椹卞姩銆傝椹卞姩鏀寔鏈€楂?3.1 鐗堟湰鐨?NTFS銆傛寕杞芥椂浣跨敤鐨?鏂囦欢绯荤粺绫诲瀷鏄?**ntfs3**銆?
- 璇ラ┍鍔ㄥ疄鐜颁簡瀵规櫘閫氭枃浠躲€佺█鐤忥紙sparse锛夋枃浠跺拰鍘嬬缉鏂囦欢鐨?NTFS 璇?鍐欐敮鎸併€?- 鏀寔鍘熺敓鐨勬棩蹇楅噸鏀撅紙journal replaying锛夈€?- 鏀寔瀵瑰凡鎸傝浇 NTFS 鍗风殑 NFS 瀵煎嚭銆?- 鏀寔鎵╁睍灞炴€э紙extended attributes锛夈€傞瀹氫箟鐨勬墿灞曞睘鎬э細

 - **system.ntfs_security** 鑾峰彇/璁剧疆瀹夊叏鎻忚堪绗?
		Descriptor: SECURITY_DESCRIPTOR_RELATIVE

 - **system.ntfs_attrib** 鑾峰彇/璁剧疆 ntfs 鏂囦欢/鐩綍灞炴€с€?
	  娉ㄦ剰锛氬簲鐢ㄤ簬绌烘枃浠舵椂锛岃繖鍏佽鍦?sparse(0x200)銆乧ompressed(0x800) 鍜?	  normal 涔嬮棿鍒囨崲绫诲瀷銆?
 - **system.ntfs_attrib_be** 鑾峰彇/璁剧疆 ntfs 鏂囦欢/鐩綍灞炴€с€?
	  涓?system.ntfs_attrib 鍙栧€肩浉鍚岋紝浣嗗缁堜互澶х锛坆ig-endian锛夎〃绀?	  锛坰ystem.ntfs_attrib 鐨勫瓧鑺傚簭涓?CPU 鐩稿悓锛夈€?
## 鎸傝浇閫夐」

涓嬮潰鐨勫垪琛ㄦ弿杩颁簡 NTFS3 椹卞姩闄ら€氱敤鎸傝浇閫夐」澶栨墍鏀寔鐨勬寕杞介€夐」銆備綘鍙互灏嗘瘡涓€夐」涓?**no** 閫夐」涓€璧蜂娇鐢ㄣ€傚鏋滄煇涓€夐」鍦ㄦ湰琛ㄤ腑鏍囪浜?no锛屾剰鍛崇潃榛樿鏄笉甯?**no** 鐨勩€?
   :widths: 1 5
   :fill-cells:

   - - iocharset=name
     - 璇ラ€夐」鍛婄煡椹卞姩濡備綍瑙ｉ噴璺緞瀛楃涓诧紝骞跺皢鍏惰浆鎹负 Unicode 浠ュ強鍙嶅悜杞崲銆傚鏋滄湭
       璁剧疆璇ラ€夐」锛屽皢浣跨敤榛樿浠ｇ爜椤碉紙CONFIG_NLS_DEFAULT锛夈€?
       绀轰緥锛歩ocharset=utf8

   - - uid=
     - `1`
   - - gid=

   - - umask=
     - 鎺у埗 NTFS 鍗锋寕杞藉悗鍒涘缓鐨勬枃浠?鐩綍鐨勯粯璁ゆ潈闄愩€?
   - - dmask=
     - `1` 涓庢寚瀹氬悓鏃堕€傜敤浜庢枃浠跺拰鐩綍鐨?umask 涓嶅悓锛宖mask 鍙簲鐢ㄤ簬鏂囦欢锛岃€?dmask
       鍙簲鐢ㄤ簬鐩綍銆?   - - fmask=

   - - nohidden
     - 甯︽湁 Windows 鐗规湁鐨?HIDDEN锛團ILE_ATTRIBUTE_HIDDEN锛夊睘鎬х殑鏂囦欢灏嗕笉浼氬湪
       Linux 涓嬫樉绀恒€?
   - - sys_immutable
     - 甯︽湁 Windows 鐗规湁鐨?SYSTEM锛團ILE_ATTRIBUTE_SYSTEM锛夊睘鎬х殑鏂囦欢灏嗚鏍囪涓?       绯荤粺涓嶅彲鍙橈紙system immutable锛夋枃浠躲€?
   - - hide_dot_files
     - 鍦ㄥ垱寤恒€佺Щ鍔ㄦ垨閲嶅懡鍚嶆枃浠舵椂鏇存柊 Windows 鐗规湁鐨?HIDDEN锛團ILE_ATTRIBUTE_HIDDEN锛?       灞炴€с€備互鍙ョ偣寮€澶寸殑鏂囦欢鍚嶅皢琚缃?HIDDEN 灞炴€э紝涓嶄互鍙ョ偣寮€澶寸殑鏂囦欢鍚嶅皢琚?       娓呴櫎璇ュ睘鎬с€?
   - - windows_names
     - 闃绘鍒涘缓鍚嶇О涓嶈 Windows 鍏佽鐨勬枃浠跺拰鐩綍锛屽師鍥犲寘鎷細鍖呭惈鏌愪簺涓嶅厑璁哥殑瀛楃
       锛堝嵆瀛楃 " * / : < > ? \\ | 浠ュ強缂栫爜灏忎簬 0x20 鐨勫瓧绗︼級锛涘悕绉帮紙甯︽垨涓嶅甫鎵╁睍鍚嶏級
       鏄繚鐣欐枃浠跺悕锛圕ON銆丄UX銆丯UL銆丳RN銆丩PT1-9銆丆OM1-9锛夛紱鎴栬€呮渶鍚庝竴涓瓧绗︽槸绌烘牸
       鎴栧彞鐐广€傚凡鏈夌殑姝ょ被鏂囦欢浠嶅彲琚鍙栧拰閲嶅懡鍚嶃€?
   - - discard
     - 鍚敤瀵?TRIM 鍛戒护鐨勬敮鎸侊紝浠ユ彁鍗囧垹闄ゆ搷浣滅殑鎬ц兘锛屽缓璁笌鍥烘€佺‖鐩橈紙SSD锛変竴璧蜂娇鐢ㄣ€?
   - - force
     - 寮哄埗椹卞姩鎸傝浇鍒嗗尯锛屽嵆浣垮嵎琚爣璁颁负鑴忥紙dirty锛夈€備笉寤鸿浣跨敤銆?
   - - sparse
     - 浠ョ█鐤忔柟寮忓垱寤烘柊鏂囦欢銆?
   - - showmeta
     - 浣跨敤姝ゅ弬鏁板彲鍦ㄥ凡鎸傝浇鐨?NTFS 鍒嗗尯涓婃樉绀烘墍鏈夊厓鏂囦欢锛圫ystem Files锛夈€傞粯璁ゆ儏鍐典笅锛?       鎵€鏈夊厓鏂囦欢閮芥槸闅愯棌鐨勩€?
   - - prealloc
     - 鍦ㄥ啓鍏ユ椂鏂囦欢澶у皬澧為暱鐨勬儏鍐典笅锛岃繃搴﹀湴涓烘枃浠堕鍒嗛厤绌洪棿銆傚湪骞惰鍐欏叆涓嶅悓鏂囦欢鏃?       鍙噺灏戠鐗囧寲銆?
   - - acl
     - 鏀寔 POSIX ACL锛堣闂帶鍒跺垪琛級銆傚湪鍐呮牳鏀寔鏃剁敓鏁堛€備笉瑕佷笌 NTFS ACL 娣锋穯銆傛寚瀹?       涓?acl 鐨勯€夐」鍚敤瀵?POSIX ACL 鐨勬敮鎸併€?
## 寰呭姙鍒楄〃

- 鍩轰簬 JBD 鐨勫畬鏁存棩蹇楋紙journaling锛夋敮鎸併€傜洰鍓嶆敮鎸佹棩蹇楅噸鏀撅紝浣嗘晥鏋滄湭蹇呰兘杈惧埌 JBD 鐨?  绋嬪害銆?
## 鍙傝€冭祫鏂?
- NTFS 椹卞姩鐨?Linux 鍟嗕笟鐗堟湰銆?	https://www.paragon-software.com/home/ntfs-linux-professional/

- NTFS3 瀹炵幇鐨勫弽棣堜笌闇€姹傜殑鐩存帴鐢靛瓙閭欢鍦板潃銆?	almaz.alexandrovich@paragon-software.com
