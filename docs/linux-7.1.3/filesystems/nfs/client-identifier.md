
## NFSv4 client identifier


姝?document explains 濡備綍 the NFSv4 鍗忚 identifies client
instances 涓轰簡 maintain 鏂囦欢 鎵撳紑 鍜?閿?鐘舵€?鏈熼棿
绯荤粺 restarts. 涓€涓?鐗规畩 identifier 鍜?principal 鏄?maintained
鍦?姣忎釜 client. 杩欎簺 鍙?涓?set 鐢?administrators, scripts
provided 鐢?site administrators, 鎴?tools provided 鐢?Linux
distributors.

瀛樺湪 risks 鑻?涓€涓?client's NFSv4 identifier 鍜?鍏?principal
鏄?涓?chosen carefully.


### Introduction


The NFSv4 鍗忚 uses "lease-based 鏂囦欢 locking". Leases help
NFSv4 servers 鎻愪緵 鏂囦欢 閿?guarantees 鍜?manage 瀹冧滑鐨?
resources.

Simply put, 涓€涓?NFSv4 server creates 涓€涓?lease 鐢ㄤ簬 姣忎釜 NFSv4 client.
The server collects 姣忎釜 client's 鏂囦欢 鎵撳紑 鍜?閿?鐘舵€?鍦ㄢ€︿笅
the lease 鐢ㄤ簬 璇?client.

The client 鏄?responsible 鐢ㄤ簬 periodically renewing 鍏?leases.
鍚屾椂 涓€涓?lease remains valid, the server holding 璇?lease
guarantees the 鏂囦欢 閿?the client 鍏锋湁 宸插垱寤?remain 鍦?place.

鑻?涓€涓?client stops renewing 鍏?lease (渚嬪, 鑻?瀹?crashes),
the NFSv4 鍗忚 allows the server 鍒?remove the client's 鎵撳紑
鍜?閿?鐘舵€?涔嬪悗 涓€涓?鏌愪簺 period 鐨?time. 褰?涓€涓?client
restarts, 瀹?indicates 鍒?servers 璇?鎵撳紑 鍜?閿?鐘舵€?
associated 涓?鍏?鍓嶄竴涓?leases 鏄?鏃?longer valid 鍜?鍙?涓?
destroyed immediately.

姝ゅ, 姣忎釜 NFSv4 server manages 涓€涓?persistent 鍒楀嚭 鐨?client
leases. 褰?the server restarts 鍜?clients attempt 鍒?recover
瀹冧滑鐨?鐘舵€? the server uses 姝?鍒楀嚭 鍒?distinguish amongst
clients 璇?held 鐘舵€?涔嬪墠 the server restarted 鍜?clients
sending fresh 鎵撳紑 鍜?閿?requests. 姝?enables 鏂囦欢 閿?鍒?
persist safely across server restarts.

### NFSv4 client identifiers


姣忎釜 NFSv4 client presents 涓€涓?identifier 鍒?NFSv4 servers 鍥犳 璇?
瀹冧滑 鍙?associate the client 涓?鍏?lease. 姣忎釜 client's
identifier consists 鐨?two elements:

  - co_ownerid: 涓€涓?arbitrary 浣?fixed 瀛楃涓?

  - boot verifier: 涓€涓?64-浣?incarnation verifier 璇?enables 涓€涓?
    server 鍒?distinguish successive boot epochs 鐨?the 鐩稿悓 client.

The NFSv4.0 specification refers 鍒?杩欎簺 two items 浣滀负 涓€涓?
"nfs_client_id4". The NFSv4.1 specification refers 鍒?杩欎簺 two
items 浣滀负 涓€涓?"client_owner4".

NFSv4 servers tie 姝?identifier 鍒?the principal 鍜?瀹夊叏
flavor 璇?the client 浣跨敤 褰?presenting 瀹? Servers 浣跨敤 姝?
principal 鍒?authorize 鍚庣画 lease modification 鎿嶄綔
sent 鐢?the client. Effectively 姝?principal 鏄?涓€涓?third element 鐨?
the identifier.

浣滀负 part 鐨?the identity presented 鍒?servers, 涓€涓?good
"co_ownerid" 瀛楃涓?鍏锋湁 鑻ュ共 閲嶈 properties:

  - The "co_ownerid" 瀛楃涓?identifies the client 鏈熼棿 reboot
    recovery, 鍥犳 the 瀛楃涓?鏄?persistent across client
    reboots.
  - The "co_ownerid" 瀛楃涓?helps servers distinguish the client
    鏉ヨ嚜 others, 鍥犳 the 瀛楃涓?鏄?globally unique. 娉ㄦ剰
    璇?瀛樺湪 鏃?central authority 璇?assigns "co_ownerid"
    strings.
  - 鍥犱负 瀹?閫氬父 appears 鍦?the 缃戠粶 鍦?the clear, the
    "co_ownerid" 瀛楃涓?鎵ц 涓?reveal 绉佹湁 information 鍏充簬
    the client itself.
  - The content 鐨?the "co_ownerid" 瀛楃涓?鏄?set 鍜?unchanging
    涔嬪墠 the client attempts NFSv4 mounts 涔嬪悗 涓€涓?restart.
  - The NFSv4 鍗忚 places 涓€涓?1024-byte limit 鍦?the 澶у皬 鐨?the
    "co_ownerid" 瀛楃涓?

