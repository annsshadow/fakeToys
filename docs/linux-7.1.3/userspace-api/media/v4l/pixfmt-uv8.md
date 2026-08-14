######## V4L2_PIX_FMT_UV8 ('UV8')

本页说明 V4L2 像素格式 V4L2_PIX_FMT_UV8（四字符码 'UV8'），该格式仅包含交错的 CbCr（UV）色度平面、不含 Y 亮度平面，常用于只传输色度信息的场景。文中给出其内存布局示例。




UV 平面交错


## 描述


该格式没有 Y 平面，只有 CbCr 平面，即（UV 交错）。

**字节序。**
每个单元为一个字节。





    :header-rows:  0
    :stub-columns: 0

    - - start + 0:
      - Cb\ `00`
      - Cr\ `00`
      - Cb\ `01`
      - Cr\ `01`
    - - start + 4:
      - Cb\ `10`
      - Cr\ `10`
      - Cb\ `11`
      - Cr\ `11`
    - - start + 8:
      - Cb\ `20`
      - Cr\ `20`
      - Cb\ `21`
      - Cr\ `21`
    - - start + 12:
      - Cb\ `30`
      - Cr\ `30`
      - Cb\ `31`
      - Cr\ `31`
