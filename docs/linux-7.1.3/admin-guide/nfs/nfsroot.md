## 閫氳繃 NFS 鎸傝浇鏍规枃浠剁郴缁燂紙nfsroot锛?
:Authors:
	Written 1996 by Gero Kuhlmann <gero@gkminix.han.de>

	Updated 1997 by Martin Mares <mj@atrey.karlin.mff.cuni.cz>

	Updated 2006 by Nico Schottelius <nico-kernel-nfsroot@schottelius.org>

	Updated 2006 by Horms <horms@verge.net.au>

	Updated 2018 by Chris Novakovic <chris@chrisn.me.uk>



涓轰簡浣跨敤涓€涓棤鐩樼郴缁燂紙渚嬪 X 缁堢鎴栨墦鍗版湇鍔″櫒锛夛紝鏍规枃浠剁郴缁熷繀椤讳綅浜庝竴涓潪纾佺洏璁惧涓娿€傝繖鍙互鏄竴涓?initramfs锛堝弬瑙?Documentation/filesystems/ramfs-rootfs-initramfs.rst锛夈€佷竴涓?ramdisk锛堝弬瑙?Documentation/admin-guide/initrd.rst锛夛紝鎴栨槸涓€涓€氳繃 NFS 鎸傝浇鐨勬枃浠剁郴缁熴€備笅闈㈢殑鏂囧瓧鎻忚堪浜嗗浣曚娇鐢?NFS 浣滀负鏍规枃浠剁郴缁熴€傚湪鏈枃鐨勫叾浣欓儴鍒嗕腑锛屸€渃lient鈥?鎸囨棤鐩樼郴缁燂紝鈥渟erver鈥?鎸?NFS 鏈嶅姟鍣ㄣ€?


## Enabling nfsroot capabilities锛堝惎鐢?nfsroot 鑳藉姏锛?
涓轰簡浣跨敤 nfsroot锛岄渶瑕佸湪閰嶇疆鏃跺皢 NFS 瀹㈡埛绔敮鎸侀€変负鍐呭缓锛坆uilt-in锛夈€備竴鏃﹂€変腑浜嗗畠锛宯fsroot 閫夐」灏变細鍙樺緱鍙敤锛岃閫夐」涔熷簲璇ヨ閫変腑銆?
鍦ㄧ綉缁滈€夐」涓紝鍙互閫変腑鍐呮牳绾ц嚜鍔ㄩ厤缃紙kernel level autoconfiguration锛夛紝浠ュ強瑕佹敮鎸佺殑鑷姩閰嶇疆绫诲瀷銆傞€変腑 DHCP銆丅OOTP 鍜?RARP 鍏ㄩ儴閮芥槸瀹夊叏鐨勩€?


## Kernel command line锛堝唴鏍稿懡浠よ锛?
褰撳唴鏍歌寮曞鍔犺浇绋嬪簭锛堣涓嬫枃锛夊姞杞藉悗锛岄渶瑕佸憡璇夊畠瑕佷娇鐢ㄥ摢涓牴鏂囦欢绯荤粺璁惧銆傝€屽湪 nfsroot 鐨勬儏鍐典笅锛岃繕瑕佸憡璇夊畠鍘诲摢閲屾壘鍒版湇鍔″櫒銆佷互鍙婃湇鍔″櫒涓婅浣滀负鏍规寕杞界殑鐩綍鍚嶃€傝繖鍙互閫氳繃浠ヤ笅鍐呮牳鍛戒护琛屽弬鏁版潵寤虹珛锛?
root=/dev/nfs
  杩欐槸鍚敤浼?NFS 璁惧鎵€蹇呴渶鐨勩€傛敞鎰忓畠骞堕潪涓€涓湡瀹炶澶囷紝鑰屽彧鏄竴涓悓涔夎瘝锛岀敤鏉ュ憡璇夊唴鏍镐娇鐢?NFS 鑰岄潪鐪熷疄璁惧銆?
