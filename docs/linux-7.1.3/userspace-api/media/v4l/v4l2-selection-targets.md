######## 选择目标


选择目标的确切含义可能取决于它们被用于哪一种接口



   \small



    :header-rows:  1
    :stub-columns: 0

    - - 目标名称
      - id
      - 定义
      - V4L2 有效
      - V4L2 subdev 有效
    - - `V4L2_SEL_TGT_CROP`
      - 0x0000
      - 裁剪矩形。定义被裁剪的区域      - Yes
      - Yes
    - - `V4L2_SEL_TGT_CROP_DEFAULT`
      - 0x0001
      - 建议的裁剪矩形，覆盖“整幅画面”        仅包含有效像素，排除其他非活动像素（如黑像素）      - Yes
      - Yes
    - - `V4L2_SEL_TGT_CROP_BOUNDS`
      - 0x0002
      - 裁剪矩形的边界。所有有效裁剪矩形均位于裁剪边界矩形内部      - Yes
      - Yes
    - - `V4L2_SEL_TGT_NATIVE_SIZE`
      - 0x0003
      - 设备的原始尺寸，例如传感器的像素阵列        `left` `top` 字段对于此目标为零      - Yes
      - Yes
    - - `V4L2_SEL_TGT_COMPOSE`
      - 0x0100
      - 合成矩形。用于配置缩放与合成      - Yes
      - Yes
    - - `V4L2_SEL_TGT_COMPOSE_DEFAULT`
      - 0x0101
      - 建议的合成矩形，覆盖“整幅画面”      - Yes
      - No
    - - `V4L2_SEL_TGT_COMPOSE_BOUNDS`
      - 0x0102
      - 合成矩形的边界。所有有效合成矩形均位于合成边界矩形内部      - Yes
      - Yes
    - - `V4L2_SEL_TGT_COMPOSE_PADDED`
      - 0x0103
      - 活动区域以及由硬件插入或修改的所有填充像素      - Yes
      - No


   \normalsize
