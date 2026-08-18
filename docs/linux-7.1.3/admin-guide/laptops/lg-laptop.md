

## LG Gram 绗旇鏈澶栫壒鎬?

By Matan Ziv-Av <matan@svgalib.org>


### 鐑敭


浠ヤ笅 FN 閿湪娌℃湁姝ら┍鍔ㄦ椂浼氳鍐呮牳蹇界暐锛?
- FN-F1锛圠G 鎺у埗闈㈡澘锛?  - 浜х敓 F15
- FN-F5锛堣Е鎽告澘寮€鍏筹級    - 浜х敓 F21
- FN-F6锛堥琛屾ā寮忥級      - 浜х敓 RFKILL
- FN-F9锛堥槄璇绘ā寮忥級      - 浜х敓 F14

鍏朵綑 FN 閿棤闇€鐗规畩椹卞姩鍗冲彲宸ヤ綔銆?

### 闃呰妯″紡


鍚?/sys/devices/platform/lg-laptop/reader_mode 鍐欏叆 0/1 鍙鐢?鍚敤闃呰妯″紡銆傚湪姝ゆā寮忎笅灞忓箷棰滆壊浼氭敼鍙橈紙钃濊壊鍑忓皯锛夛紝骞朵笖闃呰妯″紡鎸囩ず鐏?LED锛堜綅浜?F9 閿笂锛変寒璧枫€?

### FN 閿佸畾


鍚?/sys/devices/platform/lg-laptop/fn_lock 鍐欏叆 0/1 鍙鐢?鍚敤 FN 閿佸畾銆?

### 鐢垫睜淇濆吇涓婇檺


鍚?/sys/class/power_supply/CMB0/charge_control_end_threshold 鍐欏叆 80/100 鍙缃數姹犲厖鐢电殑鏈€澶у閲忋€傞檺鍒跺厖鐢靛彲鍑忓皯鐢垫睜瀹归噺闅忔椂闂存崯鑰椼€?
璇ュ€煎湪 kernel 寮曞鏃堕噸缃负 100銆?

### 椋庢墖妯″紡


鍚?/sys/devices/platform/lg-laptop/fan_mode 鍐欏叆 0/1/2 鍙垎鍒皢椋庢墖妯″紡璁句负 鏈€浼?闈欓煶/鎬ц兘銆?

### USB 鍏呯數


鍚?/sys/devices/platform/lg-laptop/usb_charge 鍐欏叆 0/1 鍙湪璁惧鍏虫満鏃剁鐢?鍚敤浠?USB 绔彛涓哄彟涓€鍙拌澶囧厖鐢点€?
璇ュ€煎湪 kernel 寮曞鏃堕噸缃负 0銆?

#### LED


椹卞姩鏀寔涓や釜 LED 璁惧锛?

### 閿洏鑳屽厜鐏?

涓€涓悕涓?kbd_led 鐨?led 璁惧鎺у埗閿洏鑳屽厜鐏€傚叡鏈変笁涓寒搴︾骇鍒細鍏抽棴锛?锛夈€佷綆锛?27锛夊拰楂橈紙255锛夈€?
閿洏鑳屽厜鐏篃鐢辨寜閿粍鍚?FN-F8 鎺у埗锛岃缁勫悎鍦ㄨ繖浜涚骇鍒棿寰幆鍒囨崲銆?

### 瑙︽懜鏉挎寚绀虹伅 LED


浣嶄簬 F5 閿笂銆傜敱鍚嶄负 tpad_led 鐨?led 璁惧鎺у埗銆?