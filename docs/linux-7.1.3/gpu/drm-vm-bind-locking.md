
## VM_BIND 閿佹満鍒?

鏈枃璇曞浘鎻忚堪瑕佷娇 VM_BIND 閿佹満鍒舵纭墍闇€鐨勫唴瀹癸紝鍖呮嫭 userptr mmu_notifier 閿併€?瀹冭繕璁ㄨ浜嗕竴浜涗紭鍖栵紝浠ユ秷闄ゅ湪鏈€绠€鍗曞疄鐜颁腑鎵€闇€鐨勯亶鍘嗘墍鏈?userptr 鏄犲皠浠ュ強
澶栭儴/鍏变韩瀵硅薄鏄犲皠鐨勫紑閿€銆傛澶栵紝杩樻湁涓€鑺傛弿杩颁簡瀹炵幇鍙仮澶嶉〉閿欒锛坮ecoverable
pagefaults锛夋墍闇€鐨?VM_BIND 閿佹満鍒躲€?
## DRM GPUVM 杈呭姪鍑芥暟闆?

瀵逛簬瀹炵幇 VM_BIND 鐨勯┍鍔紝瀛樺湪涓€缁勮緟鍔╁嚱鏁帮紝杩欑粍杈呭姪鍑芥暟瀹炵幇浜嗘湰鏂囨弿杩扮殑閿佹満鍒朵腑
鐨勫ぇ閮ㄥ垎锛堜絾骞堕潪鍏ㄩ儴锛夈€傜壒鍒槸锛屽畠鐩墠杩樼己灏?userptr 鐨勫疄鐜般€傛湰鏂囨棤鎰忚缁嗘弿杩?DRM GPUVM 鐨勫疄鐜帮紝浣嗗叾鍐呭宸插湪 :ref:`鍏惰嚜韬殑鏂囨。 <drm_gpuvm>` 涓兜鐩栥€傚己鐑堝缓璁?浠讳綍瀹炵幇 VM_BIND 鐨勯┍鍔ㄤ娇鐢?DRM GPUVM 杈呭姪鍑芥暟锛屽苟鍦ㄧ己灏戦€氱敤鍔熻兘鏃跺鍏惰繘琛屾墿灞曘€?
## 鏈琛?

- `gpu_vm`锛氬甫鏈夊厓鏁版嵁鐨勮櫄鎷?GPU 鍦板潃绌洪棿鐨勬娊璞°€傞€氬父姣忎釜瀹㈡埛绔紙DRM 鏂囦欢绉佹湁锛?  鎴栦竴涓墽琛屼笂涓嬫枃瀵瑰簲涓€涓€?- `gpu_vma`锛歡pu_vm 鍐呬竴娈靛甫鏈夌浉鍏冲厓鏁版嵁鐨?GPU 鍦板潃鑼冨洿鐨勬娊璞°€俫pu_vma 鐨勫悗澶囧瓨鍌?  鍙互鏄竴涓?GEM 瀵硅薄锛屼篃鍙互鏄悓鏃舵槧灏勫埌璇ヨ繘绋?CPU 鍦板潃绌洪棿鐨勫尶鍚嶆垨椤电紦瀛?  锛坧age-cache锛夐〉銆?- `gpu_vm_bo`锛欸EM 瀵硅薄涓?VM 涔嬮棿鍏宠仈鐨勬娊璞°€侴EM 瀵硅薄缁存姢涓€涓?gpu_vm_bos 鍒楄〃锛?  鍏朵腑姣忎釜 gpu_vm_bo 鍙堢淮鎶や竴涓?gpu_vmas 鍒楄〃銆?- `userptr gpu_vma` 鎴栫畝绉?`userptr`锛氫竴绉?gpu_vma锛屽叾鍚庡瀛樺偍鏄涓婃墍杩扮殑鍖垮悕鎴?  椤电紦瀛橀〉銆?- `revalidating`锛堥噸鏂伴獙璇侊級锛氶噸鏂伴獙璇佷竴涓?gpu_vma 鏄寚浣垮悗澶囧瓨鍌ㄧ殑鏈€鏂扮増鏈┗鐣欙紝
  骞剁‘淇濊 gpu_vma 鐨勯〉琛ㄩ」鎸囧悜璇ュ悗澶囧瓨鍌ㄣ€?- `dma_fence`锛氫竴涓笌 struct completion 绫讳技鐨?struct dma_fence锛岀敤浜庤窡韪?GPU 娲诲姩銆?  褰?GPU 娲诲姩瀹屾垚鏃讹紝dma_fence 鍙戝嚭淇″彿銆傝鍙傞槄
  [dma-buf doc </driver-api/dma-buf>](dma-buf doc </driver-api/dma-buf>) 涓殑
  `DMA Fences` 涓€鑺傘€?- `dma_resv`锛氫竴涓?struct dma_resv锛堝張绉?reservation object锛夛紝鐢ㄤ簬浠ュ涓?  dma_fences 鐨勫舰寮忚窡韪?gpu_vm 鎴?GEM 瀵硅薄涓婄殑 GPU 娲诲姩銆俤ma_resv 鍖呭惈涓€涓?  dma_fences 鐨勬暟缁?鍒楄〃锛屼互鍙婁竴涓湪鍚?dma_resv 娣诲姞棰濆 dma_fences 鏃跺繀椤绘寔鏈夌殑
  閿併€傝閿佺殑绫诲瀷鍏佽浠ヤ换鎰忛『搴忓澶氫釜 dma_resvs 杩涜鏃犳閿佺殑瀹夊叏鍔犻攣銆傝鍙傞槄
  [dma-buf doc </driver-api/dma-buf>](dma-buf doc </driver-api/dma-buf>) 涓殑
  `Reservation Objects` 涓€鑺傘€?- `exec function`锛堟墽琛屽嚱鏁帮級锛氫竴涓噸鏂伴獙璇佹墍鏈夊彈褰卞搷鐨?gpu_vmas銆佹彁浜や竴涓?GPU
  鍛戒护鎵规锛屽苟鍚戞墍鏈夊彈褰卞搷鐨?dma_resvs 娉ㄥ唽浠ｈ〃璇?GPU 鍛戒护娲诲姩鐨?dma_fence 鐨勫嚱鏁般€?  涓哄畬鏁磋捣瑙侊紙灏界鏈枃鏈兜鐩栵級锛屽€煎緱涓€鎻愮殑鏄紝exec function 涔熷彲鑳藉氨鏄煇浜涢┍鍔ㄥ湪
  璁＄畻/闀胯繍琛屾ā寮忎笅浣跨敤鐨勯噸鏂伴獙璇?worker銆?- `local object`锛堟湰鍦板璞★級锛氫粎鏄犲皠鍦ㄥ崟涓?VM 鍐呯殑 GEM 瀵硅薄銆傛湰鍦?GEM 瀵硅薄鍏变韩
  gpu_vm 鐨?dma_resv銆?- `external object`锛堝閮ㄥ璞★紝鍙堢О shared object锛夛細鍙兘琚涓?gpu_vms 鍏变韩銆佷笖鍏?  鍚庡瀛樺偍鍙兘涓庡叾浠栭┍鍔ㄥ叡浜殑 GEM 瀵硅薄銆?
