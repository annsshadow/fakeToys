


######## V4L2_PIX_FMT_Y8I ('Y8I ')


交错灰度图像，例如来自立体对


## 描述


这是一幅每像素深度为 8 位的灰度图像，但来自 2 个源的像素是交错的。每个像素
存储在 16 位字中。例如 R200 RealSense 相机将来自左传感器的像素存储在低 8 位，
将来自右传感器的像素存储在高 8 位。

**字节序。**
每个单元为一个字节。




    :header-rows:  0
    :stub-columns: 0

    - - start + 0:
      - Y'\ `00left`
      - Y'\ `00right`
      - Y'\ `01left`
      - Y'\ `01right`
      - Y'\ `02left`
      - Y'\ `02right`
      - Y'\ `03left`
      - Y'\ `03right`
    - - start + 8:
      - Y'\ `10left`
      - Y'\ `10right`
      - Y'\ `11left`
      - Y'\ `11right`
      - Y'\ `12left`
      - Y'\ `12right`
      - Y'\ `13left`
      - Y'\ `13right`
    - - start + 16:
      - Y'\ `20left`
      - Y'\ `20right`
      - Y'\ `21left`
      - Y'\ `21right`
      - Y'\ `22left`
      - Y'\ `22right`
      - Y'\ `23left`
      - Y'\ `23right`
    - - start + 24:
      - Y'\ `30left`
      - Y'\ `30right`
      - Y'\ `31left`
      - Y'\ `31right`
      - Y'\ `32left`
      - Y'\ `32right`
      - Y'\ `33left`
      - Y'\ `33right`
