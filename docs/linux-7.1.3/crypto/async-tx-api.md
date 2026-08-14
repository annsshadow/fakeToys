
## Asynchronous Transfers/Transforms API



  1. 绠€浠?

  2 璧锋簮

  3 鐢ㄦ硶
  3.1 API 鐨勪竴鑸牸寮?
  3.2 鏀寔鐨勬搷浣?
  3.3 鎻忚堪绗︾鐞?
  3.4 鎿嶄綔浣曟椂鎵ц锛?
  3.5 鎿嶄綔浣曟椂瀹屾垚锛?
  3.6 绾︽潫
  3.7 绀轰緥

  4 DMAENGINE 椹卞姩寮€鍙戣€呮敞鎰忎簨椤?
  4.1 涓€鑷存€ц鐐?
  4.2 鈥滄垜鐨勫簲鐢ㄩ渶瑕佸纭欢閫氶亾鐨勭嫭鍗犳帶鍒垛€?

  5 婧愮爜

## 1. Introduction


async_tx API 鎻愪緵浜嗕竴缁勬柟娉曪紝鐢ㄤ簬鎻忚堪涓€涓插紓姝ユ壒閲忓唴瀛樹紶杈?杞崲鎿嶄綔閾撅紝骞舵敮鎸佷簨鍔￠棿渚濊禆銆?
瀹冭瀹炵幇涓轰竴涓?dmaengine 瀹㈡埛绔紝灞忚斀浜嗕笉鍚岀‖浠跺嵏杞藉紩鎿庡疄鐜扮殑缁嗚妭銆傛寜鐓ц API 缂栧啓鐨勪唬鐮?
鍙互閽堝寮傛鎿嶄綔杩涜浼樺寲锛岃€岃 API 浼氬皢鎿嶄綔閾鹃€傞厤鍒板彲鐢ㄧ殑鍗歌浇璧勬簮涓娿€?

## 2.Genealogy


璇?API 鏈€鍒濊璁＄敤浜庝娇鐢?Intel(R) Xscale 绯诲垪 I/O 澶勭悊鍣ㄤ腑鐨勫嵏杞藉紩鎿庯紝鏉ュ嵏杞?md-raid5 椹卞姩
鐨勫唴瀛樻嫹璐濆拰 xor 濂囧伓鏍￠獙璁＄畻銆傚畠涔熷缓绔嬪湪鈥渄maengine鈥濆眰涔嬩笂锛岃灞傛槸涓哄湪缃戠粶鏍堜腑浣跨敤
Intel(R) I/OAT 寮曟搸鍗歌浇鍐呭瓨鎷疯礉鑰屽紑鍙戠殑銆傜敱姝や骇鐢熶簡浠ヤ笅璁捐鐗规€э細

1. 闅愬紡鍚屾璺緞锛欰PI 鐨勭敤鎴锋棤闇€鐭ラ亾浠栦滑鎵€杩愯鐨勫钩鍙版槸鍚﹀叿鏈夊嵏杞借兘鍔涖€傚綋寮曟搸鍙敤鏃舵搷浣滀細琚?
   鍗歌浇锛屽惁鍒欏湪杞欢涓墽琛屻€?
2. 璺ㄩ€氶亾渚濊禆閾撅細API 鍏佽鎻愪氦涓€涓蹭緷璧栨搷浣滐紝渚嬪 raid5 鎯呭喌涓嬬殑 xor->copy->xor銆侫PI 浼氳嚜鍔?
   澶勭悊浠庝竴涓搷浣滆繃娓″埌鍙︿竴涓搷浣滄剰鍛崇潃纭欢閫氶亾鍒囨崲鐨勬儏鍐点€?
3. 瀵?dmaengine 鐨勬墿灞曪紝浠ユ敮鎸佸涓鎴风浠ュ強鈥渕emcpy鈥濅箣澶栫殑鎿嶄綔绫诲瀷

## 3. Usage


### 3.1 General format of the API


```

  struct dma_async_tx_descriptor *
  async_<operation>(<op specific parameters>, struct async_submit_ctl *submit)

```
### 3.2 Supported operations