## 閿佷笌鍔犻攣椤哄簭


VM_BIND 鐨勫ソ澶勪箣涓€鏄紝鏈湴 GEM 瀵硅薄鍏变韩 gpu_vm 鐨?dma_resv 瀵硅薄锛屼粠鑰屼篃鍏变韩
dma_resv 閿併€傚洜姝わ紝鍗充娇鏈夋暟閲忓簽澶х殑鏈湴 GEM 瀵硅薄锛屼篃鍙渶涓€鎶婇攣鍗冲彲浣?exec
搴忓垪鎴愪负鍘熷瓙鐨勩€?
浣跨敤鐨勯攣涓庡姞閿侀『搴忓涓嬶細

- `gpu_vm->lock`锛堝彲閫変负 rwsem锛夈€備繚鎶?gpu_vm 涓褰?gpu_vmas 鐨勬暟鎹粨鏋勩€傚畠涔熷彲浠?  淇濇姢 gpu_vm 鐨?userptr gpu_vmas 鍒楄〃銆傜敤 CPU mm 鏉ョ被姣旂殑璇濓紝杩欑浉褰撲簬 mmap_lock銆?  涓€涓?rwsem 鍏佽澶氫釜璇昏€呭苟鍙戝湴閬嶅巻 VM 鏍戯紝浣嗚繖绉嶅苟鍙戝甫鏉ョ殑濂藉寰堝彲鑳藉洜椹卞姩鑰屽紓銆?- `userptr_seqlock`銆傝閿佸湪 gpu_vm 鐨?userptr 鍒楄〃涓殑姣忎釜 userptr gpu_vma 涓婁互
  璇绘ā寮忚幏鍙栵紝骞跺湪 mmu notifier 澶辨晥锛坕nvalidation锛夋湡闂翠互鍐欐ā寮忚幏鍙栥€傚畠骞堕潪鐪熸鐨?  seqlock锛岃€屾槸鍦?`mm/mmu_notifier.c` 涓鎻忚堪涓衡€滅鎾為噸璇曪紙Collision-retry锛夌殑
  璇讳晶/鍐欎晶鈥橀攣鈥欙紝寰堝儚 seqcount銆備笉杩囪繖鍏佽澶氫釜鍐欎晶鍚屾椂鎸佹湁瀹冣€︹€︹€濄€傝渚т复鐣屽尯鐢?  ``mmu_interval_read_begin() / mmu_interval_read_retry()` 鍖呰９锛屽綋鍐欎晶琚寔鏈夋椂
  `mmu_interval_read_begin()`` 浼氫紤鐪犮€傚啓渚у湪鍐呮牳璋冪敤 mmu interval 澶辨晥 notifier
  鏃剁敱鏍稿績 mm 鎸佹湁銆?- `gpu_vm->resv` 閿併€備繚鎶?gpu_vm 涓渶瑕侀噸鏂扮粦瀹氱殑 gpu_vmas 鍒楄〃锛屼互鍙?gpu_vm 鎵€鏈?  鏈湴 GEM 瀵硅薄鐨勯┗鐣欑姸鎬併€傛澶栵紝瀹冮€氬父杩樹繚鎶?gpu_vm 鐨勫凡鍥炴敹锛坋victed锛夊拰澶栭儴 GEM
  瀵硅薄鍒楄〃銆?- `gpu_vm->userptr_notifier_lock`銆傝繖鏄竴涓?rwsem锛屽湪 exec 鏈熼棿浠ヨ妯″紡鑾峰彇锛屽湪
  mmu notifier 澶辨晥鏈熼棿浠ュ啓妯″紡鑾峰彇銆倁serptr notifier 閿佹槸姣?gpu_vm 鐨勩€?- `gem_object->gpuva_lock`銆傝閿佷繚鎶?GEM 瀵硅薄鐨?gpu_vm_bos 鍒楄〃銆傚畠閫氬父涓?GEM 瀵硅薄
  鐨?dma_resv 鏄悓涓€鎶婇攣锛屼絾鏈変簺椹卞姩浠ヤ笉鍚屾柟寮忎繚鎶よ鍒楄〃锛岃涓嬫枃銆?- `gpu_vm 鍒楄〃鑷棆閿侊紙list spinlocks锛塦銆傚湪鏌愪簺瀹炵幇涓紝闇€瑕佸畠浠墠鑳芥洿鏂?gpu_vm 鐨?  宸插洖鏀跺拰澶栭儴瀵硅薄鍒楄〃銆傚浜庨偅浜涘疄鐜帮紝鍦ㄥ鐞嗗垪琛ㄦ椂浼氳幏鍙栬繖浜涜嚜鏃嬮攣銆傜劧鑰岋紝涓轰簡閬垮厤
  涓?dma_resv 閿佷骇鐢熷姞閿侀『搴忓啿绐侊紝鍦ㄩ亶鍘嗗垪琛ㄦ椂闇€瑕佷竴绉嶇壒娈婄殑鏂规銆?

## gpu_vm_bos 涓?gpu_vmas 鐨勪繚鎶や笌鐢熷懡鍛ㄦ湡


GEM 瀵硅薄鐨?gpu_vm_bos 鍒楄〃锛屼互鍙?gpu_vm_bo 鐨?gpu_vmas 鍒楄〃锛岀敱
`gem_object->gpuva_lock` 淇濇姢锛岃閿侀€氬父涓?GEM 瀵硅薄鐨?dma_resv 鐩稿悓锛涗絾濡傛灉椹卞姩闇€瑕?浠?dma_fence 鍙戜俊鍙凤紙signalling锛変复鐣屽尯鍐呰闂繖浜涘垪琛紝瀹冨彲浠ラ€夋嫨鏀圭敤涓€鎶婂崟鐙殑閿侊紝
璇ラ攣鍙互鍦?dma_fence 鍙戜俊鍙蜂复鐣屽尯鍐呰閿佸畾銆傝繖绫婚┍鍔ㄩ殢鍚庨渶瑕侀澶栨敞鎰忥細鍦ㄩ亶鍘?gpu_vm_bo 鍜?gpu_vma 鍒楄〃鐨勫惊鐜唴閮紝闇€瑕佽幏鍙栧摢浜涢攣锛屼互閬垮厤鍔犻攣椤哄簭鍐茬獊銆?
DRM GPUVM 杈呭姪鍑芥暟闆嗕細鎻愪緵 lockdep 鏂█锛岃〃鏄庡湪鐩稿叧鎯呭舰涓嬫閿佸凡琚寔鏈夛紝骞朵笖杩樻彁渚?涓€绉嶈鑷韩鐭ユ檽瀹為檯浣跨敤浜嗗摢鎶婇攣鐨勬墜娈碉細`drm_gem_gpuva_set_lock`銆?
姣忎釜 gpu_vm_bo 鎸佹湁鎸囧悜搴曞眰 GEM 瀵硅薄鐨勫紩鐢ㄨ鏁版寚閽堬紝姣忎釜 gpu_vma 鎸佹湁鎸囧悜
gpu_vm_bo 鐨勫紩鐢ㄨ鏁版寚閽堛€傚綋閬嶅巻 GEM 瀵硅薄鐨?gpu_vm_bos 鍒楄〃浠ュ強 gpu_vm_bo 鐨?gpu_vmas 鍒楄〃鏃讹紝涓嶅緱閲婃斁 `gem_object->gpuva_lock`锛屽惁鍒欙紝闄勫姞鍒版煇涓?gpu_vm_bo 涓婄殑
gpu_vmas 鍙兘浼氬湪姣棤寰佸厗鐨勬儏鍐典笅娑堝け锛屽洜涓哄畠浠笉鏄紩鐢ㄨ鏁扮殑銆傞┍鍔ㄥ彲浠ュ疄鐜拌嚜宸辩殑
鏂规鏉ュ厑璁歌繖鏍峰仛锛屼絾杩欎細浠ュ鍔犲鏉傛€т负浠ｄ环锛屽苟涓旇秴鍑轰簡鏈枃鐨勮寖鍥淬€?
鍦?DRM GPUVM 瀹炵幇涓紝姣忎釜 gpu_vm_bo 鍜屾瘡涓?gpu_vma 閮芥寔鏈夊 gpu_vm 鑷韩鐨勫紩鐢ㄨ鏁般€?鍥犳锛屽苟涓斾负浜嗛伩鍏嶅惊鐜紩鐢ㄨ鏁帮紝gpu_vm 鐨?gpu_vmas 鐨勬竻鐞嗕笉寰椾粠 gpu_vm 鐨勬瀽鏋勫嚱鏁颁腑
杩涜銆傞┍鍔ㄩ€氬父浼氬疄鐜颁竴涓?gpu_vm close 鍑芥暟鏉ヨ繘琛屾娓呯悊銆俫pu_vm close 鍑芥暟浼氫腑姝?浣跨敤璇?VM 鐨?GPU 鎵ц銆佽В闄ゆ墍鏈?gpu_vmas 鐨勬槧灏勫苟閲婃斁椤佃〃鍐呭瓨銆?
## 鏈湴瀵硅薄鐨勯噸鏂伴獙璇佷笌鍥炴敹


璇锋敞鎰忥紝涓嬮潰缁欏嚭鐨勬墍鏈変唬鐮佺ず渚嬫垜浠兘浣跨敤浜嗙畝鍖栫殑浼唬鐮併€傜壒鍒槸锛宒ma_resv 姝婚攣閬垮厤
绠楁硶浠ュ強涓?dma_resv fences 棰勭暀鍐呭瓨閮借鐪佺暐浜嗐€?
閲嶆柊楠岃瘉
____________
鍦?VM_BIND 涓嬶紝褰?GPU 浣跨敤 gpu_vm 鎵ц鏃讹紝鎵€鏈夋湰鍦板璞￠兘蹇呴』澶勪簬椹荤暀鐘舵€侊紝骞朵笖杩欎簺
瀵硅薄闇€瑕佸缓绔嬫寚鍚戝畠浠殑鏈夋晥 gpu_vmas銆傚洜姝わ紝閫氬父姣忔 GPU 鍛戒护缂撳啿鍖虹殑鎻愪氦涔嬪墠閮戒細
鏈変竴涓噸鏂伴獙璇侊紙re-validation锛夊尯娈碉細


   dma_resv_lock(gpu_vm->resv);

   // Validation section starts here.
   for_each_gpu_vm_bo_on_evict_list(&gpu_vm->evict_list, &gpu_vm_bo) {
           validate_gem_bo(&gpu_vm_bo->gem_bo);

           // The following list iteration needs the Gem object's
           // dma_resv to be held (it protects the gpu_vm_bo's list of
           // gpu_vmas, but since local gem objects share the gpu_vm's
           // dma_resv, it is already held at this point.
           for_each_gpu_vma_of_gpu_vm_bo(&gpu_vm_bo, &gpu_vma)
                  move_gpu_vma_to_rebind_list(&gpu_vma, &gpu_vm->rebind_list);
   }

   for_each_gpu_vma_on_rebind_list(&gpu vm->rebind_list, &gpu_vma) {
           rebind_gpu_vma(&gpu_vma);
           remove_gpu_vma_from_rebind_list(&gpu_vma);
   }
   // Validation section ends here, and job submission starts.

   add_dependencies(&gpu_job, &gpu_vm->resv);
   job_dma_fence = gpu_submit(&gpu_job));

   add_dma_fence(job_dma_fence, &gpu_vm->resv);
   dma_resv_unlock(gpu_vm->resv);

涔嬫墍浠ラ渶瑕佷竴涓崟鐙殑 gpu_vm 閲嶆柊缁戝畾鍒楄〃锛屾槸鍥犱负鍙兘瀛樺湪鍚屾牱闇€瑕侀噸鏂扮粦瀹氱殑 userptr
gpu_vmas锛岃€屽畠浠苟鏈槧灏勬煇涓紦鍐插尯瀵硅薄銆?
鍥炴敹
________
鍏朵腑涓€涓湰鍦板璞＄殑鍥炴敹灏嗙被浼间簬涓嬮潰杩欐牱锛?

   obj = get_object_from_lru();

   dma_resv_lock(obj->resv);
   for_each_gpu_vm_bo_of_obj(obj, &gpu_vm_bo);
           add_gpu_vm_bo_to_evict_list(&gpu_vm_bo, &gpu_vm->evict_list);

   add_dependencies(&eviction_job, &obj->resv);
   job_dma_fence = gpu_submit(&eviction_job);
   add_dma_fence(&obj->resv, job_dma_fence);

   dma_resv_unlock(&obj->resv);
   put_object(obj);

璇锋敞鎰忥紝鐢变簬璇ュ璞℃槸 gpu_vm 鏈湴鐨勶紝瀹冨皢鍏变韩 gpu_vm 鐨?dma_resv 閿侊紝鍗?`obj->resv == gpu_vm->resv`銆傝鏍囪涓哄洖鏀剁殑 gpu_vm_bos 琚斁鍒?gpu_vm 鐨勫洖鏀跺垪琛ㄤ笂锛?璇ュ垪琛ㄧ敱 `gpu_vm->resv` 淇濇姢銆傚湪鍥炴敹鏈熼棿锛屾墍鏈夋湰鍦板璞＄殑 dma_resv 閮借閿佸畾锛屽苟涓旂敱浜?涓婅堪绛夊紡锛屼繚鎶?gpu_vm 鍥炴敹鍒楄〃鐨?gpu_vm 鐨?dma_resv 涔熻閿佸畾銆?
鍦?VM_BIND 涓嬶紝gpu_vmas 鍦ㄥ洖鏀朵箣鍓嶆棤闇€瑙ｇ粦锛屽洜涓洪┍鍔ㄥ繀椤荤‘淇濆洖鏀剁殑 blit 鎴栨嫹璐濅細绛夊緟
GPU 绌洪棽鎴栦緷璧栦簬涔嬪墠鎵€鏈夌殑 GPU 娲诲姩銆傛澶栵紝GPU 闅忓悗閫氳繃璇?gpu_vma 璁块棶宸查噴鏀惧唴瀛樼殑
浠讳綍灏濊瘯锛岄兘浼氱敱涓€涓甫鏈夐噸鏂伴獙璇佸尯娈电殑鏂扮殑 exec function 鍏堣锛岃鍖烘浼氱‘淇濇墍鏈?gpu_vmas 閮借閲嶆柊缁戝畾銆傚洖鏀朵唬鐮佸湪閲嶆柊楠岃瘉鏈熼棿鎸佹湁瀵硅薄鐨?dma_resv锛屽皢纭繚鏂扮殑 exec
function 涓嶄細涓庡洖鏀跺彂鐢熺珵浜夈€?
椹卞姩鍙互杩欐牱瀹炵幇锛氬湪姣忔 exec function 涓紝鍙€夋嫨涓€閮ㄥ垎 vmas 杩涜閲嶆柊缁戝畾銆傚湪杩欑
鎯呭喌涓嬶紝鎵€鏈?*鏈?*琚€変腑杩涜閲嶆柊缁戝畾鐨?vmas 蹇呴』鍦?exec function 宸ヤ綔璐熻浇鎻愪氦涔嬪墠
瑙ｇ粦銆?
## 浣跨敤澶栭儴缂撳啿鍖哄璞℃椂鐨勫姞閿?

鐢变簬澶栭儴缂撳啿鍖哄璞″彲鑳借澶氫釜 gpu_vms 鍏变韩锛屽畠浠棤娉曚笌鍗曚釜 gpu_vm 鍏变韩鍏?reservation
瀵硅薄銆傜浉鍙嶏紝瀹冧滑闇€瑕佹嫢鏈夎嚜宸辩殑 reservation 瀵硅薄銆備娇鐢ㄤ竴涓垨澶氫釜 gpu_vmas 缁戝畾鍒版煇涓?gpu_vm 鐨勫閮ㄥ璞★紝鍥犳琚斁鍒颁竴涓瘡 gpu_vm 鐨勫垪琛ㄤ笂锛岃鍒楄〃鐢?gpu_vm 鐨?dma_resv 閿?鎴栨煇涓?gpu_vm 鍒楄〃鑷棆閿佷繚鎶?<Spinlock iteration>銆備竴鏃?gpu_vm 鐨?reservation 瀵硅薄
琚攣瀹氾紝閬嶅巻澶栭儴瀵硅薄鍒楄〃骞堕攣瀹氭墍鏈夊閮ㄥ璞＄殑 dma_resvs 灏辨槸瀹夊叏鐨勩€傜劧鑰岋紝濡傛灉鏀圭敤鍒楄〃
鑷棆閿侊紝鍒欓渶瑕佷娇鐢ㄤ竴绉嶆洿澶嶆潅鐨勯亶鍘嗘柟妗堛€?
鍦ㄥ洖鏀舵椂锛屽閮ㄥ璞℃墍缁戝畾鐨?*鎵€鏈?* gpu_vms 鐨?gpu_vm_bos 閮介渶瑕佽鏀惧埌瀹冧滑鍚勮嚜鐨?gpu_vm 鐨勫洖鏀跺垪琛ㄤ笂銆傜劧鑰岋紝褰撳洖鏀朵竴涓閮ㄥ璞℃椂锛岃瀵硅薄鎵€缁戝畾鐨?gpu_vms 鐨?dma_resvs
閫氬父骞舵湭琚寔鏈夈€傚彧鏈夊璞＄鏈夌殑 dma_resv 鍙互淇濊瘉琚寔鏈夈€傚鏋滃湪鍥炴敹鏃舵墜澶存湁涓€涓?ww_acquire 涓婁笅鏂囷紝鎴戜滑鍙互鑾峰彇閭ｄ簺 dma_resvs锛屼絾杩欏彲鑳藉鑷翠唬浠烽珮鏄傜殑 ww_mutex
鍥炴粴銆備竴涓畝鍗曠殑鍋氭硶鏄細浠呯敤 `evicted` 甯冨皵鍊兼爣璁拌鍥炴敹鐨?gem 瀵硅薄鐨?gpu_vm_bos锛屽苟鍦?涓嬫闇€瑕侀亶鍘嗙浉搴旂殑 gpu_vm 鍥炴敹鍒楄〃涔嬪墠妫€鏌ヨ甯冨皵鍊笺€備緥濡傦紝鍦ㄩ亶鍘嗗閮ㄥ璞″垪琛ㄥ苟閿佸畾
瀹冧滑鏃躲€傛鏃讹紝gpu_vm 鐨?dma_resv 鍜屽璞＄殑 dma_resv 閮借鎸佹湁锛屼簬鏄鏍囪涓哄凡鍥炴敹鐨?gpu_vm_bo 灏卞彲浠ヨ娣诲姞鍒?gpu_vm 鐨勫凡鍥炴敹 gpu_vm_bos 鍒楄〃涓€傝 `evicted` 甯冨皵鍊煎湪褰㈠紡
涓婄敱瀵硅薄鐨?dma_resv 淇濇姢銆?
exec function 鍙樹负锛?

   dma_resv_lock(gpu_vm->resv);

   // External object list is protected by the gpu_vm->resv lock.
   for_each_gpu_vm_bo_on_extobj_list(gpu_vm, &gpu_vm_bo) {
           dma_resv_lock(gpu_vm_bo.gem_obj->resv);
           if (gpu_vm_bo_marked_evicted(&gpu_vm_bo))
                   add_gpu_vm_bo_to_evict_list(&gpu_vm_bo, &gpu_vm->evict_list);
   }

   for_each_gpu_vm_bo_on_evict_list(&gpu_vm->evict_list, &gpu_vm_bo) {
           validate_gem_bo(&gpu_vm_bo->gem_bo);

           for_each_gpu_vma_of_gpu_vm_bo(&gpu_vm_bo, &gpu_vma)
                  move_gpu_vma_to_rebind_list(&gpu_vma, &gpu_vm->rebind_list);
   }

   for_each_gpu_vma_on_rebind_list(&gpu vm->rebind_list, &gpu_vma) {
           rebind_gpu_vma(&gpu_vma);
           remove_gpu_vma_from_rebind_list(&gpu_vma);
   }

   add_dependencies(&gpu_job, &gpu_vm->resv);
   job_dma_fence = gpu_submit(&gpu_job));

   add_dma_fence(job_dma_fence, &gpu_vm->resv);
   for_each_external_obj(gpu_vm, &obj)
          add_dma_fence(job_dma_fence, &obj->resv);
   dma_resv_unlock_all_resv_locks();

涓庝箣瀵瑰簲鐨勩€佸彲鎰熺煡鍏变韩瀵硅薄鐨勫洖鏀剁湅璧锋潵鍍忚繖鏍凤細


   obj = get_object_from_lru();

   dma_resv_lock(obj->resv);
   for_each_gpu_vm_bo_of_obj(obj, &gpu_vm_bo)
           if (object_is_vm_local(obj))
                add_gpu_vm_bo_to_evict_list(&gpu_vm_bo, &gpu_vm->evict_list);
           else
                mark_gpu_vm_bo_evicted(&gpu_vm_bo);

   add_dependencies(&eviction_job, &obj->resv);
   job_dma_fence = gpu_submit(&eviction_job);
   add_dma_fence(&obj->resv, job_dma_fence);

   dma_resv_unlock(&obj->resv);
   put_object(obj);


## 鍦ㄦ湭鎸佹湁 dma_resv 閿佺殑鎯呭喌涓嬭闂?gpu_vm 鐨勫垪琛?

鏈変簺椹卞姩鍦ㄨ闂?gpu_vm 鐨勫洖鏀跺垪琛ㄥ拰澶栭儴瀵硅薄鍒楄〃鏃朵細鎸佹湁 gpu_vm 鐨?dma_resv 閿併€傜劧鑰岋紝
涔熸湁浜涢┍鍔ㄩ渶瑕佸湪涓嶆寔鏈?dma_resv 閿佺殑鎯呭喌涓嬭闂繖浜涘垪琛紝渚嬪鐢变簬鏉ヨ嚜 dma_fence 鍙戜俊鍙?涓寸晫鍖哄唴閮ㄧ殑寮傛鐘舵€佹洿鏂般€傚湪杩欑鎯呭喌涓嬶紝鍙互浣跨敤鑷棆閿佹潵淇濇姢瀵瑰垪琛ㄧ殑鎿嶇旱銆傜劧鑰岋紝鐢变簬鍦?閬嶅巻鍒楄〃鏃堕渶瑕佸姣忎釜鍒楄〃椤硅幏鍙栨洿楂樼骇鍒殑鐫＄湢閿侊紝宸茬粡閬嶅巻杩囩殑椤归渶瑕佽涓存椂绉诲姩鍒颁竴涓鏈?鍒楄〃锛屽苟鍦ㄥ鐞嗘瘡涓€椤规椂閲婃斁鑷棆閿侊細


    struct list_head still_in_list;

    INIT_LIST_HEAD(&still_in_list);

    spin_lock(&gpu_vm->list_lock);
    do {
            struct list_head *entry = list_first_entry_or_null(&gpu_vm->list, head);

            if (!entry)
                    break;

            list_move_tail(&entry->head, &still_in_list);
            list_entry_get_unless_zero(entry);
            spin_unlock(&gpu_vm->list_lock);

            process(entry);

            spin_lock(&gpu_vm->list_lock);
            list_entry_put(entry);
    } while (true);

    list_splice_tail(&still_in_list, &gpu_vm->list);
    spin_unlock(&gpu_vm->list_lock);

鐢变簬棰濆鐨勫姞閿佸拰鍘熷瓙鎿嶄綔锛岄偅浜?*鑳藉**閬垮厤鍦ㄨ dma_resv 閿佷箣澶栬闂?gpu_vm 鍒楄〃鐨勯┍鍔紝
鍙兘涔熷笇鏈涢伩鍏嶈繖绉嶉亶鍘嗘柟妗堛€傜壒鍒槸锛屽鏋滈┍鍔ㄩ鏈熷垪琛ㄩ」鏁伴噺寰堝ぇ銆傚浜庨偅浜涢鏈熷垪琛ㄩ」鏁伴噺
杈冨皯銆佸垪琛ㄩ亶鍘嗕笉甯稿彂鐢熴€佹垨鑰呮瘡娆￠亶鍘嗘湁鏄捐憲棰濆寮€閿€鐨勬儏鍐碉紝杩欑被閬嶅巻鎵€娑夊強鐨勫師瀛愭搷浣滃紑閿€
寰堝彲鑳芥槸鍙拷鐣ョ殑銆傝娉ㄦ剰锛屽鏋滀娇鐢ㄦ鏂规锛屽繀椤荤‘淇濊鍒楄〃閬嶅巻鐢卞灞傞攣鎴栦俊鍙烽噺淇濇姢锛屽洜涓?鍒楄〃椤瑰湪閬嶅巻鏃朵細琚复鏃朵粠鍒楄〃涓婃憳涓嬶紱杩樺€煎緱涓€鎻愮殑鏄紝鏈湴鍒楄〃 `still_in_list` 涔熷簲琚涓?鍙?`gpu_vm->list_lock` 淇濇姢锛屽洜姝ゅ湪鍒楄〃閬嶅巻鏈熼棿锛岄」涔熷彲鑳戒粠鏈湴鍒楄〃涓骞跺彂绉婚櫎銆?
璇峰弬闃?:ref:`DRM GPUVM 鍔犻攣涓€鑺?<drm_gpuvm_locking>` 鍙婂叾鍐呴儴鐨?`get_next_vm_bo_from_list` 鍑芥暟銆?

## userptr gpu_vmas


userptr gpu_vma 鏄竴绉?gpu_vma锛屽畠涓嶆槸灏嗙紦鍐插尯瀵硅薄鏄犲皠鍒颁竴娈?GPU 铏氭嫙鍦板潃鑼冨洿锛岃€屾槸
鐩存帴鏄犲皠涓€娈?CPU mm 鐨勫尶鍚嶆垨鏂囦欢椤电紦瀛橀〉銆?涓€绉嶉潪甯哥畝鍗曠殑鏂规硶鏄湪缁戝畾鏃剁敤 pin_user_pages() 鍥哄畾杩欎簺椤碉紝鍦ㄨВ缁戞椂鍙栨秷鍥哄畾锛屼絾杩欎細
閫犳垚鎷掔粷鏈嶅姟锛圖enial-Of-Service锛夐殣鎮ｏ紝鍥犱负鍗曚釜鐢ㄦ埛绌洪棿杩涚▼灏辫兘澶熷浐瀹氫綇绯荤粺鐨勫叏閮ㄥ唴瀛橈紝
杩欐槸涓嶅彲鍙栫殑銆傦紙涓嶈繃锛屽浜庣壒娈婄敤渚嬪苟涓斿亣璁炬湁鎭板綋鐨勮璐︼紝鍥哄畾浠嶅彲鑳芥槸涓€涓彲鍙栫殑鐗规€э級銆?鍦ㄤ竴鑸儏鍐典笅锛屾垜浠渶瑕佸仛鐨勬槸锛氳幏鍙栨寚鍚戞墍闇€椤电殑寮曠敤锛岀‘淇濆湪 CPU mm 瑙ｉ櫎鏄犲皠杩欎簺椤典箣鍓?閫氳繃 MMU notifier 寰楀埌閫氱煡锛屽鏋滃畠浠笉鏄互鍙鏂瑰紡鏄犲皠鍒?GPU 鍒欏皢鍏舵爣璁颁负鑴忥紝鐒跺悗閲婃斁
璇ュ紩鐢ㄣ€傚綋鎴戜滑琚?MMU notifier 閫氱煡 CPU mm 鍗冲皢涓㈠純杩欎簺椤垫椂锛屾垜浠渶瑕侀€氳繃鍦ㄨ MMU
notifier 涓瓑寰?VM 绌洪棽鏉ュ仠姝?GPU 瀵硅繖浜涢〉鐨勮闂紝骞剁‘淇濆湪 GPU 涓嬫灏濊瘯璁块棶 CPU mm
鑼冨洿鍐呭綋鍓嶅瓨鍦ㄧ殑浠讳綍鍐呭涔嬪墠锛屽皢鏃х殑椤典粠 GPU 椤佃〃涓В闄ゆ槧灏勶紝骞堕噸澶嶈幏鍙栨柊椤靛紩鐢ㄧ殑杩囩▼銆?锛堝弬瑙佷笅鏂囩殑 :ref:`notifier 绀轰緥 <Invalidation example>`锛夈€傝娉ㄦ剰锛屽綋鏍稿績 mm 鍐冲畾
鍥炴敹锛坙aundry锛夐〉鏃讹紝鎴戜滑浼氭敹鍒拌繖鏍风殑瑙ｉ櫎鏄犲皠 MMU 閫氱煡锛屽苟鍙互鍦ㄤ笅娆?GPU 璁块棶涔嬪墠鍐嶆
灏嗚繖浜涢〉鏍囪涓鸿剰銆傛垜浠繕鏀跺埌绫讳技鐨勭敤浜?NUMA 璁拌处鐨?MMU 閫氱煡锛孏PU 椹卞姩鍏跺疄鏃犻渶鍏冲績杩欎簺锛?浣嗚縿浠婁负姝紝瑕佸皢鏌愪簺閫氱煡鎺掗櫎鍦ㄥ浠嶅緢鍥伴毦銆?
灏?MMU notifier 鐢ㄤ簬璁惧 DMA锛堜互鍙婂叾浠栨柟娉曪級鍦?pin_user_pages() 鏂囨。
<mmu-notifier-registration-case> 涓湁鎻忚堪銆?
鐜板湪锛屼娇鐢?get_user_pages() 鑾峰彇 struct page 寮曠敤鐨勬柟寮忥紝涓嶅垢鐨勬槸鏃犳硶鍦?dma_resv 閿佷笅
浣跨敤锛屽洜涓洪偅浼氳繚鍙?dma_resv 閿佷笌瑙ｅ喅 CPU 椤甸敊璇椂鑾峰彇鐨?mmap_lock 涔嬮棿鐨勫姞閿侀『搴忋€傝繖
鎰忓懗鐫€ gpu_vm 鐨?userptr gpu_vmas 鍒楄〃闇€瑕佺敱涓€鎶婂灞傞攣淇濇姢锛屽湪鎴戜滑鐨勪笅渚嬩腑鏄?`gpu_vm->lock`銆?
userptr gpu_vma 鐨?MMU interval seqlock 鎸夊涓嬫柟寮忎娇鐢細


   // Exclusive locking mode here is strictly needed only if there are
   // invalidated userptr gpu_vmas present, to avoid concurrent userptr
   // revalidations of the same userptr gpu_vma.
   down_write(&gpu_vm->lock);
   retry:

   // Note: mmu_interval_read_begin() blocks until there is no
   // invalidation notifier running anymore.
   seq = mmu_interval_read_begin(&gpu_vma->userptr_interval);
   if (seq != gpu_vma->saved_seq) {
           obtain_new_page_pointers(&gpu_vma);
           dma_resv_lock(&gpu_vm->resv);
           add_gpu_vma_to_revalidate_list(&gpu_vma, &gpu_vm);
           dma_resv_unlock(&gpu_vm->resv);
           gpu_vma->saved_seq = seq;
   }

   // The usual revalidation goes here.

   // Final userptr sequence validation may not happen before the
   // submission dma_fence is added to the gpu_vm's resv, from the POW
   // of the MMU invalidation notifier. Hence the
   // userptr_notifier_lock that will make them appear atomic.

   add_dependencies(&gpu_job, &gpu_vm->resv);
   down_read(&gpu_vm->userptr_notifier_lock);
   if (mmu_interval_read_retry(&gpu_vma->userptr_interval, gpu_vma->saved_seq)) {
          up_read(&gpu_vm->userptr_notifier_lock);
          goto retry;
   }

   job_dma_fence = gpu_submit(&gpu_job));

   add_dma_fence(job_dma_fence, &gpu_vm->resv);

   for_each_external_obj(gpu_vm, &obj)
          add_dma_fence(job_dma_fence, &obj->resv);

   dma_resv_unlock_all_resv_locks();
   up_read(&gpu_vm->userptr_notifier_lock);
   up_write(&gpu_vm->lock);

`mmu_interval_read_begin()` 涓?`mmu_interval_read_retry()` 涔嬮棿鐨勪唬鐮侊紝鏍囨槑浜嗘垜浠墍
绉扮殑 `userptr_seqlock` 鐨勮渚т复鐣屽尯銆傚疄闄呬笂锛実pu_vm 鐨?userptr gpu_vma 鍒楄〃琚亶鍘嗭紝
骞朵笖瀵瑰畠鐨?*鎵€鏈?* userptr gpu_vmas 閮借繘琛屼簡妫€鏌ワ紝灏界鎴戜滑杩欓噷鍙睍绀轰簡涓€涓€?
userptr gpu_vma 鐨?MMU 澶辨晥 notifier 鍙兘浠庡洖鏀讹紙reclaim锛変笂涓嬫枃涓璋冪敤锛屽苟涓斿悓鏍峰湴锛?涓轰簡閬垮厤鍔犻攣椤哄簭鍐茬獊锛屾垜浠笉鑳藉湪鍏朵腑鑾峰彇浠讳綍 dma_resv 閿佹垨 gpu_vm->lock銆?

  bool gpu_vma_userptr_invalidate(userptr_interval, cur_seq)
  {
          // Make sure the exec function either sees the new sequence
          // and backs off or we wait for the dma-fence:

          down_write(&gpu_vm->userptr_notifier_lock);
          mmu_interval_set_seq(userptr_interval, cur_seq);
          up_write(&gpu_vm->userptr_notifier_lock);

          // At this point, the exec function can't succeed in
          // submitting a new job, because cur_seq is an invalid
          // sequence number and will always cause a retry. When all
          // invalidation callbacks, the mmu notifier core will flip
          // the sequence number to a valid one. However we need to
          // stop gpu access to the old pages here.

          dma_resv_wait_timeout(&gpu_vm->resv, DMA_RESV_USAGE_BOOKKEEP,
                                false, MAX_SCHEDULE_TIMEOUT);
          return true;
  }

褰撴澶辨晥 notifier 杩斿洖鏃讹紝GPU 涓嶅啀鑳藉璁块棶 userptr gpu_vma 鐨勬棫椤碉紝骞朵笖闇€瑕佸湪鏂扮殑 GPU
鎻愪氦鎴愬姛涔嬪墠閲嶆柊杩涜椤电粦瀹氥€?
楂樻晥鐨?userptr gpu_vma exec_function 閬嶅巻
_________________________________________________
濡傛灉 gpu_vm 鐨?userptr gpu_vmas 鍒楄〃鍙樺緱寰堝ぇ锛屽湪姣忔 exec function 涓亶鍘嗗畬鏁寸殑
userptrs 鍒楄〃浠ユ鏌ユ瘡涓?userptr gpu_vma 淇濆瓨鐨勫簭鍒楀彿鏄惁杩囨湡锛屾晥鐜囧氨浼氬緢浣庛€備竴绉嶈В鍐?鏂规鏄皢鎵€鏈?*宸插け鏁?*鐨?userptr gpu_vmas 鏀惧埌涓€涓崟鐙殑 gpu_vm 鍒楄〃涓婏紝骞朵笖姣忔
exec function 鍙鏌ヨ鍒楄〃涓婂瓨鍦ㄧ殑 gpu_vmas銆傜敱浜庡湪璇?mmu notifier 涓紙鎴戜滑鍚戝垪琛ㄦ坊鍔?宸插け鏁堢殑 gpu_vmas 鐨勫湴鏂癸級鏃犳硶鑾峰彇浠讳綍鍍?`gpu_vm->lock` 鎴?`gpu_vm->resv` 杩欐牱鐨勫灞傞攣锛?璇ュ垪琛ㄩ潪甯搁€傚悎鑷棆閿佽凯浠ｄ竴鑺?<Spinlock iteration> 涓弿杩扮殑鏂规銆傝娉ㄦ剰锛宍gpu_vm->lock`
鍦ㄩ亶鍘嗘椂浠嶉渶瑕佽鎸佹湁锛屼互纭繚鍒楄〃鐨勫畬鏁存€э紝姝ｅ璇ヨ妭涓篃鎻愬埌鐨勯偅鏍枫€?
濡傛灉浣跨敤杩欐牱鐨勫凡澶辨晥 userptr 鍒楄〃锛宔xec function 涓殑閲嶈瘯妫€鏌ュ氨浼氱畝鍗曞湴鍙樻垚妫€鏌ュ凡澶辨晥
鍒楄〃鏄惁涓虹┖銆?
## 缁戝畾涓庤В缁戞椂鐨勫姞閿?

