


## 鏈湴鍘熷瓙鎿嶄綔鐨勮涔変笌琛屼负


:Author: Mathieu Desnoyers


鏈枃妗ｈВ閲婁簡鏈湴鍘熷瓙鎿嶄綔鐨勭洰鐨勩€佸浣曚负浠绘剰缁欏畾鏋舵瀯瀹炵幇瀹冧滑锛屽苟璇存槑濡備綍姝ｇ‘
浣跨敤瀹冧滑銆傚畠杩樺己璋冧簡褰撳唴瀛樺啓鍏ラ『搴忓緢閲嶈鏃讹紝璺?CPU 璇诲彇杩欎簺鏈湴鍙橀噺蹇呴』閲囧彇
鐨勯闃叉帾鏂姐€?

    娉ㄦ剰锛屽熀浜?`local_t` 鐨勬搷浣滀笉寤鸿鐢ㄤ簬閫氱敤鐨勫唴鏍稿満鏅€傞櫎闈炵‘瀹炴湁鐗规畩鐢ㄩ€旓紝
    鍚﹀垯璇锋敼鐢?`this_cpu` 鎿嶄綔銆傚唴鏍镐腑 `local_t` 鐨勫ぇ閮ㄥ垎鐢ㄦ硶閮藉凡琚?`this_cpu`
    鎿嶄綔鎵€鍙栦唬銆俙this_cpu` 鎿嶄綔灏嗛噸瀹氫綅涓庣被浼?`local_t` 鐨勮涔夊悎骞跺埌鍗曟潯鎸囦护涓紝
    浠庤€岀敓鎴愭洿绱у噾銆佹墽琛屾洿蹇殑浠ｇ爜銆?

## 鏈湴鍘熷瓙鎿嶄綔鐨勭洰鐨?

鏈湴鍘熷瓙鎿嶄綔鏃ㄥ湪鎻愪緵蹇€熶笖楂樺害鍙噸鍏ョ殑姣?CPU 璁℃暟鍣ㄣ€傚畠浠€氳繃鍘婚櫎閫氬父鐢ㄤ簬璺?CPU 鍚屾鐨?LOCK 鍓嶇紑鍜屽唴瀛樺睆闅滐紝灏嗘爣鍑嗗師瀛愭搷浣滅殑鎬ц兘寮€閿€闄嶅埌鏈€浣庛€?
鍦ㄨ澶氭儏鍐典笅锛屾嫢鏈夊揩閫熺殑姣?CPU 鍘熷瓙璁℃暟鍣ㄥ緢鏈変环鍊硷細瀹冧笉闇€瑕佺鐢ㄤ腑鏂潵淇濇姢
涓柇澶勭悊绋嬪簭锛屽苟涓斿厑璁稿湪 NMI 澶勭悊绋嬪簭涓繚鎸佷竴鑷寸殑璁℃暟鍣ㄣ€傚畠瀵逛簬璺熻釜鐩殑浠ュ強
鍚勭鎬ц兘鐩戣璁℃暟鍣ㄧ壒鍒湁鐢ㄣ€?
鏈湴鍘熷瓙鎿嶄綔浠呬繚璇佸彉閲忎慨鏀圭浉瀵逛簬鎷ユ湁璇ユ暟鎹殑 CPU 鏄師瀛愮殑銆傚洜姝わ紝蹇呴』灏忓績
纭繚鍙湁涓€涓?CPU 鍐欏叆 `local_t` 鏁版嵁銆傝繖鏄€氳繃浣跨敤姣?CPU 鏁版嵁骞剁‘淇濆湪鍙畨鍏?鎶㈠崰鐨勪笂涓嬫枃涓慨鏀瑰畠鏉ュ疄鐜扮殑銆備絾鍏佽浠庝换浣?CPU 璇诲彇 `local_t` 鏁版嵁锛氭鏃跺畠浼?琛ㄧ幇涓虹浉瀵逛簬鎵€鏈夎€?CPU 鐨勫叾浠栧唴瀛樺啓鍏ユ槸涔卞簭鐨勩€?

## 閽堝鐗瑰畾鏋舵瀯鐨勫疄鐜?

杩欏彲浠ラ€氳繃绋嶅井淇敼鏍囧噯鍘熷瓙鎿嶄綔鏉ュ疄鐜帮細鍙繚鐣欏畠浠殑 UP 鍙樹綋銆傝繖閫氬父鎰忓懗鐫€
绉婚櫎 LOCK 鍓嶇紑锛堝湪 i386 鍜?x86_64 涓婏級浠ュ強浠讳綍 SMP 鍚屾灞忛殰銆傚鏋滄灦鏋勫湪 SMP
鍜?UP 涔嬮棿娌℃湁涓嶅悓琛屼负锛岄偅涔堝湪鎮ㄦ灦鏋勭殑 `local.h` 涓寘鍚?`asm-generic/local.h`
鍗冲彲銆?
`local_t` 绫诲瀷閫氳繃灏嗕竴涓?`atomic_long_t` 宓屽叆缁撴瀯涓紝琚畾涔変负涓€涓笉閫忔槑鐨?`signed long`銆傝繖鏍峰仛鏄负浜嗕娇浠庤绫诲瀷鍒?
```
    typedef struct { atomic_long_t a; } local_t;