========  ====================================================================
memcpy    鍦ㄦ簮缂撳啿鍜岀洰鐨勭紦鍐蹭箣闂磋繘琛屽唴瀛樻嫹璐?
memset    鐢ㄦ煇涓瓧鑺傚€煎～鍏呯洰鐨勭紦鍐?
xor      瀵逛竴绯诲垪婧愮紦鍐茶繘琛?xor 骞跺皢缁撴灉鍐欏叆鐩殑缂撳啿
xor_val   瀵逛竴绯诲垪婧愮紦鍐茶繘琛?xor锛屽鏋滅粨鏋滀负闆跺垯璁剧疆涓€涓爣蹇椼€傚疄鐜颁細灏介噺閬垮厤鍐欏叆鍐呭瓨
pq       浠庝竴绯诲垪婧愮紦鍐茬敓鎴?p+q锛坮aid6 鏍￠獙鐮侊級
pq_val    楠岃瘉 p 鍜?鎴?q 缂撳啿涓庣粰瀹氱殑涓€绯诲垪婧愭槸鍚﹀悓姝?
datap    锛坮aid6_datap_recov锛変粠缁欏畾婧愪腑鎭㈠涓€涓?raid6 鏁版嵁鍧楀拰 p 鍧?
2data    锛坮aid6_2data_recov锛変粠缁欏畾婧愪腑鎭㈠ 2 涓?raid6 鏁版嵁鍧?
========  ====================================================================

### 3.3 Descriptor management


褰撴搷浣滃凡琚帓闃熶互寮傛鎵ц鏃讹紝杩斿洖鍊间负闈?NULL锛屽苟鎸囧悜涓€涓€滄弿杩扮鈥濓紙descriptor锛夈€傛弿杩扮鏄?
鍦ㄥ嵏杞藉紩鎿庨┍鍔ㄦ帶鍒朵笅琚洖鏀跺鐢ㄧ殑璧勬簮锛岄殢鐫€鎿嶄綔瀹屾垚鑰岃閲嶇敤銆傚綋搴旂敤闇€瑕佹彁浜や竴涓叉搷浣滄椂锛屽畠
蹇呴』淇濊瘉鍦ㄤ緷璧栬鎻愪氦涔嬪墠鎻忚堪绗︿笉浼氳鑷姩鍥炴敹銆傝繖瑕佹眰鎵€鏈夋弿杩扮鍦ㄨ鍗歌浇寮曟搸椹卞姩鍏佽鍥炴敹锛堟垨
閲婃斁锛変箣鍓嶏紝鍏堣搴旂敤纭锛坅cknowledge锛夈€傛弿杩扮鍙互閫氳繃浠ヤ笅浠讳竴鏂瑰紡琚‘璁わ細

1. 濡傛灉娌℃湁瑕佹彁浜ょ殑瀛愭搷浣滐紝鍒欒缃?ASYNC_TX_ACK 鏍囧織
2. 灏嗕竴涓湭纭鐨勬弿杩扮浣滀负渚濊禆鎻愪氦缁欏彟涓€涓?async_tx 璋冪敤锛屽皢闅愬紡璁剧疆纭鐘舵€併€?
3. 鍦ㄦ弿杩扮涓婅皟鐢?async_tx_ack()銆?

### 3.4 When does the operation execute?


鎿嶄綔鍦ㄤ粠 async_<operation> 璋冪敤杩斿洖鍚庝笉浼氱珛鍗冲彂鍑恒€傚嵏杞藉紩鎿庨┍鍔ㄤ細瀵规搷浣滆繘琛屾壒澶勭悊锛屼互鍑忓皯
绠＄悊閫氶亾鎵€闇€鐨?mmio 鍛ㄦ湡鏁伴噺锛屼粠鑰屾彁楂樻€ц兘銆備竴鏃﹁揪鍒伴┍鍔ㄧ壒瀹氱殑闃堝€硷紝椹卞姩浼氳嚜鍔ㄥ彂鍑哄緟澶勭悊鐨?
鎿嶄綔銆傚簲鐢ㄥ彲浠ラ€氳繃璋冪敤 async_tx_issue_pending_all() 寮哄埗瑙﹀彂璇ヤ簨浠躲€傚畠浣滅敤浜庢墍鏈夐€氶亾锛屽洜涓?
搴旂敤涓嶇煡閬撻€氶亾鍒版搷浣滅殑鏄犲皠鍏崇郴銆?

### 3.5 When does the operation complete?


搴旂敤鍙互閫氳繃涓ょ鏂规硶浜嗚В鎿嶄綔鐨勫畬鎴愩€?

1. 璋冪敤 dma_wait_for_async_tx()銆傝璋冪敤浣?CPU 鍦ㄨ疆璇㈡搷浣滃畬鎴愮殑鍚屾椂鑷棆銆傚畠澶勭悊渚濊禆閾惧苟鍙戝嚭
   寰呭鐞嗘搷浣溿€?