鍦ㄧ粦瀹氭椂锛屽亣璁炬槸涓€涓敱 GEM 瀵硅薄鏀拺鐨?gpu_vma锛屾瘡涓?gpu_vma 閮介渶瑕佷笌涓€涓?gpu_vm_bo
鍏宠仈锛岃€岃 gpu_vm_bo 鍙堥渶瑕佽娣诲姞鍒?GEM 瀵硅薄鐨?gpu_vm_bo 鍒楄〃锛屽苟鍙兘娣诲姞鍒?gpu_vm 鐨?澶栭儴瀵硅薄鍒楄〃銆傝繖琚О涓?gpu_vma 鐨?*閾炬帴锛坙inking锛?*锛屽苟涓旈€氬父闇€瑕佹寔鏈?`gpu_vm->lock`
鍜?`gem_object->gpuva_lock`銆傚湪瑙ｉ櫎涓€涓?gpu_vma 鐨勯摼鎺ユ椂锛屽簲鎸佹湁鐩稿悓鐨勯攣锛岃繖纭繚浜嗗綋鍦?`gpu_vm->resv` 鎴?GEM 瀵硅薄鐨?dma_resv 涓嬮亶鍘?``gpu_vmas` 鏃讹紝鍙鎴戜滑鎵€閬嶅巻鎵€渚濇嵁鐨勯攣
鏈閲婃斁锛実pu_vmas 灏变細淇濇寔瀛樻椿銆傚浜?userptr gpu_vmas锛岀被浼煎湴瑕佹眰鍦ㄩ攢姣?vma 鏈熼棿鎸佹湁
澶栧眰 `gpu_vm->lock`锛屽惁鍒欏綋鎸夌収涓婁竴鑺傛墍杩伴亶鍘嗗凡澶辨晥鐨?userptr 鍒楄〃鏃讹紝娌℃湁浠讳綍涓滆タ鑳?璁╅偅浜?userptr gpu_vmas 淇濇寔瀛樻椿銆?
## 鍙仮澶嶉〉閿欒椤佃〃鏇存柊鏃剁殑鍔犻攣


鍏充簬鍙仮澶嶉〉閿欒锛坮ecoverable page-faults锛夌殑鍔犻攣锛屾垜浠渶瑕佺‘淇濅袱浠堕噸瑕佺殑浜嬶細

- 鍦ㄦ垜浠皢椤靛綊杩樼粰绯荤粺/鍒嗛厤鍣ㄤ互渚涘鐢ㄦ椂锛屼笉搴斿啀鏈夊墿浣欑殑 GPU 鏄犲皠锛屽苟涓斾换浣?GPU TLB
  閮藉繀椤诲凡琚埛鏂般€?- 瀵?gpu_vma 鐨勮В鏄犲皠涓庢槧灏勪笉寰楀彂鐢熺珵浜夈€?
鐢变簬 GPU ptes 鐨勮В鏄犲皠锛堟垨 zapping锛夐€氬父鍙戠敓鍦ㄥ緢闅剧敋鑷充笉鍙兘鑾峰彇浠讳綍澶栧眰閿佺殑鍦版柟锛屾垜浠?瑕佷箞寮曞叆涓€鎶婂湪鏄犲皠鍜岃В鏄犲皠鏃堕兘鎸佹湁鐨勬柊閿侊紝瑕佷箞鏌ョ湅鎴戜滑鍦ㄨВ鏄犲皠鏃舵寔鏈夌殑閿侊紝骞剁‘淇濆畠浠湪
鏄犲皠鏃朵篃琚寔鏈夈€傚浜?userptr gpu_vmas锛屽湪 zapping 鍙戠敓鐨?mmu 澶辨晥 notifier 涓紝
`userptr_seqlock` 浠ュ啓妯″紡鎸佹湁銆傚洜姝わ紝濡傛灉 `userptr_seqlock` 浠ュ強 `gpu_vm->userptr_notifier_lock`
鍦ㄦ槧灏勬湡闂翠互璇绘ā寮忔寔鏈夛紝瀹冨氨涓嶄細涓?zapping 鍙戠敓绔炰簤銆傚浜庣敱 GEM 瀵硅薄鏀拺鐨?gpu_vmas锛?zapping 浼氬湪 GEM 瀵硅薄鐨?dma_resv 涓嬭繘琛岋紝骞朵笖纭繚瀵逛簬浠讳綍鎸囧悜璇?GEM 瀵硅薄鐨?gpu_vma锛?鍦ㄥ～鍏呭叾椤佃〃鏃朵篃鎸佹湁璇?dma_resv锛屽悓鏍疯兘纭繚鎴戜滑鏄棤绔炰簤鐨勩€?
濡傛灉鏄犲皠鐨勪换浣曢儴鍒嗘槸鍦ㄨ繖浜涢攣琚噴鏀剧殑鎯呭喌涓嬨€佸湪鏌愪釜 dma-fence 涓嬪紓姝ユ墽琛岀殑锛岄偅涔?zapping
灏嗛渶瑕佺瓑寰呰 dma-fence 鍦ㄧ浉鍏抽攣涓嬪彂鍑轰俊鍙蜂箣鍚庯紝鎵嶈兘寮€濮嬩慨鏀归〉琛ㄣ€?
鐢变簬浠ラ噴鏀鹃〉琛ㄥ唴瀛樼殑鏂瑰紡淇敼椤佃〃缁撴瀯涔熷彲鑳介渶瑕佸灞傞攣锛孏PU ptes 鐨?zapping 閫氬父鍙仛鐒︿簬
灏嗛〉琛ㄦ垨椤电洰褰曢」娓呴浂骞跺埛鏂?TLB锛岃€屽皢椤佃〃鍐呭瓨鐨勯噴鏀炬帹杩熷埌瑙ｇ粦鎴栭噸鏂扮粦瀹氭椂杩涜銆?