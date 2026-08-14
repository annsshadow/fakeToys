## AverMedia DVB-T锛圔T878锛夊彂甯冭鏄?

2006 骞?2 鏈?14 鏃?

鐩墠宸叉敮鎸佸娆?AverMedia 璁惧銆傛洿瀹屾暣銆佹洿鍙婃椂鐨勫唴瀹硅鏌ラ槄锛?

https://linuxtv.org/wiki/index.php/AVerMedia

### AverMedia DVB-T

AverMedia DVB-T 鏄竴娆句綆鎴愭湰鐨?PCI DVB 鎺ユ敹鍗★紝鎻愪緵涓変釜杈撳叆鎺ュ彛锛?

- RF 璋冭皭鍣ㄨ緭鍏ワ紙RF Tuner Input锛?
- 澶嶅悎瑙嗛杈撳叆锛圕omposite Video Input锛孯CA 鎺ュ彛锛?
- S-Video 杈撳叆锛圡ini-DIN 鎺ュ彛锛?

RF 璋冭皭鍣ㄨ緭鍏ヨ繛鎺ヨ嚦鍗′笂鐨勮皟璋愬櫒妯″潡銆傝璋冭皭鍣ㄥ湪浠ｇ爜涓€氬父琚О涓衡€滃墠绔紙Frontend锛夆€濄€侫verMedia DVB-T 鎵€鐢ㄥ墠绔负 Microtune 7202D銆傚悗缁?linux-dvb 閭欢鍒楄〃纭锛孧icrotune 7202D 鍙?sp887x 椹卞姩鏀寔锛岃椹卞姩鍙湪 dvb-hw CVS 妯″潡涓壘鍒般€?

璇?DVB-T 鍗″熀浜?BT878 鑺墖锛孊T878 鏄竴绉嶉潪甯稿父瑙佺殑澶氬獟浣撴ˉ鎺ヨ姱鐗囷紝涔熷父鍑虹幇鍦ㄦā鎷熺數瑙嗗崱涓娿€傚崱涓婁笉甯?MPEG2 瑙ｇ爜鍣紝鍥犳 MPEG2 瑙ｇ爜闇€鐢辫蒋浠跺畬鎴愶紱涓庝箣鐩稿鐨勬槸閭ｄ簺鐢辫姱鐗囩粍瀹屾垚 MPEG2 纭欢瑙ｇ爜鐨勫崱銆?

### 璁╁崱鐗囧伐浣滆捣鏉ワ紙Getting the card going锛?

鐜伴樁娈靛凡鑳界‘璁?AverMedia DVBT 鍏朵綑璁惧鑺傜偣鐨勫姛鑳姐€傜劧鑰岋紝娑夊強璋冭皭銆佹帴鏀跺苟鎻愪緵 MPEG2 鏁版嵁娴佺殑鍔熻兘锛岀洰鍓嶄粎鍦ㄤ娇鐢ㄥ彲鐢ㄧ増鏈殑椹卞姩鏃舵柟鍙疄鐜般€傚崱鐗囦笂鍙敤鐨勫叾浠栧姛鑳斤紙渚嬪鍗′笂鎻愪緵鐨勯澶栨ā鎷熻緭鍏ワ級灏氬緟娴嬭瘯銆備竴鏃﹀彂鐜扮浉鍏冲唴瀹癸紝鎴戜細闅忔椂鏇存柊鏈枃妗ｃ€?

涓哄崱鐗囦笂鐢靛悗锛岃鎸変互涓嬮『搴忓姞杞藉唴鏍告ā鍧楋細

- modprobe bttv锛堥€氬父浼氳嚜鍔ㄥ姞杞斤級
- modprobe dvb-bt8xx锛堝彲灏?dvb-bt8xx 鏀惧叆 /etc/modules 鎴栧搴旂殑妯″潡鐩綍锛?

妯″潡鎻掑叆鍚庯紝鍐呮牳浼氭縺娲荤浉搴旂殑 DVB 璁惧鑺傜偣锛岄殢鍚庡嵆鍙娇鐢?scan銆乼zap銆乨vbstream 绛夊伐鍏疯闂鍗°€?

鍓嶇妯″潡 sp887x.o 闇€瑕佸閮ㄥ浐浠躲€傝浣跨敤鍛戒护 `get_dvb_firmware sp887x` 涓嬭浇鍥轰欢锛屽苟灏嗗叾澶嶅埗鍒?/usr/lib/hotplug/firmware 鎴?/lib/firmware/锛堝叿浣撹矾寰勫彇鍐充簬鍥轰欢 hotplug 鐨勯厤缃級銆?

### 宸茬煡闄愬埗锛圞nown Limitations锛?

鐩墠鍙互纭鍓嶇鑳藉瀹屾垚璋冭皭锛屼笖 /dev/dvb/adapter{x}/frontend0 浼氬悜 /dev/dvb/adapter{x}/dvr0 鎻愪緵 MPEG2 鏁版嵁娴併€傚崱鐗囩殑鍏朵綑鍔熻兘鎴戝皻鏈祴璇曪紝鏈夌┖鏃朵細鏇存柊鏈枃妗ｃ€?

闄愬埗涓昏鏉ヨ嚜 i2c 灞傝繑鍥炵殑閿欒淇℃伅涓嶄竴鑷淬€傚敖绠¤繖浼氬湪 dmesg 绯荤粺鏃ュ織涓骇鐢熼敊璇紝浣嗕技涔庡苟涓嶅奖鍝嶅墠绔甯稿彂鎸ヤ綔鐢ㄣ€?

### 鍚庣画鏇存柊锛團urther updates锛?

dvbstream 涓?VideoLAN Client锛圵indows 鐗堬級閰嶅悎 DVB 浣跨敤鏁堟灉寰堝ソ锛屼簨瀹炰笂杩欎篃鏄垜鐩墠瑙傜湅 DVB-T 鐨勪富瑕佹柟寮忋€傛澶栵紝VLC 涔熻兘椤哄埄瑙ｇ爜 HDTV 淇″彿锛屽敖绠?PC 鍋跺皵浼氫涪鍑犲抚鈥斺€旀垜鐚滄祴杩欐簮浜庡鐞嗚兘鍔涗笉瓒筹紙瑙ｇ爜鍦?Windows 涓嬬敱杞欢瀹屾垚锛夈€?

闈炲父鎰熻阿 Nigel Pearson 鍦ㄩ┍鍔ㄨ繎鏈熶慨璁㈠悗鏇存柊浜嗘湰鏂囨。銆?
