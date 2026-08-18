## NFS 瀹㈡埛绔?

## NFS 瀹㈡埛绔?

NFS 鐗堟湰 2 鍗忚鏈€鏃╄褰曞湪 RFC1094锛?989 骞?3 鏈堬級涓€傝嚜閭ｄ互鍚庡張鍙戝竷浜嗕袱涓富瑕佺殑 NFS 鐗堟湰锛?NFSv3 璁板綍鍦?RFC1813锛?995 骞?6 鏈堬級锛孨FSv4 璁板綍鍦?RFC3530锛?003 骞?4 鏈堬級銆?
Linux NFS 瀹㈡埛绔洰鍓嶆敮鎸佷笂杩版墍鏈夊凡鍙戝竷鐨勭増鏈紝骞朵笖姝ｅ湪鐫€鎵嬪鍔犲 NFSv4 鍗忚娆¤鐗堟湰 1 鐨勬敮鎸併€?
鏈枃妗ｇ殑鐩殑鏄彁渚涗竴浜涘彲鐢辩郴缁熺鐞嗗憳閰嶇疆鐨?NFS 瀹㈡埛绔壒娈婄壒鎬х殑淇℃伅銆?

## nfs4_unique_id 鍙傛暟


NFSv4 瑕佹眰瀹㈡埛绔敤涓€涓敮涓€瀛楃涓插悜鏈嶅姟鍣ㄦ爣璇嗚嚜宸便€備竴涓鎴风涓庝竴涓湇鍔″櫒涔嬮棿鍏变韩鐨勬枃浠舵墦寮€
鍜岄攣鐘舵€侀兘鍏宠仈浜庤繖涓韩浠姐€備负浜嗘敮鎸佸仴澹殑 NFSv4 鐘舵€佹仮澶嶅拰閫忔槑鐨勭姸鎬佽縼绉伙紝杩欎釜韬唤瀛楃涓插湪
瀹㈡埛绔噸鍚湡闂翠笉鑳芥敼鍙樸€?
鍦ㄦ病鏈変换浣曞叾瀹冨共棰勭殑鎯呭喌涓嬶紝Linux 瀹㈡埛绔娇鐢ㄤ竴涓寘鍚湰鍦扮郴缁熻妭鐐瑰悕鐨勫瓧绗︿覆銆傜劧鑰岋紝绯荤粺绠＄悊鍛?甯稿父涓嶇‘淇濊妭鐐瑰悕鏄畬鍏ㄩ檺瀹氾紙fully qualified锛夌殑锛屽苟涓斿湪瀹㈡埛绔郴缁熺殑鏁翠釜鐢熷懡鍛ㄦ湡鍐呬笉鏀瑰彉銆傝妭鐐瑰悕
鍙兘鏈夊叾瀹冪鐞嗕笂鐨勮姹傦紝闇€瑕佺壒瀹氱殑琛屼负锛岃€岃繖绉嶈涓轰綔涓?nfs_client_id4 瀛楃涓茬殑涓€閮ㄥ垎骞朵笉鑳藉緢濂?鍦板伐浣溿€?
nfs.nfs4_unique_id 寮曞鍙傛暟鎸囧畾浜嗕竴涓敮涓€瀛楃涓诧紝鍙湪 NFS 瀹㈡埛绔悜鏈嶅姟鍣ㄦ爣璇嗚嚜宸辨椂涓庣郴缁熻妭鐐瑰悕
涓€璧蜂娇鐢ㄣ€傚洜姝わ紝濡傛灉绯荤粺鐨勮妭鐐瑰悕涓嶅敮涓€锛屽畠鐨?nfs.nfs4_unique_id 鏈夊姪浜庨槻姝笌鍏跺畠瀹㈡埛绔啿绐併€?
nfs.nfs4_unique_id 瀛楃涓查€氬父鏄竴涓?UUID锛屽敖绠″畠鍙互鍖呭惈浠讳綍琚涓哄湪鎵€鏈?NFS 瀹㈡埛绔箣闂村敮涓€鐨勫唴瀹广€?nfs4_unique_id 瀛楃涓插簲鍦ㄥ畨瑁呭鎴风绯荤粺鏃堕€夋嫨锛屽氨鍍忕郴缁熺殑鏍规枃浠剁郴缁熷湪瀹夎鏃舵爣绛句笂鑾峰緱涓€涓柊鐨?UUID 涓€鏍枫€?
璇ュ瓧绗︿覆搴斿湪瀹㈡埛绔殑鏁翠釜鐢熷懡鍛ㄦ湡鍐呬繚鎸佸浐瀹氥€傚鏋滃皬蹇冨湴纭繚瀹㈡埛绔共鍑€鍏抽棴涓旀墍鏈夋湭瀹屾垚鐨?NFSv4
鐘舵€侀兘宸茶繃鏈燂紝鍒欏彲浠ュ畨鍏ㄥ湴鏇存敼瀹冿紝浠ラ槻姝?NFSv4 鐘舵€佷涪澶便€?
杩欎釜瀛楃涓插彲浠ュ瓨鍌ㄥ湪 NFS 瀹㈡埛绔殑 grub.conf 涓紝涔熷彲浠ラ€氳繃 PXE 绛夌綉缁滃紩瀵艰鏂芥彁渚涖€傚畠涔熷彲浠ヤ綔涓?nfs.ko 妯″潡鍙傛暟鎸囧畾銆?
杩欎釜鍞竴鏍囪瘑瀛楃涓插浜庢墍鏈夊湪瀹瑰櫒涓繍琛岀殑 NFS 瀹㈡埛绔兘鏄浉鍚岀殑锛岄櫎闈炲畠琚啓鍏?/sys/fs/nfs/net/nfs_client/identifier 鐨勫€艰鐩栵紝璇ュ€煎皢鏄啓鍏ュ畠鐨勮繘绋嬫墍鍦ㄧ綉缁滃懡鍚嶇┖闂寸殑鏈湴鍊笺€?