2. 鎸囧畾涓€涓畬鎴愬洖璋冨嚱鏁般€傚鏋滃嵏杞藉紩鎿庨┍鍔ㄦ敮鎸佷腑鏂紝璇ュ洖璋冧緥绋嬪湪 tasklet 涓婁笅鏂囦腑杩愯锛涘鏋?
   鎿嶄綔鍦ㄨ蒋浠朵腑鍚屾鎵ц锛屽垯鍦ㄥ簲鐢ㄤ笂涓嬫枃涓皟鐢ㄣ€傝鍥炶皟鍙互鍦ㄥ async_<operation> 鐨勮皟鐢ㄤ腑璁剧疆锛?
   鎴栬€呭綋搴旂敤闇€瑕佹彁浜ら暱搴︽湭鐭ョ殑閾炬椂锛屽彲浠ヤ娇鐢?async_trigger_callback() 渚嬬▼鍦ㄩ摼鐨勬湯灏捐缃畬鎴?
   涓柇/鍥炶皟銆?

### 3.6 Constraints


1. 涓嶅厑璁稿湪 IRQ 涓婁笅鏂囦腑璋冪敤 async_<operation>銆傚彧瑕佷笉杩濆弽绾︽潫 #2锛屽叾浠栦笂涓嬫枃鏄厑璁哥殑銆?
2. 瀹屾垚鍥炶皟渚嬬▼涓嶈兘鎻愪氦鏂版搷浣溿€傝繖鍦ㄥ悓姝ユ儏鍐典笅浼氬鑷撮€掑綊锛屽湪寮傛鎯呭喌涓嬩細瀵艰嚧鑷棆閿佽鑾峰彇涓ゆ銆?

### 3.7 Example


鎵ц涓€涓?xor->copy->xor 鎿嶄綔锛屽叾涓瘡涓搷浣滀緷璧栦簬
```

    #include <linux/async_tx.h>

    static void callback(void *param)
    {
	    complete(param);
    }

    #define NDISKS  2

    static void run_xor_copy_xor(struct page **xor_srcs,
				 struct page *xor_dest,
				 size_t xor_len,
				 struct page *copy_src,
				 struct page *copy_dest,
				 size_t copy_len)
    {
	    struct dma_async_tx_descriptor *tx;
	    struct async_submit_ctl submit;
	    addr_conv_t addr_conv[NDISKS];
	    struct completion cmp;

	    init_async_submit(&submit, ASYNC_TX_XOR_DROP_DST, NULL, NULL, NULL,
			    addr_conv);
	    tx = async_xor(xor_dest, xor_srcs, 0, NDISKS, xor_len, &submit);

	    submit.depend_tx = tx;
	    tx = async_memcpy(copy_dest, copy_src, 0, 0, copy_len, &submit);

	    init_completion(&cmp);
	    init_async_submit(&submit, ASYNC_TX_XOR_DROP_DST | ASYNC_TX_ACK, tx,
			    callback, &cmp, addr_conv);
	    tx = async_xor(xor_dest, xor_srcs, 0, NDISKS, xor_len, &submit);

	    async_tx_issue_pending_all();

	    wait_for_completion(&cmp);
    }

```
鏈夊叧杩欎簺鏍囧織鐨勬洿澶氫俊鎭紝璇峰弬闃?include/linux/async_tx.h銆傛湁鍏虫洿澶氬疄鐜扮ず渚嬶紝璇峰弬闃?
drivers/md/raid5.c 涓殑 ops_run_** 鍜?ops_complete_** 渚嬬▼銆?

## 4. Driver Development Notes


### 4.1 Conformance points


dmaengine 椹卞姩闇€瑕佺鍚堣嫢骞蹭竴鑷存€ц鐐癸紝浠ラ€傚簲浣跨敤 async_tx API 鐨勫簲鐢ㄦ墍鍋氱殑鍋囪锛?

1. 瀹屾垚鍥炶皟棰勬湡鍦?tasklet 涓婁笅鏂囦腑鍙戠敓
2. dma_async_tx_descriptor 瀛楁缁濅笉鑳藉湪 IRQ 涓婁笅鏂囦腑琚搷浣?
3. 鍦ㄦ弿杩扮娓呯悊璺緞涓娇鐢?async_tx_run_dependencies() 鏉ュ鐞嗕緷璧栨搷浣滅殑鎻愪氦

