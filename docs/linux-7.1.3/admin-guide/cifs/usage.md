## 鐢ㄦ硶

鏈ā鍧楁敮鎸?SMB3 绯诲垪楂樼骇缃戠粶鍗忚锛堜互鍙婅緝鏃х殑鏂硅█锛屾渶鍒濈О涓?"CIFS" 鎴?SMB1锛夈€?
Linux 鐨?CIFS VFS 妯″潡鏀寔璁稿楂樼骇缃戠粶鏂囦欢绯荤粺鐗规€э紝渚嬪绫讳技鍒嗗眰 DFS 鐨勫懡鍚嶇┖闂淬€佺‖閾炬帴銆侀攣瀹氱瓑銆傚畠琚璁′负绗﹀悎 SNIA CIFS 鎶€鏈弬鑰冿紙鍙栦唬 1992 骞寸殑 X/Open SMB 鏍囧噯锛夛紝骞朵笌 Windows 2000銆乄indows XP銆丼amba 鍙婄瓑鏁堟湇鍔″櫒瀹炵幇鏈€浣冲疄璺电殑瀹為檯浜掓搷浣溿€傛浠ｇ爜鏄湪鍗忚鑷敱淇℃伅鍩洪噾浼氾紙Protocol Freedom Information Foundation锛夌殑鍙備笌涓嬪紑鍙戠殑銆侰IFS 浠ュ強鐜板湪鐨?SMB3 宸叉垚涓?Mac 涓?Windows 浠ュ強涓昏 NAS 璁惧涔嬮棿浜掓搷浣滅殑鏃㈠畾鏍囧噯銆?
鏇村璇︽儏璇峰弬瑙?MS-SMB2锛圫MB2/SMB3/SMB3.1.1 鍗忚瑙勮寖璇︽儏锛?鎴?https://samba.org/samba/PFIF/ 銆?
濡傛湁闂鎴栭敊璇姤鍛婅鑱旂郴锛?
    smfrench@gmail.com

椤圭洰椤甸潰瑙侊細https://wiki.samba.org/index.php/LinuxCIFS_utils

## 鏋勫缓璇存槑

瀵逛簬 Linux锛?
1) 涓嬭浇鍐呮牳锛堜緥濡備粠 https://www.kernel.org锛?   骞跺垏鎹㈠埌鍐呮牳鐩綍鏍戠殑椤跺眰鐩綍
   锛堜緥濡?/usr/src/linux-2.5.73锛?2) make menuconfig锛堟垨 make xconfig锛?3) 鍦ㄧ綉缁滄枃浠剁郴缁熼€夐」涓€夋嫨 cifs
4) 淇濆瓨骞堕€€鍑?5) make

## 瀹夎璇存槑

濡傛灉浣犲凡灏?CIFS vfs 鏋勫缓涓烘ā鍧楋紙鎴愬姛锛夛紝鍙渶閿叆 `make modules_install`锛堟垨鑰咃紝鎵嬪姩灏嗘枃浠跺鍒跺埌妯″潡鐩綍锛屼緥濡?/lib/modules/6.3.0-060300-generic/kernel/fs/smb/client/cifs.ko锛夈€?
濡傛灉浣犲凡灏?CIFS vfs 鏋勫缓杩涘唴鏍告湰韬紝璇锋寜鐓т綘鐨勫彂琛岀増鍏充簬濡備綍瀹夎鏂板唴鏍哥殑璇存槑鎿嶄綔锛堥€氬父鍙渶閿叆 `make install`锛夈€?
濡傛灉浣犳病鏈?mount.cifs 宸ュ叿锛堜綅浜?Samba 4.x 婧愮爜鏍戝強 CIFS VFS 缃戠珯锛夛紝璇峰皢鍏跺鍒跺埌鎸傝浇杈呭姪绋嬪簭鎵€鍦ㄧ殑鍚屼竴鐩綍锛堥€氬父鏄?/sbin锛夈€傝櫧鐒惰緟鍔╄蒋浠跺苟闈炲繀闇€锛屼絾鎺ㄨ崘浣跨敤 mount.cifs銆傚ぇ澶氭暟鍙戣鐗堝寘鍚?`cifs-utils` 杞欢鍖咃紝鍏朵腑鍚湁姝ゅ伐鍏凤紝鍥犳寤鸿瀹夎銆?
娉ㄦ剰锛屽湪浣犵殑鎵€鏈?Linux 瀹㈡埛绔笂杩愯 Winbind pam/nss 妯″潡锛堢櫥褰曟湇鍔★級鏈夊姪浜庡湪鍩熶腑涓€鑷村湴灏?Uid 鍜?Gid 鏄犲皠鍒版纭殑缃戠粶鐢ㄦ埛銆俶ount.cifs 鎸傝浇杈呭姪绋嬪簭鍙湪 git.samba.org 鐨?cifs-utils.git 涓壘鍒般€?
濡傛灉 cifs 琚瀯寤轰负妯″潡锛屽垯缃戠粶缂撳啿鍖虹殑澶у皬鍜屾暟閲忎互鍙婂鍗曞彴鏈嶅姟鍣ㄧ殑鏈€澶у苟鍙戣姹傛暟閮藉彲浠ラ厤缃€?
```
	modinfo <path to cifs.ko>

```
鍦?kernel/fs/smb/client/cifs.ko 涓婏紝鍙互鐪嬪埌鍙湪妯″潡鍒濆鍖栨椂锛堥€氳繃杩愯 insmod cifs.ko锛夎繘琛岀殑閰嶇疆鍙樻洿鍒楄〃銆?
## 寤鸿

涓烘彁楂樺畨鍏ㄦ€э紝SMB2.1 鏂硅█鎴栨洿楂樼増鏈紙閫氬父灏嗕娇鐢?SMB3.1.1锛夌幇鍦ㄦ槸鏂扮殑榛樿鍊笺€傝浣跨敤鏃ф柟瑷€锛堜緥濡傛寕杞?Windows XP锛夛紝璇峰湪鎸傝浇鏃朵娇鐢?"vers=1.0"锛堟垨 vers=2.0 瀵瑰簲 Windows Vista锛夈€傛敞鎰?CIFS锛坴ers=1.0锛夋瘮榛樿鏂硅█ SMB3 鏇存棫涓斿畨鍏ㄦ€ф洿浣庯紝SMB3 鍖呭惈璁稿楂樼骇瀹夊叏鐗规€э紝渚嬪闄嶇骇鏀诲嚮妫€娴嬨€佸姞瀵嗗叡浜互鍙婃洿寮虹殑绛惧悕鍜岃璇佺畻娉曘€?
杩樻湁涓€浜涢澶栫殑鎸傝浇閫夐」鍙兘鏈夊姪浜?SMB3 鑾峰緱鏀硅繘鐨?POSIX 琛屼负锛堟敞鎰忥細鍙互浣跨敤 vers=3 寮哄埗浣跨敤 SMB3 鎴栨洿楂樼増鏈紝缁濅笉瑕佺敤 2.1锛夛細

   `mfsymlinks` 浠ュ強 `cifsacl` 鎴?`modefromsid`锛堥€氬父涓?`idsfromsid` 涓€璧蜂娇鐢級

## 鍏佽鐢ㄦ埛鎸傝浇

鍏佽鐢ㄦ埛鍦ㄤ粬浠嫢鏈夌殑鐩綍涓婅繘琛屾寕杞藉拰鍗歌浇锛屼娇鐢?cifs vfs 鏄彲浠ュ疄鐜扮殑銆傚惎鐢ㄦ绫绘寕杞界殑涓€绉嶆柟娉曟槸灏?mount.cifs 宸ュ叿鏍囪涓?suid锛堜緥濡?`chmod +s /sbin/mount.cifs`锛夈€傝鍏佽鐢ㄦ埛鍗歌浇浠栦滑鎵€鎸傝浇鐨勫叡浜紝闇€瑕侊細

1) mount.cifs 鐗堟湰 1.4 鎴栨洿楂?2) /etc/fstab 涓瓨鍦ㄦ寚绀烘煇鐢ㄦ埛鍙寕杞借鍏变韩鐨勬潯鐩?
```
     //server/usersharename  /mnt/username cifs user 0 0

```
娉ㄦ剰锛屽綋 mount.cifs 宸ュ叿浠?suid 鏂瑰紡杩愯锛堝厑璁告櫘閫氱敤鎴锋寕杞斤級鏃讹紝涓洪檷浣庨闄╋紝鎸傝浇鏃朵細浼犲叆 `nosuid` 鎸傝浇鏍囧織锛屼互绂佹鎵ц鎸傝浇鍦ㄨ繙绋嬬洰鏍囦笂鐨?suid 绋嬪簭銆傚綋浠?root 韬唤鎵ц鎸傝浇鏃讹紝榛樿涓嶄細浼犲叆 nosuid锛岄粯璁ゅ皢鍚敤杩滅▼鐩爣涓?suid 绋嬪簭鐨勬墽琛屻€傝繖鍙互鏀瑰彉锛屼笌 nfs 鍙婂叾浠栨枃浠剁郴缁熶竴鏍凤紝鍙渶鍦ㄦ寕杞介€夐」涓寚瀹?`nosuid` 鍗冲彲銆傜劧鑰屽浜庣敤鎴锋寕杞斤紝瑕佽兘澶熷悜 mount 浼犻€?suid 鏍囧織锛岄渶瑕佷娇鐢ㄤ互涓嬫爣蹇楅噸寤?mount.cifs锛欳IFS_ALLOW_USR_SUID

鍦?Samba 3.0 鍙婃洿楂樼増鏈殑婧愮爜鏍戜腑鐨?docs/manpages/mount.cifs.8 鏈夊搴旂殑 cifs 鎸傝浇鎵嬪唽椤点€?
## 鍏佽鐢ㄦ埛鍗歌浇

