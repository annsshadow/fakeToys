
## GPIO 瀛楃璁惧鐢ㄦ埛绌洪棿 API


杩欐槸瀛楃璁惧 API 鐨勬渶鏂扮増鏈紙v2锛夛紝濡?`include/uapi/linux/gpio.h.` 鎵€瀹氫箟銆?
棣栨娣诲姞浜?5.10銆?
   涓嶈婊ョ敤鐢ㄦ埛绌洪棿 API 鏉ユ帶鍒跺凡鏈夊悎閫傚唴鏍搁┍鍔ㄧ殑纭欢銆傚彲鑳藉凡缁忔湁閫傚悎浣犵敤渚嬬殑椹卞姩锛岃€岀幇鏈夌殑鍐呮牳椹卞姩蹇呭畾姣斾粠鐢ㄦ埛绌洪棿浣嶆搷浣滐紙bitbashing锛夋彁渚涙洿浼樼殑鏂规銆?
   璇烽槄璇?Documentation/driver-api/gpio/drivers-on-gpio.rst 浠ラ伩鍏嶅湪鐢ㄦ埛绌洪棿涓噸鏂板彂鏄庡唴鏍歌疆瀛愩€?
   鍚屾牱锛屽浜庡鍔熻兘绾胯矾锛屽彲鑳芥湁鍏朵粬瀛愮郴缁燂紝濡?Documentation/spi/index.rst銆丏ocumentation/i2c/index.rst銆丏ocumentation/driver-api/pwm.rst銆丏ocumentation/w1/index.rst 绛夛紝涓轰綘鐨勭‖浠舵彁渚涘悎閫傜殑椹卞姩鍜?API銆?
浣跨敤瀛楃璁惧 API 鐨勫熀鏈ず渚嬪彲鍦?`tools/gpio/*` 涓壘鍒般€?
璇?API 鍥寸粫涓や釜涓昏瀵硅薄鏋勫缓锛歡pio-v2-chip 鍜?gpio-v2-line-request銆?

## Chip


Chip 浠ｈ〃鍗曚釜 GPIO 鑺墖锛屽苟閫氳繃褰㈠ `/dev/gpiochipX` 鐨勮澶囨枃浠舵毚闇茬粰鐢ㄦ埛绌洪棿銆?
姣忎釜鑺墖鏀寔涓€瀹氭暟閲忕殑 GPIO 绾胯矾锛宍chip.lines<gpiochip_info>`銆傝姱鐗囦笂鐨勭嚎璺€氳繃鑼冨洿浠?0 鍒?`chip.lines - 1` 鐨?`offset` 鏍囪瘑锛屽嵆 `[0,chip.lines)`銆?
绾胯矾閫氳繃 gpio-v2-get-line-ioctl.rst 浠庤姱鐗囪姹傦紝鎵€寰楃殑琛岃姹傜敤浜庤闂?GPIO 鑺墖鐨勭嚎璺垨鐩戣绾胯矾鐨勮竟娌夸簨浠躲€?
鍦ㄦ湰鏂囨。涓紝鍦?GPIO 璁惧鏂囦欢涓婅皟鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮绉颁负 `chip_fd`銆?
### 鎿嶄綔


鍙 chip 鎵ц浠ヤ笅鎿嶄綔锛?
- [鑾峰彇绾胯矾](gpio-v2-get-line-ioctl)
- [鑾峰彇鑺墖淇℃伅](gpio-get-chipinfo-ioctl)
- [鑾峰彇绾胯矾淇℃伅](gpio-v2-get-lineinfo-ioctl)
- [鐩戣绾胯矾淇℃伅](gpio-v2-get-lineinfo-watch-ioctl)
- [鍙栨秷鐩戣绾胯矾淇℃伅](gpio-get-lineinfo-unwatch-ioctl)
- [璇诲彇绾胯矾淇℃伅鍙樻洿浜嬩欢](gpio-v2-lineinfo-changed-read)


## 琛岃姹傦紙Line Request锛?

琛岃姹傜敱 gpio-v2-get-line-ioctl.rst 鍒涘缓锛屽苟鎻愪緵瀵逛竴缁勮璇锋眰绾胯矾鐨勮闂€傝璇锋眰閫氳繃 gpio-v2-get-line-ioctl.rst 鍦?`request.fd<gpio_v2_line_request>` 涓繑鍥炵殑鍖垮悕鏂囦欢鎻忚堪绗︽毚闇茬粰鐢ㄦ埛绌洪棿銆?
鍦ㄦ湰鏂囨。涓紝琛岃姹傛枃浠舵弿杩扮绉颁负 `req_fd`銆?
### 鎿嶄綔


鍙琛岃姹傛墽琛屼互涓嬫搷浣滐細

- [鑾峰彇绾胯矾鍊糫(gpio-v2-line-get-values-ioctl)
- [璁剧疆绾胯矾鍊糫(gpio-v2-line-set-values-ioctl)
- [璇诲彇绾胯矾杈规部浜嬩欢](gpio-v2-line-event-read)
- [閲嶆柊閰嶇疆绾胯矾](gpio-v2-line-set-config-ioctl)

## 绫诲瀷


鏈妭鍖呭惈 API v2 鎵€寮曠敤鐨勭粨鏋勪綋鍜屾灇涓撅紝瀹氫箟浜?`include/uapi/linux/gpio.h`銆?
   :identifiers:
    gpio_v2_line_attr_id
    gpio_v2_line_attribute
    gpio_v2_line_changed_type
    gpio_v2_line_config
    gpio_v2_line_config_attribute
    gpio_v2_line_event
    gpio_v2_line_event_id
    gpio_v2_line_flag
    gpio_v2_line_info
    gpio_v2_line_info_changed
    gpio_v2_line_request
    gpio_v2_line_values
    gpiochip_info

- [閿欒鐮乚(error-codes)
