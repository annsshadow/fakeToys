## CMA 璋冭瘯鎺ュ彛


CMA debugfs 鎺ュ彛瀵逛簬妫€绱㈠熀鏈俊鎭潪甯告湁鐢?
涓嶅悓鐨?CMA 鍖哄煙骞舵祴璇曟瘡涓尯鍩熺殑鍒嗛厤/閲婃斁銆?

姣忎釜CMA鍖哄煙浠ｈ〃<debugfs>/cma/涓嬬殑涓€涓洰褰曪紝琛ㄧず涓?
鍏禖MA鍚嶇О濡備笅锛?

<璋冭瘯鏂囦欢绯荤粺>/cma/<cma_name>

璇ョ洰褰曚笅鍒涘缓鐨勬枃浠剁粨鏋勫涓嬶細

 - [RO] base_pfn锛欳MA 鍖哄煙鐨勫熀鏈?PFN锛堥〉甯у彿锛夈€?
杩欎笌 range/0/base_pfn 鐩稿悓銆?
 - [RO] count锛欳MA 鍖哄煙涓殑鍐呭瓨閲忋€?
 - [RO] order_per_bit锛氫竴浣嶈〃绀虹殑椤甸『搴忋€?
 - [RO] 浣嶅浘锛氳鍖哄煙涓凡鍒嗛厤椤电殑浣嶅浘銆?
杩欎笌 range/0/base_pfn 鐩稿悓銆?
 - [RO]ranges/N/base_pfn锛氳繛缁寖鍥碞鐨勫熀鏈琍FN
鍦–MA鍦板尯銆?
 - [RO]ranges/N/bitmap: 涓垎閰嶇殑椤电殑浣嶅浘
CMA 鍖哄煙鍐呯殑 N 鑼冨洿銆?
```

	echo 5 > <debugfs>/cma/<cma_name>/alloc

```
浼氬皾璇曚粠鈥渃ma_name鈥濆尯鍩熷垎閰?5 涓〉闈€?

 - [WO] free锛氫粠璇MA鍖哄煙鍏嶈垂N涓〉闈紝涓庝笂闈㈢被浼笺€?
