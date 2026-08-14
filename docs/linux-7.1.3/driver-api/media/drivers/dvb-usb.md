
## dvb-usb 妗嗘灦鑳屽悗鐨勭悊蹇?

   #) 鏈枃妗ｅ凡缁忚繃鏃躲€傝鏌ラ槄 DVB wiki锛堜綅浜?https://linuxtv.org/wiki锛変互鑾峰彇鏇存柊鐨勪俊鎭€?
   #) **宸插簾寮冿細** 杈冩柊鐨?DVB USB 椹卞姩搴斿綋浣跨敤 dvb-usb-v2 妗嗘灦銆?
2005 骞?3 鏈堬紝鎴戞嬁鍒颁簡鏂扮殑 Twinhan USB2.0 DVB-T 璁惧銆備粬浠彁渚涗簡瑙勬牸璇存槑鍜屽浐浠躲€?
鎴戦潪甯告€ュ垏鍦版兂瑕佹妸杩欎釜椹卞姩锛堝綋鐒跺甫涓€浜涙€紓涔嬪锛夋斁杩?dibusb銆傚湪璇讳簡浜涜鏍艰鏄庛€佸仛浜?涓€浜?USB 鍡呮帰涔嬪悗锛屾垜鎰忚瘑鍒帮紝閭ｆ牱鍋氱殑璇?dibusb 椹卞姩涔嬪悗浼氬彉寰椾竴鍥㈢碂銆傛墍浠ユ垜鍐冲畾鐢?涓€绉嶄笉鍚岀殑鏂瑰紡鏉ュ仛锛氬€熷姪涓€涓?dvb-usb 妗嗘灦銆?
璇ユ鏋舵彁渚涢€氱敤鍑芥暟锛堝ぇ澶氭槸鍐呮牳 API 璋冪敤锛夛紝渚嬪锛?
- 涓?dvb-demux-feed-control 閰嶅悎鐨勪紶杈撴祦锛圱ransport Stream锛塙RB 澶勭悊
  锛堟敮鎸?bulk 鍜?isoc锛?- 涓鸿澶囨敞鍐?DVB-API
- 鍦ㄩ€傜敤鏃舵敞鍐屼竴涓?I2C 閫傞厤鍣?- 閬ユ帶鍣?杈撳叆璁惧澶勭悊
- 鍥轰欢璇锋眰涓庡姞杞斤紙鐩墠浠呴拡瀵?Cypress USB 鎺у埗鍣級
- 鍏朵粬鍙互琚涓┍鍔ㄥ叡浜殑鍑芥暟/鏂规硶锛堜緥濡傜敤浜?bulk 鎺у埗鍛戒护鐨勫嚱鏁帮級
- TODO锛氫竴涓?I2C 鍒嗗潡鍣紙chunker锛夈€傚畠鏍规嵁瀵勫瓨鍣ㄩ暱搴﹀拰鍙鍐欍€佸璇荤殑鏁板€间釜鏁帮紝鍒涘缓
  璁惧鐗瑰畾鐨勫瘎瀛樺櫒璁块棶鍧椼€?
鐗瑰畾 DVB USB 璁惧鐨勬簮浠ｇ爜鍙礋璐ｉ€氳繃鎬荤嚎涓庤澶囪繘琛岄€氫俊銆備笌 DVB-API 鍔熻兘涔嬮棿鐨勮繛鎺?鏄€氳繃鍥炶皟瀹屾垚鐨勶紝杩欎簺鍥炶皟鍦ㄦ瘡涓澶囬┍鍔ㄩ兘蹇呴』鎷ユ湁鐨勯潤鎬佽澶囨弿杩帮紙struct
dvb_usb_device锛変腑璧嬪€笺€?
浣滀负绀轰緥锛屽彲浠ユ煡鐪?drivers/media/usb/dvb-usb/vp7045*銆?
鐩爣鏄妸鎵€鏈?usb 璁惧锛坉ibusb銆乧inergyT2锛屼篃璁歌繕鏈?ttusb锛沠lexcop-usb 宸茬粡鍙楃泭浜?閫氱敤鐨?flexcop 璁惧锛夎縼绉诲埌浣跨敤 dvb-usb-lib銆?
TODO锛氭牴鎹墍璇锋眰鐨?feed 鏁伴噺锛屽姩鎬佸惎鐢ㄥ拰绂佺敤 pid 杩囨护鍣ㄣ€?
### 鍙楁敮鎸佺殑璁惧


