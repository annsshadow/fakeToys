
## Landlock锛氱郴缁熺骇绠＄悊


:Author: Micka毛l Sala眉n
:Date: January 2026

Landlock 鍙互鍒╃敤瀹¤锛坅udit锛夋鏋舵潵璁板綍浜嬩欢銆?

鐢ㄦ埛绌洪棿鏂囨。鍙湪姝ゅ鎵惧埌锛欴ocumentation/userspace-api/landlock.rst銆?

## 瀹¤


濡傛灉鍚敤浜?`audit`锛岄偅涔堣娌欑鍖栫殑绋嬪簭鎵€鍙戣捣鐨勮鎷掔粷璁块棶璇锋眰浼氶粯璁よ褰曟棩蹇椼€?
杩欎竴榛樿琛屼负鍙互閫氳繃 sys_landlock_restrict_self() 鐨勬爣蹇楁潵鏇存敼锛堝弬瑙?
Documentation/userspace-api/landlock.rst锛夈€侺andlock 鏃ュ織涔熷彲浠ラ€氳繃瀹¤瑙勫垯鏉?
灞忚斀銆侺andlock 鍙互鐢熸垚 2 绉嶅璁¤褰曠被鍨嬨€?

### 璁板綍绫诲瀷


AUDIT_LANDLOCK_ACCESS
    璇ヨ褰曠被鍨嬫爣璇嗕竴娆″琚嫆缁濈殑鍐呮牳璧勬簮璁块棶璇锋眰銆俙domain` 瀛楁鎸囩ず闃绘浜嗚
    璇锋眰鐨勫煙锛坉omain锛夌殑 ID銆俙blockers` 瀛楁鎸囩ず璇ユ嫆缁濈殑鍘熷洜锛堜互閫楀彿鍒嗛殧锛夛紝
    鍏朵綑瀛楁鏍囪瘑鍐呮牳瀵硅薄锛堢被浼间簬 SELinux锛夈€傛瘡涓璁′簨浠朵腑鍙兘鏈変笉姝竴鏉¤
    绫诲瀷鐨勮褰曘€?

```

        domain=195ba459b blockers=fs.refer path="/usr/bin" dev="vda2" ino=351
        domain=195ba459b blockers=fs.make_reg,fs.refer path="/usr/local" dev="vda2" ino=365


    ``blockers`` 瀛楁浣跨敤浠ュ彞鐐瑰垎闅旂殑鍓嶇紑鏉ヨ〃绀哄鑷存嫆缁濈殑闄愬埗绫诲瀷锛?

    **fs.*** - 鏂囦欢绯荤粺璁块棶鏉冮檺锛圓BI 1+锛夛細
        - fs.execute銆乫s.write_file銆乫s.read_file銆乫s.read_dir
        - fs.remove_dir銆乫s.remove_file
        - fs.make_char銆乫s.make_dir銆乫s.make_reg銆乫s.make_sock
        - fs.make_fifo銆乫s.make_block銆乫s.make_sym
        - fs.refer锛圓BI 2+锛?
        - fs.truncate锛圓BI 3+锛?
        - fs.ioctl_dev锛圓BI 5+锛?

    **net.*** - 缃戠粶璁块棶鏉冮檺锛圓BI 4+锛夛細
        - net.bind_tcp - TCP 绔彛缁戝畾琚嫆缁?
        - net.connect_tcp - TCP 杩炴帴琚嫆缁?

    **scope.*** - IPC 鑼冨洿闄愬埗锛圓BI 6+锛夛細
        - scope.abstract_unix_socket - 鎶借薄 UNIX 濂楁帴瀛楄繛鎺ヨ鎷掔粷
        - scope.signal - 淇″彿鍙戦€佽鎷掔粷

    褰撶己灏戝椤硅闂潈闄愭椂锛屽涓?blockers 鍙兘鍑虹幇鍦ㄥ悓涓€鏉′簨浠朵腑锛堜互閫楀彿鍒嗛殧锛夈€?
    渚嬪锛屽湪涓€涓棦缂哄皯 ``make_reg`` 鍙堢己灏?``refer`` 鏉冮檺鐨勭洰褰曚腑鍒涘缓鏅€氭枃浠讹紝
    浼氭樉绀?``blockers=fs.make_reg,fs.refer``銆?

    瀵硅薄鏍囪瘑瀛楁锛堟枃浠剁郴缁熷搴旂殑鏄?path銆乨ev銆乮no锛涗俊鍙峰搴旂殑鏄?opid銆乷comm锛?
    鍙栧喅浜庤闃绘鐨勮闂被鍨嬶紝骞舵彁渚涘叧浜庢嫆缁濇墍娑夊強璧勬簮鐨勪笂涓嬫枃銆?

```
AUDIT_LANDLOCK_DOMAIN
    璇ヨ褰曠被鍨嬫弿杩颁竴涓?Landlock 鍩熺殑鐘舵€併€俙status` 瀛楁鍙互鏄?`allocated` 鎴?
    `deallocated`銆?

    `allocated` 鐘舵€佸睘浜庡悓涓€涓璁′簨浠剁殑涓€閮ㄥ垎锛屽苟璺熷湪鏌愪釜鍩熼娆¤褰曠殑
    `AUDIT_LANDLOCK_ACCESS` 璁板綍涔嬪悗銆傚畠鏍囪瘑鍦ㄨ皟鐢?sys_landlock_restrict_self()
    鏃惰 Landlock 鍩熺殑淇℃伅锛屽寘鍚互涓嬪瓧娈碉細

    - 鍩燂紙domain锛塈D
    - 寮哄埗锛坋nforcement锛塦mode`
    - 鍩熷垱寤鸿€呯殑 `pid`
    - 鍩熷垱寤鸿€呯殑 `uid`
    - 鍩熷垱寤鸿€呯殑鍙墽琛屾枃浠惰矾寰勶紙`exe`锛?
    - 鍩熷垱寤鸿€呯殑鍛戒护琛岋紙`comm`锛?