鍏佽鏅€氱敤鎴峰嵏杞戒粬浠互鐢ㄦ埛韬唤鎸傝浇鐨勭洰褰曪紙瑙佷笂鏂囷級锛屽彲浠ヤ娇鐢?umount.cifs 宸ュ叿銆傚畠鍙互鐩存帴璋冪敤锛屾垨鑰呭鏋?umount.cifs 琚斁鍦?/sbin 涓紝umount 鍙互璋冪敤 cifs 鍗歌浇杈呭姪绋嬪簭锛堝浜庡ぇ澶氭暟鐗堟湰鐨?umount 宸ュ叿鑰岃█锛夋潵鍗歌浇 cifs 鎸傝浇锛岄櫎闈?umount 浣跨敤 -i 璋冪敤锛堣繖灏嗛伩鍏嶈皟鐢ㄥ嵏杞借緟鍔╃▼搴忥級銆備笌 mount.cifs 涓€鏍凤紝瑕佸惎鐢ㄧ敤鎴峰嵏杞斤紝umount.cifs 蹇呴』琚爣璁颁负 suid锛堜緥濡?`chmod +s /sbin/umount.cifs`锛夋垨绛夋晥鏂瑰紡锛堟煇浜涘彂琛岀増鍏佽鍚?/etc/permissions 鏂囦欢娣诲姞鏉＄洰浠ュ疄鐜扮瓑鏁堢殑 suid 鏁堟灉锛夈€傝浣胯宸ュ叿鎴愬姛锛岀洰鏍囪矾寰勫繀椤绘槸 cifs 鎸傝浇锛屼笖褰撳墠鐢ㄦ埛鐨?uid 蹇呴』涓庢寕杞借璧勬簮鐨勭敤鎴风殑 uid 鍖归厤銆?
杩橀渶娉ㄦ剰锛屽厑璁告櫘閫氱敤鎴锋寕杞藉拰鍗歌浇鐨勫父瑙勬柟寮忥紙鑰屼笉鏄皢 mount.cifs 鍜?umount.cifs 鐢ㄤ綔 suid锛夋槸锛屼负浣犲笇鏈涙寕杞界殑姣忎釜 //server/share 鍚?/etc/fstab 鏂囦欢娣诲姞涓€琛岋紝浣嗗綋娼滃湪鎸傝浇鐩爣鍖呭惈璁稿鎴栦笉鍙娴嬬殑 UNC 鍚嶇О鏃讹紝杩欎細鍙樺緱闅句互绠＄悊銆?
## Samba 娉ㄦ剰浜嬮」

澶у鏁板綋鍓嶆湇鍔″櫒鏀寔鏇村畨鍏?SMB2.1 鍜?SMB3锛屼絾瀵逛簬杈冩棫涓斿畨鍏ㄦ€ц緝浣庣殑 CIFS 鏂硅█锛屾湁涓€浜涙湁鐢ㄧ殑鍗忚鎵╁睍锛屽洜姝よ嫢瑕佷娇鐢ㄦ棫鏂硅█锛圕IFS/SMB1锛夋寕杞戒互鑾峰緱鏈€澶ф敹鐩婏紝鎴戜滑寤鸿浣跨敤鏀寔 SNIA CIFS Unix 鎵╁睍鏍囧噯鐨勬湇鍔″櫒锛堜緥濡傚嚑涔庝换浣曠増鏈殑 Samba锛屽嵆 2.2.5 鎴栨洿楂樼増鏈級锛屼絾 CIFS vfs 鍙笌鍚勭鍚勬牱鐨?CIFS 鏈嶅姟鍣ㄨ壇濂藉崗浣溿€傛敞鎰忥紝濡傛灉浣犳病鏈夋敮鎸?CIFS Unix 鎵╁睍鐨勬湇鍔″櫒锛堝 Samba 2.2.5 鎴栨洿楂樼増鏈級锛寀id銆乬id 鍜屾枃浠舵潈闄愬皢鏄剧ず榛樿鍊笺€傝鍦?Samba 鏈嶅姟鍣ㄤ笂鍚敤 Unix CIFS 鎵╁睍锛岃鍦ㄦ湇鍔″櫒鐨?smb.conf 鏂囦欢涓坊鍔?
```
	unix extensions = yes

```
娉ㄦ剰锛屽綋澶у鏁板鎴风鏄?Unix 鎴栦互涓嬭缃椂涔熷緢鏈夌敤锛堝湪 Samba 鏈嶅姟鍣ㄤ笂锛?
```
	case sensitive = yes
	delete readonly = yes
	ea support = yes

```
娉ㄦ剰锛屾湇鍔″櫒 ea 鏀寔鏄敮鎸佹潵鑷?Linux cifs 瀹㈡埛绔殑 xattrs 鎵€蹇呴渶鐨勶紝涓?EA 鏀寔瀛樺湪浜庢洿楂樼増鏈殑 Samba 涓紙渚嬪 3.0.6 鍙婃洿楂樼増鏈紝EA 鏀寔鍦ㄦ墍鏈?Windows 鐗堟湰涓篃鏈夋晥锛岃嚦灏戝 NTFS 鏂囦欢绯荤粺涓婄殑鍏变韩鏈夋晥锛夈€傛墿灞曞睘鎬э紙xattr锛夋敮鎸佹槸澶у鏁?Linux 鏂囦欢绯荤粺鐨勫彲閫夌壒鎬э紝鍙兘闇€瑕侀€氳繃 make menuconfig 鍚敤銆傚鎴风瀵规墿灞曞睘鎬х殑鏀寔锛坲ser xattr锛夊彲浠ラ€氳繃鍦ㄦ寕杞芥椂鎸囧畾 `nouser_xattr` 鎸夋寕杞界鐢ㄣ€?
CIFS 瀹㈡埛绔彲浠ヨ幏鍙栧苟璁剧疆 POSIX ACL锛坓etfacl銆乻etfacl锛夊埌 Samba 鏈嶅姟鍣ㄧ増鏈?3.10 鍙婃洿楂樼増鏈€傝缃?POSIX ACL 闇€瑕佸湪鏋勫缓 cifs 妯″潡鏃跺湪 CIFS 閰嶇疆閫夐」涓悓鏃跺惎鐢?XATTR 鍜?POSIX 鏀寔銆侾OSIX ACL 鏀寔鍙互閫氳繃鍦ㄦ寕杞芥椂鎸囧畾 `noacl` 鎸夋寕杞界鐢ㄣ€?
涓€浜涚鐞嗗憳鍙兘鎯宠灏?Samba 鐨?smb.conf `map archive` 鍜?`create mask` 鍙傛暟浠庨粯璁ゅ€兼洿鏀广€傞櫎闈炴洿鏀?create mask锛屽惁鍒欐柊鍒涘缓鐨勬枃浠舵渶缁堝彲鑳藉叿鏈変笉蹇呰鍦颁弗鏍肩殑榛樿妯″紡锛岃繖鍙兘涓嶆槸浣犳兂瑕佺殑锛屽敖绠″鏋滃湪鏈嶅姟鍣ㄥ拰瀹㈡埛绔笂鍚敤浜?CIFS Unix 鎵╁睍锛屽悗缁殑 setattr 璋冪敤锛堜緥濡?chmod锛夊彲浠ヤ慨澶嶈妯″紡銆傛敞鎰忥紝鍒涘缓鐗规畩璁惧锛坢knod锛夎繙绋嬪湴鍙兘闇€瑕佸悜 Samba 鎸囧畾涓€涓?mkdev 鍑芥暟锛屽鏋滀綘娌℃湁浣跨敤 Samba 3.0.6 鎴栨洿楂樼増鏈€傛湁鍏宠繖浜涚殑鏇村淇℃伅璇峰弬瑙?Samba 鏈嶅姟鍣ㄧ郴缁熶笂鐨勬墜鍐岄〉锛坄man smb.conf`锛夈€傛敞鎰忥紝cifs vfs 涓?smbfs vfs 涓嶅悓锛屽畠涓嶈鍙栧鎴风绯荤粺涓婄殑 smb.conf锛堝皯鏁板彲閫夎缃€氳繃 -o 鍙傛暟鍦ㄦ寕杞芥椂浼犲叆锛夈€傛敞鎰忥紝Samba 2.2.7 鎴栨洿楂樼増鏈寘鍚竴椤逛慨澶嶏紝鍏佽 CIFS VFS 鍒犻櫎鎵撳紑鐨勬枃浠讹紙涓ユ牸鐨?POSIX 鍚堣鎵€蹇呴渶锛夈€俉indows 鏈嶅姟鍣ㄥ凡缁忔敮鎸佹鐗规€с€係amba 鏈嶅姟鍣ㄤ笉鍏佽鎸囧悜鍏变韩涔嬪鏂囦欢鐨勭鍙烽摼鎺ワ紝鍥犳鍦?3.0.6 涔嬪墠鐨?Samba 鐗堟湰涓紝澶у鏁版寚鍚?
```
	 ln -s /mnt/foo bar

