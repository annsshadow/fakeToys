## s390 SCSI 杞偍宸ュ叿锛坺fcpdump锛?

System z 鏈哄櫒锛坺900 鎴栨洿楂橈級鎻愪緵纭欢鏀寔锛岀敤浜庡湪 SCSI 纾佺洏涓婂垱寤虹郴缁熻浆鍌ㄣ€傝浆鍌ㄨ繃绋嬮€氳繃鍚姩
涓€涓浆鍌ㄥ伐鍏锋潵鍙戣捣锛岃宸ュ叿蹇呴』鍒涘缓褰撳墠锛堝彲鑳藉凡宕╂簝鐨勶級Linux 鏄犲儚鐨勮浆鍌ㄣ€備负浜嗕笉鎶婂穿婧?Linux
鐨勫唴瀛樿杞偍宸ュ叿鐨勬暟鎹鐩栵紝纭欢鍦ㄥ姞杞借浆鍌ㄥ伐鍏蜂箣鍓嶄細淇濆瓨涓€浜涘唴瀛樹互鍙婂惎鍔?CPU 鐨勫瘎瀛樺櫒闆嗗悎銆?涔嬪悗瀛樺湪涓€涓?SCLP 纭欢鎺ュ彛鐢ㄤ簬鑾峰彇鎵€淇濆瓨鐨勫唴瀛樸€傚綋鍓嶄繚瀛?32 MB銆?
璇?zfcpdump 瀹炵幇鐢变竴涓?Linux 杞偍鍐呮牳鍜屼竴涓敤鎴风┖闂磋浆鍌ㄥ伐鍏风粍鎴愶紝瀹冧滑涓€璧疯鍔犺浇鍒?32 MB 浠ヤ笅
鐨勫凡淇濆瓨鍐呭瓨鍖哄煙涓€倆fcpdump 浣跨敤 zipl锛堝寘鍚湪 s390-tools 鍖呬腑锛夊畨瑁呭埌 SCSI 纾佺洏涓婏紝浠ヤ娇璇?璁惧鍙惎鍔ㄣ€侺inux 绯荤粺鐨勬搷浣滃憳闅忓悗鍙互閫氳繃鍚姩瑁呮湁 zfcpdump 鐨?SCSI 纾佺洏鏉ヨЕ鍙?SCSI 杞偍銆?
鐢ㄦ埛绌洪棿杞偍宸ュ叿閫氳繃 /proc/vmcore 鎺ュ彛璁块棶宕╂簝绯荤粺鐨勫唴瀛樸€傝鎺ュ彛浠?ELF core dump 鏍煎紡瀵煎嚭
宕╂簝绯荤粺鐨勫唴瀛樺拰瀵勫瓨鍣ㄣ€備负浜嗚闂敱纭欢淇濆瓨鐨勫唴瀛橈紝SCLP 璇锋眰灏嗗湪 /proc/vmcore 闇€瑕佽鏁版嵁鏃?鍒涘缓銆傚穿婧冪郴缁熷唴瀛樹腑鏈纭欢鏆傚瓨锛坰tash锛夌殑灏鹃儴閮ㄥ垎鍙互鐩存帴浠庣湡瀹炲唴瀛樺鍒躲€?
瑕佹瀯寤烘敮鎸佽浆鍌ㄧ殑鍐呮牳锛屽繀椤昏缃唴鏍搁厤缃€夐」 CONFIG_CRASH_DUMP銆?
瑕佽幏寰楁湁鏁堢殑 zfcpdump 鍐呮牳閰嶇疆锛屼娇鐢?鈥渕ake zfcpdump_defconfig鈥濄€?
s390 zipl 宸ュ叿鍦ㄤ互涓嬩綅缃煡鎵?zfcpdump 鍐呮牳鍜屽彲閫夌殑 initrd/initramfs锛?
- kernel:  <zfcpdump directory>/zfcpdump.image
- ramdisk: <zfcpdump directory>/zfcpdump.rd

zfcpdump 鐩綍鍦?s390-tools 鍖呬腑瀹氫箟銆?
zfcpdump 鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ラ┗鐣欏湪 intitramfs 鎴?initrd 涓€傚畠涔熷彲浠ュ寘鍚湪鍐呯疆鐨勫唴鏍?initramfs 涓€傝搴旂敤绋嬪簭浠?/proc/vmcore 鎴?zcore/mem 璇诲彇锛屽苟灏嗙郴缁熻浆鍌ㄥ啓鍏?SCSI 纾佺洏銆?
s390-tools 鍖?1.24.0 鍙婃洿楂樼増鏈瀯寤轰竴涓閮?zfcpdump initramfs锛屽叾涓甫鏈変竴涓皢杞偍鍐欏叆
SCSI 鍒嗗尯鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忋€?
鏈夊叧濡備綍浣跨敤 zfcpdump 鐨勬洿澶氫俊鎭紝璇峰弬闃?s390 鐨?鈥淯sing the Dump Tools鈥?鎵嬪唽锛岃涔﹀彲浠?IBM Knowledge Center 鑾峰彇锛?https://www.ibm.com/support/knowledgecenter/linuxonibm/liaaf/lnz_r_dt.html
