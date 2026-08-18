
##  drm/vkms 铏氭嫙鍐呮牳妯″紡璁剧疆锛圴irtual Kernel Modesetting锛?

   :doc: vkms (Virtual Kernel Modesetting)

## 璁剧疆


VKMS 椹卞姩鍙互閫氳繃浠ヤ笅姝ラ杩涜璁剧疆锛?
```
  lsmod | grep vkms

```
杩欏簲褰撲細鍒楀嚭 VKMS 椹卞姩銆傚鏋滄病鏈変换浣曡緭鍑猴紝閭ｄ箞浣犻渶瑕佸惎鐢ㄥ拰/鎴栧姞杞?VKMS 椹卞姩銆?
纭繚 VKMS 椹卞姩宸插湪浣犵殑
```
  make nconfig

  Go to `Device Drivers> Graphics support`

  Enable `Virtual KMS (EXPERIMENTAL)`

```
缂栬瘧骞舵瀯寤哄唴鏍革紝浠ヤ娇鏇存敼鐢熸晥銆?```

  sudo modprobe vkms

```
鐜板湪杩愯 lsmod 鍛戒护锛孷KMS 椹卞姩灏嗕細鍑虹幇鍦ㄥ垪琛ㄤ腑銆備綘涔熷彲浠ヨ瀵熷埌椹卞姩鍦?dmesg 鏃ュ織涓鍔犺浇銆?
VKMS 椹卞姩鍏锋湁鐢ㄤ簬妯℃嫙涓嶅悓绫诲瀷纭欢鐨勫彲閫夌壒鎬э紝瀹冧滑浣滀负妯″潡閫夐」鏆撮湶鍑烘潵銆備綘鍙互浣跨敤 `modinfo` 鍛戒护
```
  modinfo vkms

```
妯″潡閫夐」鍦ㄦ祴璇曟椂寰堟湁甯姪锛屽苟涓斿彲浠ュ湪鍔犺浇 vkms 鏃跺惎鐢ㄦā鍧椼€備緥濡傦紝瑕佸姞杞藉惎鐢ㄤ簡鍏夋爣鐨?vkms锛?```
  sudo modprobe vkms enable_cursor=1

```
```
  sudo modprobe -r vkms

```
## 閫氳繃 Configfs 閰嶇疆


鍙互閫氳繃 configfs 鍒涘缓骞堕厤缃涓?VKMS 瀹炰緥銆?
```
  sudo mount -t configfs none /config
  sudo modprobe vkms

```
涓€鏃?VKMS 琚姞杞斤紝`/config/vkms` 浼氳嚜鍔ㄥ垱寤恒€傛瘡涓洰褰?```
  sudo mkdir /config/vkms/my-vkms

```
```
  cat /config/vkms/my-vkms/enabled
  0

```
```
  tree /config/vkms/my-vkms
  鈹溾攢鈹€ connectors
  鈹溾攢鈹€ crtcs
  鈹溾攢鈹€ enabled
  鈹溾攢鈹€ encoders
  鈹斺攢鈹€ planes

```
瑕佸悜鏄剧ず娴佹按绾挎坊鍔犻」鐩紝鍦ㄥ彲鐢ㄨ矾寰勪笅鍒涘缓涓€涓垨澶氫釜鐩綍銆?
```
  sudo mkdir /config/vkms/my-vkms/planes/plane0

```
骞抽潰锛圥lane锛夋湁 1 涓彲閰嶇疆灞炴€э細

- type锛氬钩闈㈢被鍨嬶細0 鍙犲姞灞傦紙overlay锛夛紝1 涓诲钩闈紙primary锛夛紝2 鍏夋爣锛坈ursor锛夛紙涓庡钩闈㈢殑 "type" 灞炴€ф毚闇茬殑鍊肩浉鍚岋級

```
  sudo mkdir /config/vkms/my-vkms/crtcs/crtc0

```
CRTC 鏈?1 涓彲閰嶇疆灞炴€э細

- writeback锛氶€氳繃鍐欏叆 1 鎴?0 鏉ュ惎鐢ㄦ垨绂佺敤鍥炲啓锛坵riteback锛夎繛鎺ュ櫒鏀寔

