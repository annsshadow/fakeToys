## 鎺ㄦ祴鎵ц锛圫peculation锛?

鏈枃妗ｈВ閲婁簡鎺ㄦ祴鍙兘甯︽潵鐨勫奖鍝嶏紝浠ュ強濡備綍閫氳繃浣跨敤閫氱敤 API 浠ュ彲绉绘鐨勬柟寮忕紦瑙ｄ笉鑹悗鏋溿€?
------------------------------------------------------------------------------

涓轰簡鎻愰珮鎬ц兘骞堕檷浣庡钩鍧囧欢杩燂紝璁稿褰撲唬 CPU 閲囩敤浜嗘帹娴嬫墽琛屾妧鏈紝渚嬪鍒嗘敮棰勬祴锛屽嵆鎵ц涓€浜涘彲鑳藉湪鍚庣画闃舵琚涪寮冪殑宸ヤ綔銆?
閫氬父锛屼粠鏋舵瀯鐘舵€侊紙渚嬪瀵勫瓨鍣ㄧ殑鍐呭锛夋棤娉曡瀵熷埌鎺ㄦ祴鎵ц銆傜劧鑰屽湪鏌愪簺鎯呭喌涓嬶紝鍙互瑙傚療鍒板叾瀵瑰井鏋舵瀯鐘舵€侊紙渚嬪缂撳瓨涓暟鎹瓨鍦ㄤ笌鍚︼級鐨勫奖鍝嶃€傛绫荤姸鎬佸彲鑳藉舰鎴愪晶淇￠亾锛坰ide-channel锛夛紝鍙瑙傛祴浠ユ彁鍙栫瀵嗕俊鎭€?
渚嬪锛屽湪瀛樺湪鍒嗘敮棰勬祴鐨勬儏鍐典笅锛岃鎺ㄦ祴鎵ц鐨勪唬鐮佹湁鍙兘蹇界暐杈圭晫妫€鏌ャ€傝€冭檻濡備笅
```

	int load_array(int *array, unsigned int index)
	{
		if (index >= MAX_ARRAY_ELEMS)
			return 0;
		else
			return array[index];
	}

```
```

	CMP	<index>, #MAX_ARRAY_ELEMS
	B.LT	less
	MOV	<returnval>, #0
	RET
  less:
	LDR	<returnval>, [<array>, <index>]
	RET

```
CPU 鏈夊彲鑳介敊璇娴嬫潯浠跺垎鏀紝浠庤€屽嵆浣?index >= MAX_ARRAY_ELEMS锛屼篃浼氭帹娴嬫€у湴鍔犺浇 array[index]銆傝鍊奸殢鍚庝細琚涪寮冿紝浣嗚鎺ㄦ祴鐨勫姞杞藉彲鑳藉奖鍝嶅井鏋舵瀯鐘舵€侊紝鑰岃鐘舵€侀殢鍚庡彲琚祴閲忋€?
娑夊強澶氫釜鐩镐簰渚濊禆鐨勫唴瀛樿闂殑鏇村鏉傚簭鍒楀彲鑳藉鑷存晱鎰熶俊鎭硠闇层€傝€冭檻浠ヤ笅
```

	int load_dependent_arrays(int *arr1, int *arr2, int index)
	{
		int val1, val2,

		val1 = load_array(arr1, index);
		val2 = load_array(arr2, val1);

		return val2;
	}

```
鍦ㄦ帹娴嬩笅锛岀涓€娆″ load_array() 鐨勮皟鐢ㄥ彲鑳借繑鍥炰竴涓秺鐣屽湴鍧€鐨勫€硷紝鑰岀浜屾璋冪敤灏嗗奖鍝嶄緷璧栦簬璇ュ€肩殑寰灦鏋勭姸鎬併€傝繖鍙兘鎻愪緵涓€绉嶄换鎰忚鐨勫師璇€?
## 缂撹В鎺ㄦ祴渚т俊閬?

鍐呮牳鎻愪緵浜嗕竴涓€氱敤 API锛屼互纭繚鍗充娇澶勪簬鎺ㄦ祴涔嬩笅锛岃竟鐣屾鏌ヤ篃浼氳閬靛畧銆傚彈鎺ㄦ祴渚т俊閬撳奖鍝嶇殑鏋舵瀯搴斿綋瀹炵幇杩欎簺鍘熻銆?
<linux/nospec.h> 涓殑 array_index_nospec() 杈呭姪鍑芥暟鍙敤浜庨槻姝俊鎭€氳繃渚т俊閬撴硠闇层€?
瀵?array_index_nospec(index, size) 鐨勮皟鐢ㄤ細杩斿洖涓€涓粡杩囧噣鍖栫殑绱㈠紩鍊硷紝鍗充娇鍦?CPU 鎺ㄦ祴鏉′欢涓嬶紝璇ュ€间篃琚檺鍒跺湪 [0, size) 鑼冨洿鍐呫€?
```

	int load_array(int *array, unsigned int index)
	{
		if (index >= MAX_ARRAY_ELEMS)
			return 0;
		else {
			index = array_index_nospec(index, MAX_ARRAY_ELEMS);
			return array[index];
		}
	}

```