nfsroot=[<server-ip>:]<root-dir>[,<nfs-options>]
  濡傛灉鍛戒护琛屼笂娌℃湁缁欏嚭 `nfsroot` 鍙傛暟锛屽皢浣跨敤榛樿鐨?`"/tftpboot/%s"`銆?
  <server-ip>	鎸囧畾 NFS 鏈嶅姟鍣ㄧ殑 IP 鍦板潃銆?		榛樿鍦板潃鐢?ip 鍙傛暟鍐冲畾锛堣涓嬫枃锛夈€傝鍙傛暟鍏佽涓?IP 鑷姩閰嶇疆鍜?NFS 浣跨敤涓嶅悓鐨勬湇鍔″櫒銆?
  <root-dir>	鏈嶅姟鍣ㄤ笂瑕佷綔涓烘牴鎸傝浇鐨勭洰褰曞悕銆?		濡傛灉瀛楃涓蹭腑鏈変竴涓?"%s" 璁板彿锛屽畠灏嗚鏇挎崲涓哄鎴风 IP 鍦板潃鐨?ASCII 琛ㄧず銆?
  <nfs-options>	鏍囧噯 NFS 閫夐」銆傛墍鏈夐€夐」浠ラ€楀彿鍒嗛殧銆?```

			port		= as given by server portmap daemon
			rsize		= 4096
			wsize		= 4096
			timeo		= 7
			retrans		= 3
			acregmin	= 3
			acregmax	= 60
			acdirmin	= 30
			acdirmax	= 60
			flags		= hard, nointr, noposix, cto, ac


```
ip=<client-ip>:<server-ip>:<gw-ip>:<netmask>:<hostname>:<device>:<autoconf>:<dns0-ip>:<dns1-ip>:<ntp0-ip>
  璇ュ弬鏁板憡璇夊唴鏍稿浣曢厤缃澶囩殑 IP 鍦板潃锛屼互鍙婂浣曞缓绔?IP 璺敱琛ㄣ€傚畠鏈€鍒濊绉颁负 nfsaddrs锛屼絾鐜板湪寮曞鏃剁殑 IP 閰嶇疆鐙珛浜?NFS 宸ヤ綔锛屽洜姝よ閲嶅懡鍚嶄负 ip锛屾棫鍚嶇О浣滀负鍒悕淇濈暀浠ュ吋瀹广€?
  濡傛灉璇ュ弬鏁扮己澶变簬鍐呮牳鍛戒护琛岋紝鍒欐墍鏈夊瓧娈甸兘琚亣瀹氫负绌猴紝骞堕€傜敤涓嬫枃涓彁鍒扮殑榛樿鍊笺€備竴鑸€岃█锛岃繖鎰忓懗鐫€鍐呮牳浼氬皾璇曚娇鐢ㄨ嚜鍔ㄩ厤缃潵閰嶇疆涓€鍒囥€?
  <autoconf> 鍙傛暟鍙互鍗曠嫭浣滀负 ip 鍙傛暟鐨勫€煎嚭鐜帮紙鍓嶉潰涓嶅甫鎵€鏈夌殑 鈥?鈥?瀛楃锛夈€傚鏋滆鍊间负 "ip=off" 鎴?"ip=none"锛屽垯涓嶈繘琛岃嚜鍔ㄩ厤缃紝鍚﹀垯灏嗚繘琛岃嚜鍔ㄩ厤缃€傛渶甯哥敤鐨勭敤娉曟槸 "ip=dhcp"銆?
  <client-ip>	瀹㈡埛绔殑 IP 鍦板潃銆?		榛樿锛氫娇鐢ㄨ嚜鍔ㄩ厤缃‘瀹氥€?
  <server-ip>	NFS 鏈嶅姟鍣ㄧ殑 IP 鍦板潃銆?		濡傛灉浣跨敤 RARP 鏉ョ‘瀹氬鎴风鍦板潃锛屼笖璇ュ弬鏁伴潪绌猴紝鍒欏彧鎺ュ彈鏉ヨ嚜鎸囧畾鏈嶅姟鍣ㄧ殑搴旂瓟銆?
		浠呭湪 NFS 鏍规枃浠剁郴缁熸椂鎵嶆槸蹇呴渶鐨勩€備篃灏辨槸璇达紝濡傛灉瀹冪己澶变笖 NFS 鏍规湭鍚敤锛屽垯涓嶄細瑙﹀彂鑷姩閰嶇疆銆?
		璇ュ€间細琚鍑哄埌 /proc/net/pnp锛屽墠缂€涓?"bootserver "锛堣涓嬫枃锛夈€?
		榛樿锛氫娇鐢ㄨ嚜鍔ㄩ厤缃‘瀹氥€備娇鐢ㄨ嚜鍔ㄩ厤缃湇鍔″櫒鐨勫湴鍧€銆?
  <gw-ip>	濡傛灉鏈嶅姟鍣ㄤ綅浜庝笉鍚屽瓙缃戯紝鍒欎负缃戝叧鐨?IP 鍦板潃銆?		榛樿锛氫娇鐢ㄨ嚜鍔ㄩ厤缃‘瀹氥€?
  <netmask>	鏈湴缃戠粶鎺ュ彛鐨勭綉鎺╃爜銆?		濡傛灉鏈寚瀹氾紝鍒欐牴鎹鎴风 IP 鍦板潃锛堝亣璁句负鏈夌被鍦板潃锛夋帹瀵肩綉鎺╃爜銆?
		榛樿锛氫娇鐢ㄨ嚜鍔ㄩ厤缃‘瀹氥€?
  <hostname>	瀹㈡埛绔殑鍚嶇О銆?		濡傛灉瀛樺湪涓€涓?"." 瀛楃锛岀涓€涓?"." 涔嬪墠鐨勫唴瀹圭敤浣滃鎴风鐨勪富鏈哄悕锛屽叾鍚庣殑鍐呭鐢ㄤ綔鍏?NIS 鍩熷悕銆傚彲浠ョ敱鑷姩閰嶇疆鎻愪緵锛屼絾鍏剁己澶变笉浼氳Е鍙戣嚜鍔ㄩ厤缃€?		濡傛灉鎸囧畾浜嗕笖浣跨敤浜?DHCP锛岀敤鎴锋彁渚涚殑 hostname锛堜互鍙?NIS 鍩熷悕锛岃嫢瀛樺湪锛変細琚甫鍏?DHCP 璇锋眰涓紱杩欏彲鑳戒細瀵艰嚧涓哄鎴风鍒涘缓鎴栨洿鏂颁竴鏉?DNS 璁板綍銆?
		榛樿锛氫娇鐢ㄥ鎴风 IP 鍦板潃鐨?ASCII 琛ㄧず銆?
  <device>	瑕佷娇鐢ㄧ殑缃戠粶璁惧鐨勫悕绉般€?		榛樿锛氬鏋滀富鏈哄彧鏈変竴涓澶囷紝鍒欎娇鐢ㄥ畠銆傚惁鍒欓€氳繃鑷姩閰嶇疆鏉ョ‘瀹氳璁惧銆傚仛娉曟槸灏嗚嚜鍔ㄩ厤缃姹備粠鎵€鏈夎澶囧彂鍑猴紝骞朵娇鐢ㄦ敹鍒扮涓€涓簲绛旂殑閭ｄ釜璁惧銆?
  <autoconf>	鐢ㄤ簬鑷姩閰嶇疆鐨勬柟娉曘€?		鍦ㄦ寚瀹氫簡澶氫釜鑷姩閰嶇疆鍗忚鐨勬儏鍐典笅锛屼細浣跨敤鎵€鏈夊崗璁彂閫佽姹傦紝骞朵娇鐢ㄧ涓€涓簲绛旂殑鍗忚銆?
		鍙湁缂栬瘧杩涘唴鏍哥殑鑷姩閰嶇疆鍗忚鎵嶄細琚娇鐢紝鑰屼笌璇ュ弬鏁扮殑鍊兼棤鍏?```

                  off or none: don't use autoconfiguration
				(do static IP assignment instead)
		  on or any:   use any protocol available in the kernel
			       (default)
		  dhcp:        use DHCP
		  bootp:       use BOOTP
		  rarp:        use RARP
		  both:        use both BOOTP and RARP but not DHCP
		               (old option kept for backwards compatibility)

		濡傛灉浣跨敤 dhcp锛屽彲浠ユ寜涓嬭堪鏍煎紡浣跨敤瀹㈡埛绔爣璇嗙 "ip=dhcp,client-id-type,client-id-value"

                Default: any

  <dns0-ip>	涓诲煙鍚嶆湇鍔″櫒鐨?IP 鍦板潃銆?		璇ュ€间細琚鍑哄埌 /proc/net/pnp锛屽墠缂€涓?"nameserver "锛堣涓嬫枃锛夈€?
		榛樿锛氫笉浣跨敤鑷姩閰嶇疆鏃朵负 None锛涗娇鐢ㄨ嚜鍔ㄩ厤缃椂鑷姩纭畾銆?
  <dns1-ip>	杈呭姪鍩熷悕鏈嶅姟鍣ㄧ殑 IP 鍦板潃銆?		鍙傝 <dns0-ip>銆?
  <ntp0-ip>	缃戠粶鏃堕棿鍗忚锛圢TP锛夋湇鍔″櫒鐨?IP 鍦板潃銆?		璇ュ€间細琚鍑哄埌 /proc/net/ipconfig/ntp_servers锛岄櫎姝や箣澶栨湭琚娇鐢紙瑙佷笅鏂囷級銆?
		榛樿锛氫笉浣跨敤鑷姩閰嶇疆鏃朵负 None锛涗娇鐢ㄨ嚜鍔ㄩ厤缃椂鑷姩纭畾銆?
  閰嶇疆瀹屾垚锛堟棤璁烘槸鎵嬪姩杩樻槸鑷姩锛夊悗锛屼細浠ヤ笅鍒楁牸寮忓垱寤轰袱涓枃浠讹紱濡傛灉鐩稿簲鐨勫€煎湪閰嶇疆鍚庝负绌猴紝鍒欑渷鐣ヨ琛岋細

  - /proc/net/pnp:

	#PROTO: <DHCP|BOOTP|RARP|MANUAL>	(鍙栧喅浜庨厤缃柟娉?
	domain <dns-domain>			(鑻ヤ负鑷姩閰嶇疆锛屽垯涓?DNS 鍩熷悕)
	nameserver <dns0-ip>			(涓诲煙鍚嶆湇鍔″櫒 IP)
	nameserver <dns1-ip>			(杈呭姪鍩熷悕鏈嶅姟鍣?IP)
	nameserver <dns2-ip>			(绗笁鍩熷悕鏈嶅姟鍣?IP)
	bootserver <server-ip>			(NFS 鏈嶅姟鍣?IP)

  - /proc/net/ipconfig/ntp_servers:

	<ntp0-ip>				(NTP 鏈嶅姟鍣?IP)
	<ntp1-ip>				(NTP 鏈嶅姟鍣?IP)
	<ntp2-ip>				(NTP 鏈嶅姟鍣?IP)

  <dns-domain> 鍜?<dns2-ip>锛堜綅浜?/proc/net/pnp 涓級浠ュ強 <ntp1-ip> 鍜?<ntp2-ip>锛堜綅浜?/proc/net/ipconfig/ntp_servers 涓級鏄湪鑷姩閰嶇疆鏈熼棿璇锋眰鐨勶紱瀹冧滑涓嶈兘浣滀负 "ip=" 鍐呮牳鍛戒护琛屽弬鏁扮殑涓€閮ㄥ垎鏉ユ寚瀹氥€?
  鐢变簬 "domain" 鍜?"nameserver" 閫夐」浼氳 DNS 瑙ｆ瀽鍣ㄨ瘑鍒紝鍦ㄤ娇鐢?NFS 鏍规枃浠剁郴缁熺殑绯荤粺涓婏紝/etc/resolv.conf 甯稿父閾炬帴鍒?/proc/net/pnp銆?
  娉ㄦ剰锛屽唴鏍镐笉浼氫笌瀹冨彂鐜扮殑浠讳綍 NTP 鏈嶅姟鍣ㄥ悓姝ョ郴缁熸椂闂达紱杩欐槸鐢ㄦ埛绌洪棿杩涚▼鐨勮亴璐ｏ紙渚嬪锛屽湪鎸傝浇鐪熸鐨勬牴鏂囦欢绯荤粺锛堝鏋滃畠鍦?NFS 涓婏級涔嬪墠锛屽皢 /proc/net/ipconfig/ntp_servers 涓垪鍑虹殑 IP 鍦板潃浼犻€掔粰涓€涓?NTP 瀹㈡埛绔殑 initrd/initramfs 鑴氭湰锛夈€?

```
nfsrootdebug
  璇ュ弬鏁颁娇璋冭瘯淇℃伅鍦ㄥ唴鏍稿紩瀵兼椂鍑虹幇鍦ㄥ唴鏍告棩蹇椾腑锛屼互渚跨鐞嗗憳楠岃瘉姝ｇ‘鐨?NFS 鎸傝浇閫夐」銆佹湇鍔″櫒鍦板潃鍜屾牴璺緞琚紶閫掔粰浜?NFS 瀹㈡埛绔€?