```
  sudo mkdir /config/vkms/my-vkms/encoders/encoder0

```
```
  sudo mkdir /config/vkms/my-vkms/connectors/connector0

```
杩炴帴鍣紙Connector锛夋湁 1 涓彲閰嶇疆灞炴€э細

- status锛氳繛鎺ョ姸鎬侊細1 宸茶繛鎺ワ紝2 宸叉柇寮€锛? 鏈煡锛堜笌杩炴帴鍣ㄧ殑 "status" 灞炴€ф毚闇茬殑鍊肩浉鍚岋級

```
  sudo ln -s /config/vkms/my-vkms/crtcs/crtc0 /config/vkms/my-vkms/planes/plane0/possible_crtcs
  sudo ln -s /config/vkms/my-vkms/crtcs/crtc0 /config/vkms/my-vkms/encoders/encoder0/possible_crtcs
  sudo ln -s /config/vkms/my-vkms/encoders/encoder0 /config/vkms/my-vkms/connectors/connector0/possible_encoders

```
```
  echo "1" | sudo tee /config/vkms/my-vkms/planes/plane0/type

```
```
  echo "1" | sudo tee /config/vkms/my-vkms/enabled

```
```
  echo "0" | sudo tee /config/vkms/my-vkms/enabled

```
```
  sudo rm /config/vkms/my-vkms/planes/*/possible_crtcs/*
  sudo rm /config/vkms/my-vkms/encoders/*/possible_crtcs/*
  sudo rm /config/vkms/my-vkms/connectors/*/possible_encoders/*
  sudo rmdir /config/vkms/my-vkms/planes/*
  sudo rmdir /config/vkms/my-vkms/crtcs/*
  sudo rmdir /config/vkms/my-vkms/encoders/*
  sudo rmdir /config/vkms/my-vkms/connectors/*
  sudo rmdir /config/vkms/my-vkms

```
## 浣跨敤 IGT 娴嬭瘯


IGT GPU Tools 鏄竴涓笓闂ㄧ敤浜?DRM 椹卞姩璋冭瘯鍜屽紑鍙戠殑娴嬭瘯濂椾欢銆?
IGT 宸ュ叿鍙互浠?`here <https://gitlab.freedesktop.org/drm/igt-gpu-tools>`_ 瀹夎銆?
娴嬭瘯闇€瑕佸湪娌℃湁鍚堟垚鍣紙compositor锛夌殑鎯呭喌涓嬭繍琛岋紝鎵€浠ヤ綘闇€瑕佸垏鎹㈠埌鏂囨湰
```
  sudo systemctl isolate multi-user.target

```
```
  sudo systemctl isolate graphical.target

```
涓€鏃﹁繘鍏ョ函鏂囨湰妯″紡锛屼綘灏卞彲浠ヤ娇鐢?IGT_FORCE_DRIVER 鍙橀噺鏉ユ寚瀹氭兂瑕佹祴璇曠殑椹卞姩鐨勮澶囪繃婊ゅ櫒鏉ヨ繍琛屾祴璇曘€?
IGT_FORCE_DRIVER 涔熷彲浠ヤ笌 run-tests.sh 鑴氭湰涓€璧蜂娇鐢ㄦ潵杩愯
```
  sudo IGT_FORCE_DRIVER="vkms" ./build/tests/<name of test>
  sudo IGT_FORCE_DRIVER="vkms" ./scripts/run-tests.sh -t <name of test>

```
渚嬪锛岃娴嬭瘯鍥炲啓锛坵riteback锛夊簱鐨勫姛鑳斤紝
```
  sudo IGT_FORCE_DRIVER="vkms" ./build/tests/kms_writeback
  sudo IGT_FORCE_DRIVER="vkms" ./scripts/run-tests.sh -t kms_writeback

```
```
  sudo IGT_FORCE_DRIVER="vkms" ./build/tests/kms_flip --run-subtest basic-plain-flip

```
## 浣跨敤 KUnit 娴嬭瘯