鍏充簬缃戝崱/椹卞姩/鍥轰欢鐨勫畬鏁村垪琛紝璇峰弬闃?LinuxTV DVB Wiki锛屼綅浜?https://linuxtv.org锛?https://linuxtv.org/wiki/index.php/DVB_USB

0. 鍘嗗彶涓庢柊闂伙細

  2005-06-30

  - 鏂板瀵?WideView WT-220U 鐨勬敮鎸侊紙鎰熻阿 Steve Chang锛?
  2005-05-30

  - 涓?dvb-usb 妗嗘灦娣诲姞浜嗗熀鏈殑绛夋椂锛坕sochronous锛夋敮鎸?  - 鏂板瀵?Conexant 娣峰悎鍙傝€冭璁″拰 Nebula DigiTV USB 鐨勬敮鎸?
  2005-04-17

  - 鎵€鏈?dibusb 璁惧宸茶绉绘浠ヤ娇鐢?dvb-usb 妗嗘灦

  2005-04-02

  - 閲嶆柊鍚敤骞舵敼杩涗簡閬ユ帶鍣ㄤ唬鐮併€?
  2005-03-31

  - 灏?Yakumo/Hama/Typhoon DVB-T USB2.0 璁惧绉绘鍒?dvb-usb銆?
  2005-03-30

  - 鍩轰簬 dibusb 婧愮爜鐨?dvb-usb 妯″潡棣栨鎻愪氦銆?    绗竴涓澶囨槸閽堝 TwinhanDTV Alpha / MagicBox II 浠?USB2.0 鐨?DVB-T 璁惧鐨勬柊椹卞姩銆?  - 锛堜粠 dvb-dibusb 鍙樻洿涓?dvb-usb锛?
  2005-03-28

  - 鏂板瀵?AVerMedia AverTV DVB-T USB2.0 璁惧鐨勬敮鎸?    锛堟劅璋?Glen Harris 鍜?Jiun-Kuei Jung锛孉VerMedia锛?
  2005-03-14

  - 鏂板瀵?Typhoon/Yakumo/HAMA DVB-T mobile USB2.0 鐨勬敮鎸?
  2005-02-11

  - 鏂板瀵?KWorld/ADSTech Instant DVB-T USB2.0 鐨勬敮鎸併€?    闈炲父鎰熻阿 Joachim von Caron

  2005-02-02
  - 鏂板瀵?Hauppauge Win-TV Nova-T USB2 鐨勬敮鎸?
  2005-01-31
  - USB1.1 璁惧鐨勫け鐪熸祦闂宸茶В鍐?
  2005-01-13

  - 鎶婇暅鍍忕殑 pid_filter_table 绉诲洖 dvb-dibusb
    绗竴涓嚑涔庡彲鐢ㄧ殑 HanfTek UMT-010 鐗堟湰
    鍙戠幇 Yakumo/HAMA/Typhoon 鏄?HanfTek UMT-010 鐨勫墠韬?
  2005-01-10

  - 閲嶆瀯瀹屾垚锛岀幇鍦ㄤ竴鍒囬兘浠や汉闈炲父婊℃剰

  - 涓€浜涙€紓璁惧鐨勮皟璋愬櫒鎬櫀锛圓rtec T1 AN2235 璁惧鏈夋椂浼氳閰?Panasonic 璋冭皭鍣級銆?    瀹炵幇浜嗚皟璋愬櫒鎺㈡祴銆傞潪甯告劅璋?Gunnar Wittich銆?
  2004-12-29

  - 缁忚繃鍑犲ぉ涓庘€淯RB 涓嶈繑鍥炩€濊繖涓?bug 鐨勬悘鏂楋紝缁堜簬淇銆?
  2004-12-26

  - 閲嶆瀯浜?dibusb 椹卞姩锛屾媶鍒嗕负鐙珛鐨勬枃浠?  - 鍚敤浜?i2c 鎺㈡祴

  2004-12-06

  - 澧炲姞浜嗗 demod i2c 鍦板潃鎺㈡祴鐨勫彲鑳芥€?  - 鏂扮殑 USB ID锛圕ompro銆丄rtec锛?
  2004-11-23

  - 鍚堝苟浜嗘潵鑷?DiB3000MC_ver2.1 鐨勬洿鏀?  - 淇浜嗚皟璇?  - 鍙互涓?USB2.0 鎻愪緵瀹屾暣鐨?TS

  2004-11-21

  - dib3000mc/p 鍓嶇椹卞姩鐨勭涓€涓彲鐢ㄧ増鏈€?
  2004-11-12

  - 澧炲姞浜嗛澶栫殑閬ユ帶鍣ㄦ寜閿€傛劅璋?Uwe Hanke銆?
  2004-11-07

  - 澧炲姞浜嗛仴鎺у櫒鏀寔銆傛劅璋?David Matthews銆?
  2004-11-05

  - 鏂板瀵逛竴涓柊璁惧鐨勬敮鎸侊紙Grandtec/Avermedia/Artec锛?  - 鎶婃垜鐨勬洿鏀癸紙閽堝 dib3000mb/dibusb锛夊悎骞跺埌 FE_REFACTORING锛屽洜涓哄畠鎴愪簡 HEAD
  - 鎶婁紶杈撴帶鍒讹紙pid 杩囨护鍣ㄣ€乫ifo 鎺у埗锛変粠 usb 椹卞姩绉诲埌浜嗗墠绔紝鐪嬭捣鏉ユ斁鍦ㄩ偅閲屾洿濂?    锛堝鍔犱簡 xfer_ops 缁撴瀯浣擄級
  - 涓哄墠绔紙mc/p/mb锛夊垱寤轰簡鍏叡鏂囦欢

  2004-09-28

  - 鏂板瀵逛竴涓柊璁惧鐨勬敮鎸侊紙Unknown锛寁endor ID 鏄?Hyper-Paltek锛?
  2004-09-20

  - 鏂板瀵逛竴涓柊璁惧鐨勬敮鎸侊紙Compro DVB-U2000锛夛紝鎰熻阿 Amaury Demol 鐨勬姤鍛?  - 鏀瑰彉浜?usb TS 浼犺緭鏂瑰紡锛堝涓?urb锛屽湪璁剧疆鏂扮殑 pid 涔嬪墠鍋滄浼犺緭锛?
  2004-09-13

  - 鏂板瀵逛竴涓柊璁惧鐨勬敮鎸侊紙Artec T1 USB TVBOX锛夛紝鎰熻阿 Christian Motschke 鐨勬姤鍛?
  2004-09-05

  - 鍙戝竷浜?dibusb 璁惧鍜?dib3000mb 鍓嶇椹卞姩
    锛坴p7041.c 鐨勬棫娑堟伅锛?
  2004-07-15

  - 鍋剁劧鍙戠幇锛岃璁惧涓?PLL 浣跨敤浜?TUA6010XS

  2004-07-12

  - 寮勬竻妤氳椹卞姩涔熷簲褰撹兘閰嶅悎 CTS Portable锛堜腑鍗庣數瑙嗙郴缁燂級宸ヤ綔

  2004-07-08

  - 鍥轰欢鎻愬彇 2.422 闂宸茶В鍐筹紝椹卞姩鐜板湪鑳芥纭娇鐢ㄤ粠 2.422 鎻愬彇鐨勫浐浠舵甯稿伐浣?  - 閽堝 2.6.4锛坉vb锛夌殑 #if锛岀紪璇戦棶棰?  - 鏀瑰彉浜嗗浐浠跺鐞嗘柟寮忥紝瑙?vp7041.txt 绗?1.1 鑺?
  2004-07-02

  - 涓€浜涜皟璋愬櫒淇敼锛寁0.1锛屾竻鐞嗭紝棣栨鍏紑

  2004-06-28

  - 鐜板湪浣跨敤 dvb_dmx_swfilter_packets锛屼竴鍒囬兘杩愯鑹ソ

  2004-06-27

  - 鑳藉瑙傜湅骞跺垏鎹㈤閬擄紙pre-alpha锛?  - 杩樻病鏈?section 杩囨护

  2004-06-06

  - 鏀跺埌浜嗙涓€涓?TS锛屼絾鍐呮牳 oops :/

  2004-05-14

  - 鍥轰欢鍔犺浇鍣ㄥ伐浣滄甯?
  2004-05-11

  - 寮€濮嬬紪鍐欓┍鍔?
