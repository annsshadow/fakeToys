## ST VGXY61 鐩告満浼犳劅鍣ㄩ┍鍔。


ST VGXY61 驱动实现了以下控制：

### ``V4L2_CID_HDR_SENSOR_MODE``


    更改传感器的 HDR 模式。HDR 图像通过合并使用两个不同曝光周期对同一场景的两次捕获得到

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 4

    - - HDR linearize
      - 只要长曝光帧未饱和，合并器就输出长曝光帧
    - - HDR subtraction
      - 这涉及从长曝光帧中减去短曝光帧
    - - No HDR
      - 此模式用于标准动态范围（SDR）曝光
