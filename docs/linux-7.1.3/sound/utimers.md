
## 鐢ㄦ埛绌洪棿椹卞姩鐨勫畾鏃跺櫒


:Author: Ivan Orlov <ivan.orlov0322@gmail.com>

## 鍓嶈█


鏈枃妗ｄ粙缁嶇敤鎴风┖闂撮┍鍔ㄧ殑瀹氭椂鍣細鍗冲彲浠ョ敱鐢ㄦ埛绌洪棿搴旂敤绋嬪簭閫氳繃 IOCTL 璋冪敤鍒涘缓骞舵帶鍒剁殑铏氭嫙 ALSA 瀹氭椂鍣ㄣ€?褰撴垜浠兂瑕佸皢闊抽娴佷笌灏氭湭瀵煎嚭 ALSA 瀹氭椂鍣ㄧ殑瀹氭椂鍣ㄦ簮锛堜緥濡?PTP 鏃堕挓锛夊悓姝ワ紝鎴栬€呮兂瑕佷娇鐢?`snd-aloop`
灏嗕袱涓櫄鎷熷０璁惧涔嬮棿鐨勯煶棰戞祦鍚屾鏃讹紙渚嬪锛屾湁涓€涓綉缁滃簲鐢ㄧ▼搴忓悜鏌愪釜 snd-aloop 璁惧鍙戦€佸抚锛岃€屽彟涓€涓０搴旂敤绋嬪簭鐩戝惉
snd-aloop 鐨勫彟涓€绔級锛岃繖绫诲畾鏃跺櫒浼氬緢鏈夌敤銆?
## 鍚敤鐢ㄦ埛绌洪棿椹卞姩鐨勫畾鏃跺櫒


鐢ㄦ埛绌洪棿椹卞姩鐨勫畾鏃跺櫒鍙互鍦ㄥ唴鏍镐腑閫氳繃 `CONFIG_SND_UTIMER` 閰嶇疆閫夐」鍚敤銆傚畠渚濊禆浜?`CONFIG_SND_TIMER`
閫夐」锛屽洜姝よ閫夐」涔熷簲琚惎鐢ㄣ€?
## 鐢ㄦ埛绌洪棿椹卞姩鐨勫畾鏃跺櫒 API


鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙互閫氳繃鍦?`/dev/snd/timer` 璁惧鏂囦欢鎻忚堪绗︿笂鎵ц `SNDRV_TIMER_IOCTL_CREATE` ioctl 璋冪敤鏉ュ垱寤轰竴涓敤鎴风┖闂撮┍鍔ㄧ殑 ALSA 瀹氭椂鍣ㄣ€?搴斾紶閫?`snd_timer_uinfo` 缁撴瀯浣撲綔涓?ioctl 鍙傛暟锛?
```

    struct snd_timer_uinfo {
        __u64 resolution;
        int fd;
        unsigned int id;
        unsigned char reserved[16];
    }

```
`resolution` 瀛楁浠ョ撼绉掍负鍗曚綅璁剧疆铏氭嫙瀹氭椂鍣ㄦ湡鏈涚殑鍒嗚鲸鐜囥€俙resolution` 瀛楁鍙槸鎻愪緵鍏充簬铏氭嫙瀹氭椂鍣ㄧ殑淇℃伅锛?骞朵笉褰卞搷璁℃椂鏈韩銆俙id` 瀛楁浼氳 ioctl 瑕嗙洊锛岃皟鐢ㄥ悗璇ュ瓧娈典腑寰楀埌鐨勬爣璇嗙鍙互鍦ㄥ皢瀹氭椂鍣ㄤ紶閫掔粰 `snd-aloop`
鍐呮牳妯″潡鎴栧叾浠栫敤鎴风┖闂村簲鐢ㄧ▼搴忔椂鐢ㄤ綔瀹氭椂鍣ㄥ瓙璁惧缂栧彿銆傜郴缁熶腑鏌愪竴鏃跺埢鏈€澶氬彲瀛樺湪 128 涓敤鎴风┖闂撮┍鍔ㄧ殑瀹氭椂鍣紝
鍥犳 id 鐨勫彇鍊艰寖鍥翠负 0 鍒?127銆?
闄や簡瑕嗙洊 `snd_timer_uinfo` 缁撴瀯浣撲箣澶栵紝ioctl 杩樹細灏嗕竴涓彲鐢ㄤ簬瑙﹀彂璇ュ畾鏃跺櫒鐨勫畾鏃跺櫒鏂囦欢鎻忚堪绗﹀瓨鍌ㄥ湪
`snd_timer_uinfo` 缁撴瀯浣撶殑 `fd` 瀛楁涓€備负瀹氭椂鍣ㄥ垎閰嶄竴涓枃浠舵弿杩扮锛屽彲浠ヤ繚璇佽瀹氭椂鍣ㄥ彧鑳界敱鍏跺垱寤鸿繘绋嬭Е鍙戙€?闅忓悗鍙互閫氳繃瀵硅瀹氭椂鍣ㄦ枃浠舵弿杩扮鎵ц `SNDRV_TIMER_IOCTL_TRIGGER` ioctl 璋冪敤鏉ヨЕ鍙戝畾鏃跺櫒銆?
鍥犳锛屽垱寤哄苟瑙﹀彂瀹氭椂鍣ㄧ殑绀轰緥浠ｇ爜涓猴細