```
鐨勭鍙烽摼鎺ュ皢琚姝€係amba 3.0.6 鎴栨洿楂樼増鏈殑鏈嶅姟鍣ㄥ寘鍚€氳繃灏嗚涓嶅畨鍏ㄧ殑绗﹀彿閾炬帴锛堝嵆鎸囧悜鏈嶅姟鍣ㄤ笂鍏变韩涔嬪鏂囦欢鐨勭鍙烽摼鎺ワ級杞崲涓烘湇鍔″櫒涓婄殑鐗瑰畾 samba 鏍煎紡鏉ュ畨鍏ㄥ湴鍒涘缓姝ょ被绗﹀彿閾炬帴鐨勮兘鍔涳紝璇ユ牸寮忚鏈湴鏈嶅姟鍣ㄥ簲鐢ㄧ▼搴忓拰闈?cifs 瀹㈡埛绔拷鐣ワ紝涓斾笉浼氳 Samba 鏈嶅姟鍣ㄩ亶鍘嗐€傝繖瀵逛娇鐢?cifs vfs 鐨?Linux 瀹㈡埛绔簲鐢ㄧ▼搴忔槸閫忔槑鐨勩€傜粷瀵圭鍙烽摼鎺ュ湪 Samba 3.0.5 鎴栨洿楂樼増鏈笂鍙敤锛屼絾浠呴€傜敤浜庝娇鐢?CIFS Unix 鎵╁睍鐨勮繙绋嬪鎴风锛屽苟涓斿 Windows 瀹㈡埛绔笉鍙锛岄€氬父涔熶笉浼氬奖鍝嶄笌 Samba 杩愯鍦ㄥ悓涓€鏈嶅姟鍣ㄤ笂鐨勬湰鍦板簲鐢ㄧ▼搴忋€?
## 浣跨敤璇存槑

涓€鏃?CIFS VFS 鏀寔琚瀯寤鸿繘鍐呮牳鎴栦綔涓烘ā鍧楋紙cifs.ko锛夊畨瑁咃紝浣犲彲浠ヤ娇鐢ㄧ被浼间互涓嬬殑鎸傝浇璇硶鏉ヨ闂?Samba 鎴?
```
  mount -t cifs //9.53.216.11/e$ /mnt -o username=myname,password=mypassword

```
鍦?-o 涔嬪墠鍙互鎸囧畾 -v 閫夐」锛屼互浣?mount.cifs 鎸傝浇杈呭姪绋嬪簭鏇磋缁嗗湴鏄剧ず鎸傝浇姝ラ銆?鍦?-o 涔嬪悗锛屼互涓嬪父鐢ㄧ殑 cifs vfs 鐗瑰畾閫夐」

```
  username=<username>
  password=<password>
  domain=<domain name>

```
涓嬮潰鎻忚堪浜嗗叾浠?cifs 鎸傝浇閫夐」銆傚鏋滃畨瑁呬簡鎸傝浇杈呭姪绋嬪簭锛坢ount.cifs锛夛紝鍒欏彲浠ヤ娇鐢?TCP 鍚嶇О锛堥櫎浜?ip 鍦板潃锛夈€傚鏋滀綘涓嶄俊浠绘墍鎸傝浇鍒扮殑鏈嶅姟鍣紝鎴栬€呬綘娌℃湁鍚敤 cifs 绛惧悕锛堜笖鐗╃悊缃戠粶涓嶅畨鍏級锛岃鑰冭檻浣跨敤鏍囧噯鎸傝浇閫夐」 `noexec` 鍜?`nosuid` 鏉ラ檷浣庡湪鏈湴绯荤粺涓婅繍琛岃绡℃敼鐨勪簩杩涘埗鏂囦欢锛堜粠鎭舵剰鏈嶅姟鍣ㄤ笅杞芥垨琚伓鎰忚矾鐢卞櫒绡℃敼锛夌殑椋庨櫓銆?
灏界浣跨敤瀵瑰簲 CIFS URL 瑙勮寖鐨勬牸寮忚繘琛屾寕杞藉湪 mount.cifs 涓繕涓嶅彲鑳斤紝浣嗗彲浠ヤ娇鐢ㄦ湇鍔″櫒鍜屽叡浜悕鐨勬浛浠ｆ牸寮忥紙鏈夌偣绫讳技 NFS 椋庢牸鎸傝浇锛?
```
  mount -t cifs tcp_name_of_server:share_name /mnt -o user=myname,pass=mypasswd

```
褰撲娇鐢ㄦ寕杞借緟鍔╃▼搴?mount.cifs 鏃讹紝瀵嗙爜鍙互閫氳繃鏇夸唬鏈哄埗鎸囧畾锛岃€屼笉鏄湪鍛戒护琛屼笂 -o 涔嬪悗浣跨敤姝ｅ父鐨?`pass=` 璇硶鎸囧畾锛?1) 閫氳繃灏嗗叾鍖呭惈鍦ㄥ嚟璇佹枃浠朵腑銆傛寚瀹?credentials=filename 涓轰竴涓?
```
	username=someuser
	password=your_password