```
## 浣跨敤鏈湴鍘熷瓙鎿嶄綔鏃跺簲閬靛惊鐨勮鍒?

- 琚湰鍦版搷浣滆闂殑鍙橀噺蹇呴』鏄瘡 CPU 鍙橀噺銆?- **鍙湁**杩欎簺鍙橀噺鐨?CPU 鎵€鏈夎€呮墠鑳藉啓鍏ュ畠浠€?- 璇?CPU 鍙互浠庝换浣曚笂涓嬫枃锛堣繘绋嬨€乮rq銆乻oftirq銆乶mi鈥︹€︼級浣跨敤鏈湴鎿嶄綔鏉ユ洿鏂板叾
  `local_t` 鍙橀噺銆?- 鍦ㄨ繘绋嬩笂涓嬫枃涓娇鐢ㄦ湰鍦版搷浣滄椂锛屽繀椤荤鐢ㄦ姠鍗狅紙鎴栦腑鏂級锛屼互纭繚杩涚▼鍦ㄨ幏鍙栨瘡
  CPU 鍙橀噺鍒版墽琛屽疄闄呮湰鍦版搷浣滀箣闂翠笉浼氳杩佺Щ鍒颁笉鍚岀殑 CPU銆?- 鍦ㄤ腑鏂笂涓嬫枃涓娇鐢ㄦ湰鍦版搷浣滄椂锛屽湪涓荤嚎绋嬪唴鏍镐笂鏃犻渶鐗瑰埆灏忓績锛屽洜涓哄畠浠細鍦ㄦ湰鍦?  CPU 涓婅繍琛岋紝涓旀姠鍗犲凡琚鐢ㄣ€備笉杩囷紝鎴戜粛寤鸿鏄惧紡绂佺敤鎶㈠崰锛屼互纭繚鍏跺湪 -rt 鍐呮牳
  涓婁粛鑳芥甯稿伐浣溿€?- 璇诲彇鏈湴 CPU 鍙橀噺灏嗗緱鍒拌鍙橀噺鐨勫綋鍓嶅壇鏈€?- 鍙互浠庝换浣?CPU 璇诲彇杩欎簺鍙橀噺锛屽洜涓哄瀵归綈鐨?"`long`" 鍙橀噺鐨勬洿鏂板缁堟槸鍘熷瓙鐨勩€?  鐢变簬鍐欏叆 CPU 涓嶈繘琛屽唴瀛樺悓姝ワ紝褰撹鍙栨煇浜?*鍏朵粬** CPU 鐨勫彉閲忔椂锛屽彲鑳借鍒颁竴涓?  杩囨椂鐨勫彉閲忓壇鏈€?

## 濡備綍浣跨敤鏈湴鍘熷瓙鎿嶄綔


```
    #include <linux/percpu.h>
    #include <asm/local.h>

    static DEFINE_PER_CPU(local_t, counters) = LOCAL_INIT(0);

