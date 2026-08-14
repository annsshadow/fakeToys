## drm/meson AmLogic Meson 瑙嗛澶勭悊鍗曞厓

鏈枃妗ｆ弿杩?AmLogic Meson 骞冲彴鐨勬樉绀?瑙嗛澶勭悊鍗曞厓锛圴PU锛夐┍鍔ㄦ灦鏋勶紝娑电洊瑙嗛杈撳叆銆佸悗澶勭悊銆佽棰戠紪鐮併€佽棰戞椂閽熶笌 HDMI 杈撳嚭绛夌粍浠讹紝渚涜骞冲彴鍥惧舰涓庢樉绀洪┍鍔ㄥ紑鍙戣€呭弬鑰冦€?


   :doc: Video Processing Unit

## 瑙嗛澶勭悊鍗曞厓


Amlogic Meson 鏄剧ず鎺у埗鍣ㄧ敱灏嗗湪涓嬫枃妗ｅ寲鐨勮嫢骞茬粍浠舵瀯鎴愶細


  DMC|---------------VPU (Video Processing Unit)----------------|------HHI------|
     | vd1   _______     _____________    _________________     |               |
  D  |-------|      |----|            |   |                |    |   HDMI PLL    |
  D  | vd2   | VIU  |    | Video Post |   | Video Encoders |<---|-----VCLK      |
  R  |-------|      |----| Processing |   |                |    |               |
     | osd2  |      |    |            |---| Enci ----------|----|-----VDAC------|
  R  |-------| CSC  |----| Scalers    |   | Encp ----------|----|----HDMI-TX----|
  A  | osd1  |      |    | Blenders   |   | Encl ----------|----|---------------|
  M  |-------|______|----|____________|   |________________|    |               |
  ___|__________________________________________________________|_______________|

## 瑙嗛杈撳叆鍗曞厓


   :doc: Video Input Unit

## 瑙嗛鍚庡鐞?


   :doc: Video Post Processing

## 瑙嗛缂栫爜鍣?


   :doc: Video Encoder

## 瑙嗛鏃堕挓


   :doc: Video Clocks

## HDMI 瑙嗛杈撳嚭


   :doc: HDMI Output