### 濡備綍浣跨敤锛?

#### 鍥轰欢


澶у鏁?USB 椹卞姩鍦ㄥ紑濮嬪伐浣滀箣鍓嶏紝閮介渶瑕佸悜璁惧涓嬭浇涓€涓浐浠躲€?
鏌ョ湅 DVB-USB 椹卞姩鐨?Wiki 椤甸潰锛屼互浜嗚В浣犵殑璁惧闇€瑕佸摢涓浐浠讹細

https://linuxtv.org/wiki/index.php/DVB_USB

#### 缂栬瘧


鐢变簬璇ラ┍鍔ㄤ綅浜?Linux 鍐呮牳涓紝鍦ㄤ綘鍠滄鐨勯厤缃幆澧冧腑鍚敤璇ラ┍鍔ㄥ氨搴斿綋瓒冲浜嗐€傛垜寤鸿
鎶婇┍鍔ㄧ紪璇戜负妯″潡銆傚墿涓嬬殑鐢?Hotplug 瀹屾垚銆?
濡傛灉浣犱娇鐢?dvb-kernel锛岃繘鍏?build-2.6 鐩綍锛岀劧鍚庤繍琛?'make'锛屼箣鍚庤繍琛?'insmod.sh
load'銆?
#### 鍔犺浇椹卞姩