### 4.2 "My application needs exclusive control of hardware channels"


杩欎竴瑕佹眰涓昏鍑虹幇鍦?DMA 寮曟搸椹卞姩琚敤浜庢敮鎸佽澶囧埌鍐呭瓨鎿嶄綔鐨勬儏鍐点€傜敱浜庤澶氬钩鍙扮壒瀹氱殑鍘熷洜锛?
鎵ц杩欎簺鎿嶄綔鐨勯€氶亾涓嶈兘琚叡浜€傞拡瀵硅繖浜涙儏鍐垫彁渚涗簡 dma_request_channel() 鎺ュ彛銆?

```

  struct dma_chan *dma_request_channel(dma_cap_mask_t mask,
				       dma_filter_fn filter_fn,
				       void *filter_param);

```
```

  typedef bool (*dma_filter_fn)(struct dma_chan *chan, void *filter_param);

```
褰撳彲閫夌殑 'filter_fn' 鍙傛暟涓?NULL 鏃讹紝dma_request_channel 绠€鍗曞湴杩斿洖婊¤冻鑳藉姏鎺╃爜鐨勭涓€涓?
閫氶亾銆傚惁鍒欙紝褰撴帺鐮佸弬鏁颁笉瓒充互鎸囧畾鎵€闇€閫氶亾鏃讹紝鍙互浣跨敤 filter_fn 渚嬬▼鏉ヨ皟搴︾郴缁熶腑鐨勫彲鐢ㄩ€氶亾銆?
filter_fn 渚嬬▼瀵圭郴缁熶腑姣忎釜绌洪棽閫氶亾璋冪敤涓€娆°€傜湅鍒板悎閫傜殑閫氶亾鏃讹紝filter_fn 杩斿洖 DMA_ACK锛屽皢璇?
閫氶亾鏍囪涓?dma_request_channel 鐨勮繑鍥炲€笺€傞€氳繃璇ユ帴鍙ｅ垎閰嶇殑閫氶亾鍦ㄨ皟鐢?dma_release_channel()
涔嬪墠瀵硅皟鐢ㄨ€呮槸鐙崰鐨勩€?

DMA_PRIVATE 鑳藉姏鏍囧織鐢ㄤ簬鏍囪涓嶅簲琚€氱敤鍒嗛厤鍣ㄤ娇鐢ㄧ殑 dma 璁惧銆傚鏋滃凡鐭ユ煇涓€氶亾灏嗗缁堟槸绉佹湁鐨勶紝
鍙互鍦ㄥ垵濮嬪寲鏃惰缃畠銆傛垨鑰咃紝褰?dma_request_channel() 鎵惧埌涓€涓湭浣跨敤鐨勨€滃叕鍏扁€濋€氶亾鏃惰缃畠銆?

瀹炵幇椹卞姩鍜屼娇鐢ㄨ€呮椂闇€瑕佹敞鎰忓嚑鐐癸細

1. 涓€鏃︿竴涓€氶亾琚鏈夊垎閰嶏紝鍗充娇璋冪敤浜?dma_release_channel()锛岄€氱敤鍒嗛厤鍣ㄤ篃涓嶄細鍐嶈€冭檻瀹冦€?
2. 鐢变簬鑳藉姏鏄湪璁惧绾у埆鎸囧畾鐨勶紝鍏锋湁澶氫釜閫氶亾鐨?dma_device 瑕佷箞鎵€鏈夐€氶亾閮芥槸鍏叡鐨勶紝瑕佷箞鎵€鏈?
   閫氶亾閮芥槸绉佹湁鐨勩€?

### 5. Source


include/linux/dmaengine.h:
    DMA 椹卞姩鍜?api 鐢ㄦ埛鐨勬牳蹇冨ご鏂囦欢
drivers/dma/dmaengine.c:
    鍗歌浇寮曟搸閫氶亾绠＄悊渚嬬▼
drivers/dma/:
    鍗歌浇寮曟搸椹卞姩鐨勫瓨鏀句綅缃?
include/linux/async_tx.h:
    async_tx api 鐨勬牳蹇冨ご鏂囦欢
crypto/async_tx/async_tx.c:
    async_tx 鍒?dmaengine 鐨勬帴鍙ｅ強鍏叡浠ｇ爜
crypto/async_tx/async_memcpy.c:
    鎷疯礉鍗歌浇
crypto/async_tx/async_xor.c:
    xor 鍙?xor 闆跺拰鍗歌浇
