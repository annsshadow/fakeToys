## 鐜舰缂撳啿鍖?

:Author: David Howells <dhowells@redhat.com>
:Author: Paul E. McKenney <paulmck@linux.ibm.com>


Linux 鎻愪緵浜嗚嫢骞插彲鐢ㄤ簬瀹炵幇鐜舰缂撳啿锛坈ircular buffering锛夌殑鐗规€с€傝繖绫荤壒鎬ф湁涓ょ粍锛?
 (1) 鐢ㄤ簬纭畾 2 鐨勫箓澶у皬缂撳啿鍖虹浉鍏充俊鎭殑渚挎嵎鍑芥暟銆?
 (2) 褰撶紦鍐插尯鐨勭敓浜ц€呬笌娑堣垂鑰呬笉鎯冲叡浜攣鏃舵墍鐢ㄧ殑鍐呭瓨灞忛殰銆?
濡備笅鏂囨墍杩帮紝瑕佷娇鐢ㄨ繖浜涜鏂斤紝闇€瑕佷笖浠呴渶瑕佷竴涓敓浜ц€呬笌涓€涓秷璐硅€呫€傞€氳繃鎶婂畠浠覆琛屽寲锛屼篃鍙互澶勭悊澶氫釜鐢熶骇鑰咃紱閫氳繃涓茶鍖栵紝涔熷彲浠ュ鐞嗗涓秷璐硅€呫€?

 (*) 浠€涔堟槸鐜舰缂撳啿鍖猴紵

 (*) 娴嬮噺 2 鐨勫箓缂撳啿鍖恒€?
 (*) 鍦ㄧ幆褰㈢紦鍐插尯涓娇鐢ㄥ唴瀛樺睆闅溿€?     - 鐢熶骇鑰呫€?     - 娑堣垂鑰呫€?

## 浠€涔堟槸鐜舰缂撳啿鍖猴紵


棣栧厛锛屼粈涔堟槸鐜舰缂撳啿鍖猴紵鐜舰缂撳啿鍖烘槸涓€绉嶅浐瀹氥€佹湁闄愬ぇ灏忕殑缂撳啿鍖猴紝鍏朵腑鍖呭惈涓や釜绱㈠紩锛?
 (1) 'head'锛堝ご锛夌储寮曗€斺€旂敓浜ц€呭悜缂撳啿鍖烘彃鍏ユ潯鐩殑浣嶇疆銆?
 (2) 'tail'锛堝熬锛夌储寮曗€斺€旀秷璐硅€呭湪缂撳啿鍖轰腑鎵惧埌涓嬩竴涓潯鐩殑浣嶇疆銆?
閫氬父褰?tail 鎸囬拡绛変簬 head 鎸囬拡鏃讹紝缂撳啿鍖轰负绌猴紱褰?head 鎸囬拡姣?tail 鎸囬拡灏?1 鏃讹紝缂撳啿鍖轰负婊°€?
娣诲姞鏉＄洰鏃?head 绱㈠紩閫掑锛岀Щ闄ゆ潯鐩椂 tail 绱㈠紩閫掑銆倀ail 绱㈠紩缁濅笉搴旇秴杩?head 绱㈠紩锛屽苟涓斾袱涓储寮曞埌杈剧紦鍐插尯鏈熬鏃堕兘搴斿洖缁曞埌 0锛屼粠鑰屽厑璁告棤闄愰噺鐨勬暟鎹祦缁忚缂撳啿鍖恒€?
閫氬父锛屾潯鐩兘鍏锋湁鐩稿悓鐨勫崟浣嶅ぇ灏忥紝浣嗕娇鐢ㄤ笅杩版妧宸у苟涓嶄弗鏍艰姹傚姝ゃ€傚鏋滆鍚戠紦鍐插尯鏀惧叆澶氫釜鏉＄洰鎴栧彉闀挎潯鐩紝绱㈠紩鍙互涓€娆″鍔犲浜?1锛屽墠鎻愭槸涓や釜绱㈠紩閮戒笉浼氳秴杩囧鏂广€備笉杩囧疄鐜拌€呭繀椤诲皬蹇冿紝鍥犱负澶т簬涓€涓崟浣嶅ぇ灏忕殑鍖哄煙鍙兘浼氬湪缂撳啿鍖烘湯灏惧洖缁曪紝浠庤€岃鍒嗗壊鎴愪袱娈点€?
## 娴嬮噺 2 鐨勫箓缂撳啿鍖?

璁＄畻浠绘剰澶у皬鐜舰缂撳啿鍖虹殑鍗犵敤鎯呭喌鎴栧墿浣欏閲忛€氬父鏄竴椤硅緝鎱㈢殑鎿嶄綔锛岄渶瑕佷娇鐢ㄥ彇妯★紙闄ゆ硶锛夋寚浠ゃ€備笉杩囷紝濡傛灉缂撳啿鍖哄ぇ灏忎负 2 鐨勫箓锛屽氨鍙互鏀圭敤蹇緱澶氱殑鎸変綅涓庯紙bitwise-AND锛夋寚浠ゃ€?
Linux 鎻愪緵浜嗕竴缁勭敤浜庡鐞?2 鐨勫箓鐜舰缂撳啿鍖虹殑瀹忋€傝繖浜?```
	#include <linux/circ_buf.h>

```
杩欎簺瀹忓寘鎷細