KUnit锛堝唴鏍稿崟鍏冩祴璇曟鏋讹級涓?Linux 鍐呮牳涓殑鍗曞厓娴嬭瘯鎻愪緵浜嗕竴涓€氱敤妗嗘灦銆?
鏇村淇℃伅瑙?../dev-tools/kunit/index.rst銆?
```
  tools/testing/kunit/kunit.py run --kunitconfig=drivers/gpu/drm/vkms/tests

```
## TODO


濡傛灉浣犳兂鍋氫笅闈㈠垪鍑虹殑浠讳綍涓€椤癸紝璇蜂笌 VKMS 缁存姢鑰呭垎浜綘鐨勫叴瓒ｃ€?
### 鏀硅繘 IGT 鏀寔


璋冭瘯锛?
- kms_plane锛氫竴浜涙祴璇曠敤渚嬪洜涓烘崟鑾?CRC 瓒呮椂鑰屽け璐ワ紱

铏氭嫙纭欢锛堟棤 vblank锛夋ā寮忥細

- VKMS 宸茬粡鏀寔閫氳繃 hrtimer 妯℃嫙鐨?vblank锛屽彲浠ョ敤 kms_flip 娴嬭瘯鏉ラ獙璇侊紱鍦ㄦ煇绉嶇▼搴︿笂锛屽彲浠ヨ VKMS 宸茬粡妯℃嫙浜嗙湡瀹炵‖浠剁殑 vblank銆備笉杩囷紝鎴戜滑涔熸湁涓嶆敮鎸?vblank 涓柇銆佸苟绔嬪嵆瀹屾垚 page_flip 浜嬩欢鐨勮櫄鎷熺‖浠讹紱鍦ㄨ繖绉嶆儏鍐典笅锛屽悎鎴愬櫒寮€鍙戣€呭彲鑳戒細鍦ㄨ櫄鎷熺‖浠朵笂闄峰叆蹇欏惊鐜€傚湪 VKMS 涓敮鎸佽櫄鎷熺‖浠惰涓轰細寰堟湁鐢紝鍥犱负杩欏彲浠ュ府鍔╁悎鎴愬櫒寮€鍙戣€呭湪澶氱鍦烘櫙涓嬫祴璇曚粬浠殑鐗规€с€?
### 娣诲姞骞抽潰鐗规€?

鏈夊緢澶氬钩闈㈢壒鎬ф垜浠彲浠ュ鍔犳敮鎸侊細

- 娣诲姞鑳屾櫙鑹?KMS 灞炴€閫傚悎鍏ラ棬]銆?
- 缂╂斁锛圫caling锛夈€?
- 棰濆鐨勭紦鍐插尯鏍煎紡銆備綆/楂樹綅娣憋紙bpp锛夌殑 RGB 鏍煎紡浼氬緢鏈夋剰鎬漑閫傚悎鍏ラ棬]銆?
- 寮傛鏇存柊锛堢洰鍓嶄粎鑳戒娇鐢ㄦ棫鐨?cursor api 鍦ㄥ厜鏍囧钩闈笂瀹炵幇锛夈€?
瀵逛簬鎵€鏈夎繖浜涳紝鎴戜滑涔熷笇鏈涘鏌?igt 娴嬭瘯瑕嗙洊鐜囷紝骞剁‘淇濇墍鏈夌浉鍏崇殑 igt 娴嬭瘯鐢ㄤ緥鍦?vkms 涓婃甯稿伐浣溿€傚畠浠槸瀹炰範椤圭洰鐨勪笉閿欓€夋嫨銆?
### 杩愯鏃堕厤缃?

鎴戜滑甯屾湜鑳藉閲嶆柊閰嶇疆 vkms 瀹炰緥锛岃€屾棤闇€閫氳繃 configfs 閲嶆柊鍔犺浇妯″潡銆備娇鐢?娴嬭瘯鐢ㄤ緥锛?
- 鍔ㄦ€佺儹鎻掓嫈/鐑Щ闄よ繛鎺ュ櫒锛堜互渚胯兘澶熸祴璇曞悎鎴愬櫒瀵?DP MST 鐨勫鐞嗭級銆?
- 鏇存敼杈撳嚭閰嶇疆锛氭彃鎷斿睆骞曘€佹洿鏀?EDID銆佸厑璁告洿鏀瑰埛鏂扮巼銆?
### 鍥炲啓鏀寔


