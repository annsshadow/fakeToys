
## IMC锛圛n-Memory Collection Counters锛屽唴瀛樺唴閲囬泦璁℃暟鍣級


Anju T Sudhakar锛?019 骞?5 鏈?10 鏃?

    :depth: 3


## 鍩烘湰姒傝堪


IMC锛圛n-Memory collection counters锛屽唴瀛樺唴閲囬泦璁℃暟鍣級鏄竴绉嶇‖浠剁洃鎺ц鏂斤紝瀹冨湪 Nest 绾у埆锛堢墖鍐呬絾鏍稿锛夈€丆ore 绾у埆鍜?Thread 绾у埆鏀堕泦澶ч噺鐨勭‖浠舵€ц兘浜嬩欢銆?

Nest PMU 璁℃暟鍣ㄧ敱涓€涓繍琛屽湪 OCC锛圤n-Chip Controller锛岀墖涓婃帶鍒跺櫒锛夊鍚堜綋涓殑 Nest IMC 寰爜澶勭悊銆傝寰爜鏀堕泦璁℃暟鍣ㄦ暟鎹紝骞跺皢 nest IMC 璁℃暟鍣ㄦ暟鎹惉绉诲埌鍐呭瓨涓€?

Core 鍜?Thread IMC PMU 璁℃暟鍣ㄥ湪鏍稿唴澶勭悊銆侰ore 绾?PMU 璁℃暟鍣ㄤ负鎴戜滑鎻愪緵姣忎釜鏍哥殑 IMC 璁℃暟鍣ㄦ暟鎹紝鑰?thread 绾?PMU 璁℃暟鍣ㄤ负鎴戜滑鎻愪緵姣忎釜 CPU 绾跨▼鐨?IMC 璁℃暟鍣ㄦ暟鎹€?

OPAL 浠?IMC Catalog 鑾峰彇 IMC PMU 鍙婃墍鏀寔浜嬩欢鐨勪俊鎭紝骞堕€氳繃璁惧鏍戜紶閫掔粰鍐呮牳銆備簨浠剁殑淇℃伅鍖呭惈锛?

- 浜嬩欢鍚嶇О锛圗vent name锛?
- 浜嬩欢鍋忕Щ锛圗vent Offset锛?
- 浜嬩欢鎻忚堪锛圗vent description锛?

骞朵笖鍙兘杩樺寘鍚細

- 浜嬩欢缂╂斁锛圗vent scale锛?
- 浜嬩欢鍗曚綅锛圗vent unit锛?

鏌愪簺 PMU 鍙兘瀵瑰叾鎵€鏈夊彈鏀寔鐨勪簨浠跺叿鏈夊叡鍚岀殑 scale 鍜?unit 鍊笺€傚浜庤繖浜涙儏鍐碉紝杩欎簺浜嬩欢鐨?scale 鍜?unit 灞炴€у繀椤讳粠 PMU 缁ф壙銆?

鍐呭瓨涓殑浜嬩欢鍋忕Щ澶勫氨鏄鏁板櫒鏁版嵁琚疮鍔犵殑鍦版柟銆?

IMC catalog 浣嶄簬锛?
	https://github.com/open-power/ima-catalog

鍐呮牳鍦ㄨ澶囨爲鐨?`imc-counters` 璁惧鑺傜偣涓彂鐜?IMC 璁℃暟鍣ㄤ俊鎭紝璇ヨ妭鐐瑰叿鏈?compatible 瀛楁 `ibm,opal-in-memory-counters`銆傚唴鏍镐粠璁惧鏍戜腑瑙ｆ瀽 PMU 鍙婂叾浜嬩欢淇℃伅锛屽苟鍦ㄥ唴鏍镐腑娉ㄥ唽 PMU 鍙婂叾灞炴€с€?

## IMC 浣跨敤绀轰緥



  # perf list
  [...]
  nest_mcs01/PM_MCS01_64B_RD_DISP_PORT01/            [Kernel PMU event]
  nest_mcs01/PM_MCS01_64B_RD_DISP_PORT23/            [Kernel PMU event]
  [...]
  core_imc/CPM_0THRD_NON_IDLE_PCYC/                  [Kernel PMU event]
  core_imc/CPM_1THRD_NON_IDLE_INST/                  [Kernel PMU event]
  [...]
  thread_imc/CPM_0THRD_NON_IDLE_PCYC/                [Kernel PMU event]
  thread_imc/CPM_1THRD_NON_IDLE_INST/                [Kernel PMU event]

瑕佹煡鐪?nest_mcs0/PM_MCS_DOWN_128B_DATA_XFER_MC0/ 鐨勬瘡涓姱鐗囨暟鎹細


  # ./perf stat -e "nest_mcs01/PM_MCS01_64B_WR_DISP_PORT01/" -a --per-socket

瑕佹煡鐪?core 0 鐨勯潪绌洪棽鎸囦护锛?


  # ./perf stat -e "core_imc/CPM_NON_IDLE_INST/" -C 0 -I 1000

瑕佹煡鐪?"make" 鐨勯潪绌洪棽鎸囦护锛?


  # ./perf stat -e "thread_imc/CPM_NON_IDLE_PCYC/" make


## IMC 璺熻釜妯″紡锛圱race-mode锛?


