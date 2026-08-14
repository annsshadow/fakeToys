
## 瀵规槧灏勫埌鍐呭瓨鍦板潃鐨?I/O 鍐欏叆杩涜鎺掑簭


鍦ㄦ煇浜涘钩鍙颁笂锛屾墍璋撶殑鍐呭瓨鏄犲皠 I/O锛坢emory-mapped I/O锛夋槸寮卞簭鐨勩€傚湪杩欑被骞冲彴涓婏紝椹卞姩寮€鍙戣€呮湁璐ｄ换纭繚鍏惰澶囦笂瀵瑰唴瀛樻槧灏勫湴鍧€鐨?I/O 鍐欏叆鎸夐鏈熺殑椤哄簭鍒拌揪銆傝繖閫氬父閫氳繃璇诲彇涓€涓€滃畨鍏ㄢ€濈殑璁惧鎴栨ˉ鎺ュ瘎瀛樺櫒鏉ュ疄鐜帮紝浠庤€岃揩浣?I/O 鑺墖缁勫湪浠讳綍璇绘搷浣滃彂璧蜂箣鍓嶏紝灏嗘寕璧风殑鍐欏叆鍒锋柊鍒拌澶囥€傞┍鍔ㄩ€氬父浼氬湪鍙楄嚜鏃嬮攣淇濇姢鐨勪复鐣屽尯浠ｇ爜閫€鍑轰箣鍓嶇珛鍗充娇鐢ㄦ鎶€鏈€傝繖鍙‘淇濇墍鏈夊悗缁殑 I/O 绌洪棿鍐欏叆閮戒粎鍦ㄦ墍鏈夊厛鍓嶇殑鍐欏叆涔嬪悗鍒拌揪锛堝緢鍍忓唴瀛樺睆闅滄搷浣?mb()锛屽彧鏄拡瀵?I/O 鑰岃█锛夈€?
```

		...
	CPU A:  spin_lock_irqsave(&dev_lock, flags)
	CPU A:  val = readl(my_status);
	CPU A:  ...
	CPU A:  writel(newval, ring_ptr);
	CPU A:  spin_unlock_irqrestore(&dev_lock, flags)
		...
	CPU B:  spin_lock_irqsave(&dev_lock, flags)
	CPU B:  val = readl(my_status);
	CPU B:  ...
	CPU B:  writel(newval2, ring_ptr);
	CPU B:  spin_unlock_irqrestore(&dev_lock, flags)
		...

```
鍦ㄤ笂杩版儏褰笅锛岃澶囧彲鑳戒細鍦ㄦ敹鍒?newval 涔嬪墠鍏堟敹鍒?newval2锛?
```

		...
	CPU A:  spin_lock_irqsave(&dev_lock, flags)
	CPU A:  val = readl(my_status);
	CPU A:  ...
	CPU A:  writel(newval, ring_ptr);
	CPU A:  (void)readl(safe_register); /* maybe a config register? */
	CPU A:  spin_unlock_irqrestore(&dev_lock, flags)
		...
	CPU B:  spin_lock_irqsave(&dev_lock, flags)
	CPU B:  val = readl(my_status);
	CPU B:  ...
	CPU B:  writel(newval2, ring_ptr);
	CPU B:  (void)readl(safe_register); /* maybe a config register? */
	CPU B:  spin_unlock_irqrestore(&dev_lock, flags)

```
姝ゅ锛屽 safe_register 鐨勮鍙栧皢淇冧娇 I/O 鑺墖缁勫湪瀹為檯鍚戣姱鐗囩粍鍙戣捣璇绘搷浣滀箣鍓嶏紝鍒锋柊浠讳綍鎸傝捣鐨勫啓鍏ワ紝浠庤€岄槻姝㈠彲鑳界殑鏁版嵁鎹熷潖銆?