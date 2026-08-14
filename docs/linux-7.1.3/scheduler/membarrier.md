
## membarrier() 绯荤粺璋冪敤


## MEMBARRIER_CMD_{PRIVATE,GLOBAL}_EXPEDITED - 鏋舵瀯瑕佹眰


### 鏇存柊 rq->curr 涔嬪墠鐨勫唴瀛樺睆闅?


鍛戒护 MEMBARRIER_CMD_PRIVATE_EXPEDITED 鍜?MEMBARRIER_CMD_GLOBAL_EXPEDITED 瑕佹眰姣忎釜鏋舵瀯鍦ㄤ粠鐢ㄦ埛绌洪棿杩斿洖鍚庛€佹洿鏂?rq->curr 涔嬪墠鍏锋湁涓€涓畬鏁村唴瀛樺睆闅溿€傝灞忛殰鐢?__schedule() 涓殑 rq_lock(); smp_mb__after_spinlock() 搴忓垪闅愬惈鎻愪緵銆傝灞忛殰涓?membarrier 绯荤粺璋冪敤閫€鍑洪檮杩戠殑涓€涓畬鏁村睆闅滅浉鍖归厤锛屽弬瑙?membarrier_{private,global}_expedited()銆?

### 鏇存柊 rq->curr 涔嬪悗鐨勫唴瀛樺睆闅?


鍛戒护 MEMBARRIER_CMD_PRIVATE_EXPEDITED 鍜?MEMBARRIER_CMD_GLOBAL_EXPEDITED 瑕佹眰姣忎釜鏋舵瀯鍦ㄦ洿鏂?rq->curr 涔嬪悗銆佽繑鍥炵敤鎴风┖闂翠箣鍓嶅叿鏈変竴涓畬鏁村唴瀛樺睆闅溿€傚悇涓灦鏋勪笂鎻愪緵璇ュ睆闅滅殑鏂规濡備笅銆?

 - alpha銆乤rc銆乤rm銆乭exagon銆乵ips 渚濊禆 finish_lock_switch() 涓?spin_unlock() 闅愬惈鐨勫畬鏁村睆闅溿€?

 - arm64 渚濊禆 switch_to() 闅愬惈鐨勫畬鏁村睆闅溿€?

 - powerpc銆乺iscv銆乻390銆乻parc銆亁86 渚濊禆 switch_mm() 闅愬惈鐨勫畬鏁村睆闅滐紙鑻?mm 涓嶄负 NULL锛夛紱鍚﹀垯瀹冧滑渚濊禆 mmdrop() 闅愬惈鐨勫畬鏁村睆闅溿€傚湪 powerpc 鍜?riscv 涓婏紝switch_mm() 渚濊禆 membarrier_arch_switch_mm()銆?

璇ュ睆闅滀笌 membarrier 绯荤粺璋冪敤鍏ュ彛闄勮繎鐨勪竴涓畬鏁村睆闅滅浉鍖归厤锛屽弬瑙?membarrier_{private,global}_expedited()銆?