```

    static struct snd_timer_uinfo utimer_info = {
        /* 瀹氭椂鍣ㄥ皢锛堝ぇ姒傦級姣?1000000 ns 瑙﹀彂涓€娆?*/
        .resolution = 1000000ULL,
        .id = -1,
    };

    int timer_device_fd = open("/dev/snd/timer",  O_RDWR | O_CLOEXEC);

    if (ioctl(timer_device_fd, SNDRV_TIMER_IOCTL_CREATE, &utimer_info)) {
        perror("Failed to create the timer");
        return -1;
    }

    ...

    /*
     * 鐜板湪鎴戜滑鎯宠瑙﹀彂瀹氭椂鍣ㄣ€傜粦瀹氬埌璇ュ畾鏃跺櫒鐨勬墍鏈?     * 瀹氭椂鍣ㄥ疄渚嬬殑鍥炶皟灏嗗湪鏈璋冪敤涔嬪悗琚墽琛屻€?     */
    ioctl(utimer_info.fd, SNDRV_TIMER_IOCTL_TRIGGER, NULL);

    ...

    /* 鐜板湪閿€姣佸畾鏃跺櫒 */
    close(timer_info.fd);


```
鍏充簬鍒涘缓骞堕┍鍔ㄥ畾鏃跺櫒鐨勬洿璇︾粏绀轰緥锛屽彲鍦?utimer ALSA 鑷祴涓壘鍒般€?
### 鐢ㄦ埛绌洪棿椹卞姩鐨勫畾鏃跺櫒涓?snd-aloop


鍦ㄥ悓姝ヨ櫄鎷熷０鍥炵幆涓ょ鐨勪袱涓０搴旂敤绋嬪簭鏃讹紝鐢ㄦ埛绌洪棿椹卞姩鐨勫畾鏃跺櫒鍙互寰堝鏄撳湴涓?`snd-aloop` 妯″潡閰嶅悎浣跨敤銆?渚嬪锛屽鏋滃叾涓竴涓簲鐢ㄧ▼搴忎粠缃戠粶鎺ユ敹澹板抚骞跺皢鍏跺彂閫佸埌 snd-aloop 鐨?pcm 璁惧锛岃€屽彟涓€涓簲鐢ㄧ▼搴忓湪鍙︿竴涓?snd-aloop 鐨?pcm 璁惧涓婄洃鍚抚锛岄偅涔堝悎鐞嗙殑鍋氭硶鏄細ALSA 涓棿灞傚簲鍦ㄩ€氳繃缃戠粶鎺ユ敹鍒版柊涓€-period 鏁版嵁鏃跺彂璧蜂竴娆℃暟鎹紶杈擄紝
鑰屼笉鏄湪鏌愪釜 jiffies 鏁伴噺鑰楀敖鏃跺彂璧枫€傜敤鎴风┖闂撮┍鍔ㄧ殑 ALSA 瀹氭椂鍣ㄥ彲鐢ㄤ簬瀹炵幇杩欎竴鐐广€?
瑕佸皢鐢ㄦ埛绌洪棿椹卞姩鐨?ALSA 瀹氭椂鍣ㄧ敤浣?snd-aloop 鐨勫畾鏃跺櫒婧愶紝璇峰皢浠ヤ笅瀛楃涓蹭綔涓?snd-aloop 鐨?`timer_source` 鍙傛暟浼犻€掞細

```

  # modprobe snd-aloop timer_source="-1.4.<utimer_id>"

```
鍏朵腑 `utimer_id` 鏄綘鐢?`SNDRV_TIMER_IOCTL_CREATE` 鍒涘缓鐨勫畾鏃跺櫒 id锛岃€?`4` 鏄?鐢ㄦ埛绌洪棿椹卞姩瀹氭椂鍣ㄨ澶囩殑缂栧彿锛坄SNDRV_TIMER_GLOBAL_UDRIVEN`锛夈€?
鐢ㄤ簬 snd-aloop 鐨勭敤鎴风┖闂撮┍鍔?ALSA 瀹氭椂鍣ㄧ殑 `resolution` 搴旇绠椾负 `1000000000ULL / frame_rate * period_size`锛?鍥犱负瀹氭椂鍣ㄥ皢鍦ㄦ瘡鍑嗗濂戒竴涓柊-period 鐨勫抚鏃惰Е鍙戜竴娆°€?
涔嬪悗锛屾瘡褰撲綘鐢?`SNDRV_TIMER_IOCTL_TRIGGER` 瑙﹀彂瀹氭椂鍣ㄦ椂锛屾柊涓€-period 鐨勬暟鎹氨浼氫粠涓€涓?snd-aloop 璁惧浼犺緭鍒板彟涓€涓€?