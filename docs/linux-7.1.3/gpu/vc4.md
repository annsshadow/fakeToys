##  drm/vc4 Broadcom VC4 鍥惧舰椹卞姩


   :doc: Broadcom VC4 Graphics Driver

## 鏄剧ず纭欢澶勭悊


鏈妭娑电洊涓庢樉绀虹‖浠剁浉鍏崇殑涓€鍒囷紝鍖呮嫭妯″紡璁剧疆鍩虹璁炬柦銆佸钩闈紙plane锛夈€?绮剧伒锛坰prite锛変笌鍏夋爣澶勭悊鍜屾樉绀恒€佽緭鍑烘帰娴嬩互鍙婄浉鍏充富棰樸€?
### 鍍忕礌闃€锛圖RM CRTC锛?

   :doc: VC4 CRTC module

### HVS


   :doc: VC4 HVS module.

### HVS 骞抽潰


   :doc: VC4 plane module

### HDMI 缂栫爜鍣?

   :doc: VC4 Falcon HDMI module

### DSI 缂栫爜鍣?

   :doc: VC4 DSI0/DSI1 module

### DPI 缂栫爜鍣?

   :doc: VC4 DPI module

### VEC锛堝鍚堢數瑙嗚緭鍑猴級缂栫爜鍣?

   :doc: VC4 SDTV module

## KUnit 娴嬭瘯


VC4 椹卞姩浣跨敤 KUnit 鎵ц椹卞姩鐗瑰畾鐨勫崟鍏冧笌闆嗘垚娴嬭瘯銆?
杩欎簺娴嬭瘯浣跨敤妯℃嫙椹卞姩锛屽彲鍦?arm 鎴?arm64 鏋舵瀯涓婁娇鐢ㄤ互涓嬪懡浠よ繍琛岋細


	$ ./tools/testing/kunit/kunit.py run \
		--kunitconfig=drivers/gpu/drm/vc4/tests/.kunitconfig \
		--cross_compile aarch64-linux-gnu- --arch arm64

褰撳墠宸茶娴嬭瘯瑕嗙洊鐨勯┍鍔ㄩ儴鍒嗗寘鎷細
 - HVS 鍒?PixelValve 鐨勫姩鎬?FIFO 鍒嗛厤锛岄€傜敤浜?BCM2835-7 鍜?BCM2711銆?
## 鍐呭瓨绠＄悊涓?3D 鍛戒护鎻愪氦


鏈妭娑电洊 vc4 椹卞姩涓殑 GEM 瀹炵幇銆?
### GPU 缂撳啿鍖哄璞★紙BO锛夌鐞?

   :doc: VC4 GEM BO management support

### V3D binner 鍛戒护鍒楄〃锛圔CL锛夋牎楠?

   :doc: Command list validator for VC4.

### V3D 娓叉煋鍛戒护鍒楄〃锛圧CL锛夌敓鎴?

   :doc: Render command list generation

### VC4 鐨勭潃鑹插櫒鏍￠獙鍣?

   :doc: Shader validator for VC4.

### V3D 涓柇


   :doc: Interrupt management for the V3D engine