```
## 璁℃暟


璁℃暟閽堝鏈夌鍙烽暱鏁村瀷鐨勬墍鏈変綅杩涜銆?
鍦ㄥ彲鎶㈠崰涓婁笅鏂囦腑锛屽湪鏈湴鍘熷瓙鎿嶄綔鍓嶅悗浣跨敤 `get_cpu_var()` 涓?`put_cpu_var()`锛?鍙‘淇濆啓鍏ュ懆鍥寸殑鎶㈠崰琚鐢?
```
    local_inc(&get_cpu_var(counters));
    put_cpu_var(counters);

```
濡傛灉鎮ㄥ凡缁忓浜庡彲瀹夊叏鎶㈠崰鐨勪笂涓嬫枃涓紝鍙互浣跨敤

```
    local_inc(this_cpu_ptr(&counters));



```
## 璇诲彇璁℃暟鍣?

鍙互浠庡叾浠?CPU 璇诲彇杩欎簺鏈湴璁℃暟鍣ㄤ互瀵硅鏁版眰鍜屻€傝娉ㄦ剰锛岃法 CPU 鐨?local_read
鎵€瑙佺殑鏁版嵁搴旇瑙嗕负涔卞簭鐨?
```
    long sum = 0;
    for_each_online_cpu(cpu)
            sum += local_read(&per_cpu(counters, cpu));

```
濡傛灉鎮ㄦ兂浣跨敤杩滅▼ local_read 鏉ュ湪 CPU 涔嬮棿鍚屾瀵硅祫婧愮殑璁块棶锛屽垯蹇呴』鍦ㄥ啓鍏?CPU
鍜岃鍙?CPU 涓婂垎鍒娇鐢ㄦ樉寮忕殑 `smp_wmb()` 涓?`smp_rmb()` 鍐呭瓨灞忛殰銆傚鏋滄偍灏?`local_t` 鍙橀噺鐢ㄤ綔缂撳啿鍖轰腑宸插啓鍏ュ瓧鑺傜殑璁℃暟鍣紝灏变細鏄繖鏍风殑鎯呭喌锛氬湪缂撳啿鍖哄啓鍏?涓庤鏁板櫒閫掑涔嬮棿搴旀湁涓€涓?`smp_wmb()`锛屽湪璁℃暟鍣ㄨ鍙栦笌缂撳啿鍖鸿鍙栦箣闂翠篃搴旀湁涓€涓?`smp_rmb()`銆?

涓嬮潰鏄竴涓ず渚嬫ā鍧楋紝瀹冧娇鐢?
```
    /* test-local.c
     *
     * Sample module for local.h usage.
     */

    #include <asm/local.h>
    #include <linux/module.h>
    #include <linux/timer.h>

    static DEFINE_PER_CPU(local_t, counters) = LOCAL_INIT(0);

    static struct timer_list test_timer;

    /* IPI called on each CPU. */
    static void test_each(void *info)
    {
            /* Increment the counter from a non preemptible context */
            printk("Increment on cpu %d\n", smp_processor_id());
            local_inc(this_cpu_ptr(&counters));

            /* This is what incrementing the variable would look like within a
             * preemptible context (it disables preemption) :
             *
             * local_inc(&get_cpu_var(counters));
             * put_cpu_var(counters);
             */
    }

    static void do_test_timer(unsigned long data)
    {
            int cpu;

            /* Increment the counters */
            on_each_cpu(test_each, NULL, 1);
            /* Read all the counters */
            printk("Counters read from CPU %d\n", smp_processor_id());
            for_each_online_cpu(cpu) {
                    printk("Read : CPU %d, count %ld\n", cpu,
                            local_read(&per_cpu(counters, cpu)));
            }
            mod_timer(&test_timer, jiffies + 1000);
    }

    static int __init test_init(void)
    {
            /* initialize the timer that will increment the counter */
            timer_setup(&test_timer, do_test_timer, 0);
            mod_timer(&test_timer, jiffies + 1);

            return 0;
    }

    static void __exit test_exit(void)
    {
            timer_shutdown_sync(&test_timer);
    }

    module_init(test_init);
    module_exit(test_exit);

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mathieu Desnoyers");
    MODULE_DESCRIPTION("Local Atomic Ops");

```