rdinit=<executable file>
  涓轰簡鎸囧畾鍖呭惈鍚姩绯荤粺鍒濆鍖栫▼搴忕殑鏂囦欢锛岀鐞嗗憳鍙互浣跨敤杩欎釜鍛戒护琛屽弬鏁般€傝鍙傛暟鐨勯粯璁ゅ€兼槸 "/init"銆傚鏋滄寚瀹氱殑鏂囦欢瀛樺湪涓斿唴鏍歌兘澶熸墽琛屽畠锛屽垯涓庢牴鏂囦欢绯荤粺鐩稿叧鐨勫唴鏍稿懡浠よ鍙傛暟锛堝寘鎷?'nfsroot='锛夐兘浼氳蹇界暐銆?
  鍏充簬鎸傝浇鏍规枃浠剁郴缁熺殑杩囩▼鐨勬弿杩帮紝鍙互鍦?Documentation/driver-api/early-userspace/early_userspace_support.rst 涓壘鍒般€?

## Boot Loader锛堝紩瀵煎姞杞界▼搴忥級

瑕佸皢鍐呮牳杞藉叆鍐呭瓨锛屽彲浠ヤ娇鐢ㄤ笉鍚岀殑鏂规硶銆傚畠浠緷璧栦簬鍚勭鍙敤璁炬柦锛?
- Booting from a floppy using syslinux锛堜娇鐢?syslinux 浠庤蒋鐩樺紩瀵硷級

	鏋勫缓鍐呮牳鏃讹紝鍒涘缓涓€涓娇鐢?syslinux 鐨勫紩瀵艰蒋鐩樼殑涓€涓畝鍗曟柟娉曟槸浣跨敤 zdisk 鎴?bzdisk make 鐩爣锛屽畠浠垎鍒娇鐢?zimage 鍜?bzimage 闀滃儚銆備袱涓洰鏍囬兘鎺ュ彈 FDARGS 鍙傛暟锛屽彲鐢ㄤ簬璁剧疆鍐呮牳鍛戒护琛屻€?```

	   make bzdisk FDARGS="root=/dev/nfs"

   	Note that the user running this command will need to have
     	access to the floppy drive device, /dev/fd0

     	For more information on syslinux, including how to create bootdisks
     	for prebuilt kernels, see https://syslinux.zytor.com/

	.. note::
		Previously it was possible to write a kernel directly to
		a floppy using dd, configure the boot device using rdev, and
		boot using the resulting floppy. Linux no longer supports this
		method of booting.

```
- Booting from a cdrom using isolinux锛堜娇鐢?isolinux 浠庡厜鐩樺紩瀵硷級

     	鏋勫缓鍐呮牳鏃讹紝鍒涘缓涓€涓娇鐢?isolinux 鐨勫彲寮曞鍏夌洏鐨勪竴涓畝鍗曟柟娉曟槸浣跨敤 isoimage 鐩爣锛屽畠浣跨敤 bzimage 闀滃儚銆備笌 zdisk 鍜?bzdisk 绫讳技锛岃鐩爣鎺ュ彈 FDARGS 鍙傛暟锛屽彲鐢ㄤ簬璁剧疆鍐呮牳鍛戒护琛屻€?```

	  make isoimage FDARGS="root=/dev/nfs"

     	The resulting iso image will be arch/<ARCH>/boot/image.iso
     	This can be written to a cdrom using a variety of tools including
     	cdrecord.

	e.g::

	  cdrecord dev=ATAPI:1,0,0 arch/x86/boot/image.iso

     	For more information on isolinux, including how to create bootdisks
     	for prebuilt kernels, see https://syslinux.zytor.com/

```
- Using LILO锛堜娇鐢?LILO锛?
	浣跨敤 LILO 鏃讹紝鎵€鏈夊繀瑕佺殑鍛戒护琛屽弬鏁伴兘鍙互浣跨敤 LILO 閰嶇疆鏂囦欢涓殑 'append=' 鎸囦护鏉ユ寚瀹氥€?
	涓嶈繃锛岃浣跨敤 'root=' 鎸囦护锛屾偍杩橀渶瑕佸垱寤轰竴涓櫄鎷熺殑鏍硅澶囷紝瀹冨彲浠ュ湪 LILO 杩愯鍚庤绉婚櫎銆?```

	  mknod /dev/boot255 c 0 255

	鍏充簬閰嶇疆 LILO 鐨勪俊鎭紝璇峰弬鑰冨叾鏂囨。銆?```

