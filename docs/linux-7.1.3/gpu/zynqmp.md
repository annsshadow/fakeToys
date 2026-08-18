## Xilinx ZynqMP Ultrascale+ DisplayPort 瀛愮郴缁?

璇ュ瓙绯荤粺璐熻矗 ZynqMP 涓婄殑 DisplayPort 瑙嗛涓庨煶棰戣緭鍑恒€傚畠鏀寔浣跨敤 DisplayPort DMA
鎺у埗鍣紙xilinx-dpdma锛夌殑鐗囧唴甯х紦鍐诧紝浠ュ強鏉ヨ嚜鍙紪绋嬮€昏緫锛圥L锛夌殑鈥滃疄鏃垛€濊棰戜笌闊抽銆?璇ュ瓙绯荤粺鍙墽琛屽绉嶅彉鎹紝鍖呮嫭鑹插僵绌洪棿杞崲銆乤lpha 娣峰悎涓庨煶棰戞贩闊筹紝灏界鐩墠骞堕潪鎵€鏈?鐗规€ч兘鍙楁敮鎸併€?
### debugfs


涓烘敮鎸佽皟璇曚笌涓€鑷存€ф祴璇曪紝鍙€氳繃 debugfs 鍚敤鑻ュ共娴嬭瘯妯″紡銆?sys/kernel/debug/dri/X/DP-1/test/
涓嬬殑浠ヤ笅鏂囦欢鐢ㄤ簬鎺у埗 DisplayPort 娴嬭瘯妯″紡锛?
active锛?        鍚戣鏂囦欢鍐欏叆 1 灏嗘縺娲绘祴璇曟ā寮忥紝鍐欏叆 0 灏嗗仠鐢ㄦ祴璇曟ā寮忋€傚湪娴嬭瘯妯″紡宸叉縺娲?宸插仠鐢ㄦ椂
        鍐欏叆 1 鎴?0 灏嗛噸鏂版縺娲?閲嶆柊鍋滅敤娴嬭瘯妯″紡銆傚綋娴嬭瘯妯″紡鏈縺娲绘椂锛屽鍏朵粬鏂囦欢鎵€浣滅殑鏇存敼
        涓嶄細锛堢珛鍗筹級鐢熸晥锛屼絾杩欎簺璁剧疆浼氳淇濆瓨锛屽緟娴嬭瘯妯″紡婵€娲绘椂鐢熸晥銆傚綋娴嬭瘯妯″紡婵€娲绘椂锛?        瀵瑰叾浠栨枃浠舵墍浣滅殑鏇存敼浼氱珛鍗崇敓鏁堛€?
custom锛?        鑷畾涔夋祴璇曞浘妗堝€?
downspread锛?        閫氳繃鍐欏叆 1/0 鏉ュ惎鐢?绂佺敤鏃堕挓鎵╅锛坰pread-spectrum clocking锛?
enhanced锛?        鍚敤/绂佺敤澧炲己甯?
ignore_aux_errors锛?        璁句负 1 鏃跺拷鐣?AUX 閿欒銆傚璇ユ枃浠剁殑鍐欏叆浼氱珛鍗崇敓鏁堬紙鏃犺娴嬭瘯妯″紡鏄惁婵€娲伙級锛?        骞跺奖鍝嶆墍鏈?AUX 浼犺緭銆?
ignore_hpd锛?        璁句负 1 鏃跺拷鐣ョ儹鎻掓嫈浜嬩欢锛堜緥濡傜嚎缂嗘嫈闄ゆ垨鏄剧ず鍣ㄩ摼璺噸璁粌璇锋眰锛夈€?
laneX_preemphasis锛?        lane X 鐨勯鍔犻噸锛屼粠 0锛堟渶浣庯級鍒?2锛堟渶楂橈級

laneX_swing锛?        lane X 鐨勭數鍘嬫憜骞咃紝浠?0锛堟渶浣庯級鍒?3锛堟渶楂橈級

lanes锛?        瑕佷娇鐢ㄧ殑閫氶亾鏁帮紙1銆? 鎴?4锛?
pattern锛?        娴嬭瘯鍥炬銆傚彲浠ユ槸浠ヤ笅涔嬩竴锛?
                video
                        浣跨敤甯歌瑙嗛杈撳叆

                symbol-error
                        绗﹀彿閿欒娴嬮噺鍥炬

                prbs7
                        PRBS7锛坸^7 + x^6 + 1锛夊椤瑰紡鐨勮緭鍑?
                80bit-custom
                        鑷畾涔夌殑 80 浣嶅浘妗?
                cp2520
                        HBR2 涓€鑷存€х溂鍥惧浘妗?
                tps1
                        閾捐矾璁粌绗﹀彿鍥炬 TPS1锛?D10.2/锛?
                tps2
                        閾捐矾璁粌绗﹀彿鍥炬 TPS2

                tps3
                        閾捐矾璁粌绗﹀彿鍥炬 TPS3锛堢敤浜?HBR2锛?
rate锛?        閫熺巼锛堝崟浣嶈但鍏癸級銆備负浠ヤ笅涔嬩竴锛?
                - 5400000000 (HBR2)
                - 2700000000 (HBR)
                - 1620000000 (RBR)

```

        for prop in /sys/kernel/debug/dri/1/DP-1/test/*; do
                printf '%-17s ' ${prop##*/}
                if [ ${prop##*/} = custom ]; then
                        hexdump -C $prop | head -1
                else
                        cat $prop
                fi
        done

```

```

        active            1
        custom            00000000  00 00 00 00 00 00 00 00  00 00                    |..........|
        downspread        0
        enhanced          1
        ignore_aux_errors 1
        ignore_hpd        1
        lane0_preemphasis 0
        lane0_swing       3
        lane1_preemphasis 0
        lane1_swing       3
        lanes             2
        pattern           prbs7
        rate              1620000000

```
鎺ㄨ崘鐨勬祴璇曟祦绋嬫槸锛氬皢寮€鍙戞澘杩炴帴鍒版樉绀哄櫒锛岄厤缃祴璇曟ā寮忥紝婵€娲绘祴璇曟ā寮忥紝鐒跺悗鎷斾笅绾跨紗骞?杩炴帴鍒颁綘閫夋嫨鐨勬祴璇曡澶囥€備緥濡傦紝鍙互

```

        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/enhanced
        echo tps1 > /sys/kernel/debug/dri/1/DP-1/test/pattern
        echo 1620000000 > /sys/kernel/debug/dri/1/DP-1/test/rate
        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/ignore_aux_errors
        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/ignore_hpd
        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/active

```
姝ゆ椂鍗冲彲灏嗙嚎缂嗕粠鏄剧ず鍣ㄤ笂鎷斾笅銆?
### 鍐呴儴瀹炵幇





