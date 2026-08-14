
## SMB Direct - 鍩轰簬 RDMA 鐨?SMB3


鏈枃妗ｄ粙缁嶅浣曞皢 Linux 鐨?SMB 瀹㈡埛绔笌鏈嶅姟鍣ㄩ厤缃负浣跨敤 RDMA銆?
## 姒傝堪

Linux SMB 鍐呮牳瀹㈡埛绔敮鎸?SMB Direct锛岃繖鏄?SMB3 鐨勪竴绉嶄紶杈撴柟妗堬紝瀹冧娇鐢?RDMA锛堣繙绋嬬洿鎺ュ唴瀛樿闂級缁曡繃浼犵粺鐨?TCP/IP 鍗忚鏍堬紝浠庤€屾彁渚涢珮鍚炲悙閲忓拰浣庡欢杩熴€?Linux SMB 瀹㈡埛绔笂鐨?SMB Direct 鍙互閽堝 KSMBD锛堜竴涓唴鏍告€?SMB 鏈嶅姟鍣級杩涜娴嬭瘯銆?
## 瀹夎

- 瀹夎涓€涓?RDMA 璁惧銆傚彧瑕佽 RDMA 璁惧椹卞姩琚唴鏍告敮鎸侊紝鍗冲彲宸ヤ綔銆傝繖鍖呮嫭杞欢妯℃嫙鍣紙soft RoCE銆乻oft iWARP锛夊拰纭欢璁惧锛圛nfiniBand銆丷oCE銆乮WARP锛夈€?
- 瀹夎涓€涓敮鎸?SMB Direct 鐨勫唴鏍搞€傞涓湪瀹㈡埛绔拰鏈嶅姟鍣ㄧ鍧囨敮鎸?SMB Direct 鐨勫唴鏍哥増鏈槸 5.15銆傚洜姝わ紝闇€瑕佷娇鐢ㄤ笌鍐呮牳 5.15 鎴栨洿楂樼増鏈吋瀹圭殑鍙戣鐗堛€?
- 瀹夎 cifs-utils锛屽畠鎻愪緵鐢ㄤ簬鎸傝浇 SMB 鍏变韩鐨?`mount.cifs` 鍛戒护銆?
- 閰嶇疆 RDMA 鍗忚鏍?
  璇风‘淇濅綘鐨勫唴鏍搁厤缃凡鍚敤 RDMA 鏀寔銆傚湪 Device Drivers -> Infiniband support 涓嬶紝鏇存柊鍐呮牳閰嶇疆浠ュ惎鐢?Infiniband 鏀寔銆?
  鏍规嵁浣犵殑纭欢锛屽惎鐢ㄧ浉搴旂殑 IB HCA 鏀寔鎴?iWARP 閫傞厤鍣ㄦ敮鎸併€?
  濡傛灉浣犱娇鐢ㄧ殑鏄?InfiniBand锛岃鍚敤 IP-over-InfiniBand 鏀寔銆?
  瀵逛簬杞?RDMA锛岃鍚敤 soft iWARP锛坄RDMA _SIW`锛夋垨 soft RoCE锛坄RDMA_RXE`锛夋ā鍧椼€傚畨瑁?`iproute2` 杞欢鍖咃紝骞朵娇鐢?`rdma link add` 鍛戒护鍔犺浇妯″潡骞跺垱寤?RDMA 鎺ュ彛銆?
  渚嬪锛屽鏋滀綘鐨勬湰鍦颁互澶綉鎺ュ彛鏄?`eth0`锛屽彲浠ヤ娇鐢細

    .. code-block:: bash

        sudo rdma link add siw0 type siw netdev eth0

- 鍦ㄥ唴鏍搁厤缃腑涓烘湇鍔″櫒鍜屽鎴风鍚屾椂鍚敤 SMB Direct 鏀寔銆?
    Server Setup

    .. code-block:: text

        Network File Systems  --->
            <M> SMB3 server support
                [*] Support for SMB Direct protocol

    Client Setup

    .. code-block:: text

        Network File Systems  --->
            <M> SMB3 and CIFS support (advanced network filesystem)
                [*] SMB Direct support

- 缂栬瘧骞跺畨瑁呭唴鏍搞€係MB Direct 鏀寔灏嗚缂栧叆 cifs.ko 鍜?ksmbd.ko 妯″潡銆?
## 閰嶇疆涓庝娇鐢?

- 鎸夌収 `KSMBD 鏂囨。 <https://www.kernel.org/doc/Documentation/filesystems/smb/ksmbd.rst>`_ 涓墍杩版惌寤哄苟鍚姩涓€涓?KSMBD 鏈嶅姟鍣ㄣ€傚悓鏃跺湪 ksmbd.conf 涓坊鍔?"server multi channel support = yes" 鍙傛暟銆?
- 鍦ㄥ鎴风涓婏紝浣跨敤 `rdma` 鎸傝浇閫夐」鎸傝浇鍏变韩浠ヤ娇鐢?SMB Direct锛堥€氳繃 `vers` 鎸囧畾 SMB 3.0 鎴栨洿楂樼増鏈級銆?
  渚嬪锛?
    .. code-block:: bash

        mount -t cifs //server/share /mnt/point -o vers=3.1.1,rdma

- 瑕侀獙璇佹寕杞芥槸鍚︽鍦ㄤ娇鐢?SMB Direct锛屽彲鍦ㄦ寕杞藉悗妫€鏌?dmesg 涓槸鍚﹀嚭鐜颁互涓嬫棩蹇楄锛?
    .. code-block:: text

        CIFS: VFS: RDMA transport established

  鎴栬€咃紝鍦?`/proc/mounts` 涓獙璇佽鍏变韩鐨?`rdma` 鎸傝浇閫夐」锛?
    .. code-block:: bash

        cat /proc/mounts | grep cifs
