######## V4L2_PIX_FMT_Z16 ('Z16 ')


每个像素包含距离值的 16 位深度数

## Description


这是一16 位格式，表示深度数据。每个像素是到图像坐标中对应点的距离。距离单位可能不同，需要单独与设备进行协商。每个像素以小端（little endian）字节序存储在一16 位字中
**字节序*
每个单元为一个字节
   :header-rows:  0
   :stub-columns: 0

   - - start + 0:
     - Z\ `00low`
     - Z\ `00high`
     - Z\ `01low`
     - Z\ `01high`
     - Z\ `02low`
     - Z\ `02high`
     - Z\ `03low`
     - Z\ `03high`
   - - start + 8:
     - Z\ `10low`
     - Z\ `10high`
     - Z\ `11low`
     - Z\ `11high`
     - Z\ `12low`
     - Z\ `12high`
     - Z\ `13low`
     - Z\ `13high`
   - - start + 16:
     - Z\ `20low`
     - Z\ `20high`
     - Z\ `21low`
     - Z\ `21high`
     - Z\ `22low`
     - Z\ `22high`
     - Z\ `23low`
     - Z\ `23high`
   - - start + 24:
     - Z\ `30low`
     - Z\ `30high`
     - Z\ `31low`
     - Z\ `31high`
     - Z\ `32low`
     - Z\ `32high`
     - Z\ `33low`
     - Z\ `33high`