```
2) 閫氳繃鍦?PASSWD 鐜鍙橀噺涓寚瀹氬瘑鐮侊紙绫讳技鍦帮紝鐢ㄦ埛鍚嶅彲浠ヤ粠 USER 鐜鍙橀噺鑾峰彇锛夈€?3) 閫氳繃 PASSWD_FILE 鎸夊悕绉板湪鏂囦欢涓寚瀹氬瘑鐮?4) 閫氳繃 PASSWD_FD 鎸夋枃浠舵弿杩扮鍦ㄦ枃浠朵腑鎸囧畾瀵嗙爜

濡傛灉鏈彁渚涘瘑鐮侊紝mount.cifs 灏嗘彁绀鸿緭鍏ュ瘑鐮?
## 闄愬埗

鏈嶅姟鍣ㄥ繀椤绘敮鎸?"pure-TCP"锛堢鍙?445 鐨?TCP/IP CIFS 杩炴帴锛夋垨鐢ㄤ簬 "Netbios-Over-TCP/IP" 鐨?RFC 1001/1002 鏀寔銆傝繖閫氬父涓嶅お鍙兘鎴愪负闂锛屽洜涓哄ぇ澶氭暟鏈嶅姟鍣ㄩ兘鏀寔銆?
鏈夋晥鐨勬枃浠跺悕鍦?Windows 鍜?Linux 涔嬮棿鏈夋墍涓嶅悓銆俉indows 閫氬父闄愬埗鍖呭惈鏌愪簺淇濈暀瀛楃锛堜緥濡傚瓧绗?:锛學indows 鐢ㄥ畠鏉ュ垎闅旀祦鍚嶇殑寮€濮嬶級鐨勬枃浠跺悕锛岃€?Linux 鍏佽绋嶅鐨勫悎娉曞瓧绗﹂泦銆俉indows 鏈嶅姟鍣ㄥ彲浠ュ湪鏈嶅姟鍣ㄧ殑娉ㄥ唽琛ㄤ腑鎸囧畾鏄惧紡鏄犲皠鏃堕噸鏄犲皠姝ょ被瀛楃銆備粠鐗堟湰 3.10 寮€濮嬬殑 Samba 灏嗗厑璁告绫绘枃浠跺悕锛堝嵆鍖呭惈鍚堟硶 Linux 瀛楃銆侀€氬父瀵?Windows/CIFS 璇箟琚姝㈢殑鏂囦欢鍚嶏級锛屽彧瑕佹湇鍔″櫒閰嶇疆涓?Unix 鎵╁睍锛堜笖瀹㈡埛绔湭绂佺敤 /proc/fs/cifs/LinuxExtensionsEnabled锛夈€傛澶栵紝鎸傝浇閫夐」 `mapposix` 鍙敤浜?CIFS锛坴ers=1.0锛変互寮哄埗灏嗛潪娉曠殑 Windows/NTFS/SMB 瀛楃鏄犲皠鍒伴噸鏄犲皠鑼冨洿锛堟鎸傝浇鍙傛暟鏄?SMB3 鐨勯粯璁ゅ€硷級銆傛閲嶆槧灏勶紙`mapposix`锛夎寖鍥翠篃涓?Mac锛堜互鍙婃煇浜涜緝鏃?Windows 涓婄殑 "Services for Mac"锛夊吋瀹广€傚綋鍗忓晢 SMB 3.1.1 鐨?POSIX 鎵╁睍鏃讹紝閲嶆槧灏勪細鑷姩绂佺敤銆?
## CIFS VFS 鎸傝浇閫夐」

浠ヤ笅鏄彈鏀寔鎸傝浇閫夐」鐨勯儴鍒嗗垪琛細

  username
		灏濊瘯寤虹珛 CIFS 浼氳瘽鏃朵娇鐢ㄧ殑鐢ㄦ埛鍚嶃€?  password
		鐢ㄦ埛瀵嗙爜銆傚鏋滃畨瑁呬簡鎸傝浇杈呭姪绋嬪簭锛岃嫢鏈彁渚涳紝灏嗘彁绀虹敤鎴疯緭鍏ュ瘑鐮併€?  ip
		鐩爣鏈嶅姟鍣ㄧ殑 ip 鍦板潃
  unc
		瑕佹寕杞界殑鐩爣鏈嶅姟鍣ㄩ€氱敤缃戠粶鍚嶇О锛堝鍑猴級銆?  domain
		璁剧疆鍦ㄥ缓绔?CIFS 浼氳瘽鏃堕檮鍔犲埌鐢ㄦ埛鍚嶄箣鍓嶇殑 SMB/CIFS 宸ヤ綔缁勫悕绉?  forceuid
		灏?inode 鐨勯粯璁?uid 璁剧疆涓烘寕杞芥椂浼犲叆鐨?uid銆傚浜庢敮鎸?CIFS Unix 鎵╁睍鐨勬湇鍔″櫒锛堜緥濡傛纭厤缃殑 Samba 鏈嶅姟鍣級锛屾湇鍔″櫒鎻愪緵 uid銆乬id 鍜?mode锛屽洜姝ら櫎闈炴湇鍔″櫒涓庡鎴风鐨?uid 鍜?gid 缂栧彿涓嶅悓锛屽惁鍒欎笉搴旀寚瀹氭鍙傛暟銆傚鏋滄湇鍔″櫒鍜屽鎴风鍦ㄥ悓涓€鍩熶腑锛堜緥濡傝繍琛?winbind 鎴?nss_ldap锛変笖鏈嶅姟鍣ㄦ敮鎸?Unix 鎵╁睍锛屽垯鍙互浠庢湇鍔″櫒妫€绱?uid 鍜?gid锛堝苟涓斾笉蹇呭湪鎸傝浇鏃舵寚瀹?uid 鍜?gid锛夈€傚浜庝笉鏀寔 CIFS Unix 鎵╁睍鐨勬湇鍔″櫒锛屾煡鎵剧幇鏈夋枃浠舵椂杩斿洖鐨勯粯璁?uid锛堝拰 gid锛夊皢鏄墽琛屾寕杞界殑浜虹殑 uid锛坓id锛夛紙root锛岄櫎闈?mount.cifs 涓虹敤鎴锋寕杞介厤缃负 setuid锛夛紝闄ら潪鎸囧畾浜?`uid=`锛坓id锛夋寕杞介€夐」銆傚彟璇锋敞鎰忥紝瀵规枃浠惰闂殑鏉冮檺妫€鏌ワ紙鎺堟潈妫€鏌ワ級鍙戠敓鍦ㄦ湇鍔″櫒涓婏紝浣嗗湪鏌愪簺鎯呭喌涓嬶紝绠＄悊鍛樺彲鑳戒篃鎯冲湪瀹㈡埛绔姞浠ラ檺鍒躲€傚浜庨偅浜涗笉鎶ュ憡 uid/gid 鎵€鏈夎€呯殑鏈嶅姟鍣紙渚嬪 Windows锛夛紝涔熷彲浠ュ湪瀹㈡埛绔鏌ユ潈闄愶紝骞朵笖鍙互閫氳繃鍦ㄥ鎴风鎸囧畾 file_mode 鍜?dir_mode 鏉ュ惎鐢ㄤ竴绉嶇矖鐣ョ殑瀹㈡埛绔晶鏉冮檺妫€鏌ャ€傦紙榛樿锛?  forcegid
		锛堢被浼间簬涓婇潰锛屼絾鏄拡瀵圭粍 id 鑰屼笉鏄?uid锛夛紙榛樿锛?  noforceuid
		濡傛灉鍙兘锛岄€氳繃鍚戞湇鍔″櫒璇锋眰鏉ュ～鍐欐枃浠舵墍鏈夎€呬俊鎭紙uid锛夈€備娇鐢ㄦ閫夐」鏃讹紝鎸傝浇鏃?`uid=` 閫夐」涓粰鍑虹殑鍊间粎鍦ㄦ湇鍔″櫒鏃犳硶鏀寔杩斿洖 inode 涓婄殑 uid 鏃朵娇鐢ㄣ€?  noforcegid
		锛堢被浼间簬涓婇潰锛屼絾鏄拡瀵圭粍鎵€鏈夎€?gid 鑰屼笉鏄?uid锛?  uid
		璁剧疆 inode 鐨勯粯璁?uid锛屽苟鎸囩ず cifs 鍐呮牳椹卞姩鏄摢涓湰鍦扮敤鎴锋寕杞界殑銆傚鏋滄湇鍔″櫒鏀寔 unix 鎵╁睍锛岄粯璁ょ殑 uid 涓嶇敤浜庡～鍐?inode锛堟枃浠讹級鐨勬墍鏈夎€呭瓧娈碉紝闄ら潪鎸囧畾浜?`forceuid` 鍙傛暟銆?  gid
		璁剧疆 inode 鐨勯粯璁?gid锛堢被浼间簬涓婇潰锛夈€?  file_mode
		濡傛灉鏈嶅姟鍣ㄤ笉鏀寔 CIFS Unix 鎵╁睍锛岃繖灏嗚鐩栨枃浠?inode 鐨勯粯璁ゆā寮忋€?  fsc
		浣跨敤 FS-Cache 鍚敤鏈湴纾佺洏缂撳瓨锛堥粯璁ゅ叧闂級銆傛閫夐」鍙兘鏈夊姪浜庡湪鎱㈤€熼摼璺€佽礋杞藉緢閲嶇殑鏈嶅姟鍣ㄥ拰/鎴栫綉缁滀腑鎻愰珮鎬ц兘锛屽叾涓粠纾佺洏璇诲彇姣斾粠鏈嶅姟鍣紙閫氳繃缃戠粶锛夎鍙栨洿蹇€傜敱浜庡鏈嶅姟鍣ㄧ殑璋冪敤娆℃暟鍑忓皯锛岃繖涔熷彲鑳藉鍙墿灞曟€т骇鐢熺Н鏋佸奖鍝嶃€備絾鏄紝鏈湴缂撳瓨骞朵笉閫傚悎鎵€鏈夊伐浣滆礋杞斤紝渚嬪鍙涓€娆＄被鍨嬬殑宸ヤ綔璐熻浇銆傚洜姝わ紝鍦ㄤ娇鐢ㄦ閫夐」涔嬪墠锛屼綘闇€瑕佷粩缁嗚€冭檻浣犵殑宸ヤ綔璐熻浇/鍦烘櫙銆傜洰鍓嶏紝鏈湴纾佺洏缂撳瓨瀵逛互鍙鏂瑰紡鎵撳紑鐨?CIFS 鏂囦欢鏄湁鏁堢殑銆?  dir_mode
		濡傛灉鏈嶅姟鍣ㄤ笉鏀寔 CIFS Unix 鎵╁睍锛岃繖灏嗚鐩栫洰褰?inode 鐨勯粯璁ゆā寮忋€?  port
		鍦ㄥ皾璇曢€氬父鐨勭鍙ｏ紙绔彛 445锛岀劧鍚?139锛変箣鍓嶏紝灏濊瘯鍦ㄦ tcp 绔彛涓婅仈绯绘湇鍔″櫒銆?  iocharset
		鐢ㄤ簬灏嗘湰鍦拌矾寰勫悕涓?Unicode 鐩镐簰杞崲鐨勪唬鐮侀〉銆傚鏋滄湇鍔″櫒鏀寔锛岀綉缁滆矾寰勫悕榛樿浣跨敤 Unicode銆傚鏋滄湭鎸囧畾 iocharset锛屽垯灏嗕娇鐢ㄦ湰鍦板鎴风鍐呮牳鏋勫缓鏈熼棿鎸囧畾鐨?nls_default銆傚鏋滄湇鍔″櫒涓嶆敮鎸?Unicode锛屾鍙傛暟鏃犵敤銆?  rsize
		榛樿璇诲彇澶у皬锛堥€氬父涓?16K锛夈€傚鎴风褰撳墠涓嶈兘浣跨敤澶т簬 CIFSMaxBufSize 鐨?rsize銆侰IFSMaxBufSize 榛樿涓?16K锛屽苟鍙湪妯″潡瀹夎鏃朵负 cifs.ko 鏇存敼锛堜粠 8K 鍒板唴鏍稿厑璁哥殑鏈€澶?kmalloc 澶у皬锛夈€傚皢 CIFSMaxBufSize 璁剧疆涓洪潪甯稿ぇ鐨勫€煎皢瀵艰嚧 cifs 浣跨敤鏇村鍐呭瓨锛屽苟鍦ㄦ煇浜涙儏鍐典笅闄嶄綆鎬ц兘銆傝浣跨敤澶т簬 127K锛堝師濮?cifs 鍗忚鏈€澶у€硷級鐨?rsize锛岃繕闇€瑕佹湇鍔″櫒鏀寔涓€涓柊鐨?Unix 鑳藉姏鏍囧織锛堢敤浜庨潪甯稿ぇ鐨勮鍙栵級锛屾煇浜涜緝鏂扮殑鏈嶅姟鍣紙渚嬪 Samba 3.0.26 鎴栨洿楂樼増鏈級鏀寔銆俽size 鍙互璁剧疆涓烘渶灏忓€?2048 鍒版渶澶у€?130048锛?27K 鎴?CIFSMaxBufSize锛屽彇杈冨皬鑰咃級銆?  wsize
		榛樿鍐欏叆澶у皬锛堥粯璁?57344锛?		CIFS 褰撳墠鍏佽鐨勬渶澶?wsize 涓?57344锛堝崄鍥涗釜 4096 瀛楄妭椤碉級
  actimeo=n
		灞炴€х紦瀛樿秴鏃讹紙绉掞級锛堥粯璁?1 绉掞級銆?		鍦ㄦ瓒呮椂涔嬪悗锛宑ifs 瀹㈡埛绔悜鏈嶅姟鍣ㄨ姹傛柊鐨勫睘鎬т俊鎭€傛閫夐」鍏佽閽堝宸ヤ綔璐熻浇闇€瑕佽皟鏁村睘鎬х紦瀛樿秴鏃躲€傝緝鐭殑瓒呮椂鎰忓懗鐫€鏇村ソ鐨勭紦瀛樹竴鑷存€э紝浣嗗鍔犱簡瀵规湇鍔″櫒鐨勮皟鐢ㄦ鏁般€傝緝闀跨殑瓒呮椂鎰忓懗鐫€鍑忓皯瀵规湇鍔″櫒鐨勮皟鐢ㄦ鏁帮紝浠ｄ环鏄緝涓嶄弗鏍肩殑缂撳瓨涓€鑷存€ф鏌ワ紙鍗冲湪鐭椂闂村唴灞炴€х紦瀛樹笉姝ｇ‘锛夈€?  rw
		浠ヨ鍐欐柟寮忔寕杞界綉缁滃叡浜紙娉ㄦ剰鏈嶅姟鍣ㄥ彲鑳戒粛瑙嗚鍏变韩涓哄彧璇伙級
  ro
		浠ュ彧璇绘柟寮忔寕杞界綉缁滃叡浜?  version
		鐢ㄤ簬鍖哄垎鎸傝浇杈呭姪绋嬪簭宸ュ叿鐨勪笉鍚岀増鏈紙閫氬父涓嶉渶瑕侊級
  sep
		濡傛灉鏄涓€涓寕杞介€夐」锛堝湪 -o 涔嬪悗锛夛紝鍒欒鐩栦綔涓烘寕杞介€夐」涔嬮棿鍒嗛殧绗︾殑閫楀彿

```
			-o user=myname,password=mypassword,domain=mydom

		could be passed instead with period as the separator by::

			-o sep=.user=myname.password=mypassword.domain=mydom

		this might be useful when comma is contained within username
		or password or domain. This option is less important
		when the cifs mount helper cifs.mount (version 1.1 or later)
		is used.