```

        domain=195ba459b status=allocated mode=enforcing pid=300 uid=0 exe="/root/sandboxer" comm="sandboxer"

    ``deallocated`` 鐘舵€佹槸涓€涓嫭绔嬬殑浜嬩欢锛屽畠鏍囪瘑涓€娆?Landlock 鍩熺殑閲婃斁銆傚湪姝?
    浜嬩欢涔嬪悗锛屽彲浠ヤ繚璇佸湪绯荤粺鐢熷懡鍛ㄦ湡鍐呯浉鍏冲煙 ID 缁濅笉浼氳澶嶇敤銆俙`domain`` 瀛楁
    鎸囩ず琚噴鏀惧煙鐨?ID锛宍`denials`` 瀛楁鎸囩ず琚嫆缁濊闂姹傜殑鎬绘暟锛屽叾涓儴鍒嗚姹?
    鍙兘鐢变簬瀹¤瑙勫垯鍜?sys_landlock_restrict_self() 鐨勬爣蹇楄€屾湭琚褰曘€?

    Example::

        domain=195ba459b status=deallocated denials=3

```
### 浜嬩欢绀轰緥


涓嬮潰鏄袱涓棩蹇楄褰曚簨浠剁殑绀轰緥锛堝弬瑙佸簭鍒楀彿锛夈€?

鍦ㄦ绀轰緥涓紝涓€涓娌欑鍖栫殑绋嬪簭锛坄kill`锛夎瘯鍥惧悜 init 杩涚▼鍙戦€佷俊鍙凤紝璇ヨ姹傚洜
淇″彿鑼冨洿闄愬埗鑰岃鎷掔粷銆?
```

  $ LL_FS_RO=/ LL_FS_RW=/ LL_SCOPED=s LL_FORCE_LOG=1 ./sandboxer kill 1

```
璇ュ懡浠ょ敓鎴愪袱涓簨浠讹紝姣忎釜浜嬩欢閮藉甫鏈変竴涓窡闅忔椂闂存埑鐨勫敮涓€搴忓垪鍙?
锛坄msg=audit(1729738800.268:30)`锛夈€傜涓€涓簨浠讹紙搴忓垪鍙?`30`锛夊寘鍚?4 鏉¤褰曘€?
绗竴鏉¤褰曪紙`type=LANDLOCK_ACCESS`锛夋樉绀轰竴涓鍩?`1a6fdc66f` 鎷掔粷鐨勮闂€?
璇ユ嫆缁濈殑鍘熷洜鏄俊鍙疯寖鍥撮檺鍒讹紙`blockers=scope.signal`锛夈€傛湰灏嗘帴鏀惰淇″彿鐨勮繘绋嬫槸
init 杩涚▼锛坄opid=1 ocomm="systemd"`锛夈€?

绗簩鏉¤褰曪紙`type=LANDLOCK_DOMAIN`锛夋弿杩帮紙`status=allocated`锛夊煙 `1a6fdc66f`銆?
璇ュ煙鐢辫繘绋?`286` 鎵ц root 鐢ㄦ埛鍚姩鐨?`/root/sandboxer` 绋嬪簭鎵€鍒涘缓銆?

绗笁鏉¤褰曪紙`type=SYSCALL`锛夋弿杩拌 syscall銆佸叾鎻愪緵鐨勫弬鏁般€佸叾缁撴灉
锛坄success=no exit=-1`锛変互鍙婅皟鐢ㄥ畠鐨勮繘绋嬨€?

绗洓鏉¤褰曪紙`type=PROCTITLE`锛変互鍗佸叚杩涘埗鍊兼樉绀哄懡浠ゅ悕銆傚彲浠ョ敤
``python -c 'print(bytes.fromhex("6B696C6C0031"))'`` 鏉ヨ浆鎹㈠畠銆?

