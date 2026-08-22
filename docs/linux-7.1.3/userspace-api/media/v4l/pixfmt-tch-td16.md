


######## V4L2_TCH_FMT_DELTA_TD16 ('TD16')


**man V4L2_TCH_FMT_DELTA_TD16(2)**

16 位有符号小端触摸差
## 描述


该格式表示来自触摸控制器的差值数据
差值范围可-32768 32767。通常这些值会在小范围内变化，取决于传感器
是否被触摸。若某个触摸屏节点存在故障或线路未连接，则可能出现完整值
**字节序*
每个单元为一个字节
    :header-rows:  0
    :stub-columns: 0
    :widths:       2 1 1 1 1 1 1 1 1

    - - start + 0:
      - D'\ `00low`
      - D'\ `00high`
      - D'\ `01low`
      - D'\ `01high`
      - D'\ `02low`
      - D'\ `02high`
      - D'\ `03low`
      - D'\ `03high`
    - - start + 8:
      - D'\ `10low`
      - D'\ `10high`
      - D'\ `11low`
      - D'\ `11high`
      - D'\ `12low`
      - D'\ `12high`
      - D'\ `13low`
      - D'\ `13high`
    - - start + 16:
      - D'\ `20low`
      - D'\ `20high`
      - D'\ `21low`
      - D'\ `21high`
      - D'\ `22low`
      - D'\ `22high`
      - D'\ `23low`
      - D'\ `23high`
    - - start + 24:
      - D'\ `30low`
      - D'\ `30high`
      - D'\ `31low`
      - D'\ `31high`
      - D'\ `32low`
      - D'\ `32high`
      - D'\ `33low`
      - D'\ `33high`
