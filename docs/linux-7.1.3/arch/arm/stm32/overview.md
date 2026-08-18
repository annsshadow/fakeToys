## STM32 ARM Linux 姒傝堪


### 绠€浠?


鎰忔硶鍗婂浣擄紙STMicroelectronics锛夌殑 STM32 绯诲垪 Cortex-A 寰鐞嗗櫒锛圡PU锛夊拰 Cortex-M 寰帶鍒跺櫒锛圡CU锛夌敱 ARM Linux 鐨?'STM32' 骞冲彴鎻愪緵鏀寔銆?

### 閰嶇疆


瀵逛簬 MCU锛屼娇鐢ㄦ彁渚涚殑榛樿閰嶇疆锛?
        make stm32_defconfig
瀵逛簬 MPU锛屼娇鐢?multi_v7 閰嶇疆锛?
        make multi_v7_defconfig

### 甯冨眬


澶氫釜鏈哄櫒绯诲垪鐨勬墍鏈夋枃浠堕兘浣嶄簬 arch/arm/mach-stm32 鍐呯殑骞冲彴浠ｇ爜涓€?

mach 鏂囦欢澶逛腑鏈変竴涓€氱敤鐨?board-dt.c锛屾敮鎸佹墎骞宠澶囨爲锛團lattened Device Tree锛夛紝杩欐剰鍛崇潃瀹冨彲浠ヤ笌浠讳綍鍏煎鐨勮澶囨爲鏉垮崱閰嶅悎宸ヤ綔銆?

:Authors:

- Maxime Coquelin <mcoquelin.stm32@gmail.com>
- Ludovic Barre <ludovic.barre@st.com>
- Gerald Baeza <gerald.baeza@st.com>