Hotplug 鑳藉鍦ㄩ渶瑕佹椂锛堝洜涓轰綘鎻掑叆浜嗚澶囷級鍔犺浇椹卞姩銆?
濡傛灉浣犳兂瑕佸惎鐢ㄨ皟璇曡緭鍑猴紝浣犲繀椤绘墜鍔ㄥ姞杞介┍鍔紝骞朵笖鏄湪 dvb-kernel cvs 浠撳簱鍐呴儴銆?
棣栧厛鐪嬩竴涓嬫湁鍝簺璋冭瘯绾у埆鍙敤锛?

	# modinfo dvb-usb
	# modinfo dvb-usb-vp7045

	etc.


	modprobe dvb-usb debug=<level>
	modprobe dvb-usb-vp7045 debug=<level>
	etc.

搴旇灏辫兘瑙ｅ喅闂銆?
褰撻┍鍔ㄥ姞杞芥垚鍔熴€佸浐浠舵枃浠朵綅缃纭€佷笖璁惧宸茶繛鎺ユ椂锛屸€淧ower鈥?LED 搴斿綋浜捣銆?
鍒拌繖涓€姝ワ紝浣犲氨搴斿綋鑳藉鍚姩涓€涓敮鎸?DVB 鐨勫簲鐢ㄧ▼搴忎簡銆傛垜浣跨敤 (t|s)zap銆乵player 鍜?dvbscan 鏉ユ祴璇曞熀鏈姛鑳姐€俈DR-xine 鎻愪緵浜嗛暱鏈熸祴璇曞満鏅€?
### 宸茬煡闂涓庣己闄?

- 涓嶈鍦ㄨ繍琛?DVB 搴旂敤绋嬪簭鏃舵嫈闄?USB 璁惧锛屼綘鐨勭郴缁熷緢鍙兘浼氬彂鐤垨鑰呮鏈恒€?
#### 涓鸿澶囨坊鍔犳敮鎸?

TODO

#### USB1.1 甯﹀闄愬埗


鐩墠鍙楁敮鎸佺殑璁稿璁惧閮芥槸 USB1.1 鐨勶紝鍥犳褰撹繛鎺ュ埌 USB2.0 闆嗙嚎鍣ㄦ椂锛屽畠浠殑鏈€澶у甫瀹?绾︿负 5-6 MBit/s銆傝繖瀵逛簬鎺ユ敹涓€涓?DVB-T 棰戦亾锛堢害 16 MBit/s锛夌殑瀹屾暣浼犺緭娴佹潵璇存槸涓嶅
鐨勩€傞€氬父杩欎笉鎴愰棶棰橈紝濡傛灉浣犲彧鎯崇湅鐢佃锛堣繖涓嶉€傜敤浜?HDTV锛夛紝浣嗘槸鍦ㄥ悓涓€棰戠巼涓婅竟鐪嬩竴涓?棰戦亾杈瑰綍鍙︿竴涓閬撳氨宸ヤ綔寰椾笉澶ソ銆傝繖閫傜敤浜庢墍鏈?USB1.1 DVB-T 璁惧锛岃€屼笉浠呬粎鏄?dvb-usb 璁惧銆?
閭ｄ釜鍥犻噸搴︿娇鐢ㄨ澶囪€屽鑷?TS 澶辩湡鐨?bug 宸茬粡褰诲簳娑堝け浜嗐€傛垜鐢ㄨ繃鐨勬墍鏈?dvb-usb 璁惧
锛圱winhan銆並world銆丏iBcom锛夌幇鍦ㄩ厤鍚?VDR 宸ヤ綔寰楀儚榄旀硶涓€鏍枫€傛湁鏃舵垜鐢氳嚦鑳藉褰曚竴涓閬?鍚屾椂鐪嬪彟涓€涓€?
#### 璇勮


