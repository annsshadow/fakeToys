## 并行端口 LCD/键盘面板支持


有些 LCD 允许您定义最8 个字符，映射ASCII
字符 0 7。定义新字符的转义码
'\e[LG' 后面的一位数字，代表字符
数字，最8 对以分号结尾的十六进制数
(';')。每对数字代表一行，每个数字 1 
LSB 位于右侧的发光像素。行号从
字符的顶部到底部。在 5x7 矩阵上，只有较低5 
7 个字节的位用于每个字符。如果字符串
不完整，只有完整的行才会被重新定义。这里有一
```

  printf "\e[LG0010101050D1F0C04;"  => 0 = [enter]
  printf "\e[LG1040E1F0000000000;"  => 1 = [up]
  printf "\e[LG2000000001F0E0400;"  => 2 = [down]
  printf "\e[LG3040E1F001F0E0400;"  => 3 = [up-down]
  printf "\e[LG40002060E1E0E0602;"  => 4 = [left]
  printf "\e[LG500080C0E0F0E0C08;"  => 5 = [right]
  printf "\e[LG60016051516141400;"  => 6 = "IP"

  printf "\e[LG00103071F1F070301;"  => big speaker
  printf "\e[LG00002061E1E060200;"  => small speaker

```
威利
