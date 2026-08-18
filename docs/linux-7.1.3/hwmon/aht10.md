
## 鍐呮牳椹卞姩 aht10


鏀寔鐨勮姱鐗囷細

  - Aosong AHT10/AHT20

    鍓嶇紑: 'aht10'

    鎵弿鍦板潃: None

    鏁版嵁鎵嬪唽(AHT10)锛?

      涓枃: http://www.aosong.com/userfiles/files/media/AHT10%E4%BA%A7%E5%93%81%E6%89%8B%E5%86%8C%20A3%2020201210.pdf
      鑻辨枃: https://server4.eca.ir/eshop/AHT10/Aosong_AHT10_en_draft_0c.pdf

    鏁版嵁鎵嬪唽(AHT20)锛?

      鑻辨枃: http://www.aosong.com/userfiles/files/media/Data%20Sheet%20AHT20.pdf

  - Aosong DHT20

    鍓嶇紑: 'dht20'

    鎵弿鍦板潃: None

    鏁版嵁鎵嬪唽: https://www.digikey.co.nz/en/htmldatasheets/production/9184855/0/0/1/101020932

Author: Johannes Cornelis Draaijer <jcdra1@gmail.com>


### 鎻忚堪


AHT10/AHT20 鏄竴娆炬俯婀垮害浼犳劅鍣?

璇?i2c 璁惧鐨勫湴鍧€鍙兘涓?0x38

### 鐗规畩鐗规€?


AHT20銆丏HT20 鍏锋湁棰濆鐨?CRC8 鏀寔锛屼綔涓轰紶鎰熷櫒
鏁版嵁鍊肩殑鏈€鍚庝竴涓瓧鑺傚彂閫併€?

### 浣跨敤璇存槑


璇ラ┍鍔ㄤ笉浼氫富鍔ㄦ帰娴?AHT10/AHT20 璁惧锛屽洜涓烘病鏈夊彲闈?
鐨勬柟娉曞垽鏂竴涓?i2c 鑺墖鏄惁涓?AHT10/AHT20銆傝璁惧蹇呴』
浣跨敤鍦板潃 0x38 鏄惧紡瀹炰緥鍖栥€傝瑙?
Documentation/i2c/instantiating-devices.rst銆?

### Sysfs 鏉＄洰


=============== ============================================
temp1_input     娴嬮噺鐨勬俯搴︼紝鍗曚綅涓烘鎽勬皬搴?
humidity1_input 娴嬮噺鐨勬箍搴︼紝鍗曚綅涓?%H
update_interval 杞浼犳劅鍣ㄧ殑鏈€灏忛棿闅旓紝
                鍗曚綅涓烘绉掋€傚彲鍐欍€傚繀椤?
                鑷冲皯涓?2000銆?
=============== ============================================
