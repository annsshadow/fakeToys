## I915 DG1/LMEM RFC 绔犺妭


## 涓婃父璁″垝

閽堝涓婃父锛屾妸鎵€鏈?DG1 鐩稿叧浠ｇ爜钀藉湴骞舵渶缁堝惎鐢ㄣ€佸悓鏃跺寘鍚叏閮?uAPI 閮ㄥ垎鐨勬€讳綋璁″垝濡備笅锛?

- 鍚堝苟 DG1 鐨勫熀纭€纭欢鏀寔锛堜粛涓嶅甫 pciid锛?
- 鍦ㄧ壒娈婄殑 CONFIG_BROKEN锛堟垨绫讳技锛夋爣蹇椾箣鍚庡悎骞?uAPI 閮ㄥ垎
        - 姝ゆ椂鎴戜滑浠嶅彲鍋氭敼鍔紝浣嗛噸瑕佺殑鏄繖璁╂垜浠?
          鑳藉湪 CI 涓繍琛屽彲鍒╃敤鏈湴鍐呭瓨锛坙ocal-memory锛夌殑 IGTs
- 杩佺Щ鍒?TTM锛岀‘淇濅竴鍒囨寔缁彲鐢ㄣ€傞儴鍒嗗伐浣滃唴瀹癸細
        - 闈㈠悜鐙珛鏄惧崱鐨?TTM shrinker
        - 瀹屾暣 dma_resv_lock 鐨?dma_resv_lockitem锛屽嵆涓嶄粎浠呮槸 trylock
        - 浣跨敤 TTM CPU 缂洪〉澶勭悊绋嬪簭锛坧agefault handler锛?
        - 灏?shmem 鍚庣璺敱鍒扮嫭绔嬫樉鍗＄殑 TTM SYSTEM
        - TTM 鍙洖鏀跺璞★紙purgeable object锛夋敮鎸?
        - 灏?i915 buddy 鍒嗛厤鍣ㄨ縼绉诲埌 TTM
- 鍙戦€?RFC锛堟妱閫?mesa-dev锛変互鑾峰緱 uAPI 鐨勬渶缁堢缃茬‘璁?
- 涓?DG1 娣诲姞 pciid 骞剁湡姝ｅ惎鐢?uAPI