闈炲父闈炲父娆㈣繋琛ヤ竵銆佽瘎璁哄拰寤鸿銆?
### 3. 鑷磋阿


   Amaury Demol (Amaury.Demol@parrot.com) 鍜屾潵鑷?DiBcom 鐨?Francois Kanounnikoff锛?   鎻愪緵浜嗚鏍艰鏄庛€佷唬鐮佸拰甯姪锛宒vb-dibusb銆乨ib3000mb 鍜?dib3000mc 灏辨槸鍩轰簬瀹冧滑鐨勩€?
   David Matthews锛屼粬璇嗗埆浜嗕竴绉嶆柊鐨勮澶囩被鍨嬶紙甯?AN2235 鐨?Artec T1锛夛紝骞朵负 dibusb
   鎵╁睍浜嗛仴鎺у櫒浜嬩欢澶勭悊銆傝阿璋綘銆?
   Alex Woods锛屼粬棰戠箒鍦板洖绛斿叧浜?usb 鍜?dvb 鐨勯棶棰橈紝闈炲父鎰熻阿銆?
   Bernd Wagner锛屼粬鎻愪緵浜嗗ぇ閲?bug 鎶ュ憡鍜岃璁烘柟闈㈢殑甯姪銆?
   Gunnar Wittich 鍜?Joachim von Caron锛屼粬浠俊浠绘垜锛屽湪鑷繁鐨勬満鍣ㄤ笂鎻愪緵 root shell
   鏉ュ疄鐜板鏂拌澶囩殑鏀寔銆?
   Allan Third 鍜?Michael Hutchinson锛屼粬浠府鍔╃紪鍐欎簡 Nebula digitv 椹卞姩銆?
   Glen Harris锛屼粬鎻愬嚭浜嗗瓨鍦ㄦ柊鐨?dibusb 璁惧锛屼互鍙婃潵鑷?AVerMedia 鐨?Jiun-Kuei Jung锛?   浠栧杽鎰忓湴鎻愪緵浜嗕竴涓壒娈婂浐浠讹紝浣胯璁惧鍦?Linux 涓兘澶熷惎鍔ㄨ繍琛屻€?
   Jennifer Chen銆丣eff 鍜屾潵鑷?Twinhan 鐨?Jack锛屼粬浠€氳繃缂栧啓 vp7045 椹卞姩缁欎簣浜嗗杽鎰忕殑
   鏀寔銆?
   Steve Chang锛屾潵鑷?WideView锛屼粬鎻愪緵浜嗘柊璁惧鍜屽浐浠舵枃浠剁殑淇℃伅銆?
   Michael Paxton锛屼粬鎻愪氦浜嗛仴鎺у櫒閿綅鏄犲皠銆?
   linux-dvb 閭欢鍒楄〃涓婄殑涓€浜涙湅鍙嬶紝浠栦滑榧撳姳浜嗘垜銆?
   Peter Schildmann <peter.schildmann-nospam-at-web.de>锛屼粬鎻愪緵浜嗙敤鎴锋€佸浐浠跺姞杞藉櫒锛?   鑺傜渷浜嗗ぇ閲忔椂闂达紙鍦ㄧ紪鍐?vp7041 椹卞姩鏃讹級

   Ulf Hermenau锛屼粬甯垜澶勭悊绻佷綋涓枃銆?
   Andr茅 Smoktun 鍜?Christian Fr枚mmel锛屼粬浠负鎴戞彁渚涚‖浠讹紝骞堕潪甯歌€愬績鍦板惉鎴戝€捐瘔闂銆?