### Protecting NFSv4 lease 鐘舵€?


NFSv4 servers utilize the "client_owner4" 浣滀负 鎻忚堪 涓婃枃 鍒?
assign 涓€涓?unique lease 鍒?姣忎釜 client. 鍦ㄢ€︿笅 姝?scheme, 瀛樺湪
circumstances 浣曞 clients 鍙?interfere 涓?姣忎釜 鍏朵粬. 杩欐槸
referred 鍒?浣滀负 "lease stealing".

鑻?distinct clients present the 鐩稿悓 "co_ownerid" 瀛楃涓?鍜?浣跨敤
the 鐩稿悓 principal (渚嬪, AUTH_SYS 鍜?UID 0), 涓€涓?server 鏄?
unable 鍒?tell 璇?the clients 鏄?涓?the 鐩稿悓. 姣忎釜 distinct
client presents 涓€涓?涓嶅悓 boot verifier, 鍥犳 瀹?appears 鍒?the
server 浣滀负 鑻?瀛樺湪 one client 鍗?rebooting frequently.
涓よ€呴兘涓?client 鍙?maintain 鎵撳紑 鎴?閿?鐘舵€?鍦?姝?scenario.

鑻?distinct clients present the 鐩稿悓 "co_ownerid" 瀛楃涓?鍜?浣跨敤
distinct principals, the server 鏄?likely 鍒?鍏佽 the 绗竴 client
鍒?operate normally 浣?reject 鍚庣画 clients 涓?the 鐩稿悓
"co_ownerid" 瀛楃涓?

鑻?涓€涓?client's "co_ownerid" 瀛楃涓?鎴?principal 鏄?涓?stable,
鐘舵€?recovery 涔嬪悗 涓€涓?server 鎴?client reboot 鏄?涓?guaranteed.
鑻?涓€涓?client unexpectedly restarts 浣?presents 涓€涓?涓嶅悓
"co_ownerid" 瀛楃涓?鎴?principal 鍒?the server, the server orphans
the client's 鍓嶄竴涓?鎵撳紑 鍜?閿?鐘舵€? 姝?鍧?access 鍒?
locked 鏂囦欢 鐩村埌 the server removes the orphaned 鐘舵€?

鑻?the server restarts 鍜?涓€涓?client presents 涓€涓?changed "co_ownerid"
瀛楃涓?鎴?principal 鍒?the server, the server 灏?涓?鍏佽 the
client 鍒?reclaim 鍏?鎵撳紑 鍜?閿?鐘舵€? 鍜?鍙?give 閭ｄ簺 閿?
鍒?鍏朵粬 clients 鍦?the meantime. 杩欐槸 referred 鍒?浣滀负 "閿?
stealing".

Lease stealing 鍜?閿?stealing increase the potential 鐢ㄤ簬 denial
鐨?service 鍜?鍦?rare cases even 鏁版嵁 corruption.

### Selecting 涓€涓?appropriate client identifier


榛樿鎯呭喌涓? the Linux NFSv4 client implementation constructs 鍏?
"co_ownerid" 瀛楃涓?starting 涓?the words "Linux NFS" followed 鐢?
the client's UTS node name (the 鐩稿悓 node name, incidentally, 璇?
鏄?浣跨敤 浣滀负 the "machine name" 鍦?涓€涓?AUTH_SYS credential). 鍦?small
deployments, 姝?construction 鏄?閫氬父 adequate. 閫氬父, 鐒惰€?
the node name 鐢?itself 鏄?涓?adequately unique, 鍜?鍙?change
unexpectedly. Problematic situations 鍖呭惈:

  - NFS-root (diskless) clients, 浣曞 the 鏈湴 DHCP server (鎴?
    equivalent) 鎵ц 涓?鎻愪緵 涓€涓?unique host name.

  - "Containers" 涔嬪唴 涓€涓?鍗曚釜 Linux host.  鑻?姣忎釜 container 鍏锋湁
    涓€涓?separate 缃戠粶 namespace, 浣?鎵ц 涓?浣跨敤 the UTS namespace
    鍒?鎻愪緵 涓€涓?unique host name, 鐒跺悗 閭ｉ噷 鍙?涓?澶氫釜 NFS
    client instances 涓?the 鐩稿悓 host name.

  - Clients across 澶氫釜 administrative domains 璇?access 涓€涓?
    閫氱敤 NFS server. 鑻?hostnames 鏄?涓?assigned centrally
    鐒跺悗 uniqueness cannot 涓?guaranteed 闄ら潪 涓€涓?domain name 鏄?
    included 鍦?the hostname.

