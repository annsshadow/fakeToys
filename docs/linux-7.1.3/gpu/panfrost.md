
##  drm/Panfrost Mali 椹卞姩


## Panfrost DRM 瀹㈡埛绔娇鐢ㄧ粺璁″疄鐜?


drm/Panfrost 椹卞姩瀹炵幇浜?DRM 瀹㈡埛绔娇鐢ㄧ粺璁¤鑼冿紝濡?
drm-client-usage-stats 涓墍杩般€?

浠ヤ笅绀轰緥灞曠ず浜嗗凡瀹炵幇鐨勯敭鍊煎浠ュ強褰撳墠
鎵€鏈夊彲鑳界殑鏍煎紡閫夐」锛?

```
      pos:    0
      flags:  02400002
      mnt_id: 27
      ino:    531
      drm-driver:     panfrost
      drm-client-id:  14
      drm-engine-fragment:    1846584880 ns
      drm-cycles-fragment:    1424359409
      drm-maxfreq-fragment:   799999987 Hz
      drm-curfreq-fragment:   799999987 Hz
      drm-engine-vertex-tiler:        71932239 ns
      drm-cycles-vertex-tiler:        52617357
      drm-maxfreq-vertex-tiler:       799999987 Hz
      drm-curfreq-vertex-tiler:       799999987 Hz
      drm-total-memory:       290 MiB
      drm-shared-memory:      0 MiB
      drm-active-memory:      226 MiB
      drm-resident-memory:    36496 KiB
      drm-purgeable-memory:   128 KiB

```
鍙兘鐨?`drm-engine-` 閿悕涓猴細`fragment`锛屼互鍙?`vertex-tiler`銆?
`drm-curfreq-` 鍊艰〃绀鸿寮曟搸褰撳墠鐨勮繍琛岄鐜囥€?

鐢ㄦ埛蹇呴』娉ㄦ剰锛屽嚭浜庣渷鐢佃€冭檻锛屽紩鎿庡拰鍛ㄦ湡閲囨牱榛樿鏄鐢ㄧ殑锛?
`fdinfo` 鐢ㄦ埛鍜屽熀鍑嗘祴璇曞簲鐢ㄧ▼搴忓湪鏌ヨ fdinfo 鏂囦欢鏃?
蹇呴』纭繚鍒囨崲浠诲姟鐨勬€ц兘鍒嗘瀽鐘舵€侊細

```
    echo <N> > /sys/bus/platform/drivers/panfrost/[a-f0-9]*.gpu/profiling

```
鍏朵腑 `N` 涓?`0` 鎴?`1`锛屽彇鍐充簬鏈熸湜鐨勫惎鐢ㄧ姸鎬併€?
