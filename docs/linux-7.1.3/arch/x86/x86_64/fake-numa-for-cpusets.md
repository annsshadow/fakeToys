
## 鐢ㄤ簬 CPUSet 鐨勪吉 NUMA


:Author: David Rientjes <rientjes@cs.washington.edu>

浣跨敤 numa=fake 鍜?CPUSet 杩涜璧勬簮绠＄悊

鏈枃妗ｆ弿杩颁簡濡備綍缁撳悎 cpusets 浣跨敤 numa=fake x86_64 鍛戒护琛岄€夐」鏉ヨ繘琛岀矖绮掑害鐨勫唴瀛樼鐞嗐€備娇鐢ㄨ鐗规€э紝浣犲彲浠ュ垱寤轰唬琛ㄨ繛缁唴瀛樺潡鐨勪吉 NUMA 鑺傜偣锛屽苟灏嗗畠浠垎閰嶇粰 cpusets 鍙婂叾闄勫姞鐨勪换鍔°€傝繖鏄竴绉嶉檺鍒舵煇绫讳换鍔″彲鐢ㄧ郴缁熷唴瀛樻€婚噺鐨勬柟娉曘€?
鍏充簬 cpusets 鐗规€х殑鏇村淇℃伅锛岃鍙傝 Documentation/admin-guide/cgroup-v1/cpusets.rst銆?浣犲彲浠ユ牴鎹綘鐨勯渶姹備娇鐢ㄥ绉嶄笉鍚岀殑閰嶇疆銆傚叧浜?numa=fake 鍛戒护琛岄€夐」鍙婂叾閰嶇疆浼妭鐐圭殑鍚勭鏂瑰紡锛岃鍙傝 Documentation/admin-guide/kernel-parameters.txt

灏辨湰绠€浠嬭€岃█锛屾垜浠亣璁句竴涓潪甯稿師濮嬬殑 NUMA 浠跨湡璁剧疆 "numa=fake=4*512,"銆傝繖灏嗘妸绯荤粺鍐呭瓨鎷嗗垎涓哄洓涓悇 512M 鐨勭浉绛夊潡锛岀幇鍦ㄦ垜浠彲浠ュ皢瀹冧滑鍒嗛厤缁?cpusets銆傞殢鐫€浣犳洿鐔熸倝浣跨敤杩欎竴缁勫悎杩涜璧勬簮鎺у埗锛屼綘浼氱‘瀹氫竴涓洿濂界殑璁剧疆锛屼互灏介噺鍑忓皯闇€瑕佸鐞嗙殑鑺傜偣鏁伴噺銆?
```

	Faking node 0 at 0000000000000000-0000000020000000 (512MB)
	Faking node 1 at 0000000020000000-0000000040000000 (512MB)
	Faking node 2 at 0000000040000000-0000000060000000 (512MB)
	Faking node 3 at 0000000060000000-0000000080000000 (512MB)
	...
	On node 0 totalpages: 130975
	On node 1 totalpages: 131072
	On node 2 totalpages: 131072
	On node 3 totalpages: 131072

```
鐜板湪鎸夌収 Documentation/admin-guide/cgroup-v1/cpusets.rst 涓寕杞?cpuset 鏂囦欢绯荤粺鐨勮鏄庯紝浣犲彲浠ュ垎閰嶄吉鑺傜偣锛堝嵆杩炵画鍐呭瓨
```

	[root@xroads /]# mkdir exampleset
	[root@xroads /]# mount -t cpuset none exampleset
	[root@xroads /]# mkdir exampleset/ddset
	[root@xroads /]# cd exampleset/ddset
	[root@xroads /exampleset/ddset]# echo 0-1 > cpus
	[root@xroads /exampleset/ddset]# echo 0-1 > mems

```
鐜板湪杩欎釜鍚嶄负 'ddset' 鐨?cpuset 灏嗗彧鍏佽璁块棶浼妭鐐?0 鍜?1 杩涜鍐呭瓨鍒嗛厤锛?G锛夈€?
浣犵幇鍦ㄥ彲浠ュ皢浠诲姟鍒嗛厤缁欒繖浜?cpuset锛屼互闄愬埗鍐呭瓨璧勬簮
```

	[root@xroads /exampleset/ddset]# echo $$ > tasks
	[root@xroads /exampleset/ddset]# dd if=/dev/zero of=tmp bs=1024 count=1G
	[1] 13425

```
娉ㄦ剰涓婇潰鍙楅檺 cpuset 鎯呭喌涓庝笉鍙楅檺鎯呭喌锛堝嵆鍦ㄦ湭鍒嗛厤缁欎吉 NUMA cpuset 鐨勬儏鍐典笅杩愯鐩稿悓 'dd' 鍛戒护锛変箣闂达紝/proc/meminfo 鎵€鎶ュ憡鐨勭郴缁熷唴瀛樹娇鐢ㄩ噺宸紓锛?
	========	============	==========
	Name		Unrestricted	Restricted
	========	============	==========
	MemTotal	3091900 kB	3091900 kB
	MemFree		42113 kB	1513236 kB
	========	============	==========

杩欏疄鐜颁簡瀵逛綘鍒嗛厤缁欑壒瀹?cpuset 鐨勪换鍔¤繘琛岀矖绮掑害鍐呭瓨绠＄悊銆傜敱浜?cpuset 鍙互褰㈡垚灞傜骇缁撴瀯锛屼綘鍙互涓哄悇绫讳换鍔＄殑鍐呭瓨绠＄悊闇€姹傚垱寤轰竴浜涚浉褰撴湁瓒ｇ殑缁勫悎鐢ㄤ緥銆?