- Using GRUB锛堜娇鐢?GRUB锛?
	浣跨敤 GRUB 鏃讹紝鍐呮牳鍙傛暟鍙渶闄勫姞鍦ㄥ唴鏍歌鏄庝箣鍚庯細kernel <kernel> <parameters>

- Using loadlin锛堜娇鐢?loadlin锛?
	loadlin 鍙敤浜庝粠 DOS 鍛戒护鎻愮ず绗﹀紩瀵?Linux锛岃€屾棤闇€鏈湴纭洏浣滀负鏍规寕杞姐€傛湰鏂囨。鐨勪綔鑰呮病鏈夊鍏惰繘琛?thorough 娴嬭瘯锛屼絾涓€鑸€岃█锛屽簲褰撳彲浠ョ被浼间簬 LILO 鐨勯厤缃柟寮忔潵閰嶇疆鍐呮牳鍛戒护琛屻€?
	鏇村淇℃伅璇峰弬鑰?loadlin 鏂囨。銆?
- Using a boot ROM锛堜娇鐢ㄥ紩瀵?ROM锛?
	杩欏彲鑳芥槸寮曞鏃犵洏瀹㈡埛绔渶浼橀泤鐨勬柟寮忋€傚埄鐢ㄥ紩瀵?ROM锛屽唴鏍搁€氳繃 TFTP 鍗忚鍔犺浇銆傛湰鏂囨。鐨勪綔鑰呬笉鐭ラ亾鏈変换浣曞晢涓氬紩瀵?ROM 鏀寔閫氳繃缃戠粶寮曞 Linux銆備笉杩囷紝鏈変袱涓嚜鐢辩殑寮曞 ROM 瀹炵幇锛宯etboot-nfs 鍜?etherboot锛屼簩鑰呴兘鍙湪 sunsite.unc.edu 涓婅幏寰楋紝涓旈兘鍖呭惈寮曞鏃犵洏 Linux 瀹㈡埛绔墍闇€鐨勪竴鍒囥€?
