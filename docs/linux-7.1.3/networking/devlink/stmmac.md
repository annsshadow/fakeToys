## stmmac锛坰ynopsys dwmac锛塪evlink 鏀寔


鏈枃妗ｆ弿杩颁簡鐢?`stmmac` 璁惧椹卞姩瀹炵幇鐨?devlink 鍔熻兘銆?
## 鍙傛暟锛圥arameters锛?

`stmmac` 椹卞姩瀹炵幇浜嗕互涓嬮┍鍔ㄧ壒瀹氱殑鍙傛暟銆?
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `phc_coarse_adj`
     - Boolean
     - runtime
     - 鍚敤绮楃矑搴︼紙Coarse锛夋椂闂存埑妯″紡锛屽 DWMAC TRM 涓墍瀹氫箟銆?      鏈夊叧璇ユ椂闂存埑妯″紡鐨勮缁嗚鏄庯紝璇峰弬瑙?       Socfpga 鍔熻兘鎻忚堪 [^1^]銆?
       鍦?Coarse 妯″紡涓嬶紝ptp 鏃堕挓棰勬湡鐢变竴涓珮绮惧害銆佸閮ㄨ皟鏁寸殑鏃堕挓椹卞姩锛?       鐢ㄤ簬鏃堕棿鎴崇殑瀛愮澧為噺锛坰ubsecond increment锛夎缃负 1/ptp_clock_rate銆?
       鍦?Fine 妯″紡锛堝嵆 Coarse 妯″紡 == false锛変笅锛宲tp 鏃堕挓棰戠巼浼氳杩炵画璋冩暣锛?       浣嗗瓙绉掑閲忚缃负 2/ptp_clock_rate銆?
       Coarse 妯″紡閫傜敤浜?PTP 涓绘椂閽燂紙Grand Master锛夋搷浣溿€傚鏋滀笉纭畾锛岃灏?       璇ュ弬鏁颁繚鎸佷负 False銆?
       [^1^] https://www.intel.com/content/www/us/en/docs/programmable/683126/21-2/functional-description-of-the-emac.html
