
## GPIO 瀛楃璁惧鐢ㄦ埛绌洪棿 API锛坴1锛?

   鏈?API 宸茶 chardev.rst锛坴2锛夊彇浠ｃ€?
   鏂板紑鍙戝簲浣跨敤 v2 API锛屽苟榧撳姳宸叉湁寮€鍙戝敖蹇縼绉伙紝鍥犱负姝?API 灏嗗湪鏈潵琚Щ闄ゃ€倂2 API 鏄?v1 API
   鍦ㄥ姛鑳戒笂鐨勮秴闆嗭紝鍥犳浠讳綍 v1 璋冪敤閮藉彲浠ョ洿鎺ョ炕璇戜负绛変环鐨?v2 璋冪敤銆?
   鍦ㄨ縼绉绘湡闂达紝姝ゆ帴鍙ｅ皢缁х画寰楀埌缁存姢锛屼絾鏂扮壒鎬у彧浼氭坊鍔犲埌鏂扮殑 API 涓€?
棣栨鍔犲叆浜?4.8銆?
璇?API 鍥寸粫涓変釜涓昏瀵硅薄鏋勫缓锛歡pio-v1-chip銆乬pio-v1-line-handle 涓?gpio-v1-line-event銆?
褰撴湰鏂囨。涓娇鐢ㄢ€渓ine event鈥濇椂锛屽畠鎸囩殑鏄彲浠ョ洃瑙嗕竴鏉＄嚎璺笂杈规部浜嬩欢鐨勮姹傦紝鑰屼笉鏄竟娌夸簨浠舵湰韬€?
## 鑺墖


Chip 浠ｈ〃涓€涓崟鐙殑 GPIO 鑺墖锛屽苟閫氳繃褰㈠ `/dev/gpiochipX` 鐨勮澶囨枃浠舵毚闇茬粰鐢ㄦ埛绌洪棿銆?
姣忎釜鑺墖鏀寔鑻ュ共鏉?GPIO 绾匡紝`chip.lines<gpiochip_info>`銆傝姱鐗囦笂鐨勭嚎鐢变竴涓湪浠?0 鍒?`chip.lines - 1` 鑼冨洿鍐呯殑 `offset` 鏍囪瘑锛屽嵆 `[0,chip.lines)`銆?
绾块€氳繃 gpio-get-linehandle-ioctl.rst 浠庤姱鐗囪姹傦紝寰楀埌鐨勭嚎鍙ユ焺鐢ㄤ簬璁块棶 GPIO 鑺墖鐨勭嚎锛涙垨
閫氳繃 gpio-get-lineevent-ioctl.rst锛屽緱鍒扮殑绾夸簨浠剁敤浜庣洃瑙嗕竴鏉?GPIO 绾夸笂鐨勮竟娌夸簨浠躲€?
鍦ㄦ湰鏂囨。涓紝鍦?GPIO 璁惧鏂囦欢涓婅皟鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮琚О涓?`chip_fd`銆?
### 鎿嶄綔


鍙互瀵硅姱鐗囨墽琛屼互涓嬫搷浣滐細

- [鑾峰彇绾垮彞鏌刔(gpio-get-linehandle-ioctl)
- [鑾峰彇绾夸簨浠禲(gpio-get-lineevent-ioctl)
- [鑾峰彇鑺墖淇℃伅](gpio-get-chipinfo-ioctl)
- [鑾峰彇绾夸俊鎭痌(gpio-get-lineinfo-ioctl)
- [鐩戣绾夸俊鎭痌(gpio-get-lineinfo-watch-ioctl)
- [鍙栨秷鐩戣绾夸俊鎭痌(gpio-get-lineinfo-unwatch-ioctl)
- [璇诲彇绾夸俊鎭彉鏇翠簨浠禲(gpio-lineinfo-changed-read)

## 绾垮彞鏌?

绾垮彞鏌勭敱 gpio-get-linehandle-ioctl.rst 鍒涘缓锛屾彁渚涘涓€缁勫凡璇锋眰绾跨殑璁块棶銆傜嚎鍙ユ焺閫氳繃
gpio-get-linehandle-ioctl.rst 鍦?`request.fd<gpiohandle_request>` 涓繑鍥炵殑鍖垮悕鏂囦欢鎻忚堪绗?鏆撮湶缁欑敤鎴风┖闂淬€?
鍦ㄦ湰鏂囨。涓紝绾垮彞鏌勬枃浠舵弿杩扮琚О涓?`handle_fd`銆?
### 鎿嶄綔


鍙互瀵圭嚎鍙ユ焺鎵ц浠ヤ笅鎿嶄綔锛?
- [鑾峰彇绾垮€糫(gpio-handle-get-line-values-ioctl)
- [璁剧疆绾垮€糫(gpio-handle-set-line-values-ioctl)
- [閲嶆柊閰嶇疆绾縘(gpio-handle-set-config-ioctl)

## 绾夸簨浠?

绾夸簨浠剁敱 gpio-get-lineevent-ioctl.rst 鍒涘缓锛屾彁渚涘涓€鏉″凡璇锋眰绾跨殑璁块棶銆傜嚎浜嬩欢閫氳繃
gpio-get-lineevent-ioctl.rst 鍦?`request.fd<gpioevent_request>` 涓繑鍥炵殑鍖垮悕鏂囦欢鎻忚堪绗?鏆撮湶缁欑敤鎴风┖闂淬€?
鍦ㄦ湰鏂囨。涓紝绾夸簨浠舵枃浠舵弿杩扮琚О涓?`event_fd`銆?
### 鎿嶄綔


鍙互瀵圭嚎浜嬩欢鎵ц浠ヤ笅鎿嶄綔锛?
- [鑾峰彇绾垮€糫(gpio-handle-get-line-values-ioctl)
- [璇诲彇绾胯竟娌夸簨浠禲(gpio-lineevent-data-read)

## 绫诲瀷


鏈妭鍖呭惈 ABI v1 鎵€寮曠敤鐨勭粨鏋勪綋銆?
`struct gpiochip_info<gpiochip_info>` 鍦?ABI v1 涓?v2 涓€氱敤銆?
   :identifiers:
    gpioevent_data
    gpioevent_request
    gpiohandle_config
    gpiohandle_data
    gpiohandle_request
    gpioline_info
    gpioline_info_changed

- [error-codes](error-codes)