```
	CIRC_SPACE(head_index, tail_index, buffer_size);

     This returns the amount of space left in the buffer[1] into which items
     can be inserted.


 (#) Measure the maximum consecutive immediate space in a buffer::

	CIRC_SPACE_TO_END(head_index, tail_index, buffer_size);

     This returns the amount of consecutive space left in the buffer[1] into
     which items can be immediately inserted without having to wrap back to the
     beginning of the buffer.


 (#) Measure the occupancy of a buffer::

	CIRC_CNT(head_index, tail_index, buffer_size);

     This returns the number of items currently occupying a buffer[2].


 (#) Measure the non-wrapping occupancy of a buffer::

	CIRC_CNT_TO_END(head_index, tail_index, buffer_size);

     This returns the number of consecutive items[2] that can be extracted from
     the buffer without having to wrap back to the beginning of the buffer.


```

杩欎簺瀹忓悕涔変笂閮戒細杩斿洖浠嬩簬 0 涓?buffer_size-1 涔嬮棿鐨勫€硷紝浣嗘槸锛?
 (1) CIRC_SPACE*() 鐢ㄤ簬鐢熶骇鑰呬竴绔€傚鐢熶骇鑰呰€岃█锛屽畠浠繑鍥炵殑鏄笅鐣岋紝鍥犱负鐢熶骇鑰呮帶鍒剁潃 head 绱㈠紩锛屼絾娑堣垂鑰呭彲鑳戒粛鍦ㄥ彟涓€涓?CPU 涓婃秷鑰楃紦鍐插尯骞剁Щ鍔?tail 绱㈠紩銆傚娑堣垂鑰呰€岃█锛屽畠鏄剧ず鐨勬槸涓婄晫锛屽洜涓虹敓浜ц€呭彲鑳芥蹇欎簬娑堣€楃┖闂淬€?
 (2) CIRC_CNT*() 鐢ㄤ簬娑堣垂鑰呬竴绔€傚娑堣垂鑰呰€岃█锛屽畠浠繑鍥炵殑鏄笅鐣岋紝鍥犱负娑堣垂鑰呮帶鍒剁潃 tail 绱㈠紩锛屼絾鐢熶骇鑰呭彲鑳戒粛鍦ㄥ彟涓€涓?CPU 涓婂～鍏呯紦鍐插尯骞剁Щ鍔?head 绱㈠紩銆傚鐢熶骇鑰呰€岃█锛屽畠鏄剧ず鐨勬槸涓婄晫锛屽洜涓烘秷璐硅€呭彲鑳芥蹇欎簬娓呯┖缂撳啿鍖恒€?
 (3) 瀵圭涓夋柟鑰岃█锛岀敓浜ц€呭拰娑堣垂鑰呭绱㈠紩鐨勫啓鍏ヤ綍鏃跺彉寰楀彲瑙侊紝鏄棤娉曚繚璇侀『搴忕殑锛屽洜涓轰袱鑰呯浉浜掔嫭绔嬶紝涓斿彲鑳藉彂鐢熷湪涓嶅悓鐨?CPU 涓娾€斺€斿洜姝よ繖绉嶆儏鍐典笅鐨勭粨鏋滃彧鑳界畻鐚滄祴锛岀敋鑷冲彲鑳戒负璐熸暟銆?
## 鍦ㄧ幆褰㈢紦鍐插尯涓娇鐢ㄥ唴瀛樺睆闅?

閫氳繃鍦ㄧ幆褰㈢紦鍐插尯涓粨鍚堜娇鐢ㄥ唴瀛樺睆闅滐紝浣犲彲浠ラ伩鍏嶏細

 (1) 浣跨敤鍗曚釜閿佹潵绠＄悊缂撳啿鍖轰袱绔殑璁块棶锛屼粠鑰屽厑璁哥紦鍐插尯鍚屾椂琚～鍏呭拰娓呯┖锛涗互鍙?
 (2) 浣跨敤鍘熷瓙璁℃暟鍣ㄦ搷浣溿€?
杩欐湁涓ゆ柟锛氬～鍏呯紦鍐插尯鐨勭敓浜ц€咃紝浠ュ強娓呯┖瀹冪殑娑堣垂鑰呫€備换浣曟椂鍒诲簲褰撳彧鏈変竴涓富浣撳湪濉厖缂撳啿锛屼篃搴斿綋鍙湁涓€涓富浣撳湪娓呯┖缂撳啿锛屼絾涓ゆ柟鍙互鍚屾椂鎿嶄綔銆?

### 鐢熶骇鑰?