## DNS 瑙ｆ瀽鍣?

NFSv4 鍏佽涓€涓湇鍔″櫒閫氳繃鐗规畩鐨?"fs_locations" 灞炴€ф妸 NFS 瀹㈡埛绔紩鍚戝凡杩佺Щ鍒板彟涓€鍙版湇鍔″櫒涓婄殑鏁版嵁銆?璇峰弬闃?`RFC3530 Section 6: Filesystem Migration and Replication`_ 鍜?`Implementation Guide for Referrals in NFSv4`_銆?

fs_locations 淇℃伅鍙互閲囩敤 ip 鍦板潃鍔犺矾寰勶紝鎴?DNS 涓绘満鍚嶅姞璺緞鐨勫舰寮忋€傚悗鑰呰姹?NFS 瀹㈡埛绔仛涓€娆?DNS 鏌ユ壘浠ユ寕杞芥柊鍗凤紝鍥犳闇€瑕侀€氳繃涓€娆?upcall 璁╃敤鎴锋€佹潵鎻愪緵姝ゆ湇鍔°€?
鍋囪鐢ㄦ埛宸插皢 'rpc_pipefs' 鏂囦欢绯荤粺鎸傝浇鍦ㄩ€氬父鐨?/var/lib/nfs/rpc_pipefs锛寀pcall 鐢变互涓嬫楠ょ粍鎴愶細

   (1) 杩涚▼妫€鏌?dns_resolve 缂撳瓨锛岀湅瀹冩槸鍚﹀寘鍚竴涓湁鏁堟潯鐩€傚鏋滄湁锛屽氨杩斿洖璇ユ潯鐩苟閫€鍑恒€?
   (2) 濡傛灉涓嶅瓨鍦ㄦ湁鏁堟潯鐩紝鍒欒繍琛岃緟鍔╄剼鏈?'/sbin/nfs_cache_getent'
       锛堝彲浠ヤ娇鐢?'nfs.cache_getent' 鍐呮牳寮曞鍙傛暟鏇存敼锛夛紝甯︿袱涓弬鏁帮細
       - 缂撳瓨鍚嶏紝"dns_resolve"
       - 瑕佽В鏋愮殑涓绘満鍚?
   (3) 鏌ユ壘鍒扮浉搴旂殑 ip 鍦板潃鍚庯紝杈呭姪鑴氭湰浠ュ涓嬶紙鏂囨湰锛夋牸寮忔妸缁撴灉鍐欏叆 rpc_pipefs 浼枃浠?       '/var/lib/nfs/rpc_pipefs/cache/dns_resolve/channel'锛?
		"<ip address> <hostname> <ttl>\n"

       鍏朵腑 <ip address> 閲囩敤閫氬父鐨?IPv4锛?23.456.78.90锛夋垨 IPv6
       锛坒fee:ddcc:bbaa:9988:7766:5544:3322:1100銆乫fee::1100 绛夛級鏍煎紡銆?       <hostname> 涓庤緟鍔╄剼鏈殑绗簩涓弬鏁扮浉鍚岋紝<ttl> 鏄缂撳瓨鏉＄洰鐨勨€滅敓瀛樻椂闂粹€濓紙浠ョ涓哄崟浣嶏級銆?
```

            If <ip address> is invalid, say the string "0", then a negative
            entry is created, which will cause the kernel to treat the hostname
            as having no valid DNS translation.



```
## 涓€涓熀鏈殑绀轰緥 /sbin/nfs_cache_getent


    #!/bin/bash
    #
    ttl=600
    #
    cut=/usr/bin/cut
    getent=/usr/bin/getent
    rpc_pipefs=/var/lib/nfs/rpc_pipefs
    #
    die()
    {
        echo "Usage: $0 cache_name entry_name"
        exit 1
    }

    [ $# -lt 2 ] && die
    cachename="$1"
    cache_path=${rpc_pipefs}/cache/${cachename}/channel

    case "${cachename}" in
        dns_resolve)
            name="$2"
            result="$(${getent} hosts ${name} | ${cut} -f1 -d\ )"
            [ -z "${result}" ] && result="0"
            ;;
        *)
            die
            ;;
    esac
    echo "${result} ${name} ${ttl}" >${cache_path}
