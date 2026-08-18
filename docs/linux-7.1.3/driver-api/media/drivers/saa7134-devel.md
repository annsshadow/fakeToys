
## saa7134 椹卞姩

鏈枃闈㈠悜椹卞姩寮€鍙戣€咃紝璁板綍 saa7134 澶氬獟浣撹棰戦噰闆嗚姱鐗囬┍鍔ㄧ殑瀹炵幇缁嗚妭锛屾兜鐩栦笉鍚岃姱鐗囧瀷鍙凤紙saa7130/7133/7134/7135锛夌殑宸紓銆佹櫠鎸厤缃紝浠ュ強 LifeView 绛夋澘鍗＄殑 GPIO 鎺ョ嚎璇存槑銆?



Author Gerd Hoffmann


### 鑺墖鍨嬪彿宸紓锛?


鐢佃鍗″彲浠ヤ娇鐢ㄤ互涓嬩袱绉嶆櫠鎸紙xtal锛変箣涓€锛?

- 32.11 MHz -> .audio_clock=0x187de7
- 24.576MHz -> .audio_clock=0x200000 (xtal * .audio_clock = 51539600)

鍏充簬 30/34/35 鐨勪竴浜涚粏鑺傦細

- saa7130 - 浣庝环鑺墖锛屾病鏈夐潤闊冲姛鑳斤紝鍥犳鎵€鏈夎繖浜?
  鍗＄殑 tuner 缁撴瀯浣撲腑搴斿畾涔?.mute 瀛楁銆?

- saa7134 - 甯歌鑺墖

- saa7133/35 - saa7135 鍙兘鏄竴涓競鍦哄喅绛栵紝鍥犱负鎵€鏈夎繖浜?
  鑺墖鍦?pci 涓婃爣璇嗚嚜韬负 33銆?

### LifeView GPIO


鏈妭鐢?Peter Missel <peter.missel@onlinehome.de> 鎾板啓

- LifeView FlyTV Platinum FM (LR214WF)

    - GP27    MDT2005 PB4 pin 10
    - GP26    MDT2005 PB3 pin 9
    - GP25    MDT2005 PB2 pin 8
    - GP23    MDT2005 PB1 pin 7
    - GP22    MDT2005 PB0 pin 6
    - GP21    MDT2005 PB5 pin 11
    - GP20    MDT2005 PB6 pin 12
    - GP19    MDT2005 PB7 pin 13
    - nc      MDT2005 PA3 pin 2
    - Remote  MDT2005 PA2 pin 1
    - GP18    MDT2005 PA1 pin 18
    - nc      MDT2005 PA0 pin 17 strap low
    - GP17    Strap "GP7"=High
    - GP16    Strap "GP6"=High

 - 0=Radio 1=TV
 - 椹卞姩 SA630D ENCH1 鍜?HEF4052 A1 寮曡剼锛岄€氳繃
	  SIF 杈撳叆瀹炵幇 FM 鏀堕煶

    - GP15    nc
    - GP14    nc
    - GP13    nc
    - GP12    Strap "GP5" = High
    - GP11    Strap "GP4" = High
    - GP10    Strap "GP3" = High
    - GP09    Strap "GP2" = Low
    - GP08    Strap "GP1" = Low
    - GP07.00 nc

### 鑷磋阿


andrew.stevens@philips.com + werner.leeb@philips.com 鎻愪緵浜?
saa7134 纭欢瑙勬牸鍜屾牱渚嬫澘銆?

