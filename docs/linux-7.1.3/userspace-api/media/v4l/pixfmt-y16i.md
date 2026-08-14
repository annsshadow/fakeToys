
######## V4L2_PIX_FMT_Y16I ('Y16I')


交错灰度图像，例如来自立体相机对


## 描述


这是每个像素深度为 16 位的灰度图像，但来自 2 个源的像素交错且未打包。每个像素以小端顺序存储在一个 16 位字中。第一个像素来自左侧源。

**像素未打包表示。**
左/右像素为 16 位未打包——每个交错像素 16 位。

    :header-rows:  0
    :stub-columns: 0

    - - Y'\ `0L[7:0]`
      - Y'\ `0L[15:8]`
      - Y'\ `0R[7:0]`
      - Y'\ `0R[15:8]`

**字节序。**
每个单元为一个字节。

    :header-rows:  0
    :stub-columns: 0

    - - start + 0:
      - Y'\ `00Llow`
      - Y'\ `00Lhigh`
      - Y'\ `00Rlow`
      - Y'\ `00Rhigh`
      - Y'\ `01Llow`
      - Y'\ `01Lhigh`
      - Y'\ `01Rlow`
      - Y'\ `01Rhigh`
    - - start + 8:
      - Y'\ `10Llow`
      - Y'\ `10Lhigh`
      - Y'\ `10Rlow`
      - Y'\ `10Rhigh`
      - Y'\ `11Llow`
      - Y'\ `11Lhigh`
      - Y'\ `11Rlow`
      - Y'\ `11Rhigh`
    - - start + 16:
      - Y'\ `20Llow`
      - Y'\ `20Lhigh`
      - Y'\ `20Rlow`
      - Y'\ `20Rhigh`
      - Y'\ `21Llow`
      - Y'\ `21Lhigh`
      - Y'\ `21Rlow`
      - Y'\ `21Rhigh`
    - - start + 24:
      - Y'\ `30Llow`
      - Y'\ `30Lhigh`
      - Y'\ `30Rlow`
      - Y'\ `30Rhigh`
      - Y'\ `31Llow`
      - Y'\ `31Lhigh`
      - Y'\ `31Rlow`
      - Y'\ `31Rhigh`
