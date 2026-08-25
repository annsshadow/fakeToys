## drm/meson AmLogic Meson 视频处理单元

本文档描AmLogic Meson 平台的显视频处理单元（VPU）驱动架构，涵盖视频输入、后处理、视频编码、视频时钟与 HDMI 输出等组件，供该平台图形与显示驱动开发者参考


   :doc: Video Processing Unit

## 视频处理单元


Amlogic Meson 显示控制器由将在下文档化的若干组件构成：


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

## 视频输入单元


   :doc: Video Input Unit

## 视频后处


   :doc: Video Post Processing

## 视频编码


   :doc: Video Encoder

## 视频时钟


   :doc: Video Clocks

## HDMI 视频输出


   :doc: HDMI Output