鏈€鍚庯紝鏈€鍚庝竴鏉¤褰曪紙`type=LANDLOCK_DOMAIN`锛変篃鏄浜屼釜浜嬩欢锛堝簭鍒楀彿 `31`锛変腑
鍞竴鐨勮褰曘€傚畠骞朵笉瀵瑰簲浜庢煇涓洿鎺ョ殑鐢ㄦ埛绌洪棿鍔ㄤ綔锛岃€屾槸涓€涓紓姝ュ姩浣滐紝鐢ㄤ簬閲婃斁
涓庢煇涓?Landlock 鍩熺浉鍏崇殑璧勬簮锛坄status=deallocated`锛夈€傝繖鏈夊姪浜庝簡瑙ｅ悗缁棩蹇?
灏嗕笉鍐嶆秹鍙婂煙 `1a6fdc66f`銆傝璁板綍杩樻眹鎬讳簡璇ュ煙鎷掔粷鐨勮姹傛暟閲忥紙`denials=1`锛夛紝
鏃犺瀹冧滑鏄惁琚褰曘€?

```
  type=LANDLOCK_ACCESS msg=audit(1729738800.268:30): domain=1a6fdc66f blockers=scope.signal opid=1 ocomm="systemd"
  type=LANDLOCK_DOMAIN msg=audit(1729738800.268:30): domain=1a6fdc66f status=allocated mode=enforcing pid=286 uid=0 exe="/root/sandboxer" comm="sandboxer"
  type=SYSCALL msg=audit(1729738800.268:30): arch=c000003e syscall=62 success=no exit=-1 [..] ppid=272 pid=286 auid=0 uid=0 gid=0 [...] comm="kill" [...]
  type=PROCTITLE msg=audit(1729738800.268:30): proctitle=6B696C6C0031
  type=LANDLOCK_DOMAIN msg=audit(1729738800.324:31): domain=1a6fdc66f status=deallocated denials=1

```

  $ LL_FS_RO=/ LL_FS_RW=/tmp LL_FORCE_LOG=1 ./sandboxer sh -c "echo > /etc/passwd"

```
鐩稿叧鐨勫璁℃棩蹇楀寘鍚潵鑷?3 涓笉鍚屼簨浠讹紙搴忓垪鍙?33銆?
```

  type=LANDLOCK_ACCESS msg=audit(1729738800.221:33): domain=1a6fdc679 blockers=fs.write_file path="/dev/tty" dev="devtmpfs" ino=9
  type=LANDLOCK_DOMAIN msg=audit(1729738800.221:33): domain=1a6fdc679 status=allocated mode=enforcing pid=289 uid=0 exe="/root/sandboxer" comm="sandboxer"
  type=SYSCALL msg=audit(1729738800.221:33): arch=c000003e syscall=257 success=no exit=-13 [...] ppid=272 pid=289 auid=0 uid=0 gid=0 [...] comm="sh" [...]
  type=PROCTITLE msg=audit(1729738800.221:33): proctitle=7368002D63006563686F203E202F6574632F706173737764
  type=LANDLOCK_ACCESS msg=audit(1729738800.221:34): domain=1a6fdc679 blockers=fs.write_file path="/etc/passwd" dev="vda2" ino=143821
  type=SYSCALL msg=audit(1729738800.221:34): arch=c000003e syscall=257 success=no exit=-13 [...] ppid=272 pid=289 auid=0 uid=0 gid=0 [...] comm="sh" [...]
  type=PROCTITLE msg=audit(1729738800.221:34): proctitle=7368002D63006563686F203E202F6574632F706173737764
  type=LANDLOCK_DOMAIN msg=audit(1729738800.261:35): domain=1a6fdc679 status=deallocated denials=2

### 浜嬩欢杩囨护


濡傛灉浣犺涓?Landlock 鐩稿叧鐨勫璁℃棩蹇楀埛灞忥紝杩欒涔堟槸涓€娆℃敾鍑诲皾璇曪紝瑕佷箞鏄畨鍏?
绛栫暐涓殑 bug銆傛垜浠彲浠ラ€氳繃涓ょ浜掕ˉ鐨勬柟寮忔潵璁剧疆涓€浜涜繃婊ゅ櫒浠ラ檺鍒跺櫔闊筹細

- 濡傛灉鎴戜滑鑳戒慨澶嶈娌欑鍖栫殑绋嬪簭锛屽彲浠ヤ娇鐢?sys_landlock_restrict_self() 鐨勬爣蹇楋紱
- 鎴栦娇鐢ㄥ璁¤鍒欙紙鍙傝 `auditctl(8)`锛夈€?

## 琛ュ厖鏂囨。


- `Linux Audit Documentation`_
- Documentation/userspace-api/landlock.rst
- Documentation/security/landlock.rst
- https://landlock.io

   https://github.com/linux-audit/audit-documentation/wiki
