


# 绗簩閮ㄥ垎 - 鏁板瓧鐢佃 API


   鏈?API 涔熺О涓?Linux **DVB API**銆?
   瀹冩渶鍒濇槸涓轰簡鏀寔娆ф床鏁板瓧鐢佃鏍囧噯锛圖VB锛夎€岀紪鍐欑殑锛屽悗鏉ヨ鎵╁睍浠ユ敮鎸佹墍鏈夋暟瀛楃數瑙?   鏍囧噯銆?
   涓轰簡閬垮厤娣锋穯锛屽湪鏈枃妗ｄ腑锛岄€夋嫨灏嗗叾浠ュ強鐩稿叧鐨勭‖浠剁О涓?**Digital TV锛堟暟瀛楃數瑙嗭級**銆?
   **DVB** 涓€璇嶈淇濈暀鐢ㄤ簬锛?
     - 鏁板瓧鐢佃 API 鐗堟湰
       锛堜緥濡?DVB API version 3 鎴?DVB API version 5锛夛紱
     - 鏁板瓧鐢佃鏁版嵁绫诲瀷锛堟灇涓俱€佺粨鏋勪綋銆佸畯瀹氫箟绛夛級锛?     - 鏁板瓧鐢佃璁惧鑺傜偣锛坄/dev/dvb/...`锛夛紱
     - 娆ф床 DVB 鏍囧噯銆?
**鐗堟湰 5.10**

- [intro](intro)
- [frontend](frontend)
- [demux](demux)
- [ca](ca)
- [net](net)
- [legacy_dvb_apis](legacy_dvb_apis)
- [examples](examples)
- [headers](headers)

######## 淇涓庣増鏉?

Authors:

- J. K. Metzler, Ralph <rjkm@metzlerbros.de>

 - 鏁板瓧鐢佃 API 鏂囨。鐨勫師濮嬩綔鑰呫€?
- O. C. Metzler, Marcus <rjkm@metzlerbros.de>

 - 鏁板瓧鐢佃 API 鏂囨。鐨勫師濮嬩綔鑰呫€?
- Carvalho Chehab, Mauro <mchehab+samsung@kernel.org>

 - 灏嗘枃妗ｇЩ妞嶅埌 Docbook XML锛屾柊澧?DVBv5 API锛屼慨澶嶆枃妗ｄ腑鐨勭己澶遍儴鍒嗐€?
**Copyright** |copy| 2002-2003 : Convergence GmbH

**Copyright** |copy| 2009-2017 : Mauro Carvalho Chehab

######## 淇鍘嗗彶


:revision: 2.2.0 / 2017-09-01 (**mcc**)

闈為仐鐣?API 涓?uAPI 鏂囨。涓庡唴鏍稿疄鐜颁箣闂寸殑澶у鏁扮己澶遍儴鍒嗗凡淇銆?
:revision: 2.1.0 / 2015-05-29 (**mcc**)

瀵?DocBook 杩涜浜嗘敼杩涗笌娓呯悊锛屼互鏇存爣鍑嗙殑鏂瑰紡璁板綍绯荤粺璋冪敤锛屽苟鎻愪緵瀵瑰綋鍓嶆暟瀛楃數瑙?API 鐨勬洿澶氭弿杩般€?
:revision: 2.0.4 / 2011-05-06 (**mcc**)

娣诲姞鍏充簬 DVBv5 API 鐨勬洿澶氫俊鎭紝鏇村ソ鍦版弿杩颁簡鍓嶇 GET/SET props ioctl銆?

:revision: 2.0.3 / 2010-07-03 (**mcc**)

娣诲姞涓€浜涘唴鏍镐腑瀛樺湪浣嗚鑼冧腑缂哄け鐨勫墠绔兘鍔涙爣蹇椼€?

:revision: 2.0.2 / 2009-10-25 (**mcc**)

璁板綍 FE_SET_FRONTEND_TUNE_MODE 涓?FE_DISHETWORK_SEND_LEGACY_CMD ioctl銆?

:revision: 2.0.1 / 2009-09-16 (**mcc**)

娣诲姞鏈€鍒濈敱 Patrick Boettcher 缂栧啓鐨?ISDB-T 娴嬭瘯


:revision: 2.0.0 / 2009-09-06 (**mcc**)

浠?LaTex 杞崲涓?DocBook XML銆傚唴瀹逛笌鍘熷 LaTex 鐗堟湰鐩稿悓銆?

:revision: 1.0.0 / 2003-07-24 (**rjkm**)

LaTEX 涓婄殑鍒濆淇銆?