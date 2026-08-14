
## Synopsys DesignWare PCIe 娴侀噺鐢熸垚鍣紙浜︾О xData锛夐┍鍔?

鏀寔鐨勮姱鐗囷細
Synopsys DesignWare PCIe 鍘熷瀷鏂规

鏁版嵁鎵嬪唽锛?涓嶅叕寮€鎻愪緵

浣滆€咃細
Gustavo Pimentel <gustavo.pimentel@synopsys.com>

### 鎻忚堪


璇ラ┍鍔ㄥ簲浣滀负涓绘満渚э紙Root Complex锛夐┍鍔ㄤ互鍙婂寘鍚 IP 鐨?Synopsys DesignWare
鍘熷瀷浣跨敤銆?
dw-xdata-pcie 椹卞姩鍙敤浜庡惎鐢?绂佺敤浠讳竴鏂瑰悜锛堜簰鏂ワ級鐨?PCIe 娴侀噺鐢熸垚鍣紝骞跺厑璁?杩涜 PCIe 閾捐矾鎬ц兘鍒嗘瀽銆?
涓庤椹卞姩鐨勪氦浜掗€氳繃妯″潡鍙傛暟瀹屾垚锛屽苟鍙湪杩愯鏃舵洿鏀广€傞┍鍔ㄥ皢璇锋眰鐨勫懡浠ょ姸鎬?淇℃伅杈撳嚭鍒?`/var/log/kern.log` 鎴?dmesg銆?
### 绀轰緥


#### 鍐?TLPs 娴侀噺鐢熸垚 - Root Complex 鍒?Endpoint 鏂瑰悜


```

 # echo 1 > /sys/class/misc/dw-xdata-pcie.0/write


```
```

 # cat /sys/class/misc/dw-xdata-pcie.0/write
 204


```
```

 # echo 0 > /sys/class/misc/dw-xdata-pcie.0/write


```
#### 璇?TLPs 娴侀噺鐢熸垚 - Endpoint 鍒?Root Complex 鏂瑰悜


```

 # echo 1 > /sys/class/misc/dw-xdata-pcie.0/read


```
```

 # cat /sys/class/misc/dw-xdata-pcie.0/read
 199


```
```

 # echo 0 > /sys/class/misc/dw-xdata-pcie.0/read


```