- Using pxelinux锛堜娇鐢?pxelinux锛?
	Pxelinux 鍙敤浜庡埄鐢ㄨ澶氱幇浠ｇ綉鍗′笂瀛樺湪鐨?PXE 寮曞鍔犺浇绋嬪簭鏉ュ紩瀵?Linux銆?
	浣跨敤 pxelinux 鏃讹紝鍐呮牳闀滃儚閫氳繃 "kernel <relative-path-below /tftpboot>" 鎸囧畾銆俷fsroot 鍙傛暟閫氳繃灏嗗畠浠坊鍔犲埌 "append" 琛屾潵浼犻€掔粰鍐呮牳銆傞€氬父浼氶厤鍚堜娇鐢ㄤ覆鍙ｆ帶鍒跺彴涓?pxelinux锛屾洿澶氫俊鎭弬瑙?Documentation/admin-guide/serial-console.rst銆?
	鍏充簬 isolinux 鐨勬洿澶氫俊鎭紝鍖呮嫭濡備綍涓洪鏋勫缓鍐呮牳鍒涘缓寮曞鐩橈紝璇峰弬瑙?https://syslinux.zytor.com/



## Credits锛堣嚧璋級

 鍐呮牳涓殑 nfsroot 浠ｇ爜浠ュ強 RARP 鏀寔鐢?Gero Kuhlmann <gero@gkminix.han.de> 缂栧啓銆?
  鍏朵綑鐨?IP 灞傝嚜鍔ㄩ厤缃唬鐮佺敱 Martin Mares <mj@atrey.karlin.mff.cuni.cz> 缂栧啓銆?
  涓轰簡缂栧啓 nfsroot 鐨勫垵濮嬬増鏈紝鎴戣鎰熻阿 Jens-Uwe Mager <jum@anubis.han.de> 鐨勫府鍔┿€?