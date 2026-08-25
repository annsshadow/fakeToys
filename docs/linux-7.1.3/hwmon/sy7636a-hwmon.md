## 内核驱动 sy7636a-hwmon


支持的芯片：

 - Silergy SY7636A PMIC


### 描述


该驱动为 Silergy SY7636A PMIC 添加硬件温度读取支持

支持的传感器如下

  - 温度
      - 外部 NTC 的温度，单位为毫摄氏

### sysfs 接口


temp1_input
 - 外部 NTC 的温度（毫摄氏度
