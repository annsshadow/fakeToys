
## Devlink Reload锛坉evlink 閲嶆柊鍔犺浇锛?

`devlink-reload` 鎻愪緵浜嗕竴绉嶆満鍒讹紝鐢ㄤ簬閲嶆柊鍒濆鍖栭┍鍔ㄥ疄浣擄紝骞跺簲鐢?`devlink-params` 涓?`devlink-resources` 鐨勬柊鍊笺€傚畠杩樻彁渚涗簡涓€绉嶆縺娲诲浐浠剁殑
鏈哄埗銆?
## 閲嶆柊鍔犺浇鍔ㄤ綔锛圧eload Actions锛?

鐢ㄦ埛鍙互閫夋嫨涓€涓噸鏂板姞杞藉姩浣溿€傞粯璁ら€夋嫨鐨勬槸 `driver_reinit` 鍔ㄤ綔銆?
   :widths: 5 90

   - - Name
     - Description
   - - `driver-reinit`
     - devlink 椹卞姩瀹炰綋鐨勯噸鏂板垵濮嬪寲锛屽寘鎷湪椹卞姩鍔犺浇杩囩▼涓墍浣跨敤鐨?devlink
       瀹炰綋涓婂簲鐢ㄦ柊鍊硷紝杩欎簺瀹炰綋鍖呮嫭锛?
       - 閰嶇疆妯″紡涓?`driverinit` 鐨?`devlink-params`
       - `devlink-resources`

       鍏朵粬 devlink 瀹炰綋鍦ㄩ噸鏂板垵濮嬪寲杩囩▼涓彲浠ヤ繚鎸佷笉鍙橈細

       - `devlink-health-reporter`
       - `devlink-region`

       鍏朵綑鐨?devlink 瀹炰綋鍒欏繀椤昏绉婚櫎骞堕噸鏂版坊鍔犮€?   - - `fw_activate`
     - 婵€娲诲浐浠躲€傚鏋滃瓨鍦ㄥ緟婵€娲荤殑鍥轰欢闀滃儚锛屽垯婵€娲绘柊鍥轰欢銆傚鏋滄病鏈夋寚瀹氫换浣?       闄愬埗锛岃鍔ㄤ綔鍙兘浼氭秹鍙婂浐浠跺浣嶃€傚鏋滄病鏈夊緟婵€娲荤殑鏂伴暅鍍忥紝鍒欒鍔ㄤ綔浼?       閲嶆柊鍔犺浇褰撳墠鐨勫浐浠堕暅鍍忋€?
璇锋敞鎰忥紝鍗充娇鐢ㄦ埛璇锋眰浜嗘煇涓壒瀹氬姩浣滐紝椹卞姩鐨勫疄鐜颁篃鍙兘闇€瑕佸悓鏃舵墽琛屽彟涓€涓?鍔ㄤ綔銆備緥濡傦紝鏌愪簺椹卞姩涓嶆敮鎸佸湪涓嶆縺娲诲浐浠剁殑鎯呭喌涓嬭繘琛岄┍鍔ㄩ噸鏂板垵濮嬪寲銆傚洜姝わ紝
devlink reload 鍛戒护浼氳繑鍥炲疄闄呮墽琛岀殑鍔ㄤ綔鍒楄〃銆?
## 閲嶆柊鍔犺浇闄愬埗锛圧eload Limits锛?

榛樿鎯呭喌涓嬶紝閲嶆柊鍔犺浇鍔ㄤ綔涓嶅彈闄愬埗锛岄┍鍔ㄥ疄鐜板彲浠ユ牴鎹渶瑕佹墽琛屽浣嶆垨鍋滄満浠?瀹屾垚鐩稿簲鍔ㄤ綔銆?
涓嶈繃锛屾煇浜涢┍鍔ㄦ敮鎸佸姩浣滈檺鍒讹紝灏嗗姩浣滅殑瀹炵幇闄愬畾鍦ㄧ壒瀹氱害鏉熶箣鍐呫€?
   :widths: 5 90

   - - Name
     - Description
   - - `no_reset`
     - 涓嶅厑璁稿浣嶏紝涓嶅厑璁稿仠鏈猴紝涓嶅厑璁搁摼璺姈鍔紝涓斾笉浼氫涪澶变换浣曢厤缃€?
## 鍒囨崲鍛藉悕绌洪棿锛圕hange Namespace锛?

netns 閫夐」鍏佽鐢ㄦ埛鍦?devlink reload 鎿嶄綔杩囩▼涓皢 devlink 瀹炰緥绉诲姩鍒板叾浠?鍛藉悕绌洪棿銆傞粯璁ゆ儏鍐典笅锛屾墍鏈?devlink 瀹炰緥閮藉湪 init_net 涓垱寤哄苟淇濈暀鍦ㄩ偅閲屻€?
### 浣跨敤绀轰緥锛坋xample usage锛?

    $ devlink dev reload help
    $ devlink dev reload DEV [ netns { PID | NAME | ID } ] [ action { driver_reinit | fw_activate } ] [ limit no_reset ]

    # 杩愯閲嶆柊鍔犺浇鍛戒护浠ラ噸鏂板垵濮嬪寲 devlink 椹卞姩瀹炰綋锛?    $ devlink dev reload pci/0000:82:00.0 action driver_reinit
    reload_actions_performed:
      driver_reinit

    # 杩愯閲嶆柊鍔犺浇鍛戒护浠ユ縺娲诲浐浠讹細
    # 娉ㄦ剰锛歮lx5 椹卞姩鍦ㄦ縺娲诲浐浠剁殑鍚屾椂浼氶噸鏂板姞杞介┍鍔?    $ devlink dev reload pci/0000:82:00.0 action fw_activate
    reload_actions_performed:
      driver_reinit fw_activate