Linux 鎻愪緵 two mechanisms 鍒?add uniqueness 鍒?鍏?"co_ownerid"
瀛楃涓?

    nfs.nfs4_unique_id
      姝?妯″潡 鍙傛暟 鍙?set 涓€涓?arbitrary uniquifier 瀛楃涓?
      閫氳繃 the 鍐呮牳 鍛戒护 line, 鎴?褰?the "nfs" 妯″潡 鏄?
      loaded.

    /sys/fs/nfs/net/nfs_client/identifier
      姝?铏氭嫙 鏂囦欢, 鍙敤 since Linux 5.3, 鏄?鏈湴 鍒?the
      缃戠粶 namespace 鍦?鍏?瀹冩槸 accessed 鍜?鍥犳 鍙?鎻愪緵
      distinction 涔嬮棿 缃戠粶 namespaces (containers) 褰?the
      hostname remains uniform.

娉ㄦ剰 璇?姝?鏂囦欢 鏄?empty 鍦?name-space creation. 鑻?the
container 绯荤粺 鍏锋湁 access 鍒?涓€浜?sort 鐨?per-container identity
鐒跺悗 璇?uniquifier 鍙?涓?浣跨敤. 渚嬪, 涓€涓?uniquifier 鍙兘
涓?formed 鍦?boot 浣跨敤 the container's 鍐呴儴 identifier:

    sha256sum /绛?machine-id | awk '{print $1}' \\
        > /sys/fs/nfs/net/nfs_client/identifier

### 瀹夊叏 considerations


The 浣跨敤 鐨?cryptographic 瀹夊叏 鐢ㄤ簬 lease 绠＄悊 鎿嶄綔
鏄?strongly encouraged.

鑻?NFS 涓?Kerberos 鏄?涓?configured, 涓€涓?Linux NFSv4 client uses
AUTH_SYS 鍜?UID 0 浣滀负 the principal part 鐨?鍏?client identity.
姝?閰嶇疆 鏄?涓?浠?insecure, 瀹?increases the risk 鐨?
lease 鍜?閿?stealing. 鐒惰€? 瀹?鍙兘 涓?the 浠?choice 鐢ㄤ簬
client configurations 璇?鍏锋湁 鏃?鏈湴 persistent storage.
"co_ownerid" 瀛楃涓?uniqueness 鍜?persistence 鏄?critical 鍦?姝?
case.

褰?涓€涓?Kerberos keytab 鏄?present 鍦?涓€涓?Linux NFS client, the client
attempts 鍒?浣跨敤 one 鐨?the principals 鍦?璇?keytab 褰?
identifying itself 鍒?servers. The "sec=" mount 閫夐」 鎵ц 涓?
control 姝?behavior. Alternately, 涓€涓?single-user client 涓?涓€涓?
Kerberos principal 鍙?浣跨敤 璇?principal 鍦?place 鐨?the client's
host principal.

浣跨敤 Kerberos 鐢ㄤ簬 姝?purpose enables the client 鍜?server 鍒?
浣跨敤 the 鐩稿悓 lease 鐢ㄤ簬 鎿嶄綔 covered 鐢?鍏ㄩ儴 "sec=" 璁剧疆.
Additionally, the Linux NFS client uses the RPCSEC_GSS 瀹夊叏
flavor 涓?Kerberos 鍜?the integrity QOS 鍒?prevent in-transit
modification 鐨?lease modification requests.

### 棰濆 notes

The Linux NFSv4 client establishes 涓€涓?鍗曚釜 lease 鍦?姣忎釜 NFSv4
server 瀹?accesses. NFSv4 mounts 鏉ヨ嚜 涓€涓?Linux NFSv4 client 鐨?涓€涓?
鐗瑰畾 server 鐒跺悗 share 璇?lease.

涓€鏃?涓€涓?client establishes 鎵撳紑 鍜?閿?鐘舵€? the NFSv4 鍗忚
enables lease 鐘舵€?鍒?transition 鍒?鍏朵粬 servers, 浠ヤ笅 鏁版嵁
璇?鍏锋湁 宸茬粡 migrated. 姝?hides 鏁版嵁 migration completely 鏉ヨ嚜
杩愯涓?applications. The Linux NFSv4 client facilitates 鐘舵€?
migration 鐢?presenting the 鐩稿悓 "client_owner4" 鍒?鍏ㄩ儴 servers 瀹?
encounters.

## 鍙傝 涔?


  - nfs(5)
  - kerberos(7)
  - RFC 7530 鐢ㄤ簬 the NFSv4.0 specification
  - RFC 8881 鐢ㄤ簬 the NFSv4.1 specification.
