## 涓?ALSA dapm 鍒涘缓 codec 鍒?codec 鐨?dai link


澶у鏁版儏鍐典笅锛岄煶棰戞祦鎬绘槸浠?CPU 鍒?codec锛屽洜姝や綘鐨勭郴缁熺湅璧锋潵濡備笅锛?```

   ---------          ---------
  |         |  dai   |         |
      CPU    ------->    codec
  |         |        |         |
   ---------          ---------

```
濡傛灉浣犵殑绯荤粺鐪嬭捣鏉ュ涓嬶細
```

                       ---------
                      |         |
                        codec-2
                      |         |
                      ---------
                           |
                         dai-2
                           |
   ----------          ---------
  |          |  dai-1 |         |
      CPU     ------->  codec-1
  |          |        |         |
   ----------          ---------
                           |
                         dai-3
                           |
                       ---------
                      |         |
                        codec-3
                      |         |
                       ---------

```
鍋囪 codec-2 鏄竴涓摑鐗欒姱鐗囷紝codec-3 杩炴帴鍒颁竴涓壃澹板櫒锛屽苟涓斾綘鏈変互涓嬪満鏅細
codec-2 灏嗘帴鏀堕煶棰戞暟鎹紝鑰岀敤鎴峰笇鏈涗笉缁忚繃 CPU 灏遍€氳繃 codec-3 鎾斁璇ラ煶棰戙€備笂杩版儏鍐垫鏄簲璇ヤ娇鐢?codec 鍒?codec 杩炴帴鐨勭悊鎯虫儏褰€?
浣犵殑 dai_link 鍦ㄤ綘鐨勬満鍣ㄦ枃浠朵腑搴斿涓嬫墍绀猴細
```

 /*
  * 姝?pcm 娴佷粎鏀寔 24 bit銆? 閫氶亾鍜?  * 48k 閲囨牱鐜囥€?  */
 static const struct snd_soc_pcm_stream dsp_codec_params = {
        .formats = SNDRV_PCM_FMTBIT_S24_LE,
        .rate_min = 48000,
        .rate_max = 48000,
        .channels_min = 2,
        .channels_max = 2,
 };

 {
    .name = "CPU-DSP",
    .stream_name = "CPU-DSP",
    .cpu_dai_name = "samsung-i2s.0",
    .codec_name = "codec-2,
    .codec_dai_name = "codec-2-dai_name",
    .platform_name = "samsung-i2s.0",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF
            | SND_SOC_DAIFMT_CBP_CFP,
    .ignore_suspend = 1,
    .c2c_params = &dsp_codec_params,
    .num_c2c_params = 1,
 },
 {
    .name = "DSP-CODEC",
    .stream_name = "DSP-CODEC",
    .cpu_dai_name = "wm0010-sdi2",
    .codec_name = "codec-3,
    .codec_dai_name = "codec-3-dai_name",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF
            | SND_SOC_DAIFMT_CBP_CFP,
    .ignore_suspend = 1,
    .c2c_params = &dsp_codec_params,
    .num_c2c_params = 1,
 },

```
涓婅堪浠ｇ爜鐗囨鐨勭伒鎰熸潵鑷?sound/soc/samsung/speyside.c銆?
娉ㄦ剰 鈥渃2c_params鈥?鍥炶皟锛屽畠璁?dapm 鐭ラ亾姝?dai_link 鏄竴涓?codec 鍒?codec 鐨勮繛鎺ャ€?
鍦?dapm 鏍稿績涓紝浼氬湪 cpu_dai 鎾斁锛坧layback锛墂idget 鍜?codec_dai 鎹曡幏锛坈apture锛墂idget 涔嬮棿鍒涘缓涓€鏉¤矾鐢辩敤浜庢挱鏀捐矾寰勶紝鍙嶄箣浜︾劧鐢ㄤ簬鎹曡幏璺緞銆備负浜嗕娇涓婅堪杩欐潯璺敱琚Е鍙戯紝DAPM 闇€瑕佹壘鍒颁竴涓湁鏁堢殑绔偣锛岃绔偣鍙互鍒嗗埆鏄搴斾簬鎾斁鍜屾崟鑾疯矾寰勭殑 sink 鎴?source widget銆?
涓轰簡瑙﹀彂姝?dai_link widget锛屽彲浠ヤ负鎵０鍣ㄦ斁澶у櫒鍒涘缓涓€涓交閲忕殑 codec 椹卞姩锛屽 wm8727.c 鏂囦欢鎵€绀猴紝鍗充娇涓嶉渶瑕佷换浣曟帶鍒讹紝瀹冧篃浼氫负璁惧璁剧疆閫傚綋鐨勭害鏉熴€?
纭繚灏嗙浉搴旂殑 cpu 鍜?codec 鎾斁涓庢崟鑾?dai 鍚嶇О鍒嗗埆浠?鈥淧layback鈥?鍜?鈥淐apture鈥?缁撳熬鍛藉悕锛屽洜涓?dapm 鏍稿績浼氭牴鎹悕绉伴摼鎺ュ苟涓鸿繖浜?dai 渚涚數銆?
鍦?鈥渟imple-audio-card鈥?涓紝褰撻摼鎺ヤ笂鐨勬墍鏈?DAI 閮藉睘浜?codec 缁勪欢鏃讹紝璇?dai_link 浼氳鑷姩璇嗗埆涓?codec 鍒?codec銆傝 dai_link 灏嗕娇鐢ㄩ摼鎺ヤ笂鎵€鏈?DAI 鏀寔鐨勬祦鍙傛暟锛堥€氶亾鏁般€佹牸寮忋€侀噰鏍风巼锛夌殑瀛愰泦杩涜鍒濆鍖栥€傜敱浜庡湪璁惧鏍戜腑鏃犳硶鎻愪緵杩欎簺鍙傛暟锛岃繖涓昏鐢ㄤ簬涓庣畝鍗曠殑鍥哄畾鍔熻兘 codec 閫氫俊锛屼緥濡傝摑鐗欐帶鍒跺櫒鎴栬渹绐濊皟鍒惰В璋冨櫒銆?