- 鍥炲啓鍜?CRC 鎹曡幏鎿嶄綔鍏变韩 composer_enabled 甯冨皵鍊肩殑浣跨敤浠ョ‘淇?vblank銆傚彲鑳藉綋杩欎簺鎿嶄綔涓€璧峰伐浣滄椂锛宑omposer_enabled 闇€瑕佸 composer 鐘舵€佸仛寮曠敤璁℃暟鎵嶈兘姝ｅ父宸ヤ綔銆俒閫傚悎鍏ラ棬]

- 澧炲姞瀵瑰厠闅嗗洖鍐欒緭鍑虹殑鏀寔锛屼互鍙婄浉鍏崇殑娴嬭瘯鐢ㄤ緥锛堝湪 IGT kms_writeback 涓娇鐢ㄥ厠闅嗚緭鍑猴級銆?
- 浣滀负涓€涓?v4l 璁惧銆傝繖瀵逛簬鍦ㄧ壒娈?vkms 閰嶇疆涓婅皟璇曞悎鎴愬櫒寰堟湁鐢紝浠ヤ究寮€鍙戣€呯湅鍒扮湡姝ｅ彂鐢熺殑鎯呭喌銆?
### 杈撳嚭鐗规€?

- 鍙彉鍒锋柊鐜?freesync 鏀寔銆傝繖鍙兘 Prime 缂撳啿鍖哄叡浜敮鎸侊紝浠ヤ究鎴戜滑鍙互浣跨敤 vgem fence 鍦ㄦ祴璇曚腑妯℃嫙娓叉煋銆傝繕闇€瑕佹敮鎸佹寚瀹?EDID銆?
- 澧炲姞瀵?link status 鐨勬敮鎸侊紝浠ヤ究鍚堟垚鍣ㄥ彲浠ュ湪渚嬪 Display Port 閾捐矾鍑洪棶棰樻椂楠岃瘉瀹冧滑鐨勮繍琛屾椂鍥為€€鏂规銆?
### CRC API 鏀硅繘


- 浼樺寲 CRC 璁＄畻 `compute_crc()` 鍜屽钩闈㈡贩鍚?`blend()`

### 浣跨敤 eBPF 杩涜鍘熷瓙妫€鏌?

鍘熷瓙锛圓tomic锛夐┍鍔ㄦ湁璁稿闄愬埗锛岃繖浜涢檺鍒跺苟鏈互浠讳綍鏄惧紡褰㈠紡锛堜緥濡傞€氳繃鍙兘鐨勫睘鎬у€硷級鏆撮湶缁欑敤鎴风┖闂淬€傜敤鎴风┖闂村彧鑳介€氳繃 atomic IOCTL锛堝彲鑳戒娇鐢?TEST_ONLY 鏍囧織锛夋潵鏌ヨ杩欎簺闄愬埗銆傝瘯鍥句负鎵€鏈夎繖浜涢檺鍒舵坊鍔犲彲閰嶇疆浠ｇ爜锛屼互渚垮悎鎴愬櫒鑳藉閽堝瀹冧滑琚祴璇曪紝灏嗘槸涓€椤圭浉褰撳緬鍔崇殑宸ヤ綔銆傜浉鍙嶏紝鎴戜滑鍙互澧炲姞瀵?eBPF 鐨勬敮鎸佹潵楠岃瘉浠讳綍绫诲瀷鐨勫師瀛愮姸鎬侊紝骞跺疄鐜颁竴涓寘鍚笉鍚岄檺鍒剁殑搴撱€?
杩欓渶瑕佷竴澶ф壒鐗规€э紙骞抽潰鍚堟垚銆佸杈撳嚭鈥︹€︼級宸茬粡鍚敤鎵嶆湁鎰忎箟銆?