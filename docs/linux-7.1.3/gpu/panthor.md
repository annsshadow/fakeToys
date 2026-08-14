
## drm/Panthor CSF 椹卞姩


## Panthor DRM 瀹㈡埛绔娇鐢ㄧ粺璁″疄鐜?

drm/Panthor 椹卞姩瀹炵幇浜?DRM 瀹㈡埛绔娇鐢ㄧ粺璁¤鑼冿紝濡?drm-client-usage-stats 涓墍杩般€?
灞曠ず鎵€瀹炵幇鐨勯敭鍊煎浠ュ強褰撳墠鍙兘鏍煎紡閫夐」鐨勫畬鏁存€х殑杈撳嚭绀轰緥锛?
```
     pos:    0
     flags:  02400002
     mnt_id: 29
     ino:    491
     drm-driver:     panthor
     drm-client-id:  10
     drm-engine-panthor:     111110952750 ns
     drm-cycles-panthor:     94439687187
     drm-maxfreq-panthor:    1000000000 Hz
     drm-curfreq-panthor:    1000000000 Hz
     panthor-resident-memory:        10396 KiB
     panthor-active-memory:  10396 KiB
     drm-total-memory:       16480 KiB
     drm-shared-memory:      0
     drm-active-memory:      16200 KiB
     drm-resident-memory:    16480 KiB
     drm-purgeable-memory:   0

```
鍙兘鐨?`drm-engine-` 閿悕鏈夛細`panthor`銆?`drm-curfreq-` 鍊艰〃绀鸿寮曟搸褰撳墠鐨勮繍琛岄鐜囥€?
鐢ㄦ埛蹇呴』璁颁綇锛屽嚭浜庣渷鐢佃€冭檻锛屽紩鎿庡拰鍛ㄦ湡閲囨牱榛樿鏄鐢ㄧ殑銆俙fdinfo` 鐢ㄦ埛浠ュ強鏌ヨ fdinfo 鏂囦欢鐨勫熀鍑嗘祴璇曞簲鐢ㄧ▼搴忓繀椤荤‘淇濆垏鎹綔涓氱殑鎬ц兘鍒嗘瀽鐘舵€侊細
```

    echo <N> > /sys/bus/platform/drivers/panthor/[a-f0-9]*.gpu/profiling

```
鍏朵腑 `N` 鏄竴涓綅鎺╃爜锛屽叾涓懆鏈熷拰鏃堕棿鎴抽噰鏍峰垎鍒敱绗竴浣嶅拰绗簩浣嶅惎鐢ㄣ€?
鍙兘鐨?`panthor-*-memory` 閿湁锛歚active` 鍜?`resident`銆?杩欎簺鍊艰〃绀哄唴閮ㄧ敱椹卞姩鎷ユ湁鐨?shmem BO 鐨勫ぇ灏忥紝杩欎簺 BO 娌℃湁閫氳繃 DRM handle 鏆撮湶缁欑敤鎴风┖闂达紝渚嬪闃熷垪鐜舰缂撳啿鍖恒€佸悓姝ュ璞℃暟缁勫拰鍫嗗潡銆傚洜涓哄畠浠兘鍦ㄥ垱寤烘椂鍒嗛厤骞跺浐瀹氾紝鎵€浠ュ彧闇€ `panthor-resident-memory` 鍗冲彲璇存槑瀹冧滑鐨勫ぇ灏忋€俙panthor-active-memory` 鏄剧ず褰撳墠姝ｈ GPU 璋冨害鎵ц銆佷笌 VM 鍜岀粍鍏宠仈鐨勯┍鍔?BO 鐨勫ぇ灏忋€?