```
  nosuid
		涓嶅厑璁告墽琛屽甫鏈?suid 浣嶇殑杩滅▼鍙墽琛岀▼搴忋€傝繖浠呭鏀寔 CIFS Unix 鎵╁睍鐨勬湇鍔″櫒锛堝 Samba锛夋湁鎰忎箟銆傚鏋滀綘涓嶄俊浠荤綉缁滀腑鐨勬湇鍔″櫒锛堜綘鐨勬寕杞界洰鏍囷級锛屽缓璁綘鎸囧畾姝ら€夐」浠ヨ幏寰楁洿楂樼殑瀹夊叏鎬с€?  exec
		鍏佽鍦ㄦ寕杞戒笂鎵ц浜岃繘鍒舵枃浠躲€?  noexec
		涓嶅厑璁稿湪鎸傝浇涓婃墽琛屼簩杩涘埗鏂囦欢銆?  dev
		璇嗗埆杩滅▼鎸傝浇涓婄殑鍧楄澶囥€?  nodev
		涓嶈瘑鍒繙绋嬫寕杞戒笂鐨勮澶囥€?  suid
		鍏佽鍦ㄦ鎸傝浇鐐逛笂甯︽湁 suid 鐨勮繙绋嬫枃浠惰鎵ц锛堜互 root 鎵ц鎸傝浇鏃剁殑榛樿鍊硷紝nosuid 鏄敤鎴锋寕杞界殑榛樿鍊硷級銆?  credentials
		铏界劧琚?cifs 鍐呮牳缁勪欢蹇界暐锛屼絾瀹冭鎸傝浇杈呭姪绋嬪簭 mount.cifs 浣跨敤銆傚畨瑁?mount.cifs 鍚庯紝瀹冧細鎵撳紑骞惰鍙栨寚瀹氱殑鍑瘉鏂囦欢锛屼互鑾峰彇浼犻€掔粰 cifs vfs 鐨?userid 鍜?password 鍙傛暟銆?  guest
		铏界劧琚唴鏍哥粍浠跺拷鐣ワ紝浣嗗鏋滃湪鎸傝浇閫夐」涓婃寚瀹氫簡 guest锛宮ount.cifs 鎸傝浇杈呭姪绋嬪簭灏嗕笉浼氭彁绀虹敤鎴疯緭鍏ュ瘑鐮併€傚鏋滄湭鎸囧畾瀵嗙爜锛屽皢浣跨敤绌哄瘑鐮併€?  perm
		瀹㈡埛绔繘琛屾潈闄愭鏌ワ紙灏嗘枃浠剁殑 uid 鍜?gid 瀵圭収 mode 鍜屾湡鏈涙搷浣滆繘琛?vfs_permission 妫€鏌ワ級锛?		娉ㄦ剰杩欐槸闄ょ洰鏍囨満鍣ㄤ笂鐢辨湇鍔″櫒杞欢瀹屾垚鐨勬甯?ACL 妫€鏌ヤ箣澶栫殑棰濆妫€鏌ャ€?		瀹㈡埛绔潈闄愭鏌ラ粯璁ゅ惎鐢ㄣ€?  noperm
		瀹㈡埛绔笉杩涜鏉冮檺妫€鏌ャ€傝繖浼氬皢姝ゆ寕杞戒笂鐨勬枃浠舵毚闇茬粰鏈湴瀹㈡埛绔郴缁熶笂鐨勫叾浠栫敤鎴疯闂€傚畠閫氬父浠呭湪鏈嶅姟鍣ㄦ敮鎸?CIFS Unix 鎵╁睍锛屼絾瀹㈡埛绔拰鏈嶅姟鍣ㄧ郴缁熶笂鐨?UID/GID 涓嶅鎺ヨ繎浠ュ厑璁告墽琛屾寕杞界殑鐢ㄦ埛璁块棶鏃舵墠闇€瑕侊紝浣嗗畠鍙兘瀵归潪 CIFS Unix 鎵╁睍鎸傝浇鏈夌敤锛屼緥濡傚綋榛樿 mode 鍦ㄦ寕杞芥椂鎸囧畾浣嗕笉搴斿湪瀹㈡埛绔己鍒舵墽琛屾椂锛堜緥濡傚彲鑳藉湪鍚敤 MultiUserMount 鏃讹級銆?		娉ㄦ剰杩欎笉褰卞搷鐩爣鏈哄櫒涓婄敱鏈嶅姟鍣ㄨ蒋浠跺畬鎴愮殑姝ｅ父 ACL 妫€鏌ワ紙鏈嶅姟鍣?ACL 瀵规寕杞芥椂鎻愪緵鐨勭敤鎴峰悕鐨勬鏌ワ級銆?  serverino
		浣跨敤鏈嶅姟鍣ㄧ殑 inode 鍙凤紝鑰屼笉鏄湪瀹㈡埛绔嚜鍔ㄧ敓鎴愰€掑鐨?inode 鍙枫€傝櫧鐒惰繖灏嗘洿瀹规槗鍙戠幇纭摼鎺ユ枃浠讹紙鍥犱负瀹冧滑灏嗘湁鐩稿悓鐨?inode 鍙凤級锛屽苟涓?inode 鍙峰彲鑳芥槸鎸佷箙鐨勶紝浣嗚娉ㄦ剰锛屽鏋滃湪鍗曚釜鍏变韩涓嬪鍑轰簡澶氫釜鏈嶅姟鍣ㄧ鎸傝浇锛屾湇鍔″櫒涓嶄繚璇?inode 鍙锋槸鍞竴鐨勶紙鍥犱负濡傛灉鍦ㄥ悓涓€鍏变韩鐨勬洿楂樼骇鐩綍涓嬭浇鎸傝浇浜嗗涓枃浠剁郴缁燂紝鏈嶅姟鍣ㄤ笂鐨?inode 鍙峰彲鑳戒笉鍞竴锛夈€傛敞鎰忎竴浜涜緝鏃х殑锛堜緥濡?Windows 2000 涔嬪墠锛変笉鏀寔杩斿洖 UniqueID 鎴栫瓑鏁堢殑 CIFS Unix 鎵╁睍锛屽浜庤繖浜涳紝姝ゆ寕杞介€夐」灏嗕笉璧蜂綔鐢ㄣ€傚湪 nfsd 涓嬪鍑?cifs 鎸傝浇闇€瑕佸湪 cifs 鎸傝浇涓婁娇鐢ㄦ閫夐」銆?		濡傛灉鏈嶅姟鍣ㄦ敮鎸佹墍闇€鐨勭綉缁滄搷浣滐紝杩欑幇鍦ㄦ槸榛樿鍊笺€?  noserverino
		瀹㈡埛绔敓鎴?inode 鍙凤紙鑰屼笉鏄娇鐢ㄦ潵鑷湇鍔″櫒鐨勫疄闄?inode 鍙凤級銆傝繖浜?inode 鍙峰湪鍗歌浇鎴栭噸鍚悗浼氬彉鍖栵紝杩欏彲鑳戒娇鏌愪簺搴旂敤绋嬪簭鍥版儜锛屼絾骞堕潪鎵€鏈夋湇鍔″櫒鏂囦欢绯荤粺閮芥敮鎸佸敮涓€鐨?inode 鍙枫€?  setuids
		濡傛灉涓庢湇鍔″櫒鍗忓晢浜?CIFS Unix 鎵╁睍锛屽鎴风灏嗗皾璇曞湪鏂板垱寤虹殑鏂囦欢銆佺洰褰曞拰璁惧锛坈reate銆乵kdir銆乵knod锛変笂璁剧疆鏈湴杩涚▼鐨勬湁鏁?uid 鍜?gid銆傚鏋滄湭鍗忓晢 CIFS Unix 鎵╁睍锛屽浜庢柊鍒涘缓鐨勬枃浠跺拰鐩綍锛屽鎴风灏嗙紦瀛樻柊鏂囦欢鐨?uid 鍜?gid 鏈湴锛岃繖鎰忓懗鐫€鏂囦欢鐨?uid 鍦?inode 閲嶆柊鍔犺浇锛堟垨鐢ㄦ埛閲嶆柊鎸傝浇鍏变韩锛夋椂鍙互鏇存敼锛岃€屼笉鏄娇鐢ㄦ寕杞芥椂鎸囧畾鐨勯粯璁?uid 鍜?gid銆?  nosetuids
		瀹㈡埛绔笉浼氬皾璇曞湪鏂板垱寤虹殑鏂囦欢銆佺洰褰曞拰璁惧锛坈reate銆乵kdir銆乵knod锛変笂璁剧疆 uid 鍜?gid锛岃繖灏嗗鑷存湇鍔″櫒灏?uid 鍜?gid 璁剧疆涓洪粯璁ゅ€硷紙閫氬父鏄寕杞藉叡浜殑鐢ㄦ埛鐨勬湇鍔″櫒 uid锛夈€傝鏈嶅姟鍣紙鑰屼笉鏄鎴风锛夎缃?uid 鍜?gid 鏄粯璁ゅ€笺€傚鏋滄湭鍗忓晢 CIFS Unix 鎵╁睍锛屽垯鏂版枃浠剁殑 uid 鍜?gid 灏嗘樉绀轰负鎸傝浇鑰呯殑 uid锛坓id锛夋垨鎸傝浇鏃舵寚瀹氱殑 uid锛坓id锛夊弬鏁般€?  netbiosname
		褰撻€氳繃绔彛 139 鎸傝浇鍒版湇鍔″櫒鏃讹紝鎸囧畾鍦?RFC1001 netbios 浼氳瘽鍒濆鍖栨椂鐢ㄤ簬琛ㄧず瀹㈡埛绔?netbios 鏈哄櫒鍚嶇殑 RFC1001 婧愬悕绉般€?  direct
		涓嶅姝ゆ寕杞戒笂鎵撳紑鐨勬枃浠惰繘琛?inode 鏁版嵁缂撳瓨銆?		杩欐帓闄や簡鍦ㄦ鎸傝浇涓?mmap 鏂囦欢銆傚湪鏌愪簺鎯呭喌涓嬶紝鍏锋湁蹇€熺綉缁滀笖鍦ㄥ鎴风鍑犱箮娌℃湁鎴栨病鏈夌紦瀛樻敹鐩婏紙渚嬪褰撳簲鐢ㄧ▼搴忚繘琛屽ぇ浜庨〉澶у皬涓斾笉閲嶈鐩稿悓鏁版嵁鐨勫ぇ鍨嬮『搴忚鍙栨椂锛夛紝杩欏彲浠ユ彁渚涙瘮榛樿琛屼负鏇村ソ鐨勬€ц兘锛岄粯璁よ涓哄湪鑾峰彇 oplock锛堢紦瀛樹护鐗岋級鏃堕€氳繃鏈湴 Linux 瀹㈡埛绔?pagecache 缂撳瓨璇诲彇锛坮eadahead锛夊拰鍐欏叆锛坵ritebehind锛夈€傛敞鎰?direct 鍏佽灏嗗ぇ浜庨〉澶у皬鐨勫啓鎿嶄綔鍙戦€佸埌鏈嶅姟鍣ㄣ€?  strictcache
		鐢ㄤ簬寮€鍚弗鏍肩紦瀛樻ā寮忋€傚湪姝ゆā寮忎笅锛屽鎴风鍦ㄦ嫢鏈?Oplock Level II 鏃跺缁堜粠缂撳瓨璇诲彇锛屽惁鍒欎粠鏈嶅姟鍣ㄨ鍙栥€傛墍鏈夊啓鍏ョ殑鏁版嵁閮藉瓨鍌ㄥ湪缂撳瓨涓紝浣嗗鏋滃鎴风娌℃湁 Exclusive Oplock锛屽畠浼氬皢鏁版嵁鍐欏叆鏈嶅姟鍣ㄣ€?  rwpidforward
		灏嗘墦寮€鏂囦欢鐨勮繘绋嬬殑 pid 杞彂鍒拌鏂囦欢涓婄殑浠讳綍璇诲彇鎴栧啓鎿嶄綔銆傝繖鍙互闃叉鍍?WINE 杩欐牱鐨勫簲鐢ㄧ▼搴忓湪浣跨敤寮哄埗 brlock 椋庢牸鏃惰鍐欏け璐ャ€?  acl
		濡傛灉鏈嶅姟鍣ㄦ敮鎸侊紝鍏佽 setfacl 鍜?getfacl 绠＄悊 posix ACL銆傦紙榛樿锛?  noacl
		涓嶅厑璁稿湪姝ゆ寕杞戒笂杩涜 setfacl 鍜?getfacl 璋冪敤
  user_xattr
		鍏佽灏嗙敤鎴?xattr锛堝悕绉颁互 ``user.`` 鎴?``os2.`` 寮€澶寸殑灞炴€э級浣滀负 OS/2 EA锛堟墿灞曞睘鎬э級鑾峰彇鍜岃缃埌鏈嶅姟鍣ㄣ€傝繖鍏佽鏀寔 setfattr 鍜?getfattr 宸ュ叿銆傦紙榛樿锛?  nouser_xattr
		涓嶅厑璁?getfattr/setfattr 鑾峰彇/璁剧疆/鍒楀嚭 xattrs
  mapchars
		灏嗕竷涓繚鐣欏瓧绗︿腑鐨勫叚涓紙鍙嶆枩鏉犻櫎澶栵級缈昏瘧涓猴細

			*?<>|:

		閲嶆槧灏勮寖鍥达紙0xF000 浠ヤ笂锛夛紝杩欎篃鍏佽 CIFS 瀹㈡埛绔瘑鍒敱 Windows 鐨?POSIX 妯℃嫙浠ヨ繖浜涘瓧绗﹀垱寤虹殑鏂囦欢銆傚綋鎸傝浇鍒板ぇ澶氱増鏈殑 Samba锛堝畠涔熺姝㈠垱寤哄拰鎵撳紑鍚嶇О鍖呭惈杩欎竷涓瓧绗︿腑浠讳綍涓€涓殑鏂囦欢锛夋椂锛岃繖涔熷緢鏈夌敤銆傚鏋滄湇鍔″櫒涓嶆敮鎸佺嚎璺笂鐨?Unicode锛屽垯杩欎笉璧蜂綔鐢ㄣ€?  nomapchars
		涓嶇炕璇戣繖涓冧釜瀛楃涓殑浠讳綍涓€涓紙榛樿锛夈€?  nocase
		璇锋眰涓嶅尯鍒嗗ぇ灏忓啓鐨勮矾寰勫悕鍖归厤锛堝鏋滄湇鍔″櫒鏀寔锛屽垯鍖哄垎澶у皬鍐欐槸榛樿锛夈€?		锛堟寕杞介€夐」 ``ignorecase`` 涓?``nocase`` 鐩稿悓锛?  posixpaths
		濡傛灉鏀寔 CIFS Unix 鎵╁睍锛屽皾璇曞崗鍟?posix 璺緞鍚嶆敮鎸侊紝瀹冨厑璁告煇浜涘湪鍏稿瀷 CIFS 鏂囦欢鍚嶄腑琚姝㈢殑瀛楃锛岃€屾棤闇€閲嶆槧灏勩€傦紙榛樿锛?  noposixpaths
		濡傛灉鏀寔 CIFS Unix 鎵╁睍锛屼笉璇锋眰 posix 璺緞鍚嶆敮鎸侊紙杩欏彲鑳藉鑷存湇鍔″櫒鎷掔粷鍒涘缓鍖呭惈鏌愪簺淇濈暀瀛楃鐨勬枃浠讹級銆?  nounix
		瀵规鎸傝浇锛堟爲杩炴帴锛夌鐢?CIFS Unix 鎵╁睍銆傝繖寰堝皯闇€瑕侊紝浣嗗畠鍙兘鐢ㄤ簬涓€娆″叧闂涓缃紙鍗?posix acls銆乸osix locks銆乸osix paths銆乻ymlink 鏀寔浠ュ強浠庢湇鍔″櫒妫€绱?uids/gids/mode锛夛紝鎴栫敤浜庤閬垮疄鐜颁簡 Unix 鎵╁睍鐨勬湇鍔″櫒涓殑 bug銆?  nobrl
		涓嶅悜鏈嶅姟鍣ㄥ彂閫佸瓧鑺傝寖鍥撮攣璇锋眰銆?		杩欏浜庢煇浜涘洜 cifs 椋庢牸寮哄埗瀛楄妭鑼冨洿閿佽€屼腑鏂紙涓斿ぇ澶氭暟 cifs 鏈嶅姟鍣ㄥ皻涓嶆敮鎸佽姹傚缓璁€у瓧鑺傝寖鍥撮攣锛夌殑搴旂敤绋嬪簭鏄繀瑕佺殑銆?  forcemandatorylock
		鍗充娇鏈嶅姟鍣ㄦ敮鎸?posix锛堝缓璁€э級瀛楄妭鑼冨洿閿佸畾锛屼篃鍙彂閫佸己鍒堕攣璇锋眰銆傚浜庝竴浜涳紙澶ф寰堝皯瑙侊級鏈€鍒濅负 DOS/Windows 缂栧啓銆侀渶瑕?Windows 椋庢牸寮哄埗瀛楄妭鑼冨洿閿佺殑搴旂敤绋嬪簭锛屽畠浠彲鑳借兘澶熷埄鐢ㄦ閫夐」锛屽己鍒?cifs 瀹㈡埛绔彧鍙戦€佸己鍒堕攣锛屽嵆浣?cifs 鏈嶅姟鍣ㄦ敮鎸?posix 寤鸿鎬ч攣銆?		``forcemand`` 琚帴鍙椾负姝ゆ寕杞介€夐」鐨勭畝鍐欏舰寮忋€?  nostrictsync
		濡傛灉璁剧疆浜嗘鎸傝浇閫夐」锛屽綋搴旂敤绋嬪簭杩涜 fsync 璋冪敤鏃讹紝cifs 瀹㈡埛绔笉浼氬悜鏈嶅姟鍣ㄥ彂閫?SMB Flush锛堝己鍒舵湇鍔″櫒绔嬪嵆灏嗚鏂囦欢鐨勬墍鏈夎剰鏁版嵁鍐欏叆纾佺洏锛夛紝灏界 cifs 浠嶅皢鎵€鏈夎剰锛堢紦瀛橈級鏂囦欢鏁版嵁鍙戦€佸埌鏈嶅姟鍣ㄥ苟绛夊緟鏈嶅姟鍣ㄥ搷搴斿啓鍏ャ€傜敱浜?SMB Flush 鍙兘闈炲父鎱紝涓旀煇浜涙湇鍔″櫒鍙兘瓒冲鍙潬锛堝彲浠ュ啋绋嶅井寤惰繜灏嗘暟鎹埛鏂板埌鏈嶅姟鍣ㄧ鐩樼殑椋庨櫓锛夛紝寮€鍚閫夐」鍙兘鏈夊姪浜庢敼鍠勯偅浜?fsync 杩囧鐨勫簲鐢ㄧ▼搴忕殑鎬ц兘锛屼絾鏈夋湇鍔″櫒宕╂簝鐨勫皬椋庨櫓銆傚鏋滄湭璁剧疆姝ゆ寕杞介€夐」锛岄粯璁ゆ儏鍐典笅 cifs 浼氬湪姣忔 fsync 璋冪敤鏃跺彂閫?SMB flush 璇锋眰锛堝苟绛夊緟鍝嶅簲锛夈€?  nodfs
		鍗充娇鏈嶅姟鍣ㄥ０绉版敮鎸侊紝涔熺鐢?DFS锛堝叏灞€鍛藉悕绌洪棿鏀寔锛夈€傝繖鏈夊姪浜庤閬?Samba 鏈嶅姟鍣ㄧ増鏈?3.0.24 鍜?3.0.25 瑙ｆ瀽 DFS 璺緞鐨勯棶棰樸€?  remount
		閲嶆柊鎸傝浇鍏变韩锛堝父鐢ㄤ簬浠?ro 鏀逛负 rw 鎸傝浇鎴栧弽涔嬶級
  cifsacl
		鏍规嵁鏂囦欢鐨?Windows ACL 鎶ュ憡 mode 浣嶏紙渚嬪鍦?stat 涓婏級銆傦紙瀹為獙鎬э級
  servern
		鎸囧畾灏濊瘯涓庢湇鍔″櫒寤虹珛浼氳瘽鏃惰浣跨敤鐨勬湇鍔″櫒 netbios 鍚嶇О锛圧FC1001 鍚嶇О锛夈€?		杩欏浜庢寕杞藉埌鏌愪簺杈冩棫鐨勬湇鍔″櫒锛堜緥濡?OS/2 鎴?Windows 98 鍜?Windows ME锛夋槸蹇呴渶鐨勶紝鍥犱负瀹冧滑涓嶆敮鎸侀粯璁ゆ湇鍔″櫒鍚嶇О銆傛湇鍔″櫒鍚嶇О鏈€闀垮彲杈?15 涓瓧绗︼紝閫氬父澶у啓銆?  sfu
		褰撴湭鍗忓晢 CIFS Unix 鎵╁睍鏃讹紝灏濊瘯浠ヤ笌 Unix 鏈嶅姟锛圫FU锛夊吋瀹圭殑鏍煎紡鍒涘缓璁惧鏂囦欢鍜?fifos銆傛澶栭€氳繃 SETFILEBITS 鎵╁睍灞炴€э紙濡?SFU 閭ｆ牱锛夋绱?mode 鐨勭 10-12 浣嶃€傚皢鏉?mode 鐨勪綆 9 浣嶄篃灏嗛€氳繃鏌ヨ瀹夊叏鎻忚堪绗︼紙ACL锛夋潵妯℃嫙銆?  mfsymlinks
		鍚敤瀵?Minshall+French 绗﹀彿閾炬帴鐨勬敮鎸?		锛堣 http://wiki.samba.org/index.php/UNIX_Extensions#Minshall.2BFrench_symlinks锛?		褰撲笌 'sfu' 閫夐」涓€璧锋寚瀹氭椂锛屾閫夐」琚拷鐣ャ€傚嵆浣挎湇鍔″櫒鏀寔 CIFS Unix 鎵╁睍锛屼篃浼氫娇鐢?Minshall+French 绗﹀彿閾炬帴銆?  sign
		蹇呴』浣跨敤鍖呯鍚嶏紙鏈夊姪浜庨伩鍏嶈矾鐢变腑涓棿绯荤粺瀵规暟鎹殑闈為鏈熶慨鏀癸級銆傛敞鎰忕鍚嶄笉鑳戒笌 lanman 鎴栨槑鏂囪璇佷竴璧峰伐浣溿€?  seal
		蹇呴』鍦ㄦ鎸傝浇鍏变韩涓婂瘑灏侊紙鍔犲瘑锛夋墍鏈夋暟鎹紝鐒跺悗鍐嶅湪缃戠粶涓婂彂閫併€傞渶瑕?Unix 鎵╁睍鏀寔銆傛敞鎰忚繖涓?sign 鎸傝浇閫夐」鐨勪笉鍚屼箣澶勫湪浜庯紝瀹冨鑷撮€氳繃姝ゆ寕杞藉叡浜彂閫佺殑鏁版嵁琚姞瀵嗭紝浣嗘寕杞藉埌鍚屼竴鏈嶅姟鍣ㄧ殑鍏朵粬鍏变韩涓嶅彈褰卞搷銆?  locallease
		姝ら€夐」寰堝皯闇€瑕併€傛煇浜涘簲鐢ㄧ▼搴忥紙濡?Samba 鍜?NFSv4 鏈嶅姟鍣級浣跨敤 fcntl F_SETLEASE 鏉ユ鏌ユ枃浠舵槸鍚﹀彲缂撳瓨銆侰IFS 鏃犳硶鏄惧紡璇锋眰绉熺害锛屼絾鍙互妫€鏌ユ枃浠舵槸鍚﹀彲缂撳瓨锛坥plocked锛夈€備笉骞哥殑鏄紝鍗充娇鏂囦欢鏈 oplocked锛屽畠浠嶅彲鑳芥槸鍙紦瀛樼殑锛堝嵆濡傛灉娌℃湁鍏朵粬鏈湴杩涚▼浣跨敤璇ユ枃浠讹紝cifs 瀹㈡埛绔彲浠ユ巿浜?fcntl 绉熺害锛夛紝渚嬪褰撴湇鍔″櫒涓嶆敮鎸?oplocks 涓旂敤鎴风‘淇″璇ユ枃浠剁殑鍞竴鏇存柊灏嗘潵鑷瀹㈡埛绔椂銆傛寚瀹氭鎸傝浇閫夐」灏嗗厑璁?cifs 瀹㈡埛绔粎涓烘湭琚?oplocked 鐨勬枃浠跺湪鏈湴妫€鏌ョ绾︼紝鑰屼笉鏄湪杩欑鎯呭喌涓嬫嫆缁濈绾︺€傦紙瀹為獙鎬э級
  sec
		瀹夊叏妯″紡銆傚厑璁哥殑鍊间负锛?
			none
				灏濊瘯浣滀负绌虹敤鎴凤紙鏃犲悕锛夎繛鎺?			krb5
				浣跨敤 Kerberos 鐗堟湰 5 璁よ瘉
			krb5i
				浣跨敤 Kerberos 璁よ瘉鍜屽寘绛惧悕
			ntlm
				浣跨敤 NTLM 瀵嗙爜鍝堝笇锛堥粯璁わ級
			ntlmi
				浣跨敤甯︾鍚嶇殑 NTLM 瀵嗙爜鍝堝笇锛堝鏋?				/proc/fs/cifs/PacketSigningEnabled 寮€鍚紝
				鎴栬€呭鏋滄湇鍔″櫒涔熼渶瑕佺鍚嶏紝涔熷彲浠ヤ綔涓洪粯璁わ級
			ntlmv2
				浣跨敤 NTLMv2 瀵嗙爜鍝堝笇
			ntlmv2i
				浣跨敤甯﹀寘绛惧悕鐨?NTLMv2 瀵嗙爜鍝堝笇
			lanman
				锛堝鏋滃湪鍐呮牳閰嶇疆涓厤缃級浣跨敤杈冩棫鐨?				lanman 鍝堝笇
  hard
		濡傛灉鏈嶅姟鍣ㄦ棤鍝嶅簲锛岄噸璇曟枃浠舵搷浣?  soft
		闄愬埗瀵规棤鍝嶅簲鏈嶅姟鍣ㄧ殑閲嶈瘯锛堥€氬父浠呬竴娆￠噸璇曪級鐒跺悗杩斿洖閿欒銆傦紙榛樿锛?
```
The mount.cifs mount helper also accepts a few mount options before -o
including:

=============== ===============================================================
	-S      take password from stdin (equivalent to setting the environment
		variable `PASSWD_FD=0`
	-V      print mount.cifs version
	-?      display simple usage information
=============== ===============================================================

With most 2.6 kernel versions of modutils, the version of the cifs kernel
module can be displayed via modinfo.

```
mount.cifs 鎸傝浇杈呭姪绋嬪簭鍦?-o 涔嬪墠涔熸帴鍙椾竴浜涙寕杞介€夐」锛屽寘鎷細

=============== ===============================================================
	-S      浠?stdin 鑾峰彇瀵嗙爜锛堢瓑鏁堜簬璁剧疆鐜鍙橀噺 `PASSWD_FD=0`锛?	-V      鎵撳嵃 mount.cifs 鐗堟湰
	-?      鏄剧ず绠€鍗曠殑鐢ㄦ硶淇℃伅
=============== ===============================================================

瀵逛簬澶у鏁?2.6 鍐呮牳鐗堟湰鐨?modutils锛宑ifs 鍐呮牳妯″潡鐨勭増鏈彲浠ラ€氳繃 modinfo 鏄剧ず銆?
## Misc /proc/fs/cifs 鏍囧織涓庤皟璇曚俊鎭?
淇℃伅浼枃浠讹細

======================= =======================================================
DebugData		Displays information about active CIFS sessions and
			shares, features enabled as well as the cifs.ko
			version.
Stats			Lists summary resource usage information as well as per
			share statistics.
open_files		List all the open file handles on all active SMB sessions.
mount_params            List of all mount parameters available for the module
======================= =======================================================

閰嶇疆浼枃浠讹細

======================= =======================================================
SecurityFlags		Flags which control security negotiation and
			also packet signing. Authentication (may/must)
			flags (e.g. for NTLMv2) may be combined with
			the signing flags.  Specifying two different password
			hashing mechanisms (as "must use") on the other hand
```
				0x00C5

			(NTLMv2 and packet signing allowed). Some SecurityFlags
			may require enabling a corresponding menuconfig option.

			  may use packet signing			0x00001
			  must use packet signing			0x01001
			  may use NTLMv2				0x00004
			  must use NTLMv2				0x04004
			  may use Kerberos security (krb5)		0x00008
			  must use Kerberos                             0x08008
			  may use NTLMSSP               		0x00080
			  must use NTLMSSP           			0x80080
			  seal (packet encryption)			0x00040
			  must seal                                     0x40040

```
cifsFYI			If set to non-zero value, additional debug information
			will be logged to the system error log.  This field
			contains three flags controlling different classes of
			debugging entries.  The maximum value it can be set
			to is 7 which enables all debugging points (default 0).
			Some debugging statements are not compiled into the
			cifs kernel unless CONFIG_CIFS_DEBUG2 is enabled in the
			kernel configuration. cifsFYI may be set to one or
```
			  +-----------------------------------------------+------+
			  | log cifs informational messages		  | 0x01 |
			  +-----------------------------------------------+------+
			  | log return codes from cifs entry points	  | 0x02 |
			  +-----------------------------------------------+------+
			  | log slow responses				  | 0x04 |
			  | (ie which take longer than 1 second)	  |      |
			  |                                               |      |
			  | CONFIG_CIFS_STATS2 must be enabled in .config |      |
			  +-----------------------------------------------+------+

```
traceSMB		If set to one, debug information is logged to the
			system error log with the start of smb requests
			and responses (default 0)
LookupCacheEnable	If set to one, inode information is kept cached
			for one second improving performance of lookups
			(default 1)
LinuxExtensionsEnabled	If set to one then the client will attempt to
			use the CIFS "UNIX" extensions which are optional
			protocol enhancements that allow CIFS servers
			to return accurate UID/GID information as well
			as support symbolic links. If you use servers
			such as Samba that support the CIFS Unix
			extensions but do not want to use symbolic link
			support and want to map the uid and gid fields
			to values supplied at mount (rather than the
			actual values, then set this to zero. (default 1)
dfscache		List the content of the DFS cache.
			If set to 0, the client will clear the cache.
======================= =======================================================

杩欎簺瀹為獙鎬х壒鎬у拰璺熻釜鍙互閫氳繃鏇存敼 /proc/fs/cifs 涓殑鏍囧織鏉ュ惎鐢紙鍦?cifs 妯″潡宸插畨瑁呮垨鏋勫缓杩涘唴鏍镐箣鍚庯紝渚嬪 insmod cifs锛夈€傝鍚敤鏌愰」鐗规€э紝灏嗗叾璁剧疆涓?1锛屼緥濡傝鍚敤

```
	echo 7 > /proc/fs/cifs/cifsFYI

```
cifsFYI 鍏呭綋浣嶆帺鐮併€傚皢鍏惰缃负 1 浼氬惎鐢ㄥ悇绉嶄俊鎭€ф秷鎭殑棰濆鍐呮牳鏃ュ織璁板綍銆? 鍚敤闈為浂 SMB 杩斿洖鐮佺殑鏃ュ織璁板綍锛岃€?4 鍚敤鑰楁椂瓒呰繃涓€绉掑畬鎴愮殑璇锋眰锛堝瓧鑺傝寖鍥撮攣璇锋眰闄ゅ锛夌殑鏃ュ織璁板綍銆傚皢鍏惰缃负 4 闇€瑕佸湪鍐呮牳閰嶇疆锛?config锛変腑璁剧疆 CONFIG_CIFS_STATS2銆傚皢鍏惰缃负 7 浼氬惎鐢ㄥ叏閮ㄤ笁椤广€傛渶鍚庯紝璺熻釜

```
	echo 1 > /proc/fs/cifs/traceSMB

```
姣忎釜鍏变韩锛堟瘡涓鎴风鎸傝浇锛夌殑缁熻淇℃伅鍙湪 /proc/fs/cifs/Stats 涓壘鍒般€傚鏋滃唴鏍搁厤缃紙.config锛変腑鍚敤浜?CONFIG_CIFS_STATS2锛屽垯鍙幏寰楁洿澶氫俊鎭€傝繑鍥炵殑缁熻鏁版嵁鍖呮嫭琛ㄧず鎸夎姹傜被鍨嬶紙read銆亀rite銆乧lose 绛夛級鍒嗙粍鐨勫凡灏濊瘯鍜屽け璐ワ紙鍗虫湇鍔″櫒鐨勯潪闆惰繑鍥炵爜锛夌殑 SMB3锛堟垨 cifs锛夎姹傛暟閲忕殑璁℃暟鍣ㄣ€傝繕璁板綍浜嗗悜璇ュ叡浜殑鏈嶅姟鍣ㄨ鍙栧拰鍐欏叆鐨勬€诲瓧鑺傛暟銆傛敞鎰忥紝鐢变簬瀹㈡埛绔紦瀛樻晥搴旓紝杩欏彲鑳藉皯浜庡鎴风涓婅繍琛岀殑搴旂敤绋嬪簭璇诲彇鍜屽啓鍏ョ殑瀛楄妭鏁般€傚彲浠ラ€氳繃 `echo 0 > /proc/fs/cifs/Stats` 灏嗙粺璁′俊鎭噸缃负闆讹紝杩欏湪姣旇緝涓や釜涓嶅悓鍦烘櫙鐨勬€ц兘鏃跺彲鑳芥湁鐢ㄣ€?
鍙﹁娉ㄦ剰锛宍cat /proc/fs/cifs/DebugData` 灏嗘樉绀烘湁鍏虫椿鍔ㄤ細璇濆拰宸叉寕杞藉叡浜殑淇℃伅銆?
鍚敤 Kerberos锛堟墿灞曞畨鍏級鍙互宸ヤ綔锛屼絾闇€瑕佹湁鐗堟湰 1.2 鎴栨洿楂樼殑杈呭姪绋嬪簭 cifs.upcall 瀛樺湪骞堕厤缃湪 /etc/request-key.conf 鏂囦欢涓€俢ifs.upcall 杈呭姪绋嬪簭鏉ヨ嚜 Samba 椤圭洰锛坔ttps://www.samba.org锛夈€侼TLM銆丯TLMv2 鍜?LANMAN 鏀寔涓嶉渶瑕佹杈呭姪绋嬪簭銆傛敞鎰忥紝NTLMv2 瀹夊叏锛堜笉闇€瑕?cifs.upcall 杈呭姪绋嬪簭锛夛紝鑰屼笉鏄娇鐢?Kerberos锛屽涓€浜涚敤渚嬪凡缁忚冻澶熴€?
DFS 鏀寔鍏佽閫忔槑閲嶅畾鍚戝埌 MS-DFS 鍛藉悕绌洪棿涓殑鍏变韩銆傛澶栵紝瀵逛簬鎸囧畾涓轰互涓绘満鍚嶏紙鑰屼笉鏄?IP 鍦板潃锛夊紑澶寸殑 UNC 鍚嶇О鐨勭洰鏍囧叡浜殑 DFS 鏀寔锛岄渶瑕佷竴涓敤鎴风┖闂磋緟鍔╃▼搴忥紙濡?cifs.upcall锛夊瓨鍦紝浠ヤ究灏嗕富鏈哄悕杞崲涓?ip 鍦板潃锛屽苟涓旇鐢ㄦ埛绌洪棿杈呭姪绋嬪簭涔熷繀椤婚厤缃湪 /etc/request-key.conf 鏂囦欢涓€係amba銆乄indows 鏈嶅姟鍣ㄥ拰璁稿 NAS 璁惧鏀寔 DFS锛屼綔涓烘瀯寤哄叏灞€鍛藉悕绌洪棿浠ョ畝鍖栫綉缁滈厤缃苟鎻愰珮鍙潬鎬х殑涓€绉嶆柟寮忋€?
瑕佷娇鐢?cifs Kerberos 鍜?DFS 鏀寔锛屽簲瀹夎 Linux keyutils 杞欢鍖咃紝骞朵笖搴斿悜

```
  create cifs.spnego * * /usr/local/sbin/cifs.upcall %k
  create dns_resolver * * /usr/local/sbin/cifs.upcall %k

```
## CIFS 鍐呮牳妯″潡鍙傛暟

杩欎簺妯″潡鍙傛暟鍙互鍦ㄤ互涓嬫椂闂存寚瀹氭垨淇敼锛?
```
	/sys/module/cifs/parameters/<param>

```
```
    echo "value" > /sys/module/cifs/parameters/<param>

```
鍏充簬鍙敤妯″潡鍙傛暟鍙婂叾鍊肩殑鏇磋缁嗘弿杩板彲閫氳繃浠ヤ笅鏂瑰紡鏌ョ湅锛?
    modinfo cifs (or modinfo smb3)

================= ==========================================================
1. enable_oplocks 鍚敤鎴栫鐢?oplocks銆俹plocks 榛樿鍚敤銆?		  [Y/y/1] 鍚敤銆傝绂佺敤鍙娇鐢?[N/n/0]銆?================= ==========================================================
