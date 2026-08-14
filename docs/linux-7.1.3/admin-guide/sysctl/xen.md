## /proc/sys/xen/


鐗堟潈 (c) 2026, Shubham Chakraborty <chakrabortyshubham66@gmail.com>

鏈夊叧涓€鑸俊鎭拰娉曞緥澹版槑锛岃鍙傞槄
Documentation/admin-guide/sysctl/index.rst銆?
------------------------------------------------------------------------------

杩欎簺鏂囦欢鏄惁鍑虹幇鍦?`/proc/sys/xen/` 涓紝鍙栧喅浜庡唴鏍搁厤缃細

## balloon/hotplug_unpopulated


姝ゆ爣蹇楁帶鍒舵槸鍚﹀皢鏈～鍏呯殑鍐呭瓨鑼冨洿鑷姩鐑彃鎷斾负绯荤粺 RAM銆?
- `0`锛氭湭濉厖鐨勮寖鍥翠笉琚儹鎻掓嫈锛堥粯璁わ級銆?- `1`锛氭湭濉厖鐨勮寖鍥磋鑷姩鐑彃鎷斻€?
鍚敤鍚庯紝Xen balloon 椹卞姩绋嬪簭浼氬皢 Xen 鍐呭瓨鏄犲皠涓爣璁颁负鏈～鍏呯殑鍐呭瓨鍖哄煙浣滀负
鍙敤 RAM 娣诲姞鍒扮郴缁熶腑銆傝繖鍏佽鍦?Xen 瀹㈡埛鍩熶腑鍔ㄦ€佹墿灞曞唴瀛樸€?
姝ら€夐」浠呭湪浠ュ唴鏍搁厤缃簡 `CONFIG_XEN_BALLOON_MEMORY_HOTPLUG` 鏃舵墠鍙敤銆?