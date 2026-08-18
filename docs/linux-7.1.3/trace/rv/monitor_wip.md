## Monitor wip锛堟姠鍗犲紡鍞ら啋鐩戣鍣級


- 鍚嶇О锛歸ip - 鎶㈠崰寮忓敜閱?
- 绫诲瀷锛氭瘡 CPU 纭畾鎬ц嚜鍔ㄦ満
- 浣滆€咃細Daniel Bristot de Oliveira <bristot@kernel.org>

### 鎻忚堪


鎶㈠崰寮忓敜閱掞紙wip锛夌洃瑙嗗櫒鏄竴涓ず渚嬫€х殑姣?CPU 鐩戣鍣紝鐢ㄤ簬楠岃瘉鍞ら啋浜嬩欢鏄惁濮嬬粓鍦ㄤ互涓嬬姸鎬佷笅鍙戠敓锛?
```

                     |
                     |
                     v
                   #==================#
                   H    preemptive    H <+
                   #==================#  |
                     |                   |
                     | preempt_disable   | preempt_enable
                     v                   |
    sched_waking   +------------------+  |
  +--------------- |                  |  |
  |                |  non_preemptive  |  |
  +--------------> |                  | -+
                   +------------------+

```
鐢变簬璋冨害鍣ㄥ悓姝ョ殑鍘熷洜锛屽敜閱掍簨浠跺缁堝湪鎶㈠崰琚鐢ㄧ殑鎯呭喌涓嬪彂鐢熴€傜劧鑰岋紝鐢变簬 preempt_count 鍙婂叾 trace 浜嬩欢鐩稿浜庝腑鏂苟闈炲師瀛愭搷浣滐紝鏌愪簺
```
  preempt_disable() {
	__preempt_count_add(1)
	------->	smp_apic_timer_interrupt() {
				preempt_disable()
					do not trace (preempt count >= 1)

				wake up a thread

				preempt_enable()
					 do not trace (preempt count >= 1)
			}
	<------
	trace_preempt_disable();
  }
```
姝ら棶棰樺湪姝ゅ琚姤鍛婂苟璁ㄨ锛?
  https://lore.kernel.org/r/cover.1559051152.git.bristot@redhat.com/

### 瑙勬牸璇存槑

Grapviz Dot 鏂囦欢浣嶄簬 tools/verification/models/wip.dot