```
	spin_lock(&producer_lock);

	unsigned long head = buffer->head;
	/* The spin_unlock() and next spin_lock() provide needed ordering. */
	unsigned long tail = READ_ONCE(buffer->tail);

	if (CIRC_SPACE(head, tail, buffer->size) >= 1) {
		/* insert one item into the buffer */
		struct item *item = buffer[head];

		produce_item(item);

		smp_store_release(buffer->head,
				  (head + 1) & (buffer->size - 1));

		/* wake_up() will make sure that the head is committed before
		 * waking anyone up */
		wake_up(consumer);
	}

	spin_unlock(&producer_lock);

```
杩欎細鎸囩ず CPU锛氭柊鏉＄洰鐨勫唴瀹瑰繀椤诲湪 head 绱㈠紩灏嗗叾瀵规秷璐硅€呭彲瑙佷箣鍓嶅啓鍏ワ紱闅忓悗鎸囩ず CPU锛氫慨鏀瑰悗鐨?head 绱㈠紩蹇呴』鍦ㄥ敜閱掓秷璐硅€呬箣鍓嶅啓鍏ャ€?
娉ㄦ剰锛寃ake_up() 骞朵笉鑳戒繚璇佷换浣曞舰寮忕殑鍐呭瓨灞忛殰锛岄櫎闈炵‘瀹炴湁瀵硅薄琚敜閱掋€傚洜姝ゆ垜浠笉鑳戒緷璧栧畠鏉ヤ繚璇侀『搴忋€備笉杩囷紝鏁扮粍涓€讳細鐣欎竴涓厓绱犱负绌恒€傚洜姝わ紝鐢熶骇鑰呭繀椤诲厛鐢熶骇涓や釜鍏冪礌锛屾墠鍙兘鐮村潖娑堣垂鑰呭綋鍓嶆鍦ㄨ鍙栫殑鍏冪礌銆傚洜姝わ紝娑堣垂鑰呰繛缁袱娆¤皟鐢ㄤ箣闂寸殑瑙ｉ攣-鍔犻攣瀵癸紝鎻愪緵浜嗗繀瑕佺殑椤哄簭淇濊瘉锛氬畠浠嬩簬"璇诲彇琛ㄦ槑娑堣垂鑰呭凡鑵惧嚭鏌愬厓绱犵殑绱㈠紩"涓?鐢熶骇鑰呭悜璇ュ悓涓€鍏冪礌鍐欏叆"涔嬮棿銆?

### 娑堣垂鑰?

```
	spin_lock(&consumer_lock);

	/* Read index before reading contents at that index. */
	unsigned long head = smp_load_acquire(buffer->head);
	unsigned long tail = buffer->tail;

	if (CIRC_CNT(head, tail, buffer->size) >= 1) {

		/* extract one item from the buffer */
		struct item *item = buffer[tail];

		consume_item(item);

		/* Finish reading descriptor before incrementing tail. */
		smp_store_release(buffer->tail,
				  (tail + 1) & (buffer->size - 1));
	}

	spin_unlock(&consumer_lock);

```
杩欎細鎸囩ず CPU锛氬湪璇诲彇鏂版潯鐩箣鍓嶅厛纭繚绱㈠紩鏄渶鏂扮殑锛涢殢鍚庣‘淇?CPU 宸插畬鎴愬璇ユ潯鐩殑璇诲彇锛屽啀鍐欏叆鏂扮殑 tail 鎸囬拡鈥斺€旇鎸囬拡浼氭姽鎺夎鏉＄洰銆?
娉ㄦ剰杩欓噷浣跨敤 READ_ONCE() 鍜?smp_load_acquire() 鏉ヨ鍙栧鏂圭殑绱㈠紩銆傝繖鍙互闃叉缂栬瘧鍣ㄤ涪寮冨苟閲嶆柊鍔犺浇鍏剁紦瀛樼殑鍊笺€傚鏋滀綘鑳界‘瀹氬鏂圭储寮曞彧浼氫娇鐢ㄤ竴娆★紝閭ｄ箞涓ユ牸鏉ヨ杩欏苟闈炲繀闇€銆俿mp_load_acquire() 杩樹細寮哄埗 CPU 瀵瑰悗缁殑鍐呭瓨璁块棶杩涜鎺掑簭銆傜被浼煎湴锛屼袱绉嶇畻娉曚腑閮戒娇鐢?smp_store_release() 鏉ュ啓鍏ユ湰绾跨▼鐨勭储寮曘€傝繖璁板綍浜?鎴戜滑姝ｅ湪鍐欏叆涓€涓彲鑳借骞跺彂璇诲彇鐨勫璞?杩欎竴浜嬪疄锛岄槻姝㈢紪璇戝櫒瀵瑰啓鍏ヨ繘琛屾媶鍒嗭紙tearing锛夛紝骞跺己鍒剁浉瀵逛簬鍏堝墠鐨勮闂繘琛屾帓搴忋€?

## 寤朵几闃呰


鍙﹁ Documentation/memory-barriers.txt锛屽叾涓弿杩颁簡 Linux 鐨勫唴瀛樺睆闅滆鏂姐€?