POWER9 鏀寔 IMC 鐨勪袱绉嶆ā寮忥細绱姞锛圓ccumulation锛夋ā寮忓拰璺熻釜锛圱race锛夋ā寮忋€傚湪绱姞妯″紡涓嬶紝浜嬩欢璁℃暟鍦ㄧ郴缁熷唴瀛樹腑绱姞銆傜劧鍚?Hypervisor 浼氬懆鏈熸€у湴鎴栧湪琚姹傛椂璇诲彇杩欎簺宸叉彁浜ょ殑璁℃暟銆傚湪 IMC 璺熻釜妯″紡涓嬶紝64 浣嶇殑 trace SCOM 鍊艰鍒濆鍖栦负浜嬩欢淇℃伅銆倀race SCOM 涓殑 CPMCxSEL 鍜?CPMC_LOAD 鎸囧畾浜嗚鐩戞帶鐨勪簨浠朵互鍙婇噰鏍锋椂闀裤€傚湪 CPMCxSEL 姣忔婧㈠嚭鏃讹紝纭欢浼氬揩鐓х▼搴忚鏁板櫒浠ュ強浜嬩欢璁℃暟锛屽苟鍐欏叆鐢?LDBAR 鎸囧悜鐨勫唴瀛樸€?

LDBAR 鏄竴涓?64 浣嶇殑姣忕嚎绋嬬壒娈婄敤閫斿瘎瀛樺櫒锛屽畠鐨勪綅鐢ㄤ簬鎸囩ず纭欢鏄厤缃负绱姞妯″紡杩樻槸璺熻釜妯″紡銆?

### LDBAR 瀵勫瓨鍣ㄥ竷灞€


  +-------+----------------------+
  | 0     | Enable/Disable       |
  +-------+----------------------+
  | 1     | 0: Accumulation Mode |
  |       +----------------------+
  |       | 1: Trace Mode        |
  +-------+----------------------+
  | 2:3   | Reserved             |
  +-------+----------------------+
  | 4-6   | PB scope             |
  +-------+----------------------+
  | 7     | Reserved             |
  +-------+----------------------+
  | 8:50  | Counter Address      |
  +-------+----------------------+
  | 51:63 | Reserved             |
  +-------+----------------------+

### TRACE_IMC_SCOM 浣嶈〃绀?


  +-------+------------+
  | 0:1   | SAMPSEL    |
  +-------+------------+
  | 2:33  | CPMC_LOAD  |
  +-------+------------+
  | 34:40 | CPMC1SEL   |
  +-------+------------+
  | 41:47 | CPMC2SEL   |
  +-------+------------+
  | 48:50 | BUFFERSIZE |
  +-------+------------+
  | 51:63 | RESERVED   |
  +-------+------------+

CPMC_LOAD 鍖呭惈閲囨牱鏃堕暱銆係AMPSEL 鍜?CPMCxSEL 鍐冲畾瑕佽鏁扮殑浜嬩欢銆侭UFFERSIZE 鎸囩ず鍐呭瓨鑼冨洿銆傛瘡娆℃孩鍑烘椂锛岀‖浠朵細蹇収绋嬪簭璁℃暟鍣ㄤ互鍙婁簨浠惰鏁帮紝骞舵洿鏂板唴瀛樺苟閲嶆柊鍔犺浇 CMPC_LOAD 鍊间互杩涜涓嬩竴娆￠噰鏍枫€侷MC 纭欢涓嶆敮鎸佸紓甯革紝鍥犳濡傛灉鍐呭瓨缂撳啿鍖哄埌杈炬湯灏撅紝瀹冧細闈欓粯鍦板洖缁曘€?

**鐩墠锛岃窡韪ā寮忎笅鐩戞帶鐨勪簨浠跺浐瀹氫负 cycle銆?*

## 璺熻釜 IMC 浣跨敤绀轰緥



  # perf list
  [....]
  trace_imc/trace_cycles/                            [Kernel PMU event]

瑕佽褰曚竴涓娇鐢?trace-imc 浜嬩欢鐨勫簲鐢ㄧ▼搴?杩涚▼锛?


  # perf record -e trace_imc/trace_cycles/ yes > /dev/null
  [ perf record: Woken up 1 times to write data ]
  [ perf record: Captured and wrote 0.012 MB perf.data (21 samples) ]

鐢熸垚鐨?`perf.data` 鍙互浣跨敤 perf report 璇诲彇銆?

## 浣跨敤 IMC 璺熻釜妯″紡鐨勫ソ澶?


閬垮厤浜?PMI锛圥erformance Monitoring Interrupts锛屾€ц兘鐩戞帶涓柇锛変腑鏂鐞嗭紝鍥犱负 IMC 璺熻釜妯″紡浼氬揩鐓х▼搴忚鏁板櫒骞舵洿鏂板埌鍐呭瓨銆傝繖涔熸彁渚涗簡涓€绉嶆柟寮忥紝璁╂搷浣滅郴缁熷湪涓嶄骇鐢?PMI 澶勭悊寮€閿€鐨勬儏鍐典笅瀹炴椂杩涜鎸囦护閲囨牱銆?

浣跨敤 `perf top` 甯︿笌涓嶅甫 trace-imc 浜嬩欢鏃剁殑鎬ц兘鏁版嵁銆?

鎵ц `perf top` 鍛戒护浣嗕笉甯?trace-imc 浜嬩欢鏃讹紝浼氱粺璁?PMI 涓柇璁℃暟銆?


  # grep PMI /proc/interrupts
  PMI:          0          0          0          0   Performance monitoring interrupts
  # ./perf top
  ...
  # grep PMI /proc/interrupts
  PMI:      39735       8710      17338      17801   Performance monitoring interrupts
  # ./perf top -e trace_imc/trace_cycles/
  ...
  # grep PMI /proc/interrupts
  PMI:      39735       8710      17338      17801   Performance monitoring interrupts


涔熷氨鏄锛屼娇鐢?`trace_imc` 浜嬩欢鏃讹紝PMI 涓柇璁℃暟涓嶄細澧炲姞銆?
