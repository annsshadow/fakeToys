## 鑷姩鎸傝浇鏀寔


甯屾湜鏀寔鑷姩鎸傝浇鐨勬枃浠剁郴缁燂紙渚嬪鍙湪 fs/afs/ 鎵惧埌鐨?kAFS锛屼互鍙?fs/nfs/ 涓殑 NFS锛夊彲浠ヤ娇鐢ㄨ鏀寔銆傝鏈哄埗鍖呮嫭鍏佽鎵ц鍐呮牳鍐呮寕杞斤紝
浠ュ強璇锋眰鎸傝浇鐐归檷绾с€傚悗鑰呬篃鍙敱鐢ㄦ埛绌洪棿璇锋眰銆?

## 鍐呮牳鍐呰嚜鍔ㄦ寕杞?

璇峰弬闃?Documentation/filesystems/autofs.rst 鐨勨€滄寕杞介櫡闃憋紙Mount Traps锛夆€濅竴鑺傘€?
```
	[root@andromeda root]# mount -t afs \#root.afs. /afs
	[root@andromeda root]# ls /afs
	asd  cambridge  cambridge.redhat.com  grand.central.org
	[root@andromeda root]# ls /afs/cambridge
	afsdoc
	[root@andromeda root]# ls /afs/cambridge/afsdoc/
	ChangeLog  html  LICENSE  pdf  RELNOTES-1.2.2

```
```
	[root@andromeda root]# cat /proc/mounts
	...
	#root.afs. /afs afs rw 0 0
	#root.cell. /afs/cambridge.redhat.com afs rw 0 0
	#afsdoc. /afs/cambridge.redhat.com/afsdoc afs rw 0 0


```
## 鎸傝浇鐐硅嚜鍔ㄨ繃鏈?

鍙浣犲湪鍓嶈堪鑷姩鎸傝浇娴佺▼涓寕杞戒簡灏嗚杩囨湡鐨勬寕杞界偣锛屾寕杞界偣鐨勮嚜鍔ㄨ繃鏈?灏卞緢绠€鍗曘€?
瑕佽繘琛岃繃鏈熷鐞嗭紝浣犻渶瑕侀伒寰互涓嬫楠わ細

 (1) 鍒涘缓鑷冲皯涓€涓垪琛紝鐢ㄤ簬鎸傛帴灏嗚杩囨湡鐨?vfsmount銆?
 (2) 鍦?->d_automount 鏂规硶涓垱寤烘柊鐨勬寕杞界偣鏃讹紝娣诲姞
```
             mnt_set_expiry(newmnt, &afs_vfsmounts);

 (3) When you want mountpoints to be expired, call mark_mounts_for_expiry()
     with a pointer to this list. This will process the list, marking every
     vfsmount thereon for potential expiry on the next call.

     If a vfsmount was already flagged for expiry, and if its usage count is 1
     (it's only referenced by its parent vfsmount), then it will be deleted
     from the namespace and thrown away (effectively unmounted).

     It may prove simplest to simply call this at regular intervals, using
     some sort of timed event to drive it.

```
杩囨湡鏍囧織鐢卞 mntput 鐨勮皟鐢ㄦ竻闄ゃ€傝繖鎰忓懗鐫€杩囨湡鍙細鍦ㄦ寕杞界偣鏈€鍚庝竴娆¤璁块棶涔嬪悗鐨?绗簩娆¤繃鏈熻姹傛椂鍙戠敓銆?
濡傛灉鎸傝浇鐐硅绉诲姩锛屽畠浼氫粠杩囨湡鍒楄〃涓Щ闄ゃ€傚鏋滃湪鍙繃鏈熸寕杞戒笂寤虹珛浜嗙粦瀹氭寕杞斤紝
鏂扮殑 vfsmount 灏嗕笉鍦ㄨ繃鏈熷垪琛ㄤ腑锛屼篃涓嶄細杩囨湡銆?
濡傛灉鍛藉悕绌洪棿琚鍒讹紝鍏朵腑鍖呭惈鐨勬墍鏈夋寕杞界偣閮藉皢琚鍒讹紝骞朵笖閭ｄ簺浣嶄簬杩囨湡鍒楄〃涓殑
鎸傝浇鐐圭殑鍓湰浼氳鍔犲叆鍚屼竴涓繃鏈熷垪琛ㄣ€?

## 鐢ㄦ埛绌洪棿椹卞姩鐨勮繃鏈?

浣滀负鏇夸唬锛岀敤鎴风┖闂村彲浠ヨ姹備换浣曟寕杞界偣鐨勮繃鏈燂紙灏界鏈変簺浼氳鎷掔粷鈥斺€斾緥濡傚綋鍓嶈繘绋?鎵€璁や负鐨?rootfs锛夈€傚畠閫氳繃鍚?umount() 浼犲叆 MNT_EXPIRE 鏍囧織鏉ュ疄鐜般€傝鏍囧織琚涓?涓?MNT_FORCE 鍜?MNT_DETACH 涓嶅吋瀹广€?
濡傛灉鐩稿叧鎸傝浇鐐硅 umount() 鎴栧叾鐖舵寕杞界偣浠ュ鐨勪笢瑗挎墍寮曠敤锛屽皢杩斿洖 EBUSY 閿欒锛?骞朵笖璇ユ寕杞界偣涓嶄細琚爣璁颁负杩囨湡鎴栧嵏杞姐€?
濡傛灉璇ユ寕杞界偣褰撴椂灏氭湭琚爣璁颁负杩囨湡锛屽皢缁欏嚭 EAGAIN 閿欒锛屼笖涓嶄細琚嵏杞姐€?
鍚﹀垯锛屽鏋滃畠宸茶鏍囪涓旀湭琚紩鐢紝鍗歌浇灏嗙収甯歌繘琛屻€?
鍚屾牱锛屾瘡褰撻櫎 umount() 涔嬪鐨勪换浣曚笢瑗挎煡鐪嬫煇涓寕杞界偣鏃讹紝杩囨湡鏍囧織閮戒細琚竻闄ゃ€?