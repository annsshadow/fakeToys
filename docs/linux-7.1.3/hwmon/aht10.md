
## 内核驱动 aht10


支持的芯片：

  - Aosong AHT10/AHT20

    前缀: 'aht10'

    扫描地址: None

    数据手册(AHT10)：

      中文: http://www.aosong.com/userfiles/files/media/AHT10%E4%BA%A7%E5%93%81%E6%89%8B%E5%86%8C%20A3%2020201210.pdf
      英文: https://server4.eca.ir/eshop/AHT10/Aosong_AHT10_en_draft_0c.pdf

    数据手册(AHT20)：

      英文: http://www.aosong.com/userfiles/files/media/Data%20Sheet%20AHT20.pdf

  - Aosong DHT20

    前缀: 'dht20'

    扫描地址: None

    数据手册: https://www.digikey.co.nz/en/htmldatasheets/production/9184855/0/0/1/101020932

Author: Johannes Cornelis Draaijer <jcdra1@gmail.com>


### 描述


AHT10/AHT20 是一款温湿度传感器

该 i2c 设备的地址只能为 0x38

### 特殊特性


AHT20、DHT20 具有额外的 CRC8 支持，作为传感器
数据值的最后一个字节发送。

### 使用说明


该驱动不会主动探测 AHT10/AHT20 设备，因为没有可靠
的方法判断一个 i2c 芯片是否为 AHT10/AHT20。该设备必须
使用地址 0x38 显式实例化。详见
Documentation/i2c/instantiating-devices.rst。

### Sysfs 条目


=============== ============================================
temp1_input     测量的温度，单位为毫摄氏度
humidity1_input 测量的湿度，单位为 %H
update_interval 轮询传感器的最小间隔，
                单位为毫秒。可写。必须
                至少为 2000。
=============== ============================================
