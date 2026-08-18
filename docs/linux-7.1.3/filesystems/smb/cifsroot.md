
## 閫氳繃 SMB 鎸傝浇鏍规枃浠剁郴缁燂紙cifs.ko锛?

Written 2019 by Paulo Alcantara <palcantara@suse.de>

Written 2019 by Aurelien Aptel <aaptel@suse.com>

CONFIG_CIFS_ROOT 閫夐」閫氳繃 cifs.ko 鍚敤鍩轰簬 SMB 鍗忚鐨勬牴鏂囦欢绯荤粺瀹為獙鎬ф敮鎸併€?
瀹冨紩鍏ヤ簡涓€涓悕涓?'cifsroot=' 鐨勬柊鍐呮牳鍛戒护琛岄€夐」锛岀敤浜庡憡璇夊唴鏍搁€氳繃缃戠粶鍒╃敤 SMB 鎴?CIFS 鍗忚鎸傝浇鏍规枃浠剁郴缁熴€?
涓轰簡杩涜鎸傝浇锛岃繕闇€瑕佷娇鐢?'ip=' 閰嶇疆閫夐」鏉ュ缓绔嬬綉缁滄爤銆傛洿澶氱粏鑺傦紝璇峰弬闃?Documentation/admin-guide/nfs/nfsroot.rst銆?
CIFS 鏍规寕杞界洰鍓嶉渶瑕佷娇鐢?SMB1+UNIX 鎵╁睍锛岃鎵╁睍浠呯敱 Samba 鏈嶅姟鍣ㄦ敮鎸併€係MB1 鏄鍗忚鐨勮緝鏃т笖宸茶寮冪敤鐨勭増鏈紝浣嗗畠宸茶鎵╁睍浠ユ敮鎸?POSIX 鐗规€э紙鍙傝 [^1^]锛夈€傛柊鐗堟湰锛堟帹鑽愮殑鍗忚鐗堟湰 SMB3锛夌殑绛夋晥鎵╁睍灏氭湭瀹屽叏瀹炵幇锛岃繖鎰忓懗鐫€ SMB3 涓嶆敮鎸佹煇浜涘繀闇€鐨?POSIX 鏂囦欢绯荤粺瀵硅薄锛堜緥濡傚潡璁惧銆佺閬撱€佸鎺ュ瓧锛夈€?
鍥犳锛孋IFS 鏍圭洰鍓嶉粯璁や娇鐢?SMB1锛屼絾鎵€浣跨敤鐨勭増鏈粛鍙€氳繃 'vers=' 鎸傝浇閫夐」鏇存敼銆備竴鏃?SMB3 POSIX 鎵╁睍瀹屽叏瀹炵幇锛岃榛樿鍊煎皢浼氭敼鍙樸€?
## 鏈嶅姟鍣ㄩ厤缃?

瑕佸惎鐢?SMB1+UNIX 鎵╁睍锛屼綘闇€瑕佽缃繖浜涘叏灞€
```

    [global]
    server min protocol = NT1
    unix extension = yes        # default

```
## 鍐呮牳鍛戒护琛?

```

    root=/dev/cifs

```
杩欏彧鏄竴涓櫄鎷熻澶囷紝鍩烘湰涓婂憡璇夊唴鏍搁€氳繃 SMB 鍗忚鎸傝浇鏍规枃浠剁郴缁熴€?
```

    cifsroot=//<server-ip>/<share>[,options]

```
浣垮唴鏍歌兘澶熸寕杞戒綅浜庢湰閫夐」涓寚瀹氱殑 <server-ip> 鍜?<share> 涓€侀€氳繃 SMB 鎻愪緵鐨勬牴鏂囦欢绯荤粺銆?
榛樿鎸傝浇閫夐」璁剧疆鍦?fs/smb/client/cifsroot.c 涓€?
server-ip
	鏈嶅姟鍣ㄧ殑 IPv4 鍦板潃銆?
share
	SMB 鍏变韩锛坮ootfs锛夌殑璺緞銆?
options
	鍙€夌殑鎸傝浇閫夐」銆傛洿澶氫俊鎭紝璇峰弬闃?mount.cifs(8)銆?
## 绀轰緥


```

    ...
    [linux]
	    path = /path/to/rootfs
	    read only = no
	    guest ok = yes
	    force user = root
	    force group = root
	    browseable = yes
	    writeable = yes
	    admin users = root
	    public = yes
	    create mask = 0777
	    directory mask = 0777
    ...

```
```

    # systemctl restart smb

```
鍦ㄥ惎鐢ㄤ簡 CONFIG_CIFS_ROOT 鐨勫唴鏍镐笅浣跨敤 QEMU 杩涜娴嬭瘯锛屼互鍙?```

    # qemu-system-x86_64 -enable-kvm -cpu host -m 1024 \
    -kernel /path/to/linux/arch/x86/boot/bzImage -nographic \
    -append "root=/dev/cifs rw ip=dhcp cifsroot=//10.0.2.2/linux,username=foo,password=bar console=ttyS0 3"


```
1: https://wiki.samba.org/index.php/UNIX_Extensions
