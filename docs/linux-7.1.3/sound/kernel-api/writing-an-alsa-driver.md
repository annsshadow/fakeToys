## Writing an ALSA Driver


:Author: Takashi Iwai <tiwai@suse.de>

## Preface


鏈枃妗ｆ弿杩颁簡濡備綍缂栧啓 `ALSA锛圓dvanced Linux Sound
Architecture锛孡inux 楂樼骇澹伴煶鏋舵瀯锛?http://www.alsa-project.org/>`__ 椹卞姩銆傛湰鏂囨。
涓昏鍏虫敞 PCI 澹板崱銆傚浜庡叾浠栬澶囩被鍨嬶紝鍏?API 涔熷彲鑳芥湁鎵€涓嶅悓銆備笉杩囷紝鑷冲皯 ALSA
鍐呮牳 API 鏄竴鑷寸殑锛屽洜姝ゅ畠瀵圭紪鍐欒繖浜涢┍鍔ㄤ粛浼氭湁涓€浜涘府鍔┿€?

鏈枃妗ｉ潰鍚戝凡缁忓叿澶囪冻澶?C 璇█鎶€鑳姐€佸苟鎷ユ湁鍩烘湰 Linux 鍐呮牳缂栫▼鐭ヨ瘑鐨勪汉缇ゃ€?
鏈枃妗ｄ笉瑙ｉ噴 Linux 鍐呮牳缂栫爜鐨勯€氱敤涓婚锛屼篃涓嶆兜鐩栧簳灞傞┍鍔ㄧ殑瀹炵幇缁嗚妭銆傚畠鍙弿杩?
鍦?ALSA 涓婄紪鍐?PCI 澹伴煶椹卞姩鐨勬爣鍑嗘柟娉曘€?

## File Tree Structure


### General


```
            sound
                    /core
                            /oss
                            /seq
                                    /oss
                    /include
                    /drivers
                            /mpu401
                            /opl3
                    /i2c
                    /synth
                            /emux
                    /pci
                            /(cards)
                    /isa
                            /(cards)
                    /arm
                    /ppc
                    /sparc
                    /usb
                    /pcmcia /(cards)
                    /soc
                    /oss
```

### core 鐩綍


璇ョ洰褰曞寘鍚簡浣滀负 ALSA 椹卞姩鏍稿績鐨勪腑闂村眰銆傝鐩綍涓瓨鏀剧潃鍘熺敓鐨?ALSA 妯″潡銆傚叾
瀛愮洰褰曚腑鍖呭惈涓嶅悓鐨勬ā鍧楋紝骞朵笖渚濊禆浜庡唴鏍搁厤缃€?

#### core/oss


OSS PCM 涓庢贩闊冲櫒锛坢ixer锛夋ā鎷熸ā鍧楃殑浠ｇ爜瀛樻斁鍦ㄨ鐩綍涓€侽SS rawmidi 妯℃嫙鍥犱负
鐩稿綋灏忥紝琚寘鍚湪 ALSA rawmidi 浠ｇ爜涓€傞煶搴忓櫒锛坰equencer锛変唬鐮佸瓨鏀惧湪
`core/seq/oss` 鐩綍涓紙瑙?`涓嬫柟 <core/seq/oss_>`__锛夈€?

#### core/seq


璇ョ洰褰曞強鍏跺瓙鐩綍鐢ㄤ簬 ALSA 闊冲簭鍣ㄣ€傝鐩綍鍖呭惈浜嗛煶搴忓櫒鏍稿績浠ュ強涓昏鐨勯煶搴忓櫒妯″潡锛?
渚嬪 snd-seq-midi銆乻nd-seq-virmidi 绛夈€傚彧鏈夊綋鍐呮牳閰嶇疆涓缃簡
`CONFIG_SND_SEQUENCER` 鏃讹紝瀹冧滑鎵嶄細琚紪璇戙€?

#### core/seq/oss


璇ョ洰褰曞寘鍚簡 OSS 闊冲簭鍣ㄦā鎷熶唬鐮併€?

### include 鐩綍


杩欓噷鏄?ALSA 椹卞姩鍏叡澶存枃浠剁殑鎵€鍦ㄤ綅缃紝杩欎簺澶存枃浠跺皢琚鍑哄埌鐢ㄦ埛绌洪棿锛屾垨琚笉鍚?
鐩綍涓殑澶氫釜鏂囦欢鍖呭惈銆傚熀鏈笂锛岀鏈夊ご鏂囦欢涓嶅簲鏀惧湪璇ョ洰褰曚腑锛屼絾鐢变簬鍘嗗彶鍘熷洜锛?
浣犱粛鍙兘鍦ㄩ偅閲屽彂鐜颁竴浜涙枃浠?:)

### drivers 鐩綍


璇ョ洰褰曞寘鍚簡鍦ㄤ笉鍚屾灦鏋勪笂銆佷笉鍚岄┍鍔ㄤ箣闂村叡浜殑浠ｇ爜銆傚洜姝ゅ畠浠簲褰撴槸鏋舵瀯鏃犲叧鐨勩€?
渚嬪锛岃櫄鎷?PCM 椹卞姩鍜屼覆琛?MIDI 椹卞姩灏变綅浜庢鐩綍涓€傚湪鍏跺瓙鐩綍涓紝鏀剧潃涓庢€荤嚎
鍜?CPU 鏋舵瀯鏃犲叧鐨勭粍浠朵唬鐮併€?

#### drivers/mpu401


MPU401 涓?MPU401-UART 妯″潡瀛樻斁鍦ㄦ澶勩€?

#### drivers/opl3 涓?opl4


OPL3 涓?OPL4 FM 鍚堟垚锛團M-synth锛夌浉鍏崇殑涓滆タ鍙互鍦ㄨ繖閲屾壘鍒般€?

### i2c 鐩綍


璇ョ洰褰曞寘鍚簡 ALSA 鐨?i2c 缁勪欢銆?

铏界劧 Linux 涓婃湁涓€涓爣鍑嗙殑 i2c 灞傦紝浣?ALSA 瀵规煇浜涘０鍗℃嫢鏈夎嚜宸辩殑 i2c 浠ｇ爜锛?
鍥犱负澹板崱鍙渶瑕佺畝鍗曠殑鎿嶄綔锛岃€屾爣鍑?i2c API 瀵逛簬姝ょ被鐢ㄩ€旀潵璇磋繃浜庡鏉傘€?

### synth 鐩綍


璇ョ洰褰曞寘鍚簡鍚堟垚鍣紙synth锛変腑闂村眰妯″潡銆?

鍒扮洰鍓嶄负姝紝鍦?`synth/emux` 瀛愮洰褰曚笅鍙湁 Emu8000/Emu10k1 鍚堟垚鍣ㄩ┍鍔ㄣ€?

### pci 鐩綍


璇ョ洰褰曞強鍏跺瓙鐩綍淇濆瓨鐫€ PCI 澹板崱鐨勯《灞傚０鍗℃ā鍧楋紝浠ュ強涓?PCI 鎬荤嚎鐩稿叧鐨勪唬鐮併€?

鐢卞崟涓枃浠剁紪璇戣€屾潵鐨勯┍鍔ㄧ洿鎺ュ瓨鏀惧湪 pci 鐩綍涓紝鑰岀敱澶氫釜婧愭枃浠剁粍鎴愮殑椹卞姩鍒?
瀛樻斁鍦ㄥ畠浠悇鑷殑瀛愮洰褰曚腑锛堜緥濡?emu10k1銆乮ce1712锛夈€?

### isa 鐩綍


璇ョ洰褰曞強鍏跺瓙鐩綍淇濆瓨鐫€ ISA 澹板崱鐨勯《灞傚０鍗℃ā鍧椼€?

### arm銆乸pc 涓?sparc 鐩綍


瀹冧滑鐢ㄤ簬鐗瑰畾浜庝笂杩版煇涓€绉嶆灦鏋勭殑椤跺眰澹板崱妯″潡銆?

### usb 鐩綍


璇ョ洰褰曞寘鍚簡 USB 闊抽椹卞姩銆俇SB MIDI 椹卞姩宸茬粡琚泦鎴愯繘 usb-audio 椹卞姩涓€?

### pcmcia 鐩綍


PCMCIA锛屽挨鍏舵槸 PCCard 椹卞姩灏嗘斁鍦ㄨ繖閲屻€侰ardBus 椹卞姩灏嗕綅浜?pci 鐩綍涓紝
鍥犱负瀹冧滑鐨?API 涓庢爣鍑?PCI 鍗＄浉鍚屻€?

### soc 鐩綍


璇ョ洰褰曞寘鍚簡 ASoC锛圓LSA System on Chip锛孉LSA 鐗囦笂绯荤粺锛夊眰鐨勪唬鐮侊紝鍖呮嫭
ASoC 鏍稿績銆佺紪瑙ｇ爜鍣紙codec锛変互鍙婃満鍣紙machine锛夐┍鍔ㄣ€?

### oss 鐩綍


璇ョ洰褰曞寘鍚簡 OSS/Lite 浠ｇ爜銆傚湪鎾板啓鏈枃妗ｆ椂锛岄櫎浜?m68k 涓婄殑 dmasound 涔嬪锛?
鎵€鏈変唬鐮侀兘宸茶绉婚櫎銆?


## PCI 椹卞姩鐨勫熀鏈祦绋?


### 姒傝堪


PCI 澹板崱鐨勬渶灏忔祦绋嬪涓嬶細

- 瀹氫箟 PCI ID 琛紙瑙?`PCI Entries`_ 涓€鑺傦級銆?

- 鍒涘缓 `probe` 鍥炶皟鍑芥暟銆?

- 鍒涘缓 `remove` 鍥炶皟鍑芥暟銆?

- 鍒涘缓涓€涓?struct pci_driver 缁撴瀯浣擄紝
   鍏朵腑鍖呭惈涓婅堪涓変釜鎸囬拡銆?

- 鍒涘缓涓€涓?`init` 鍑芥暟锛屼粎璋冪敤
   `pci_register_driver()` 鏉ユ敞鍐屼笂闈㈠畾涔夌殑 pci_driver
   琛ㄣ€?

- 鍒涘缓涓€涓?`exit` 鍑芥暟鏉ヨ皟鐢?
   `pci_unregister_driver()` 鍑芥暟銆?

### 瀹屾暣浠ｇ爜绀轰緥


涓嬮潰鐨勪唬鐮佷緥瀛愬睍绀轰簡涓婅堪娴佺▼銆傛煇浜涢儴鍒嗙洰鍓嶅皻鏈疄鐜帮紝浣嗕細鍦ㄥ悗缁皬鑺備腑琛ュ叏銆?
`snd_mychip_probe()` 鍑芥暟娉ㄩ噴琛屼腑鐨勬暟瀛楀搴斾簬涓嬩竴鑺備腑瑙ｉ噴鐨勮缁嗚鏄庛€?

```

      #include <linux/init.h>
      #include <linux/pci.h>
      #include <linux/slab.h>
      #include <sound/core.h>
      #include <sound/initval.h>

      /* module parameters (see "Module Parameters") */
      /* SNDRV_CARDS: maximum number of cards supported by this module */
      static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
      static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
      static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

      /* definition of the chip-specific record */
      struct mychip {
              struct snd_card *card;
              /* the rest of the implementation will be in section
               * "PCI Resource Management"
               */
      };

      /* chip-specific destructor
       * (see "PCI Resource Management")
       */
      static int snd_mychip_free(struct mychip *chip)
      {
              .... /* will be implemented later... */
      }

      /* component-destructor
       * (see "Management of Cards and Components")
       */
      static int snd_mychip_dev_free(struct snd_device *device)
      {
              return snd_mychip_free(device->device_data);
      }

      /* chip-specific constructor
       * (see "Management of Cards and Components")
       */
      static int snd_mychip_create(struct snd_card *card,
                                   struct pci_dev *pci,
                                   struct mychip **rchip)
      {
              struct mychip *chip;
              int err;
              static const struct snd_device_ops ops = {
                     .dev_free = snd_mychip_dev_free,
              };

              *rchip = NULL;

              /* check PCI availability here
               * (see "PCI Resource Management")
               */
              ....

              /* allocate a chip-specific data with zero filled */
              chip = kzalloc(sizeof(*chip), GFP_KERNEL);
              if (chip == NULL)
                      return -ENOMEM;

              chip->card = card;

              /* rest of initialization here; will be implemented
               * later, see "PCI Resource Management"
               */
              ....

              err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip, &ops);
              if (err < 0) {
                      snd_mychip_free(chip);
                      return err;
              }

              *rchip = chip;
              return 0;
      }

      /* constructor -- see "Driver Constructor" sub-section */
      static int snd_mychip_probe(struct pci_dev *pci,
                                  const struct pci_device_id *pci_id)
      {
              static int dev;
              struct snd_card *card;
              struct mychip *chip;
              int err;

              /* (1) */
              if (dev >= SNDRV_CARDS)
                      return -ENODEV;
              if (!enable[dev]) {
                      dev++;
                      return -ENOENT;
              }

              /* (2) */
              err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                                 0, &card);
              if (err < 0)
                      return err;

              /* (3) */
              err = snd_mychip_create(card, pci, &chip);
              if (err < 0)
                      goto error;

              /* (4) */
              strcpy(card->driver, "My Chip");
              strcpy(card->shortname, "My Own Chip 123");
              sprintf(card->longname, "%s at 0x%lx irq %i",
                      card->shortname, chip->port, chip->irq);

              /* (5) */
              .... /* implemented later */

              /* (6) */
              err = snd_card_register(card);
              if (err < 0)
                      goto error;

              /* (7) */
              pci_set_drvdata(pci, card);
              dev++;
              return 0;

      error:
              snd_card_free(card);
              return err;
      }

      /* destructor -- see the "Destructor" sub-section */
      static void snd_mychip_remove(struct pci_dev *pci)
      {
              snd_card_free(pci_get_drvdata(pci));
      }



```

### 椹卞姩鏋勯€犲嚱鏁?


PCI 椹卞姩鐪熸鐨勬瀯閫犲嚱鏁版槸 `probe` 鍥炶皟鍑芥暟銆俙probe` 鍥炶皟鍑芥暟浠ュ強浠?
`probe` 鍥炶皟鍑芥暟璋冪敤鐨勫叾浠栫粍浠舵瀯閫犲嚱鏁颁笉鑳戒娇鐢?`__init` 鍓嶇紑锛?
鍥犱负浠讳綍 PCI 璁惧閮藉彲鑳芥槸鐑彃鎷旓紙hotplug锛夎澶囥€?

鍦?`probe` 鍥炶皟鍑芥暟涓紝閫氬父浼氫娇鐢ㄥ涓嬫柟妗堛€?

#### 1) 妫€鏌ュ苟閫掑璁惧绱㈠紩銆?


```

  static int dev;
  ....
  if (dev >= SNDRV_CARDS)
          return -ENODEV;
  if (!enable[dev]) {
          dev++;
          return -ENOENT;
  }


```

鍏朵腑 `enable[dev]` 鏄ā鍧楅€夐」銆?

姣忔璋冪敤 `probe` 鍥炶皟鍑芥暟鏃讹紝妫€鏌ヨ澶囩殑鍙敤鎬с€傚鏋滀笉鍙敤锛屽氨绠€鍗曞湴閫掑
璁惧绱㈠紩骞惰繑鍥炪€俤ev 绋嶅悗杩樹細琚€掑锛坄姝ラ 7
<7) Set the PCI driver data and return zero._>`__锛夈€?

#### 2) 鍒涘缓澹板崱瀹炰緥


```

  struct snd_card *card;
  int err;
  ....
  err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                     0, &card);


```

鐩稿叧缁嗚妭灏嗗湪 `Management of Cards and
Components`_ 涓€鑺備腑瑙ｉ噴銆?

#### 3) 鍒涘缓涓荤粍浠?


```

  struct mychip *chip;
  ....
  err = snd_mychip_create(card, pci, &chip);
  if (err < 0)
          goto error;

```

鐩稿叧缁嗚妭灏嗗湪 `PCI Resource
Management`_ 涓€鑺備腑瑙ｉ噴銆?

褰撳彂鐢熼敊璇椂锛宲robe 鍑芥暟闇€瑕佸鐞嗚閿欒銆傚湪鏈緥涓紝鎴戜滑鏈変竴鏉＄粺涓€鐨勯敊璇鐞嗚矾寰勶紝
鏀惧湪

```

  error:
          snd_card_free(card);
          return err;

```

鐢变簬姣忎釜缁勪欢閮藉彲浠ヨ姝ｇ‘鍦伴噴鏀撅紝鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝鍗曠嫭涓€娆?
`snd_card_free()` 璋冪敤灏辫冻澶熶簡銆?


#### 4) 璁剧疆椹卞姩 ID 涓庡悕绉板瓧绗︿覆銆?


```

  strcpy(card->driver, "My Chip");
  strcpy(card->shortname, "My Own Chip 123");
  sprintf(card->longname, "%s at 0x%lx irq %i",
          card->shortname, chip->port, chip->irq);

```

driver 瀛楁淇濆瓨鐫€鑺墖鐨勬渶灏?ID 瀛楃涓层€傚畠琚?alsa-lib 鐨勯厤缃櫒鎵€浣跨敤锛屽洜姝?
瑕佷繚鎸佺畝鍗曡€屽敮涓€銆傚嵆渚挎槸鍚屼竴涓┍鍔紝涔熷彲浠ユ嫢鏈変笉鍚岀殑椹卞姩 ID锛屼互鍖哄垎姣忕鑺墖绫诲瀷
鐨勫姛鑳姐€?

shortname 瀛楁鏄綔涓烘洿璇︾粏鍚嶇О鏄剧ず鐨勫瓧绗︿覆銆俵ongname 瀛楁鍖呭惈鐨?
淇℃伅鏄剧ず鍦?`/proc/asound/cards` 涓€?

#### 5) 鍒涘缓鍏朵粬缁勪欢锛屼緥濡傛贩闊冲櫒銆丮IDI 绛夈€?


鍦ㄨ繖閲屼綘瀹氫箟鍩烘湰鐨勭粍浠讹紝渚嬪 `PCM <PCM Interface_>`__銆佹贩闊冲櫒锛堜緥濡?
`AC97 <API for AC97 Codec_>`__锛夈€丮IDI锛堜緥濡?
`MPU-401 <MIDI (MPU401-UART) Interface_>`__锛変互鍙婂叾浠栨帴鍙ｃ€傛澶栵紝濡傛灉浣?
鎯宠涓€涓?`proc 鏂囦欢 <Proc Interface_>`__锛屼篃瑕佸湪杩欓噷瀹氫箟瀹冦€?

#### 6) 娉ㄥ唽澹板崱瀹炰緥銆?


```

  err = snd_card_register(card);
  if (err < 0)
          goto error;

```

杩欓儴鍒嗕篃浼氬湪 `Management of Cards and
Components`_ 涓€鑺備腑瑙ｉ噴銆?

#### 7) 璁剧疆 PCI 椹卞姩鏁版嵁骞惰繑鍥為浂銆?


```

  pci_set_drvdata(pci, card);
  dev++;
  return 0;

```

鍦ㄤ笂闈紝澹板崱璁板綍琚繚瀛樹笅鏉ャ€傝繖涓寚閽堝湪 remove 鍥炶皟鍑芥暟浠ュ強鐢垫簮绠＄悊鍥炶皟鍑芥暟涓?
涔熶細琚娇鐢ㄣ€?

### 鏋愭瀯鍑芥暟


鏋愭瀯鍑芥暟锛屽嵆 remove 鍥炶皟鍑芥暟锛屽彧鏄畝鍗曞湴閲婃斁澹板崱瀹炰緥銆傞殢鍚?ALSA 涓棿灞備細鑷姩
閲婃斁鎵€鏈夊凡鎸傝浇鐨勭粍浠躲€?

```

  static void snd_mychip_remove(struct pci_dev *pci)
  {
          snd_card_free(pci_get_drvdata(pci));
  }


```

涓婇潰鐨勪唬鐮佸亣瀹氬０鍗℃寚閽堝凡琚缃负 PCI 椹卞姩鏁版嵁銆?

### 澶存枃浠?


瀵逛簬涓婇潰鐨勪緥瀛愶紝鑷冲皯闇€瑕佸寘鍚互涓嬪ご鏂囦欢锛?

```

  #include <linux/init.h>
  #include <linux/pci.h>
  #include <linux/slab.h>
  #include <sound/core.h>
  #include <sound/initval.h>

```

鍏朵腑鏈€鍚庝竴涓彧鏈夊湪婧愭枃浠朵腑瀹氫箟浜嗘ā鍧楅€夐」鏃舵墠闇€瑕併€傚鏋滀唬鐮佽鎷嗗垎鎴愬涓枃浠讹紝
閭ｄ箞娌℃湁妯″潡閫夐」鐨勬枃浠跺氨涓嶉渶瑕佸畠浠€?

闄や簡杩欎簺澶存枃浠朵箣澶栵紝涓柇澶勭悊闇€瑕?`<linux/interrupt.h>`锛孖/O 璁块棶闇€瑕?
`<linux/io.h>`銆傚鏋滀綘浣跨敤浜?`mdelay()` 鎴?`udelay()` 鍑芥暟锛?
杩橀渶瑕佸寘鍚?`<linux/delay.h>`銆?

鍍?PCM 鍜屾帶鍒讹紙control锛堿PI 杩欐牱鐨?ALSA 鎺ュ彛瀹氫箟鍦ㄥ叾浠?
`<sound/xxx.h>` 澶存枃浠朵腑銆傚畠浠繀椤诲湪 `<sound/core.h>` 涔嬪悗琚寘鍚€?

## 澹板崱涓庣粍浠剁殑绠＄悊


### 澹板崱瀹炰緥


瀵逛簬姣忓紶澹板崱锛岄兘蹇呴』鍒嗛厤涓€涓€滃０鍗★紙card锛夆€濊褰曘€?

澹板崱璁板綍鏄０鍗＄殑鎬绘寚鎸ラ儴銆傚畠绠＄悊鐫€澹板崱涓婃暣涓澶囷紙缁勪欢锛夊垪琛紝渚嬪 PCM銆佹贩闊冲櫒銆?
MIDI銆佸悎鎴愬櫒绛夈€傛澶栵紝澹板崱璁板綍淇濆瓨鐫€澹板崱鐨?ID 涓庡悕绉板瓧绗︿覆锛岀鐞嗙潃 proc 鏂囦欢
鐨勬牴鐩綍锛屽苟鎺у埗鐫€鐢垫簮绠＄悊鐘舵€佷笌鐑彃鎷旀柇寮€銆傚０鍗¤褰曚笂鐨勭粍浠跺垪琛ㄧ敤浜庡湪閿€姣佹椂
绠＄悊璧勬簮鐨勬纭噴鏀俱€?

濡備笂鎵€杩帮紝瑕佸垱寤哄０鍗″疄渚嬶紝璋冪敤

```

  struct snd_card *card;
  int err;
  err = snd_card_new(&pci->dev, index, id, module, extra_size, &card);


```

璇ュ嚱鏁版帴鍙楀叚涓弬鏁帮細鐖惰澶囨寚閽堛€佸０鍗＄储寮曞彿銆乮d 瀛楃涓层€佹ā鍧楁寚閽堬紙閫氬父涓?
`THIS_MODULE`锛夈€侀澶栨暟鎹┖闂寸殑澶у皬锛屼互鍙婄敤浜庤繑鍥炲０鍗″疄渚嬬殑鎸囬拡銆俥xtra_size
鍙傛暟鐢ㄤ簬涓鸿姱鐗囦笓鏈夋暟鎹垎閰?card->private_data銆傛敞鎰忚繖浜涙暟鎹槸鐢?
`snd_card_new()` 鍒嗛厤鐨勩€?

绗竴涓弬鏁帮紝鍗?struct device 鐨勬寚閽堬紝鎸囧畾浜嗙埗璁惧銆傚浜?PCI 璁惧锛岄€氬父
浼犲叆 `&pci->`銆?

### 缁勪欢


鍦ㄥ０鍗″垱寤轰箣鍚庯紝浣犲彲浠ュ皢缁勪欢锛堣澶囷級鎸傝浇鍒板０鍗″疄渚嬩笂銆傚湪 ALSA 椹卞姩涓紝涓€涓粍浠?
鐢?struct snd_device 瀵硅薄琛ㄧず銆備竴涓粍浠跺彲浠ユ槸涓€涓?PCM 瀹炰緥銆佷竴涓帶鍒舵帴鍙ｃ€?
涓€涓?raw MIDI 鎺ュ彛绛夈€傛瘡涓€涓繖鏍风殑瀹炰緥閮芥湁涓€涓粍浠舵潯鐩€?

鍙互閫氳繃 `snd_device_new()` 鍒涘缓涓€涓粍浠讹細

```

  snd_device_new(card, SNDRV_DEV_XXX, chip, &ops);

```

瀹冩帴鍙楀０鍗℃寚閽堛€佽澶囩骇鍒紙`SNDRV_DEV_XXX`锛夈€佹暟鎹寚閽堜互鍙婂洖璋冩寚閽堬紙`&ops`锛夈€?
璁惧绾у埆瀹氫箟浜嗙粍浠剁殑绫诲瀷浠ュ強娉ㄥ唽鍜屽弽娉ㄥ唽鐨勯『搴忋€傚浜庡ぇ澶氭暟缁勪欢锛岃澶囩骇鍒凡缁?
瀹氫箟濂戒簡銆傚浜庣敤鎴疯嚜瀹氫箟鐨勭粍浠讹紝鍙互浣跨敤 `SNDRV_DEV_LOWLEVEL`銆?

璇ュ嚱鏁版湰韬苟涓嶅垎閰嶆暟鎹┖闂淬€傛暟鎹繀椤讳簨鍏堟墜鍔ㄥ垎閰嶏紝鍏舵寚閽堜綔涓哄弬鏁颁紶鍏ャ€傝繖涓寚閽?
锛堜笂闈緥瀛愪腑鐨?`chip`锛夎鐢ㄤ綔璇ュ疄渚嬬殑鏍囪瘑绗︺€?

姣忎釜棰勫畾涔夌殑 ALSA 缁勪欢锛堝 AC97 鍜?PCM锛夐兘浼氬湪鍏舵瀯閫犲嚱鏁板唴閮ㄨ皟鐢?
`snd_device_new()`銆傛瘡涓粍浠剁殑鏋愭瀯鍑芥暟瀹氫箟鍦ㄥ洖璋冩寚閽堜腑銆傚洜姝わ紝浣犱笉闇€瑕?
鍏冲績涓鸿繖鏍风殑缁勪欢璋冪敤鏋愭瀯鍑芥暟銆?

濡傛灉浣犲笇鏈涘垱寤鸿嚜宸辩殑缁勪欢锛屽垯闇€瑕佸皢鏋愭瀯鍑芥暟璁剧疆鍒?`ops` 鐨?dev_free 鍥炶皟涓紝
浠ヤ究瀹冭兘閫氳繃 `snd_card_free()` 鑷姩閲婃斁銆備笅涓€涓緥瀛愬皢灞曠ず鑺墖涓撴湁鏁版嵁鐨勫疄鐜般€?

### 鑺墖涓撴湁鏁版嵁


鑺墖涓撴湁淇℃伅锛屼緥濡?I/O 绔彛鍦板潃銆佸叾璧勬簮绛夛細

```

  struct mychip {
          ....
  };


```

涓€鑸潵璇达紝鍒嗛厤鑺墖璁板綍鏈変袱绉嶆柟寮忋€?

#### 1. 閫氳繃 :c:func:`snd_card_new()` 鍒嗛厤銆?


濡備笂鎵€杩帮紝浣犲彲浠ュ皢棰濆鏁版嵁闀垮害浼犵粰绗?5 涓弬鏁帮細

```

  err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                     sizeof(struct mychip), &card);

```

struct mychip 鏄姱鐗囪褰曠殑绫诲瀷銆?

浣滀负鍥炴姤锛屽凡鍒嗛厤鐨勮褰曞彲浠ュ涓嬫柟寮忚闂細

```

  struct mychip *chip = card->private_data;

```

浣跨敤杩欑鏂规硶锛屼綘涓嶅繀鍒嗛厤涓ゆ銆傝璁板綍浼氶殢澹板崱瀹炰緥涓€璧疯閲婃斁銆?

#### 2. 鍒嗛厤涓€涓澶栫殑璁惧銆?


鍦ㄩ€氳繃 `snd_card_new()` 鍒嗛厤澹板崱瀹炰緥涔嬪悗锛?

```

  struct snd_card *card;
  struct mychip *chip;
  err = snd_card_new(&pci->dev, index[dev], id[dev], THIS_MODULE,
                     0, &card);
  .....
  chip = kzalloc(sizeof(*chip), GFP_KERNEL);

```

鑺墖璁板綍鑷冲皯搴斿綋鍖呭惈鐢ㄤ簬淇濆瓨澹板崱鎸囬拡鐨勫瓧娈碉細

```

  struct mychip {
          struct snd_card *card;
          ....
  };


```

```

  chip->card = card;

```

鎺ヤ笅鏉ワ紝鍒濆鍖栧悇瀛楁锛屽苟灏嗚繖涓姱鐗囪褰曟敞鍐屼负涓€涓粍浠讹細

```

  static const struct snd_device_ops ops = {
          .dev_free =        snd_mychip_dev_free,
  };
  ....
  snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip, &ops);

```

`snd_mychip_dev_free()` 鏄澶囨瀽鏋勫嚱鏁帮細

```

  static int snd_mychip_dev_free(struct snd_device *device)
  {
          return snd_mychip_free(device->device_data);
  }

```

鍏朵腑 `snd_mychip_free()` 鏄湡姝ｇ殑鏋愭瀯鍑芥暟銆?

杩欑鏂规硶鐨勭己鐐规樉鐒舵槸浠ｇ爜閲忔洿澶с€備絾鍏朵紭鐐规槸锛屼綘鍙互閫氳繃 snd_device_ops 涓殑
璁剧疆锛屽湪娉ㄥ唽涓庢柇寮€澹板崱鏃惰Е鍙戜綘鑷繁鐨勫洖璋冦€傚叧浜庢敞鍐屽拰鏂紑澹板崱锛岃鍙傞槄涓嬮潰鐨勫皬鑺傘€?


### 娉ㄥ唽涓庨噴鏀?


鍦ㄦ墍鏈夌粍浠堕兘琚垎閰嶄箣鍚庯紝閫氳繃璋冪敤 `snd_card_register()` 娉ㄥ唽澹板崱瀹炰緥銆?
姝ゆ椂璁惧鐨勬枃浠惰闂鍚敤銆備篃灏辨槸璇达紝鍦ㄨ皟鐢?`snd_card_register()` 涔嬪墠锛?
缁勪欢浠庡閮ㄦ槸鏃犳硶瀹夊叏璁块棶鐨勩€傚鏋滆璋冪敤澶辫触锛屽垯鍦ㄩ€氳繃 `snd_card_free()`
閲婃斁澹板崱涔嬪悗閫€鍑?probe 鍑芥暟銆?

瑕侀噴鏀惧０鍗″疄渚嬶紝浣犲彲浠ョ畝鍗曞湴璋冪敤 `snd_card_free()`銆傚鍓嶆墍杩帮紝鎵€鏈夌粍浠堕兘浼?
閫氳繃璇ヨ皟鐢ㄨ鑷姩閲婃斁銆?

瀵逛簬鍏佽鐑彃鎷旂殑璁惧锛屼綘鍙互浣跨敤 `snd_card_free_when_closed()`銆傝繖涓嚱鏁颁細
灏嗛攢姣佹帹杩熷埌鎵€鏈夎澶囬兘鍏抽棴涔嬪悗銆?

## PCI 璧勬簮绠＄悊


### 瀹屾暣浠ｇ爜绀轰緥


鍦ㄦ湰鑺備腑锛屾垜浠皢琛ュ叏鑺墖涓撴湁鏋勯€犲嚱鏁帮細

```

      struct mychip {
              struct snd_card *card;
              struct pci_dev *pci;

              unsigned long port;
              int irq;
      };

      static int snd_mychip_free(struct mychip *chip)
      {
              /* disable hardware here if any */
              .... /* (not implemented in this document) */

              /* release the irq */
              if (chip->irq >= 0)
                      free_irq(chip->irq, chip);
              /* release the I/O ports & memory */
              pci_release_regions(chip->pci);
              /* disable the PCI entry */
              pci_disable_device(chip->pci);
              /* release the data */
              kfree(chip);
              return 0;
      }

      /* chip-specific constructor */
      static int snd_mychip_create(struct snd_card *card,
                                   struct pci_dev *pci,
                                   struct mychip **rchip)
      {
              struct mychip *chip;
              int err;
              static const struct snd_device_ops ops = {
                     .dev_free = snd_mychip_dev_free,
              };

              *rchip = NULL;

              /* initialize the PCI entry */
              err = pci_enable_device(pci);
              if (err < 0)
                      return err;
              /* check PCI availability (28bit DMA) */
              if (pci_set_dma_mask(pci, DMA_BIT_MASK(28)) < 0 ||
                  pci_set_consistent_dma_mask(pci, DMA_BIT_MASK(28)) < 0) {
                      printk(KERN_ERR "error to set 28bit mask DMA\n");
                      pci_disable_device(pci);
                      return -ENXIO;
              }

              chip = kzalloc(sizeof(*chip), GFP_KERNEL);
              if (chip == NULL) {
                      pci_disable_device(pci);
                      return -ENOMEM;
              }

              /* initialize the stuff */
              chip->card = card;
              chip->pci = pci;
              chip->irq = -1;

              /* (1) PCI resource allocation */
              err = pci_request_regions(pci, "My Chip");
              if (err < 0) {
                      kfree(chip);
                      pci_disable_device(pci);
                      return err;
              }
              chip->port = pci_resource_start(pci, 0);
              if (request_irq(pci->irq, snd_mychip_interrupt,
                              IRQF_SHARED, KBUILD_MODNAME, chip)) {
                      printk(KERN_ERR "cannot grab irq %d\n", pci->irq);
                      snd_mychip_free(chip);
                      return -EBUSY;
              }
              chip->irq = pci->irq;
              card->sync_irq = chip->irq;

              /* (2) initialization of the chip hardware */
              .... /*   (not implemented in this document) */

              err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip, &ops);
              if (err < 0) {
                      snd_mychip_free(chip);
                      return err;
              }

              *rchip = chip;
              return 0;
      }

      /* PCI IDs */
      static struct pci_device_id snd_mychip_ids[] = {
              { PCI_VENDOR_ID_FOO, PCI_DEVICE_ID_BAR,
                PCI_ANY_ID, PCI_ANY_ID, 0, 0, 0, },
              ....
              { 0, }
      };
      MODULE_DEVICE_TABLE(pci, snd_mychip_ids);

      /* pci_driver definition */
      static struct pci_driver driver = {
              .name = KBUILD_MODNAME,
              .id_table = snd_mychip_ids,
              .probe = snd_mychip_probe,
              .remove = snd_mychip_remove,
      };

      /* module initialization */
      static int __init alsa_card_mychip_init(void)
      {
              return pci_register_driver(&driver);
      }

      /* module clean up */
      static void __exit alsa_card_mychip_exit(void)
      {
              pci_unregister_driver(&driver);
      }

      module_init(alsa_card_mychip_init)
      module_exit(alsa_card_mychip_exit)

      EXPORT_NO_SYMBOLS; /* for old kernels only */

```

### 涓€浜涜鐐?


PCI 璧勬簮鐨勫垎閰嶅湪 `probe` 鍑芥暟涓畬鎴愶紝閫氬父涓烘浼氫笓闂ㄧ紪鍐欎竴涓澶栫殑
`xxx_create()` 鍑芥暟銆?

瀵逛簬 PCI 璁惧锛屽湪鍒嗛厤璧勬簮涔嬪墠锛屼綘蹇呴』棣栧厛璋冪敤 `pci_enable_device()` 鍑芥暟銆?
姝ゅ锛屼綘杩橀渶瑕佽缃悎閫傜殑 PCI DMA 鎺╃爜锛坢ask锛夋潵闄愬埗鍙闂殑 I/O 鑼冨洿銆傚湪鏌愪簺
鎯呭喌涓嬶紝浣犲彲鑳借繕闇€瑕佽皟鐢?`pci_set_master()` 鍑芥暟銆?

```

  err = pci_enable_device(pci);
  if (err < 0)
          return err;
  if (pci_set_dma_mask(pci, DMA_BIT_MASK(28)) < 0 ||
      pci_set_consistent_dma_mask(pci, DMA_BIT_MASK(28)) < 0) {
          printk(KERN_ERR "error to set 28bit mask DMA\n");
          pci_disable_device(pci);
          return -ENXIO;
  }


```

### 璧勬簮鍒嗛厤


I/O 绔彛涓庝腑鏂殑鍒嗛厤鏄€氳繃鏍囧噯鍐呮牳鍑芥暟瀹屾垚鐨勩€傝繖浜涜祫婧愬繀椤诲湪鏋愭瀯鍑芥暟涓閲婃斁
锛堣涓嬫枃锛夈€?

鐜板湪鍋囪璇?PCI 璁惧鏈変竴涓?8 瀛楄妭鐨?I/O 绔彛鍜屼竴涓腑鏂€傞偅涔?struct mychip
灏嗗寘鍚細

```

  struct mychip {
          struct snd_card *card;

          unsigned long port;
          int irq;
  };


```

瀵逛簬 I/O 绔彛锛堜互鍙婂唴瀛樺尯鍩燂級锛屼綘闇€瑕佷负鏍囧噯璧勬簮绠＄悊淇濆瓨璧勬簮鎸囬拡銆傚浜庝腑鏂紝浣?
鍙渶淇濆瓨涓柇鍙凤紙鏁存暟锛夊嵆鍙€備絾闇€瑕佸湪瀹為檯鍒嗛厤涔嬪墠灏嗗叾鍒濆鍖栦负 -1锛屽洜涓轰腑鏂?0
涔熸槸鏈夋晥鐨勩€傜鍙ｅ湴鍧€鍙婂叾璧勬簮鎸囬拡浼氳 `kzalloc()` 鑷姩鍒濆鍖栦负 null锛屽洜姝?
浣犱笉蹇呭叧蹇冮噸缃畠浠€?

```

  err = pci_request_regions(pci, "My Chip");
  if (err < 0) { 
          kfree(chip);
          pci_disable_device(pci);
          return err;
  }
  chip->port = pci_resource_start(pci, 0);

```

瀹冧細淇濈暀璇ョ粰瀹?PCI 璁惧 8 瀛楄妭鐨?I/O 绔彛鍖哄煙銆傝繑鍥炲€?`chip->res_port`
鏄敱 `request_region()` 閫氳繃 `kmalloc()` 鍒嗛厤鐨勩€傝鎸囬拡蹇呴』閫氳繃
`kfree()` 閲婃斁锛屼絾杩欓噷瀛樺湪涓€涓棶棰樸€傝繖涓棶棰樺皢鍦ㄥ悗闈㈣В閲娿€?

```

  if (request_irq(pci->irq, snd_mychip_interrupt,
                  IRQF_SHARED, KBUILD_MODNAME, chip)) {
          printk(KERN_ERR "cannot grab irq %d\n", pci->irq);
          snd_mychip_free(chip);
          return -EBUSY;
  }
  chip->irq = pci->irq;

```

鍏朵腑 `snd_mychip_interrupt()` 鏄腑鏂鐞嗗嚱鏁帮紝瀹氫箟浜?
`鍚庢枃 <PCM Interrupt Handler_>`__銆傛敞鎰?`chip->irq` 搴斾粎鍦?
`request_irq()` 鎴愬姛鏃舵墠琚畾涔夈€?

鍦?PCI 鎬荤嚎涓婏紝涓柇鏄彲浠ュ叡浜殑銆傚洜姝わ紝`IRQF_SHARED` 琚敤浣?
`request_irq()` 鐨勪腑鏂爣蹇椼€?

`request_irq()` 鐨勬渶鍚庝竴涓弬鏁版槸浼犻€掔粰涓柇澶勭悊鍑芥暟鐨勬暟鎹寚閽堛€傞€氬父锛岃姱鐗?
涓撴湁璁板綍琚敤浣滆鎸囬拡锛屼絾浣犱篃鍙互浣跨敤浠讳綍浣犲枩娆㈢殑涓滆タ銆?

鎴戠幇鍦ㄤ笉鎵撶畻缁欏嚭涓柇澶勭悊鍑芥暟鐨勭粏鑺傦紝浣嗚嚦灏戝畠鐨勬牱瀛愮幇鍦ㄥ彲浠ヨ鏄庛€備腑鏂鐞嗗嚱鏁?
鐪嬭捣鏉ュ儚杩欐牱锛?

```

  static irqreturn_t snd_mychip_interrupt(int irq, void *dev_id)
  {
          struct mychip *chip = dev_id;
          ....
          return IRQ_HANDLED;
  }

```

璇锋眰 IRQ 涔嬪悗锛屼綘鍙互灏嗗叾浼犻€掔粰 `card->sync_irq`锛?

```

          card->irq = chip->irq;

```

杩欏厑璁?PCM 鏍稿績鍦ㄥ悎閫傜殑鏃舵満锛堜緥濡?`hw_free` 涔嬪墠锛夎嚜鍔ㄨ皟鐢?
`synchronize_irq()`銆傝瑙佸悗鏂囩殑 `sync_stop callback`_ 涓€鑺傘€?

鐜板湪璁╂垜浠负涓婅堪璧勬簮缂栧啓鐩稿簲鐨勬瀽鏋勫嚱鏁般€傛瀽鏋勫嚱鏁扮殑瑙掕壊寰堢畝鍗曪細绂佺敤纭欢锛堝鏋?
宸茬粡婵€娲伙級骞堕噴鏀捐祫婧愩€傚埌鐩墠涓烘鎴戜滑杩樻病鏈夌‖浠堕儴鍒嗭紝鍥犳杩欓噷娌℃湁鍐欏嚭绂佺敤浠ｇ爜銆?

瀵逛簬閲婃斁璧勬簮锛屸€滄鏌ュ苟閲婃斁鈥濈殑鏂规硶鏄竴绉嶆洿瀹夊叏鐨勬柟寮忋€?

```

  if (chip->irq >= 0)
          free_irq(chip->irq, chip);

```

鐢变簬涓柇鍙峰彲浠ヤ粠 0 寮€濮嬶紝浣犲簲璇ョ敤涓€涓礋鍊硷紙渚嬪 -1锛夊垵濮嬪寲 `chip->irq`锛?
杩欐牱浣犲氨鍙互鍍忎笂闈㈤偅鏍锋鏌ヤ腑鏂彿鐨勬湁鏁堟€с€?

褰撲綘鍍忔湰渚嬩腑涓€鏍烽€氳繃 `pci_request_region()` 鎴?
`pci_request_regions()` 璇锋眰浜?I/O 绔彛鎴栧唴瀛樺尯鍩熸椂锛屼娇鐢ㄧ浉搴旂殑鍑芥暟
`pci_release_region()` 鎴?

```

  pci_release_regions(chip->pci);

```

鏉ラ噴鏀捐祫婧愩€?

褰撲綘閫氳繃 `request_region()` 鎴?`request_mem_region()` 鎵嬪姩璇锋眰鏃讹紝
鍙互閫氳繃 `release_resource()` 閲婃斁瀹冦€傚亣璁句綘淇濆瓨浜嗙敱 `request_region()`
杩斿洖鐨勬寚閽堬細

```

  release_and_free_resource(chip->res_port);

```

鍦ㄧ粨鏉熶箣鍓嶏紝鍒繕浜嗚皟鐢?`pci_disable_device()`銆?

```

  kfree(chip);

```

鎴戜滑涓婇潰娌℃湁瀹炵幇纭欢绂佺敤閮ㄥ垎銆傚鏋滀綘闇€瑕佽繖鏍峰仛锛岃娉ㄦ剰锛屾瀽鏋勫嚱鏁扮敋鑷冲湪鑺墖鍒濆鍖?
瀹屾垚涔嬪墠灏卞彲鑳借璋冪敤銆傛渶濂芥湁涓€涓爣蹇楋紝浠ヤ究鍦ㄦ病鏈夊垵濮嬪寲纭欢鏃惰烦杩囩‖浠剁鐢ㄣ€?

褰撹姱鐗囨暟鎹€氳繃 `snd_device_new()` 閰嶅悎 `SNDRV_DEV_LOWLELVEL` 琚垎閰嶇粰
澹板崱鏃讹紝瀹冪殑鏋愭瀯鍑芥暟鏄渶鍚庤璋冪敤鐨勩€備篃灏辨槸璇达紝鍙互淇濊瘉鎵€鏈夊叾浠栫粍浠讹紙濡?PCM 鍜?
鎺у埗锛夐兘宸茬粡琚噴鏀俱€備綘涓嶅繀鏄惧紡鍦板仠姝?PCM 绛夛紝鍙渶璋冪敤搴曞眰纭欢鍋滄鍗冲彲銆?

鍐呭瓨鏄犲皠锛坢emory-mapped锛夊尯鍩熺殑绠＄悊鍑犱箮涓庝笂闈㈢浉鍚岋細

```

  struct mychip {
          ....
          unsigned long iobase_phys;
          void __iomem *iobase_virt;
  };


```

```

  err = pci_request_regions(pci, "My Chip");
  if (err < 0) {
          kfree(chip);
          return err;
  }
  chip->iobase_phys = pci_resource_start(pci, 0);
  chip->iobase_virt = ioremap(chip->iobase_phys,
                                      pci_resource_len(pci, 0));

```

```

  static int snd_mychip_free(struct mychip *chip)
  {
          ....
          if (chip->iobase_virt)
                  iounmap(chip->iobase_virt);
          ....
          pci_release_regions(chip->pci);
          ....
  }

```

褰撶劧锛屼娇鐢?`pci_iomap()` 鐨勭幇浠ｆ柟寮忎細璁╀簨鎯呭彉寰?

```

  err = pci_request_regions(pci, "My Chip");
  if (err < 0) {
          kfree(chip);
          return err;
  }
  chip->iobase_virt = pci_iomap(pci, 0, 0);

```

杩欏湪鏋愭瀯鍑芥暟涓笌 `pci_iounmap()` 閰嶅浣跨敤銆?


### PCI 鏉＄洰


鍒扮洰鍓嶄负姝紝涓€鍒囬『鍒┿€傝鎴戜滑瀹屾垚缂哄け鐨?PCI 閮ㄥ垎銆傞鍏堬紝鎴戜滑闇€瑕佷竴涓?
struct pci_device_id 琛紝鐢ㄤ簬杩欎釜鑺墖缁勩€傚畠鏄竴涓?PCI 鍘傚晢/璁惧 ID 鍙?
浠ュ強鏌愪簺鎺╃爜鐨勮〃銆?

```

  static struct pci_device_id snd_mychip_ids[] = {
          { PCI_VENDOR_ID_FOO, PCI_DEVICE_ID_BAR,
            PCI_ANY_ID, PCI_ANY_ID, 0, 0, 0, },
          ....
          { 0, }
  };
  MODULE_DEVICE_TABLE(pci, snd_mychip_ids);

```

struct pci_device_id 鐨勭涓€涓拰绗簩涓瓧娈垫槸鍘傚晢鍜岃澶?ID銆傚鏋滀綘娌℃湁鐞嗙敱杩囨护
鍖归厤鐨勮澶囷紝鍙互灏嗗叾浣欏瓧娈典繚鎸佸涓娿€俿truct pci_device_id 鐨勬渶鍚庝竴涓瓧娈靛寘鍚?
璇ユ潯鐩殑绉佹湁鏁版嵁銆備綘鍙互鍦ㄨ繖閲屾寚瀹氫换鎰忓€硷紝渚嬪涓哄彈鏀寔鐨勮澶?ID 瀹氫箟鐗瑰畾鐨?
鎿嶄綔銆傝繖鏍风殑渚嬪瓙鍙互鍦?intel8x0 椹卞姩涓壘鍒般€?

璇ュ垪琛ㄧ殑鏈€鍚庝竴涓潯鐩槸缁堟绗︺€備綘蹇呴』鎸囧畾杩欎釜鍏ㄩ浂鏉＄洰銆?

鐒跺悗锛屽噯澶?struct pci_driver锛?

```

  static struct pci_driver driver = {
          .name = KBUILD_MODNAME,
          .id_table = snd_mychip_ids,
          .probe = snd_mychip_probe,
          .remove = snd_mychip_remove,
  };

```

`probe` 鍜?`remove` 鍑芥暟宸茬粡鍦ㄥ墠闈㈠嚑鑺備腑瀹氫箟杩囦簡銆俙name` 瀛楁鏄繖涓?
璁惧鐨勫悕绉板瓧绗︿覆銆傛敞鎰忥紝浣犱笉鑳藉湪璇ュ瓧绗︿覆涓娇鐢ㄦ枩鏉狅紙鈥?鈥濓級銆?

```

  static int __init alsa_card_mychip_init(void)
  {
          return pci_register_driver(&driver);
  }

  static void __exit alsa_card_mychip_exit(void)
  {
          pci_unregister_driver(&driver);
  }

  module_init(alsa_card_mychip_init)
  module_exit(alsa_card_mychip_exit)

```

娉ㄦ剰锛岃繖浜涙ā鍧楁潯鐩兘甯︽湁 `__init` 鍜?`__exit` 鍓嶇紑銆?

灏辫繖浜涗簡锛?

## PCM 鎺ュ彛


### 姒傝堪


ALSA 鐨?PCM 涓棿灞傜浉褰撳己澶э紝姣忎釜椹卞姩鍙渶瀹炵幇璁块棶鍏剁‖浠剁殑浣庡眰鍑芥暟鍗冲彲銆?

瑕佽闂?PCM 灞傦紝浣犻渶瑕佸厛鍖呭惈 `<sound/pcm.h>`銆傛澶栵紝濡傛灉浣犺闂竴浜涗笌
hw_param 鐩稿叧鐨勫嚱鏁帮紝鍙兘杩橀渶瑕?`<sound/pcm_params.h>`銆?

姣忓紶澹板崱璁惧鏈€澶氬彲浠ユ湁鍥涗釜 PCM 瀹炰緥銆備竴涓?PCM 瀹炰緥瀵瑰簲涓€涓?PCM 璁惧鏂囦欢銆傚疄渚?
鏁伴噺鐨勯檺鍒朵粎鏉ヨ嚜 Linux 璁惧鍙峰彲鐢ㄧ殑浣嶅ぇ灏忋€備竴鏃︿娇鐢?64 浣嶈澶囧彿锛屾垜浠氨浼氭湁
鏇村鍙敤鐨?PCM 瀹炰緥銆?

涓€涓?PCM 瀹炰緥鐢?PCM 鎾斁锛坧layback锛夊拰鎹曡幏锛坈apture锛夋祦缁勬垚锛岃€屾瘡涓?PCM 娴佺敱
涓€涓垨澶氫釜 PCM 瀛愭祦锛坰ubstream锛夌粍鎴愩€傛煇浜涘０鍗℃敮鎸佸绉嶆挱鏀惧姛鑳姐€備緥濡傦紝emu10k1
鎷ユ湁 32 涓珛浣撳０瀛愭祦鐨?PCM 鎾斁銆傚湪杩欑鎯呭喌涓嬶紝姣忔鎵撳紑鏃讹紝锛堥€氬父锛変細鑷姩閫夋嫨
骞舵墦寮€涓€涓┖闂茬殑瀛愭祦銆傚悓鏃讹紝褰撳彧瀛樺湪涓€涓瓙娴佷笖瀹冨凡缁忚鎵撳紑鏃讹紝闅忓悗鐨勬墦寮€灏嗘牴鎹?
鏂囦欢鎵撳紑妯″紡闃诲鎴栦互 `EAGAIN` 閿欒杩斿洖銆備絾浣犱笉蹇呭湪椹卞姩涓叧蹇冭繖浜涚粏鑺傘€侾CM
涓棿灞備細澶勭悊杩欑被宸ヤ綔銆?

### 瀹屾暣浠ｇ爜绀轰緥


涓嬮潰鐨勭ず渚嬩唬鐮佷笉鍖呭惈浠讳綍纭欢璁块棶渚嬬▼锛屼絾

```

      #include <sound/pcm.h>
      ....

      /* hardware definition */
      static struct snd_pcm_hardware snd_mychip_playback_hw = {
              .info = (SNDRV_PCM_INFO_MMAP |
                       SNDRV_PCM_INFO_INTERLEAVED |
                       SNDRV_PCM_INFO_BLOCK_TRANSFER |
                       SNDRV_PCM_INFO_MMAP_VALID),
              .formats =          SNDRV_PCM_FMTBIT_S16_LE,
              .rates =            SNDRV_PCM_RATE_8000_48000,
              .rate_min =         8000,
              .rate_max =         48000,
              .channels_min =     2,
              .channels_max =     2,
              .buffer_bytes_max = 32768,
              .period_bytes_min = 4096,
              .period_bytes_max = 32768,
              .periods_min =      1,
              .periods_max =      1024,
      };

      /* hardware definition */
      static struct snd_pcm_hardware snd_mychip_capture_hw = {
              .info = (SNDRV_PCM_INFO_MMAP |
                       SNDRV_PCM_INFO_INTERLEAVED |
                       SNDRV_PCM_INFO_BLOCK_TRANSFER |
                       SNDRV_PCM_INFO_MMAP_VALID),
              .formats =          SNDRV_PCM_FMTBIT_S16_LE,
              .rates =            SNDRV_PCM_RATE_8000_48000,
              .rate_min =         8000,
              .rate_max =         48000,
              .channels_min =     2,
              .channels_max =     2,
              .buffer_bytes_max = 32768,
              .period_bytes_min = 4096,
              .period_bytes_max = 32768,
              .periods_min =      1,
              .periods_max =      1024,
      };

      /* open callback */
      static int snd_mychip_playback_open(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              struct snd_pcm_runtime *runtime = substream->runtime;

              runtime->hw = snd_mychip_playback_hw;
              /* more hardware-initialization will be done here */
              ....
              return 0;
      }

      /* close callback */
      static int snd_mychip_playback_close(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              /* the hardware-specific codes will be here */
              ....
              return 0;

      }

      /* open callback */
      static int snd_mychip_capture_open(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              struct snd_pcm_runtime *runtime = substream->runtime;

              runtime->hw = snd_mychip_capture_hw;
              /* more hardware-initialization will be done here */
              ....
              return 0;
      }

      /* close callback */
      static int snd_mychip_capture_close(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              /* the hardware-specific codes will be here */
              ....
              return 0;
      }

      /* hw_params callback */
      static int snd_mychip_pcm_hw_params(struct snd_pcm_substream *substream,
                                   struct snd_pcm_hw_params *hw_params)
      {
              /* the hardware-specific codes will be here */
              ....
              return 0;
      }

      /* hw_free callback */
      static int snd_mychip_pcm_hw_free(struct snd_pcm_substream *substream)
      {
              /* the hardware-specific codes will be here */
              ....
              return 0;
      }

      /* prepare callback */
      static int snd_mychip_pcm_prepare(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              struct snd_pcm_runtime *runtime = substream->runtime;

              /* set up the hardware with the current configuration
               * for example...
               */
              mychip_set_sample_format(chip, runtime->format);
              mychip_set_sample_rate(chip, runtime->rate);
              mychip_set_channels(chip, runtime->channels);
              mychip_set_dma_setup(chip, runtime->dma_addr,
                                   chip->buffer_size,
                                   chip->period_size);
              return 0;
      }

      /* trigger callback */
      static int snd_mychip_pcm_trigger(struct snd_pcm_substream *substream,
                                        int cmd)
      {
              switch (cmd) {
              case SNDRV_PCM_TRIGGER_START:
                      /* do something to start the PCM engine */
                      ....
                      break;
              case SNDRV_PCM_TRIGGER_STOP:
                      /* do something to stop the PCM engine */
                      ....
                      break;
              default:
                      return -EINVAL;
              }
      }

      /* pointer callback */
      static snd_pcm_uframes_t
      snd_mychip_pcm_pointer(struct snd_pcm_substream *substream)
      {
              struct mychip *chip = snd_pcm_substream_chip(substream);
              unsigned int current_ptr;

              /* get the current hardware pointer */
              current_ptr = mychip_get_hw_pointer(chip);
              return current_ptr;
      }

      /* operators */
      static struct snd_pcm_ops snd_mychip_playback_ops = {
              .open =        snd_mychip_playback_open,
              .close =       snd_mychip_playback_close,
              .hw_params =   snd_mychip_pcm_hw_params,

              .hw_free =     snd_mychip_pcm_hw_free,
              .prepare =     snd_mychip_pcm_prepare,
              .trigger =     snd_mychip_pcm_trigger,
              .pointer =     snd_mychip_pcm_pointer,
      };

      /* operators */
      static struct snd_pcm_ops snd_mychip_capture_ops = {
              .open =        snd_mychip_capture_open,
              .close =       snd_mychip_capture_close,
              .hw_params =   snd_mychip_pcm_hw_params,
              .hw_free =     snd_mychip_pcm_hw_free,
              .prepare =     snd_mychip_pcm_prepare,
              .trigger =     snd_mychip_pcm_trigger,
              .pointer =     snd_mychip_pcm_pointer,
      };

      /*
       *  definitions of capture are omitted here...
       */

      /* create a pcm device */
      static int snd_mychip_new_pcm(struct mychip *chip)
      {
              struct snd_pcm *pcm;
              int err;

              err = snd_pcm_new(chip->card, "My Chip", 0, 1, 1, &pcm);
              if (err < 0)
                      return err;
              pcm->private_data = chip;
              strcpy(pcm->name, "My Chip");
              chip->pcm = pcm;
              /* set operators */
              snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK,
                              &snd_mychip_playback_ops);
              snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE,
                              &snd_mychip_capture_ops);
              /* pre-allocation of buffers */
              /* NOTE: this may fail */
              snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
                                             &chip->pci->dev,
                                             64*1024, 64*1024);
              return 0;
      }


```

### PCM 鏋勯€犲嚱鏁?


涓€涓?PCM 瀹炰緥鏄€氳繃 `snd_pcm_new()` 鍒嗛厤鐨勶細

```

  static int snd_mychip_new_pcm(struct mychip *chip)
  {
          struct snd_pcm *pcm;
          int err;

          err = snd_pcm_new(chip->card, "My Chip", 0, 1, 1, &pcm);
          if (err < 0) 
                  return err;
          pcm->private_data = chip;
          strcpy(pcm->name, "My Chip");
          chip->pcm = pcm;
          ...
          return 0;
  }

```

`snd_pcm_new()` 鍑芥暟鎺ュ彈鍏釜鍙傛暟銆傜涓€涓弬鏁版槸璇?PCM 鎵€鍒嗛厤鍒扮殑澹板崱鎸囬拡锛?
绗簩涓槸 ID 瀛楃涓层€?

绗笁涓弬鏁帮紙`index`锛屼笂闈负 0锛夋槸杩欎釜鏂?PCM 鐨勭储寮曘€傚畠浠庨浂寮€濮嬨€傚鏋滀綘鍒涘缓
澶氫釜 PCM 瀹炰緥锛岃鍦ㄨ鍙傛暟涓寚瀹氫笉鍚岀殑鏁板瓧銆備緥濡傦紝绗簩涓?PCM 璁惧浣跨敤 ``index =
1``銆?

绗洓涓拰绗簲涓弬鏁板垎鍒槸鎾斁鍜屾崟鑾风殑瀛愭祦鏁伴噺銆傝繖閲屼袱涓弬鏁伴兘浣跨敤 1銆傚綋娌℃湁
鎾斁鎴栨崟鑾峰瓙娴佸彲鐢ㄦ椂锛屽悜鐩稿簲鍙傛暟浼犲叆 0銆?

濡傛灉涓€涓姱鐗囨敮鎸佸涓挱鏀炬垨鎹曡幏锛屼綘鍙互鎸囧畾鏇村ぇ鐨勬暟瀛楋紝浣嗗畠浠繀椤诲湪 open/close
绛夊洖璋冧腑琚纭鐞嗐€傚綋浣犻渶瑕佺煡閬撲綘寮曠敤鐨勬槸鍝釜瀛愭祦鏃讹紝鍙互浠庝紶閫掔粰姣忎釜鍥炶皟鐨?
struct snd_pcm_substream 鏁版嵁涓幏鍙栵細

```

  struct snd_pcm_substream *substream;
  int index = substream->number;


```

```

  snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK,
                  &snd_mychip_playback_ops);
  snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE,
                  &snd_mychip_capture_ops);

```

```

  static struct snd_pcm_ops snd_mychip_playback_ops = {
          .open =        snd_mychip_pcm_open,
          .close =       snd_mychip_pcm_close,
          .hw_params =   snd_mychip_pcm_hw_params,
          .hw_free =     snd_mychip_pcm_hw_free,
          .prepare =     snd_mychip_pcm_prepare,
          .trigger =     snd_mychip_pcm_trigger,
          .pointer =     snd_mychip_pcm_pointer,
  };

```

鎵€鏈夊洖璋冮兘鍦?Operators_ 灏忚妭涓弿杩般€?

璁剧疆濂借繍绠楃涔嬪悗锛屼綘鍙兘鎯宠棰勫垎閰嶇紦鍐插尯骞惰缃墭绠″垎閰嶆ā寮忋€?

```

  snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV,
                                 &chip->pci->dev,
                                 64*1024, 64*1024);

```

榛樿鎯呭喌涓嬪畠浼氬垎閰嶄竴涓渶澶?64kB 鐨勭紦鍐插尯銆傜紦鍐插尯绠＄悊缁嗚妭灏嗗湪鍚庢枃鐨?
`Buffer and Memory Management`_ 涓€鑺備腑鎻忚堪銆?

姝ゅ锛屼綘鍙互鍦?`pcm->info_flags` 涓负杩欎釜 PCM 璁剧疆涓€浜涢澶栦俊鎭€傚彲鐢ㄥ€煎湪
`<sound/asound.h>` 涓畾涔変负 `SNDRV_PCM_INFO_XXX`锛屽畠鐢ㄤ簬纭欢瀹氫箟
锛堝悗鏂囨弿杩帮級銆傚綋浣犵殑澹伴煶鑺墖鍙敮鎸?

```

  pcm->info_flags = SNDRV_PCM_INFO_HALF_DUPLEX;


```

### 鈥︹€﹂偅涔堟瀽鏋勫嚱鏁板憿锛?


PCM 瀹炰緥鐨勬瀽鏋勫嚱鏁板苟涓嶆€绘槸蹇呰鐨勩€傜敱浜?PCM 璁惧浼氳涓棿灞備唬鐮佽嚜鍔ㄩ噴鏀撅紝浣犱笉蹇?
鏄惧紡鍦拌皟鐢ㄦ瀽鏋勫嚱鏁般€?

濡傛灉浣犲湪鍐呴儴鍒涘缓浜嗙壒娈婄殑璁板綍骞朵笖闇€瑕侀噴鏀惧畠浠紝鍒欐瀽鏋勫嚱鏁版槸蹇呰鐨勩€傚湪杩欑鎯呭喌涓嬶紝
璁剧疆

```

      static void mychip_pcm_free(struct snd_pcm *pcm)
      {
              struct mychip *chip = snd_pcm_chip(pcm);
              /* free your own data */
              kfree(chip->my_private_pcm_data);
              /* do what you like else */
              ....
      }

      static int snd_mychip_new_pcm(struct mychip *chip)
      {
              struct snd_pcm *pcm;
              ....
              /* allocate your own data */
              chip->my_private_pcm_data = kmalloc(...);
              /* set the destructor */
              pcm->private_data = chip;
              pcm->private_free = mychip_pcm_free;
              ....
      }



```

### 杩愯鏃舵寚閽堚€斺€擯CM 淇℃伅鐨勫疂搴?


褰?PCM 瀛愭祦琚墦寮€鏃讹紝浼氬垎閰嶄竴涓?PCM 杩愯鏃讹紙runtime锛夊疄渚嬪苟璧嬪€肩粰璇ュ瓙娴併€傝繖涓?
鎸囬拡鍙互閫氳繃 `substream->runtime` 璁块棶銆傝繖涓繍琛屾椂鎸囬拡淇濆瓨浜嗕綘鎺у埗 PCM
鎵€闇€鐨勫ぇ閮ㄥ垎淇℃伅锛歨w_params 鍜?sw_params 閰嶇疆鐨勫壇鏈€佺紦鍐插尯鎸囬拡銆乵map 璁板綍銆?
鑷棆閿佺瓑銆?

杩愯鏃跺疄渚嬬殑瀹氫箟浣嶄簬 `<sound/pcm.h>` 涓€傝繖閲?

```

  struct _snd_pcm_runtime {
          /* -- Status -- */
          struct snd_pcm_substream *trigger_master;
          snd_timestamp_t trigger_tstamp;	/* trigger timestamp */
          int overrange;
          snd_pcm_uframes_t avail_max;
          snd_pcm_uframes_t hw_ptr_base;	/* Position at buffer restart */
          snd_pcm_uframes_t hw_ptr_interrupt; /* Position at interrupt time*/

          /* -- HW params -- */
          snd_pcm_access_t access;	/* access mode */
          snd_pcm_format_t format;	/* SNDRV_PCM_FORMAT_* */
          snd_pcm_subformat_t subformat;	/* subformat */
          unsigned int rate;		/* rate in Hz */
          unsigned int channels;		/* channels */
          snd_pcm_uframes_t period_size;	/* period size */
          unsigned int periods;		/* periods */
          snd_pcm_uframes_t buffer_size;	/* buffer size */
          unsigned int tick_time;		/* tick time */
          snd_pcm_uframes_t min_align;	/* Min alignment for the format */
          size_t byte_align;
          unsigned int frame_bits;
          unsigned int sample_bits;
          unsigned int info;
          unsigned int rate_num;
          unsigned int rate_den;

          /* -- SW params -- */
          struct timespec tstamp_mode;	/* mmap timestamp is updated */
          unsigned int period_step;
          unsigned int sleep_min;		/* min ticks to sleep */
          snd_pcm_uframes_t start_threshold;
          /*
           * The following two thresholds alleviate playback buffer underruns; when
           * hw_avail drops below the threshold, the respective action is triggered:
           */
          snd_pcm_uframes_t stop_threshold;	/* - stop playback */
          snd_pcm_uframes_t silence_threshold;	/* - pre-fill buffer with silence */
          snd_pcm_uframes_t silence_size;       /* max size of silence pre-fill; when >= boundary,
                                                 * fill played area with silence immediately */
          snd_pcm_uframes_t boundary;	/* pointers wrap point */

          /* internal data of auto-silencer */
          snd_pcm_uframes_t silence_start; /* starting pointer to silence area */
          snd_pcm_uframes_t silence_filled; /* size filled with silence */

          snd_pcm_sync_id_t sync;		/* hardware synchronization ID */

          /* -- mmap -- */
          volatile struct snd_pcm_mmap_status *status;
          volatile struct snd_pcm_mmap_control *control;
          atomic_t mmap_count;

          /* -- locking / scheduling -- */
          spinlock_t lock;
          wait_queue_head_t sleep;
          struct timer_list tick_timer;
          struct fasync_struct *fasync;

          /* -- private section -- */
          void *private_data;
          void (*private_free)(struct snd_pcm_runtime *runtime);

          /* -- hardware description -- */
          struct snd_pcm_hardware hw;
          struct snd_pcm_hw_constraints hw_constraints;

          /* -- timer -- */
          unsigned int timer_resolution;	/* timer resolution */

          /* -- DMA -- */           
          unsigned char *dma_area;	/* DMA area */
          dma_addr_t dma_addr;		/* physical bus address (not accessible from main CPU) */
          size_t dma_bytes;		/* size of DMA area */

          struct snd_dma_buffer *dma_buffer_p;	/* allocated buffer */

  #if defined(CONFIG_SND_PCM_OSS) || defined(CONFIG_SND_PCM_OSS_MODULE)
          /* -- OSS things -- */
          struct snd_pcm_oss_runtime oss;
  #endif
  };


```

瀵逛簬姣忎釜澹伴煶椹卞姩鐨勮繍绠楃锛堝洖璋冿級锛岃繖浜涜褰曞ぇ澶氬簲鏄彧璇荤殑銆傚彧鏈?PCM 涓棿灞備細鏀瑰彉
/ 鏇存柊瀹冧滑銆備緥澶栨槸纭欢鎻忚堪锛坔w锛夈€丏MA 缂撳啿鍖轰俊鎭互鍙婄鏈夋暟鎹€傛澶栵紝濡傛灉浣犱娇鐢?
鏍囧噯鐨勬墭绠＄紦鍐插尯鍒嗛厤妯″紡锛屼綘涓嶉渶瑕佽嚜宸辫缃?DMA 缂撳啿鍖轰俊鎭€?

鍦ㄤ笅闈㈢殑灏忚妭涓紝灏嗚В閲婇噸瑕佺殑璁板綍銆?

#### 纭欢鎻忚堪


纭欢鎻忚堪绗︼紙struct snd_pcm_hardware锛夊寘鍚簡鍩烘湰纭欢閰嶇疆鐨勫畾涔夈€傛渶閲嶈鐨勬槸锛?
浣犻渶瑕佸湪 `PCM open callback`_ 涓畾涔夊畠銆傛敞鎰忥紝杩愯鏃跺疄渚嬩繚瀛樼殑鏄鎻忚堪绗︾殑
鍓湰锛岃€屼笉鏄寚鍚戠幇鏈夋弿杩扮鐨勬寚閽堛€備篃灏辨槸璇达紝鍦?open 鍥炶皟涓紝浣犲彲浠ユ牴鎹渶瑕佷慨鏀?
琚鍒剁殑鎻忚堪绗︼紙`runtime->hw`锛夈€備緥濡傦紝濡傛灉鏌愪簺鑺墖鍨嬪彿鐨勬渶澶ч€氶亾鏁板彧鏈?1锛?
浣犱粛鐒跺彲浠ヤ娇鐢ㄧ浉鍚岀殑

```

          struct snd_pcm_runtime *runtime = substream->runtime;
          ...
          runtime->hw = snd_mychip_playback_hw; /* common definition */
          if (chip->model == VERY_OLD_ONE)
                  runtime->hw.channels_max = 1;

```

```

  static struct snd_pcm_hardware snd_mychip_playback_hw = {
          .info = (SNDRV_PCM_INFO_MMAP |
                   SNDRV_PCM_INFO_INTERLEAVED |
                   SNDRV_PCM_INFO_BLOCK_TRANSFER |
                   SNDRV_PCM_INFO_MMAP_VALID),
          .formats =          SNDRV_PCM_FMTBIT_S16_LE,
          .rates =            SNDRV_PCM_RATE_8000_48000,
          .rate_min =         8000,
          .rate_max =         48000,
          .channels_min =     2,
          .channels_max =     2,
          .buffer_bytes_max = 32768,
          .period_bytes_min = 4096,
          .period_bytes_max = 32768,
          .periods_min =      1,
          .periods_max =      1024,
  };

```

- `info` 瀛楁鍖呭惈杩欎釜 PCM 鐨勭被鍨嬩笌鑳藉姏銆備綅鏍囧織鍦?`<sound/asound.h>` 涓?
   瀹氫箟涓?`SNDRV_PCM_INFO_XXX`銆傝繖閲屼綘鑷冲皯蹇呴』鎸囧畾鏄惁鏀寔 mmap 浠ュ強鏀寔
   鍝浜ら敊锛坕nterleaving锛夋牸寮忋€傚綋纭欢鏀寔 mmap 鏃讹紝鍦ㄨ繖閲屾坊鍔?
   `SNDRV_PCM_INFO_MMAP` 鏍囧織銆傚綋纭欢鏀寔浜ら敊鎴栭潪浜ら敊鏍煎紡鏃讹紝蹇呴』鍒嗗埆璁剧疆
   `SNDRV_PCM_INFO_INTERLEAVED` 鎴?`SNDRV_PCM_INFO_NONINTERLEAVED`
   鏍囧織銆傚鏋滀袱鑰呴兘鏀寔锛屼綘涔熷彲浠ュ悓鏃惰缃袱鑰呫€?

   鍦ㄤ笂闈㈢殑渚嬪瓙涓紝涓?OSS mmap 妯″紡鎸囧畾浜?`MMAP_VALID` 鍜?`BLOCK_TRANSFER`銆?
   閫氬父涓よ€呴兘浼氳缃€傚綋鐒讹紝`MMAP_VALID` 鍙湁鍦?mmap 鐪熸鍙楁敮鎸佹椂鎵嶈缃€?

   鍏朵粬鍙兘鐨勬爣蹇楁槸 `SNDRV_PCM_INFO_PAUSE` 鍜?`SNDRV_PCM_INFO_RESUME`銆?
   `PAUSE` 浣嶈〃绀?PCM 鏀寔鈥滄殏鍋溾€濇搷浣滐紝鑰?`RESUME` 浣嶈〃绀?PCM 鏀寔瀹屾暣鐨?
   鈥滄寕璧?鎭㈠锛坰uspend/resume锛夆€濇搷浣溿€傚鏋滆缃簡 `PAUSE` 鏍囧織锛屽垯涓嬮潰鐨?
   `trigger` 鍥炶皟蹇呴』澶勭悊鐩稿簲鐨勶紙鏆傚仠鎺ㄥ叆/閲婃斁锛夊懡浠ゃ€傚嵆浣挎病鏈?`RESUME`
   鏍囧織锛屼篃鍙互瀹氫箟鎸傝捣/鎭㈠瑙﹀彂鍛戒护銆傝瑙?`Power Management`_ 涓€鑺傘€?

   褰?PCM 瀛愭祦鍙互鍚屾鏃讹紙鍏稿瀷鎯呭喌鏄挱鏀炬祦鍜屾崟鑾锋祦鐨勫悓姝ュ惎鍔?鍋滄锛夛紝浣犱篃鍙互
   缁欏嚭 `SNDRV_PCM_INFO_SYNC_START`銆傚湪杩欑鎯呭喌涓嬶紝浣犻渶瑕佸湪 trigger 鍥炶皟涓?
   妫€鏌?PCM 瀛愭祦鐨勯摼琛ㄣ€傝繖灏嗗湪鍚庨潰鐨勪竴鑺備腑鎻忚堪銆?

- `formats` 瀛楁鍖呭惈鍙楁敮鎸佹牸寮忕殑浣嶆爣蹇楋紙`SNDRV_PCM_FMTBIT_XXX`锛夈€傚鏋滅‖浠?
   鏀寔澶氱鏍煎紡锛岃缁欏嚭鎵€鏈夋寜浣嶆垨锛坥r锛夊悗鐨勪綅銆傚湪涓婇潰鐨勪緥瀛愪腑锛屾寚瀹氫簡鏈夌鍙?
   16 浣嶅皬绔紙little-endian锛夋牸寮忋€?

- `rates` 瀛楁鍖呭惈鍙楁敮鎸侀€熺巼鐨勪綅鏍囧織锛坄SNDRV_PCM_RATE_XXX`锛夈€傚綋鑺墖鏀寔
   杩炵画閫熺巼鏃讹紝棰濆浼犲叆 `CONTINUOUS` 浣嶃€傞瀹氫箟鐨勯€熺巼浣嶄粎閽堝鍏稿瀷閫熺巼鎻愪緵銆?
   濡傛灉浣犵殑鑺墖鏀寔闈炴爣鍑嗙殑閫熺巼锛屼綘闇€瑕佹坊鍔?`KNOT` 浣嶅苟鎵嬪姩璁剧疆纭欢绾︽潫
   锛堝悗鏂囪В閲婏級銆?

- `rate_min` 鍜?`rate_max` 瀹氫箟鏈€灏忓拰鏈€澶ч噰鏍风巼銆傚畠搴斿綋鍦ㄦ煇绉嶇▼搴﹀搴斾簬
   `rates` 浣嶃€?

- `channels_min` 鍜?`channels_max` 瀹氫箟浜嗕綘鍙兘宸茬粡棰勬枡鍒扮殑銆侀€氶亾鐨勬渶灏忓拰
   鏈€澶ф暟閲忋€?

- `buffer_bytes_max` 瀹氫箟缂撳啿鍖虹殑鏈€澶уぇ灏忥紙浠ュ瓧鑺傝锛夈€傛病鏈?
   `buffer_bytes_min` 瀛楁锛屽洜涓哄畠鍙互浠庢渶灏忓懆鏈熷ぇ灏忓拰鏈€灏忔椂鏈熸暟璁＄畻鍑烘潵銆?
   鍚屾椂锛宍period_bytes_min` 鍜?`period_bytes_max` 瀹氫箟浜嗗懆鏈燂紙period锛夌殑
   鏈€灏忓拰鏈€澶уぇ灏忥紙浠ュ瓧鑺傝锛夈€俙periods_max` 鍜?`periods_min` 瀹氫箟浜嗙紦鍐插尯涓?
   鍛ㄦ湡鐨勬渶澶у拰鏈€灏忔暟閲忋€?

   鈥滃懆鏈燂紙period锛夆€濊繖涓瘝瀵瑰簲浜?OSS 涓栫晫涓殑纰庣墖锛坒ragment锛夈€傚懆鏈熷畾涔変簡鐢熸垚
   PCM 涓柇鐨勭偣銆傝繖涓偣寮虹儓渚濊禆浜庣‖浠躲€備竴鑸潵璇达紝杈冨皬鐨勫懆鏈熷ぇ灏忎細缁欎綘鏇村鐨?
   涓柇锛屼粠鑰岃兘澶熷強鏃跺湴濉厖/鎺掔┖缂撳啿鍖恒€傚湪鎹曡幏鐨勬儏鍐典笅锛岃繖涓ぇ灏忓畾涔変簡杈撳叆
   寤惰繜銆傚彟涓€鏂归潰锛屾暣涓紦鍐插尯澶у皬瀹氫箟浜嗘挱鏀炬柟鍚戠殑杈撳嚭寤惰繜銆?

- 杩樻湁涓€涓?`fifo_size` 瀛楁銆傚畠鎸囧畾纭欢 FIFO 鐨勫ぇ灏忥紝浣嗙洰鍓嶅畠鏃笉琚┍鍔?
   浣跨敤锛屼篃涓嶅湪 alsa-lib 涓娇鐢ㄣ€傚洜姝わ紝浣犲彲浠ュ拷鐣ヨ繖涓瓧娈点€?

#### PCM 閰嶇疆


濂斤紝璁╂垜浠啀娆″洖鍒?PCM 杩愯鏃惰褰曘€傝繍琛屾椂瀹炰緥涓渶甯歌寮曠敤鐨勮褰曟槸 PCM 閰嶇疆銆?
PCM 閰嶇疆鏄湪搴旂敤绋嬪簭閫氳繃 alsa-lib 鍙戦€?`hw_params` 鏁版嵁涔嬪悗锛屽瓨鍌ㄥ湪杩愯鏃?
瀹炰緥涓殑銆傛湁璁稿瀛楁鏄粠 hw_params 鍜?sw_params 缁撴瀯澶嶅埗杩囨潵鐨勩€備緥濡傦紝
`format` 淇濆瓨鐫€搴旂敤绋嬪簭閫夋嫨鐨勬牸寮忕被鍨嬨€傝瀛楁鍖呭惈鏋氫妇鍊?
`SNDRV_PCM_FORMAT_XXX`銆?

闇€瑕佹敞鎰忕殑涓€鐐规槸锛岄厤缃ソ鐨勭紦鍐插尯鍜屽懆鏈熷ぇ灏忓湪杩愯鏃朵腑浠モ€滃抚锛坒rames锛夆€濆瓨鍌ㄣ€傚湪
ALSA 涓栫晫涓紝``1 甯?= 閫氶亾鏁?脳 鏍锋湰澶у皬``銆備负浜嗗湪甯у拰瀛楄妭涔嬮棿杞崲锛屼綘鍙互浣跨敤
`frames_to_bytes()` 浠ュ強

```

  period_bytes = frames_to_bytes(runtime, runtime->period_size);

```

姝ゅ锛岃澶氳蒋浠跺弬鏁帮紙sw_params锛変篃浠ュ抚瀛樺偍銆傝妫€鏌ュ瓧娈电殑绫诲瀷銆?
`snd_pcm_uframes_t` 鐢ㄤ簬鏃犵鍙锋暣鏁板舰寮忕殑甯э紝鑰?`snd_pcm_sframes_t` 鐢ㄤ簬
鏈夌鍙锋暣鏁板舰寮忕殑甯с€?

#### DMA 缂撳啿鍖轰俊鎭?


DMA 缂撳啿鍖虹敱浠ヤ笅鍥涗釜瀛楁瀹氫箟锛歚dma_area`銆乣dma_addr`銆乣dma_bytes` 鍜?
`dma_private`銆俙dma_area` 淇濆瓨缂撳啿鍖烘寚閽堬紙閫昏緫鍦板潃锛夈€備綘鍙互瀵硅繖涓寚閽堣皟鐢?
`memcpy()`銆傚悓鏃讹紝`dma_addr` 淇濆瓨缂撳啿鍖虹殑鐗╃悊鍦板潃銆傝瀛楁浠呭湪缂撳啿鍖烘槸绾挎€?
缂撳啿鍖烘椂鎵嶆寚瀹氥€俙dma_bytes` 淇濆瓨缂撳啿鍖虹殑澶у皬锛堜互瀛楄妭璁★級銆俙dma_private` 鐢ㄤ簬
ALSA DMA 鍒嗛厤鍣ㄣ€?

濡傛灉浣犱娇鐢ㄦ墭绠＄紦鍐插尯鍒嗛厤妯″紡鎴栨爣鍑?API 鍑芥暟 `snd_pcm_lib_malloc_pages()`
鏉ュ垎閰嶇紦鍐插尯锛岃繖浜涘瓧娈电敱 ALSA 涓棿灞傝缃紝浣?*涓嶅簲**鑷繁淇敼瀹冧滑銆備綘鍙互璇诲彇
瀹冧滑浣嗕笉鑳藉啓鍏ュ畠浠€傚彟涓€鏂归潰锛屽鏋滀綘鎯宠嚜宸卞垎閰嶇紦鍐插尯锛屼綘闇€瑕佸湪 hw_params 鍥炶皟涓?
绠＄悊瀹冦€傝嚦灏戯紝`dma_bytes` 鏄繀闇€鐨勩€俙dma_area` 鍦ㄧ紦鍐插尯琚?mmap 鏃舵槸蹇呴渶鐨勩€?
濡傛灉浣犵殑椹卞姩涓嶆敮鎸?mmap锛岃繖涓瓧娈靛氨涓嶆槸蹇呴渶鐨勩€俙dma_addr` 涔熸槸鍙€夌殑銆備綘涔熷彲浠?
闅忔剰浣跨敤 dma_private銆?

#### 杩愯鐘舵€?


杩愯鐘舵€佸彲浠ラ€氳繃 `runtime->status` 寮曠敤銆傝繖鏄竴涓寚鍚?struct
snd_pcm_mmap_status 璁板綍鐨勬寚閽堛€備緥濡傦紝浣犲彲浠ラ€氳繃 `runtime->status->hw_ptr`
鑾峰彇褰撳墠鐨?DMA 纭欢鎸囬拡銆?

DMA 搴旂敤鎸囬拡鍙互閫氳繃 `runtime->control` 寮曠敤锛屽畠鎸囧悜涓€涓?struct
snd_pcm_mmap_control 璁板綍銆備絾鏄紝涓嶅缓璁洿鎺ヨ闂繖涓€笺€?

#### 绉佹湁鏁版嵁


浣犲彲浠ヤ负瀛愭祦鍒嗛厤涓€涓褰曞苟灏嗗叾瀛樺偍鍦?`runtime->private_data` 涓€傞€氬父锛岃繖
鏄湪 `PCM open callback`_ 涓畬鎴愮殑銆備笉瑕佸皢瀹冧笌 `pcm->private_data` 娣锋穯銆?
`pcm->private_data` 閫氬父鎸囧悜鍦?PCM 璁惧鍒涘缓鏃堕潤鎬佸垎閰嶇殑鑺墖瀹炰緥锛岃€?
`runtime->private_data` 鎸囧悜鍦?PCM open 鍥炶皟涓垱寤虹殑鍔ㄦ€佹暟鎹粨鏋勶細

```

  static int snd_xxx_open(struct snd_pcm_substream *substream)
  {
          struct my_pcm_data *data;
          ....
          data = kmalloc(sizeof(*data), GFP_KERNEL);
          substream->runtime->private_data = data;
          ....
  }


```

鎵€鍒嗛厤鐨勫璞″繀椤诲湪 `close callback`_ 涓閲婃斁銆?

### 杩愮畻绗?


濂斤紝鐜板湪璁╂垜缁欏嚭姣忎釜 PCM 鍥炶皟锛坄ops`锛夌殑缁嗚妭銆備竴鑸潵璇达紝姣忎釜鍥炶皟鍦ㄦ垚鍔熸椂蹇呴』
杩斿洖 0锛屾垨鑰呰繑鍥炰竴涓礋鐨勯敊璇彿锛屼緥濡?`-EINVAL`銆傝閫夋嫨鍚堥€傜殑閿欒鍙凤紝寤鸿妫€鏌?
褰撳悓涓€绫昏姹傚け璐ユ椂鍐呮牳鍏朵粬閮ㄥ垎杩斿洖浠€涔堝€笺€?

姣忎釜鍥炶皟鍑芥暟鑷冲皯鎺ュ彈涓€涓寘鍚?struct snd_pcm_substream 鎸囬拡鐨勫弬鏁般€傝浠庣粰瀹氱殑
瀛愭祦瀹炰緥涓彇鍥炶姱鐗囪褰曪紝浣犲彲浠ヤ娇鐢ㄤ互涓嬫柟娉曪細

```

  int xxx(...) {
          struct mychip *chip = snd_pcm_substream_chip(substream);
          ....
  }

```

杩欎釜瀹忚鍙?`substream->private_data`锛屽畠鏄?`pcm->private_data` 鐨勫壇鏈€?
濡傛灉闇€瑕佷负姣忎釜 PCM 瀛愭祦鍒嗛厤涓嶅悓鐨勬暟鎹褰曪紝浣犲彲浠ヨ鐩栧墠鑰呫€備緥濡傦紝cmi8330 椹卞姩
涓烘挱鏀惧拰鎹曡幏鏂瑰悜鍒嗛厤浜嗕笉鍚岀殑 `private_data`锛屽洜涓哄畠瀵逛笉鍚岀殑鏂瑰悜浣跨敤涓や釜涓嶅悓鐨?
缂栬В鐮佸櫒锛圫B 鍏煎鍜?AD 鍏煎锛夈€?

#### PCM open 鍥炶皟


```

  static int snd_xxx_open(struct snd_pcm_substream *substream);

```

褰?PCM 瀛愭祦琚墦寮€鏃惰皟鐢ㄣ€?

鑷冲皯锛屽湪杩欓噷浣犲繀椤诲垵濮嬪寲 `runtime->hw`锛?

```

  static int snd_xxx_open(struct snd_pcm_substream *substream)
  {
          struct mychip *chip = snd_pcm_substream_chip(substream);
          struct snd_pcm_runtime *runtime = substream->runtime;

          runtime->hw = snd_mychip_playback_hw;
          return 0;
  }

```

鍏朵腑 `snd_mychip_playback_hw` 鏄瀹氫箟鐨勭‖浠舵弿杩般€?

浣犲彲浠ュ湪杩欎釜鍥炶皟涓垎閰嶇鏈夋暟鎹紝濡?`Private Data`_ 涓€鑺傛墍杩般€?

濡傛灉纭欢閰嶇疆闇€瑕佹洿澶氱害鏉燂紝涔熻鍦ㄨ繖閲岃缃‖浠剁害鏉熴€傝瑙?Constraints_銆?

#### close 鍥炶皟


```

  static int snd_xxx_close(struct snd_pcm_substream *substream);


```

鏄剧劧锛屽綋 PCM 瀛愭祦琚叧闂椂璋冪敤銆?

浠讳綍鍦?`open` 鍥炶皟涓负 PCM 瀛愭祦鍒嗛厤鐨勭鏈夊疄渚嬪繀椤诲湪杩欓噷閲婃斁锛?

```

  static int snd_xxx_close(struct snd_pcm_substream *substream)
  {
          ....
          kfree(substream->runtime->private_data);
          ....
  }

```

#### ioctl 鍥炶皟


杩欑敤浜庝换浣曞 PCM ioctl 鐨勭壒娈婅皟鐢ㄣ€備絾閫氬父浣犲彲浠ュ皢鍏朵繚鐣欎负 NULL锛岀劧鍚?PCM 鏍稿績
浼氳皟鐢ㄩ€氱敤 ioctl 鍥炶皟鍑芥暟 `snd_pcm_lib_ioctl()`銆傚鏋滀綘闇€瑕佸鐞嗛€氶亾淇℃伅鎴?
閲嶇疆杩囩▼鐨勭嫭鐗硅缃紝鍙互鍦ㄨ繖閲屼紶鍏ヤ綘鑷繁鐨勫洖璋冨嚱鏁般€?

#### hw_params 鍥炶皟


```

  static int snd_xxx_hw_params(struct snd_pcm_substream *substream,
                               struct snd_pcm_hw_params *hw_params);

```

褰撳簲鐢ㄧ▼搴忚缃簡纭欢鍙傛暟锛坄hw_params`锛夋椂璋冪敤锛屼篃灏辨槸褰撶紦鍐插尯澶у皬銆佸懆鏈熷ぇ灏忋€?
鏍煎紡绛夎涓?PCM 瀛愭祦瀹氫箟濂芥椂璋冪敤涓€娆°€?

璁稿纭欢璁剧疆搴旇鍦ㄨ繖涓洖璋冧腑瀹屾垚锛屽寘鎷紦鍐插尯鐨勫垎閰嶃€?

瑕佸垵濮嬪寲鐨勫弬鏁伴€氳繃 `params_xxx()` 瀹忚幏鍙栥€?

褰撲綘涓哄瓙娴侀€夋嫨鎵樼缂撳啿鍖哄垎閰嶆ā寮忔椂锛屽湪璇ュ洖璋冭璋冪敤涔嬪墠缂撳啿鍖哄氨宸茬粡琚垎閰嶅ソ浜嗐€?
鎴栬€咃紝浣犲彲浠ヨ皟鐢ㄤ笅闈㈢殑杈呭姪鍑芥暟锛?

```

  snd_pcm_lib_malloc_pages(substream, params_buffer_bytes(hw_params));

```

`snd_pcm_lib_malloc_pages()` 鍙湁鍦?DMA 缂撳啿鍖哄凡琚鍒嗛厤鏃舵墠鍙敤銆傝瑙?
`Buffer Types`_ 涓€鑺傘€?

娉ㄦ剰锛岃繖涓洖璋冨拰 `prepare` 鍥炶皟鍙兘鍦ㄦ瘡娆″垵濮嬪寲鏃惰澶氭璋冪敤銆備緥濡傦紝OSS 妯℃嫙
鍙兘鍦ㄥ叾 ioctl 鐨勬瘡娆″彉鏇翠腑璋冪敤杩欎簺鍥炶皟銆?

鍥犳锛屼綘闇€瑕佹敞鎰忎笉瑕佸娆″垎閰嶇浉鍚岀殑缂撳啿鍖猴紝閭ｄ細瀵艰嚧鍐呭瓨娉勬紡锛佸娆¤皟鐢ㄤ笂闈㈢殑杈呭姪
鍑芥暟鏄病闂鐨勩€傚鏋滅紦鍐插尯涔嬪墠宸茬粡鍒嗛厤杩囷紝瀹冧細鑷姩閲婃斁涔嬪墠鐨勭紦鍐插尯銆?

鍙︿竴涓敞鎰忕偣鏄紝榛樿鎯呭喌涓嬭繖涓洖璋冩槸闈炲師瀛愮殑锛堝彲璋冨害锛夛紝鍗冲綋娌℃湁璁剧疆 `nonatomic`
鏍囧織鏃躲€傝繖寰堥噸瑕侊紝鍥犱负 `trigger` 鍥炶皟鏄師瀛愮殑锛堜笉鍙皟搴︼級銆備篃灏辨槸璇达紝鍦?
`trigger` 鍥炶皟涓笉鑳戒娇鐢ㄤ簰鏂ヤ綋鎴栦换浣曚笌璋冨害鐩稿叧鐨勫嚱鏁般€傝瑙?Atomicity_ 灏忚妭銆?

#### hw_free 鍥炶皟


```

  static int snd_xxx_hw_free(struct snd_pcm_substream *substream);

```

杩欑敤浜庨噴鏀鹃€氳繃 `hw_params` 鍒嗛厤鐨勮祫婧愩€?

杩欎釜鍑芥暟鎬绘槸鍦?close 鍥炶皟鍑芥暟琚皟鐢ㄤ箣鍓嶈璋冪敤銆傛澶栵紝璇ュ洖璋冧篃鍙兘琚娆¤皟鐢ㄣ€?
璇疯窡韪瘡涓祫婧愭槸鍚﹀凡缁忚閲婃斁銆?

褰撲綘涓?PCM 瀛愭祦閫夋嫨浜嗘墭绠＄紦鍐插尯鍒嗛厤妯″紡鏃讹紝鍒嗛厤鐨?PCM 缂撳啿鍖哄皢鍦ㄨ鍥炶皟琚皟鐢ㄥ悗
鑷姩閲婃斁銆傚惁鍒欎綘灏嗕笉寰椾笉鎵嬪姩閲婃斁缂撳啿鍖恒€傚吀鍨嬬殑鍋氭硶鏄紝褰撶紦鍐插尯鏄粠棰勫垎閰嶆睜涓?
鍒嗛厤鏃讹紝浣犲彲浠ヤ娇鐢ㄦ爣鍑?API 鍑芥暟

```

  snd_pcm_lib_free_pages(substream);

```

#### prepare 鍥炶皟


```

  static int snd_xxx_prepare(struct snd_pcm_substream *substream);

```

褰?PCM 鈥滃噯澶囧ソ锛坧repared锛夆€濇椂璋冪敤杩欎釜鍥炶皟銆備綘鍙互鍦ㄨ繖閲岃缃牸寮忕被鍨嬨€侀噰鏍风巼绛夈€?
涓?`hw_params` 鐨勫尯鍒湪浜庯紝`prepare` 鍥炶皟浼氬湪姣忔璋冪敤 `snd_pcm_prepare()`
鏃惰璋冪敤锛屽嵆鍦ㄦ瑺杞斤紙underrun锛夌瓑鎭㈠涔嬪悗銆?

娉ㄦ剰杩欎釜鍥炶皟鏄潪鍘熷瓙鐨勩€備綘鍙互鍦ㄨ繖涓洖璋冧腑瀹夊叏鍦颁娇鐢ㄤ笌璋冨害鐩稿叧鐨勫嚱鏁般€?

鍦ㄨ繖涓互鍙婂悗缁洖璋冧腑锛屼綘鍙互閫氳繃杩愯鏃惰褰?`substream->runtime` 寮曠敤鍊笺€?
渚嬪锛岃鑾峰彇褰撳墠鐨勯€熺巼銆佹牸寮忔垨閫氶亾锛屽垎鍒闂?`runtime->rate`銆?
`runtime->format` 鎴?`runtime->channels`銆傚凡鍒嗛厤缂撳啿鍖虹殑鐗╃悊鍦板潃琚缃负
`runtime->dma_area`銆傜紦鍐插尯鍜屽懆鏈熷ぇ灏忓垎鍒湪 `runtime->buffer_size` 鍜?
`runtime->period_size` 涓€?

娉ㄦ剰杩欎釜鍥炶皟鍦ㄦ瘡娆¤缃椂涔熶細琚皟鐢ㄥ娆°€?

#### trigger 鍥炶皟


```

  static int snd_xxx_trigger(struct snd_pcm_substream *substream, int cmd);

```

褰?PCM 琚惎鍔ㄣ€佸仠姝㈡垨鏆傚仠鏃惰皟鐢ㄣ€?

鍔ㄤ綔鍦ㄧ浜屼釜鍙傛暟涓寚瀹氾紝鍗?`<sound/pcm.h>` 涓畾涔夌殑
`SNDRV_PCM_TRIGGER_XXX`銆傝嚦灏戯紝`START`

```

  switch (cmd) {
  case SNDRV_PCM_TRIGGER_START:
          /* do something to start the PCM engine */
          break;
  case SNDRV_PCM_TRIGGER_STOP:
          /* do something to stop the PCM engine */
          break;
  default:
          return -EINVAL;
  }

```

濡傛灉 PCM 鏀寔鏆傚仠鎿嶄綔锛堝湪纭欢琛ㄧ殑 info 瀛楁涓粰鍑猴級锛屽垯 `PAUSE_PUSH` 鍜?
`PAUSE_RELEASE` 鍛戒护涔熷繀椤诲湪杩欓噷澶勭悊銆傚墠鑰呮槸鏆傚仠 PCM 鐨勫懡浠わ紝鍚庤€呮槸閲嶆柊鍚姩
PCM 鐨勫懡浠ゃ€?

褰?PCM 鏀寔鎸傝捣/鎭㈠鎿嶄綔鏃讹紝鏃犺鏄惁鏀寔瀹屾暣鎴栭儴鍒嗙殑鎸傝捣/鎭㈠锛岄兘蹇呴』澶勭悊
`SUSPEND` 鍜?`RESUME` 鍛戒护銆傝繖浜涘懡浠ゅ湪鐢垫簮绠＄悊鐘舵€佹敼鍙樻椂鍙戝嚭銆傛樉鐒讹紝
`SUSPEND` 鍜?`RESUME` 鍛戒护鍒嗗埆鎸傝捣鍜屾仮澶?PCM 瀛愭祦锛岄€氬父瀹冧滑鍒嗗埆绛夊悓浜?
`STOP` 鍜?`START` 鍛戒护銆傝瑙?`Power Management`_ 涓€鑺傘€?

濡傚墠鎵€杩帮紝闄ら潪璁剧疆浜?`nonatomic` 鏍囧織锛屽惁鍒欒繖涓洖璋冮粯璁ゆ槸鍘熷瓙鐨勶紝浣犱笉鑳借皟鐢?
鍙兘浼戠湢鐨勫嚱鏁般€俙trigger` 鍥炶皟搴斿綋灏藉彲鑳界簿绠€锛屼粎浠呯湡姝ｈЕ鍙?DMA銆傚叾浠栭儴鍒嗗簲褰?
棰勫厛鍦?`hw_params` 鍜?`prepare` 鍥炶皟涓纭垵濮嬪寲銆?

#### sync_stop 鍥炶皟


```

  static int snd_xxx_sync_stop(struct snd_pcm_substream *substream);

```

杩欎釜鍥炶皟鏄彲閫夌殑锛屽彲浠ヤ紶鍏?NULL銆傚畠鍦?PCM 鏍稿績鍋滄娴佷箣鍚庛€佸湪瀹冮€氳繃 `prepare`銆?
`hw_params` 鎴?`hw_free` 鏀瑰彉娴佺姸鎬佷箣鍓嶈璋冪敤銆傜敱浜?IRQ 澶勭悊鍑芥暟鍙兘浠嶅湪
鎸傝捣锛屾垜浠渶瑕佺瓑寰呮寕璧风殑浠诲姟瀹屾垚锛岀劧鍚庡啀杩涘叆涓嬩竴姝ワ紱鍚﹀垯鍙兘浼氱敱浜庤祫婧愬啿绐佹垨
璁块棶宸查噴鏀捐祫婧愯€屽鑷村穿婧冦€傚吀鍨嬬殑琛屼负鏄湪杩欓噷璋冪敤鍍?`synchronize_irq()` 杩欐牱
鐨勫悓姝ュ嚱鏁般€?

瀵逛簬鍙渶瑕佽皟鐢?`synchronize_irq()` 鐨勫ぇ澶氭暟椹卞姩锛屼篃鏈変竴涓洿绠€鍗曠殑璁剧疆銆傚湪淇濇寔
`sync_stop` PCM 鍥炶皟涓?NULL 鐨勫悓鏃讹紝椹卞姩鍙互鍦ㄨ姹?IRQ 涔嬪悗灏?`card->sync_irq`
瀛楁璁剧疆涓鸿繑鍥炵殑涓柇鍙枫€傜劧鍚?PCM 鏍稿績浼氱敤缁欏畾鐨?IRQ 閫傚綋鍦拌皟鐢?
`synchronize_irq()`銆?

濡傛灉 IRQ 澶勭悊鍑芥暟鐢卞０鍗℃瀽鏋勫嚱鏁伴噴鏀撅紝浣犱笉闇€瑕佹竻闄?`card->sync_irq`锛屽洜涓哄０鍗?
鏈韩姝ｅ湪琚噴鏀俱€傛墍浠ワ紝閫氬父浣犲彧闇€瑕佸湪椹卞姩浠ｇ爜涓坊鍔犱竴琛屾潵璧嬪€?`card->sync_irq`锛?
闄ら潪椹卞姩閲嶆柊鑾峰彇 IRQ銆傚綋椹卞姩鍔ㄦ€侀噴鏀惧苟閲嶆柊鑾峰彇 IRQ锛堜緥濡傚湪鎸傝捣/鎭㈠鏃讹級锛屽畠闇€瑕?
鍐嶆閫傚綋鍦版竻闄ゅ苟閲嶆柊璁剧疆 `card->sync_irq`銆?

#### pointer 鍥炶皟


```

  static snd_pcm_uframes_t snd_xxx_pointer(struct snd_pcm_substream *substream)

```

褰?PCM 涓棿灞傛煡璇㈢紦鍐插尯涓殑褰撳墠纭欢浣嶇疆鏃惰皟鐢ㄨ繖涓洖璋冦€備綅缃繀椤讳互甯х殑褰㈠紡杩斿洖锛?
鑼冨洿浠?0 鍒?`buffer_size - 1`銆?

杩欓€氬父浠?PCM 涓棿灞傜殑缂撳啿鍖烘洿鏂颁緥绋嬩腑璋冪敤锛岃渚嬬▼鍦ㄤ腑鏂緥绋嬭皟鐢?
`snd_pcm_period_elapsed()` 鏃惰璋冪敤銆傜劧鍚?PCM 涓棿灞傛洿鏂颁綅缃苟璁＄畻鍙敤绌洪棿锛?
骞跺敜閱掔潯鐪犵殑 poll 绾跨▼绛夈€?

榛樿鎯呭喌涓嬭繖涓洖璋冧篃鏄師瀛愮殑銆?

#### copy 涓?fill_silence 杩愮畻绗?


杩欎簺鍥炶皟涓嶆槸寮哄埗鐨勶紝鍦ㄥぇ澶氭暟鎯呭喌涓嬪彲浠ョ渷鐣ャ€傚綋纭欢缂撳啿鍖轰笉鍦ㄦ甯哥殑瀛樺偍绌洪棿鏃讹紝
浣跨敤杩欎簺鍥炶皟銆傛煇浜涜姱鐗囧湪纭欢涓湁鑷繁鐨勭紦鍐插尯锛岃缂撳啿鍖轰笉鍙槧灏勩€傚湪杩欑鎯呭喌涓嬶紝浣?
蹇呴』鎵嬪姩灏嗘暟鎹粠鍐呭瓨缂撳啿鍖轰紶杈撳埌纭欢缂撳啿鍖恒€傛垨鑰咃紝濡傛灉缂撳啿鍖哄湪鐗╃悊鍜岃櫄鎷熷唴瀛樼┖闂?
涓婇兘鏄潪杩炵画鐨勶紝涔熷繀椤诲畾涔夎繖浜涘洖璋冦€?

濡傛灉瀹氫箟浜嗚繖涓や釜鍥炶皟锛屽鍒跺拰濉厖闈欓煶锛坰et-silence锛夋搷浣滅敱瀹冧滑瀹屾垚銆傜粏鑺傚皢鍦ㄥ悗鏂?
鐨?`Buffer and Memory Management`_ 涓€鑺備腑鎻忚堪銆?

#### ack 鍥炶皟


杩欎釜鍥炶皟涔熶笉鏄己鍒剁殑銆傚綋鍦ㄨ鍙栨垨鍐欏叆鎿嶄綔涓?`appl_ptr` 琚洿鏂版椂璋冪敤杩欎釜鍥炶皟銆?
鏌愪簺椹卞姩濡?emu10k1-fx 鍜?cs46xx 闇€瑕佷负鍐呴儴缂撳啿鍖鸿窡韪綋鍓嶇殑 `appl_ptr`锛岃繖涓?
鍥炶皟浠呭姝ょ被鐢ㄩ€旀湁鐢ㄣ€?

鍥炶皟鍑芥暟鍙互杩斿洖 0 鎴栬礋鐨勯敊璇€傚綋杩斿洖鍊间负 `-EPIPE` 鏃讹紝PCM 鏍稿績灏嗗叾瑙嗕负缂撳啿鍖?
XRUN锛屽苟鑷姩灏嗙姸鎬佹洿鏀逛负 `SNDRV_PCM_STATE_XRUN`銆?

榛樿鎯呭喌涓嬭繖涓洖璋冩槸鍘熷瓙鐨勩€?

#### page 鍥炶皟


杩欎釜鍥炶皟涔熸槸鍙€夌殑銆俶map 璋冪敤杩欎釜鍥炶皟鏉ヨ幏鍙栫己椤靛湴鍧€銆?

瀵逛簬鏍囧噯鐨?SG 缂撳啿鍖烘垨 vmalloc 缂撳啿鍖猴紝浣犱笉闇€瑕佺壒娈婄殑鍥炶皟銆傚洜姝よ繖涓洖璋冨簲璇ュ緢灏?
浣跨敤銆?

#### mmap 鍥炶皟


杩欐槸鍙︿竴涓敤浜庢帶鍒?mmap 琛屼负鐨勫彲閫夊洖璋冦€傚綋瀹氫箟浜嗗畠鏃讹紝PCM 鏍稿績鍦ㄥ唴瀛樿鏄犲皠鏃朵細
璋冪敤杩欎釜鍥炶皟锛岃€屼笉鏄娇鐢ㄦ爣鍑嗚緟鍔╁嚱鏁般€傚鏋滀綘闇€瑕佺壒娈婂鐞嗭紙鐢变簬鏌愪簺鏋舵瀯鎴栬澶?
鐗瑰畾鐨勯棶棰橈級锛屽彲浠ュ儚浣犲枩娆㈢殑閭ｆ牱鍦ㄨ繖閲屽疄鐜版墍鏈夊唴瀹广€?


### PCM 涓柇澶勭悊鍑芥暟


PCM 鍓╀綑鐨勯儴鍒嗘槸 PCM 涓柇澶勭悊鍑芥暟銆傚０闊抽┍鍔ㄤ腑 PCM 涓柇澶勭悊鍑芥暟鐨勮鑹叉槸鏇存柊
缂撳啿鍖轰綅缃紝骞跺湪缂撳啿鍖轰綅缃秺杩囨寚瀹氱殑鍛ㄦ湡杈圭晫鏃堕€氱煡 PCM 涓棿灞傘€備负姝わ紝璋冪敤
`snd_pcm_period_elapsed()` 鍑芥暟銆?

澹伴煶鑺墖鐢熸垚涓柇鐨勬柟寮忔湁鍑犵銆?

#### 鍦ㄥ懆鏈燂紙纰庣墖锛夎竟鐣屽鐨勪腑鏂?


杩欐槸鏈€甯歌鐨勭被鍨嬶細纭欢鍦ㄦ瘡涓懆鏈熻竟鐣屽鐢熸垚涓€涓腑鏂€傚湪杩欑鎯呭喌涓嬶紝浣犲彲浠ュ湪姣忔
涓柇鏃惰皟鐢?`snd_pcm_period_elapsed()`銆?

`snd_pcm_period_elapsed()` 浠ュ瓙娴佹寚閽堜綔涓哄叾鍙傛暟銆傚洜姝や綘闇€瑕佷繚鎸佸瓙娴佹寚閽堝彲浠?
鑺墖瀹炰緥璁块棶銆備緥濡傦紝鍦ㄨ姱鐗囪褰曚腑瀹氫箟 `substream` 瀛楁鏉ヤ繚瀛樺綋鍓嶈繍琛岀殑瀛愭祦鎸囬拡锛?
骞跺湪 `open` 鍥炶皟涓缃鎸囬拡鍊硷紙鍦?`close` 鍥炶皟涓噸缃級銆?

濡傛灉浣犲湪涓柇澶勭悊鍑芥暟涓幏鍙栦簡鑷棆閿侊紝骞朵笖璇ラ攣涔熷湪鍏朵粬 PCM 鍥炶皟涓娇鐢紝閭ｄ箞浣犲繀椤诲湪
璋冪敤 `snd_pcm_period_elapsed()` 涔嬪墠閲婃斁璇ラ攣锛屽洜涓?`snd_pcm_period_elapsed()`
浼氬湪鍐呴儴璋冪敤鍏朵粬 PCM 鍥炶皟銆?

```


      static irqreturn_t snd_mychip_interrupt(int irq, void *dev_id)
      {
              struct mychip *chip = dev_id;
              spin_lock(&chip->lock);
              ....
              if (pcm_irq_invoked(chip)) {
                      /* call updater, unlock before it */
                      spin_unlock(&chip->lock);
                      snd_pcm_period_elapsed(chip->substream);
                      spin_lock(&chip->lock);
                      /* acknowledge the interrupt if necessary */
              }
              ....
              spin_unlock(&chip->lock);
              return IRQ_HANDLED;
      }

```

姝ゅ锛屽綋璁惧鍙互妫€娴嬪埌缂撳啿鍖烘瑺杞?婧㈠嚭锛坲nderrun/overrun锛夋椂锛岄┍鍔ㄥ彲浠ラ€氳繃璋冪敤
`snd_pcm_stop_xrun()` 灏?XRUN 鐘舵€侀€氱煡缁?PCM 鏍稿績銆傝繖涓嚱鏁板仠姝㈡祦骞跺皢 PCM 鐘舵€?
璁剧疆涓?`SNDRV_PCM_STATE_XRUN`銆傛敞鎰忓畠蹇呴』鍦?PCM 娴侀攣涔嬪璋冪敤锛屽洜姝ゆ棤娉曚粠鍘熷瓙
鍥炶皟涓皟鐢ㄣ€?


#### 楂橀瀹氭椂鍣ㄤ腑鏂?


褰撶‖浠朵笉鍦ㄥ懆鏈熻竟鐣屽鐢熸垚涓柇锛岃€屾槸浠ュ浐瀹氱殑瀹氭椂鍣ㄩ€熺巼鍙戝嚭瀹氭椂鍣ㄤ腑鏂椂锛堜緥濡?es1968
鎴?ymfpci 椹卞姩锛夛紝浼氬彂鐢熻繖绉嶆儏鍐点€傚湪杩欑鎯呭喌涓嬶紝浣犻渶瑕佹鏌ュ綋鍓嶇殑纭欢浣嶇疆骞跺湪姣忔
涓柇鏃剁疮鍔犲凡澶勭悊鐨勬牱鏈暱搴︺€傚綋绱姞鐨勫ぇ灏忚秴杩囧懆鏈熷ぇ灏忔椂锛岃皟鐢?
`snd_pcm_period_elapsed()` 骞堕噸缃疮鍔犲櫒銆?

```


      static irqreturn_t snd_mychip_interrupt(int irq, void *dev_id)
      {
              struct mychip *chip = dev_id;
              spin_lock(&chip->lock);
              ....
              if (pcm_irq_invoked(chip)) {
                      unsigned int last_ptr, size;
                      /* get the current hardware pointer (in frames) */
                      last_ptr = get_hw_ptr(chip);
                      /* calculate the processed frames since the
                       * last update
                       */
                      if (last_ptr < chip->last_ptr)
                              size = runtime->buffer_size + last_ptr
                                       - chip->last_ptr;
                      else
                              size = last_ptr - chip->last_ptr;
                      /* remember the last updated point */
                      chip->last_ptr = last_ptr;
                      /* accumulate the size */
                      chip->size += size;
                      /* over the period boundary? */
                      if (chip->size >= runtime->period_size) {
                              /* reset the accumulator */
                              chip->size %= runtime->period_size;
                              /* call updater */
                              spin_unlock(&chip->lock);
                              snd_pcm_period_elapsed(substream);
                              spin_lock(&chip->lock);
                      }
                      /* acknowledge the interrupt if necessary */
              }
              ....
              spin_unlock(&chip->lock);
              return IRQ_HANDLED;
      }



```

#### 鍏充簬璋冪敤 :c:func:`snd_pcm_period_elapsed()`


鍦ㄤ袱绉嶆儏鍐典笅锛屽嵆浣垮凡缁忚繃浜嗕笉姝竴涓懆鏈燂紝浣犱篃涓嶅繀澶氭璋冪敤
`snd_pcm_period_elapsed()`銆傚彧璋冪敤涓€娆°€侾CM 灞備細妫€鏌ュ綋鍓嶇殑纭欢鎸囬拡骞舵洿鏂板埌
鏈€鏂扮姸鎬併€?

### 鍘熷瓙鎬?


鍐呮牳缂栫▼涓渶閲嶈锛堜篃鍥犳鏈€闅捐皟璇曪級鐨勯棶棰樹箣涓€鏄珵鎬佹潯浠讹紙race conditions锛夈€傚湪
Linux 鍐呮牳涓紝瀹冧滑閫氬父閫氳繃鑷棆閿併€佷簰鏂ヤ綋鎴栦俊鍙烽噺鏉ラ伩鍏嶃€備竴鑸潵璇达紝濡傛灉绔炴€佹潯浠?
鍙兘鍙戠敓鍦ㄤ腑鏂鐞嗗嚱鏁颁腑锛屽畠蹇呴』浠ュ師瀛愭柟寮忓鐞嗭紝浣犲繀椤讳娇鐢ㄨ嚜鏃嬮攣鏉ヤ繚鎶や复鐣屽尯銆?
濡傛灉涓寸晫鍖轰笉鍦ㄤ腑鏂鐞嗗嚱鏁颁唬鐮佷腑锛屽苟涓旀墽琛岃緝闀挎椂闂存槸鍙互鎺ュ彈鐨勶紝浣犲簲璇ヤ娇鐢ㄤ簰鏂ヤ綋
鎴栦俊鍙烽噺銆?

濡傚凡缁忕湅鍒扮殑锛屾煇浜?PCM 鍥炶皟鏄師瀛愮殑锛屾煇浜涘垯涓嶆槸銆備緥濡傦紝`hw_params` 鍥炶皟鏄?
闈炲師瀛愮殑锛岃€?`trigger` 鍥炶皟鏄師瀛愮殑銆傝繖鎰忓懗鐫€锛屽悗鑰呭凡缁忓湪鐢?PCM 涓棿灞傛寔鏈夌殑
鑷棆閿侊紙PCM 娴侀攣锛変腑琚皟鐢ㄣ€傚湪涓哄洖璋冮€夋嫨鍔犻攣鏂规鏃讹紝璇疯€冭檻杩欑鍘熷瓙鎬с€?

鍦ㄥ師瀛愬洖璋冧腑锛屼綘涓嶈兘浣跨敤鍙兘璋冪敤 `schedule()` 鎴栬繘鍏?`sleep()` 鐨勫嚱鏁般€?
淇″彿閲忓拰浜掓枼浣撳彲鑳戒細浼戠湢锛屽洜姝ゅ畠浠笉鑳界敤浜庡師瀛愬洖璋冨唴閮紙渚嬪 `trigger` 鍥炶皟锛夈€?
瑕佸湪姝ょ被鍥炶皟涓疄鐜版煇绉嶅欢杩燂紝璇蜂娇鐢?`udelay()` 鎴?`mdelay()`銆?

鎵€鏈変笁涓師瀛愬洖璋冿紙trigger銆乸ointer 鍜?ack锛夐兘鍦ㄦ湰鍦颁腑鏂绂佺敤鐨勭姸鎬佷笅璋冪敤銆?

鐒惰€岋紝鍙互璇锋眰鎵€鏈?PCM 鎿嶄綔涓洪潪鍘熷瓙鐨勩€傝繖鍋囪鎵€鏈夎皟鐢ㄧ偣閮藉浜庨潪鍘熷瓙涓婁笅鏂囥€備緥濡傦紝
`snd_pcm_period_elapsed()` 閫氬父浠庝腑鏂鐞嗗嚱鏁拌皟鐢ㄣ€備絾鏄紝濡傛灉浣犲皢椹卞姩璁剧疆涓轰娇鐢?
绾跨▼鍖栦腑鏂鐞嗗嚱鏁帮紝杩欎釜璋冪敤涔熷彲浠ュ浜庨潪鍘熷瓙涓婁笅鏂囦腑銆傚湪杩欑鎯呭喌涓嬶紝浣犲彲浠ュ湪鍒涘缓
struct snd_pcm 瀵硅薄涔嬪悗璁剧疆瀹冪殑 `nonatomic` 瀛楁銆傚綋璁剧疆浜嗚繖涓爣蹇楁椂锛孭CM 鏍稿績
鍐呴儴浣跨敤浜掓枼浣撳拰 rwsem 浠ｆ浛鑷棆閿佸拰 rwlocks锛岃繖鏍蜂綘灏卞彲浠ュ湪闈炲師瀛愪笂涓嬫枃涓畨鍏ㄥ湴
璋冪敤鎵€鏈?PCM 鍑芥暟銆?

姝ゅ锛屽湪鏌愪簺鎯呭喌涓嬶紝浣犲彲鑳介渶瑕佸湪鍘熷瓙涓婁笅鏂囦腑璋冪敤 `snd_pcm_period_elapsed()`
锛堜緥濡傦紝鍛ㄦ湡鍦?`ack` 鎴栧叾浠栧洖璋冩湡闂磋繃鍘伙級銆備负姝や篃鏈変竴涓彲浠ュ湪 PCM 娴侀攣鍐呴儴璋冪敤
鐨勫彉浣?`snd_pcm_period_elapsed_under_stream_lock()`銆?

### 绾︽潫


鐢变簬鐗╃悊闄愬埗锛岀‖浠朵笉鏄棤闄愬彲閰嶇疆鐨勩€傝繖浜涢檺鍒堕€氳繃璁剧疆鐨勭害鏉熸潵琛ㄨ揪銆?

渚嬪锛屼负浜嗗皢閲囨牱鐜囬檺鍒朵负鏌愪簺鍙楁敮鎸佺殑鍊硷紝浣跨敤 `snd_pcm_hw_constraint_list()`銆?
浣犻渶瑕?

```

      static unsigned int rates[] =
              {4000, 10000, 22050, 44100};
      static struct snd_pcm_hw_constraint_list constraints_rates = {
              .count = ARRAY_SIZE(rates),
              .list = rates,
              .mask = 0,
      };

      static int snd_mychip_pcm_open(struct snd_pcm_substream *substream)
      {
              int err;
              ....
              err = snd_pcm_hw_constraint_list(substream->runtime, 0,
                                               SNDRV_PCM_HW_PARAM_RATE,
                                               &constraints_rates);
              if (err < 0)
                      return err;
              ....
      }

```

鏈夎澶氫笉鍚岀殑绾︽潫銆傛煡鐪?`sound/pcm.h` 鑾峰彇瀹屾暣鍒楄〃銆備綘鐢氳嚦鍙互瀹氫箟鑷繁鐨勭害鏉熻鍒欍€?
渚嬪锛屽亣璁?my_chip 褰撲笖浠呭綋鏍煎紡涓?`S16_LE` 鏃舵墠鑳界鐞?1 閫氶亾鐨勫瓙娴侊紝鍚﹀垯瀹冩敮鎸?
struct snd_pcm_hardware锛堟垨浠讳綍鍏朵粬锛変腑鎸囧畾鐨勪换浣曟牸寮忥細

```

      static int hw_rule_channels_by_format(struct snd_pcm_hw_params *params,
                                            struct snd_pcm_hw_rule *rule)
      {
              struct snd_interval *c = hw_param_interval(params,
                            SNDRV_PCM_HW_PARAM_CHANNELS);
              struct snd_mask *f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
              struct snd_interval ch;

              snd_interval_any(&ch);
              if (f->bits[0] == SNDRV_PCM_FMTBIT_S16_LE) {
                      ch.min = ch.max = 1;
                      ch.integer = 1;
                      return snd_interval_refine(c, &ch);
              }
              return 0;
      }


```

```

  snd_pcm_hw_rule_add(substream->runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
                      hw_rule_channels_by_format, NULL,
                      SNDRV_PCM_HW_PARAM_FORMAT, -1);

```

褰撳簲鐢ㄧ▼搴忚缃?PCM 鏍煎紡鏃惰皟鐢ㄨ鍒欏嚱鏁帮紝骞剁浉搴斿湴缁嗗寲閫氶亾鏁伴噺銆備絾搴旂敤绋嬪簭鍙兘鍦ㄨ缃?
鏍煎紡涔嬪墠璁剧疆閫氶亾鏁伴噺銆傚洜姝や綘杩橀渶瑕?

```

      static int hw_rule_format_by_channels(struct snd_pcm_hw_params *params,
                                            struct snd_pcm_hw_rule *rule)
      {
              struct snd_interval *c = hw_param_interval(params,
                    SNDRV_PCM_HW_PARAM_CHANNELS);
              struct snd_mask *f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
              struct snd_mask fmt;

              snd_mask_any(&fmt);    /* Init the struct */
              if (c->min < 2) {
                      fmt.bits[0] &= SNDRV_PCM_FMTBIT_S16_LE;
                      return snd_mask_refine(f, &fmt);
              }
              return 0;
      }


```

```

  snd_pcm_hw_rule_add(substream->runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT,
                      hw_rule_format_by_channels, NULL,
                      SNDRV_PCM_HW_PARAM_CHANNELS, -1);

```

hw 绾︽潫鐨勪竴涓吀鍨嬬敤閫旀槸灏嗙紦鍐插尯澶у皬涓庡懆鏈熷ぇ灏忓榻愩€傞粯璁ゆ儏鍐典笅锛孉LSA PCM 鏍稿績
涓嶅己鍒剁紦鍐插尯澶у皬涓哄懆鏈熷ぇ灏忕殑鏁存暟鍊嶃€備緥濡傦紝鍙兘浼氬嚭鐜?256 鍛ㄦ湡瀛楄妭鎼厤 999 缂撳啿鍖?
瀛楄妭杩欐牱鐨勭粍鍚堛€?

鐒惰€岋紝璁稿璁惧鑺墖瑕佹眰缂撳啿鍖烘槸鍛ㄦ湡鏁扮殑鏁存暟鍊嶃€傚湪杩欑鎯呭喌涓嬶紝璋冪敤
`snd_pcm_hw_constraint_integer()` 鐢ㄤ簬

```

  snd_pcm_hw_constraint_integer(substream->runtime,
                                SNDRV_PCM_HW_PARAM_PERIODS);

```

杩欑‘淇濅簡鍛ㄦ湡鐨勬暟閲忔槸鏁存暟锛屽洜姝ょ紦鍐插尯澶у皬涓庡懆鏈熷ぇ灏忓榻愩€?

hw 绾︽潫鏄畾涔夐閫?PCM 閰嶇疆鐨勪竴涓潪甯稿己澶х殑鏈哄埗锛屽苟涓旀湁鐩稿叧鐨勮緟鍔╁嚱鏁般€傛垜鍦ㄨ繖閲?
涓嶇粰鍑烘洿澶氱粏鑺傦紝鑰屾槸鎯宠锛屸€淟uke锛屼娇鐢ㄦ簮鐮侊紙use the source锛夈€傗€?

## 鎺у埗鎺ュ彛


### 姒傝堪


鎺у埗鎺ュ彛琚箍娉涚敤浜庤澶氬紑鍏炽€佹粦鍧楃瓑锛岃繖浜涘彲浠庣敤鎴风┖闂磋闂€傚畠鏈€閲嶈鐨勭敤閫旀槸娣烽煶鍣?
锛坢ixer锛夋帴鍙ｃ€傛崲鍙ヨ瘽璇达紝鑷?ALSA 0.9.x 璧凤紝鎵€鏈夋贩闊冲櫒鐩稿叧鐨勫唴瀹归兘瀹炵幇鍦ㄦ帶鍒跺唴鏍?
API 涓娿€?

ALSA 鏈変竴涓畾涔夎壇濂界殑 AC97 鎺у埗妯″潡銆傚鏋滀綘鐨勮姱鐗囧彧鏀寔 AC97 鑰屾病鏈夊叾浠栦笢瑗匡紝
浣犲彲浠ヨ烦杩囨湰鑺傘€?

鎺у埗 API 瀹氫箟鍦?`<sound/control.h>` 涓€傚鏋滀綘鎯虫坊鍔犺嚜宸辩殑鎺у埗锛岃鍖呭惈杩欎釜鏂囦欢銆?

### 鎺т欢鐨勫畾涔?


瑕佸垱寤轰竴涓柊鐨勬帶浠讹紝浣犻渶瑕佸畾涔変互涓嬩笁涓洖璋冿細`info`銆乣get` 鍜?`put`銆傜劧鍚庯紝
瀹氫箟涓€涓?

```


      static struct snd_kcontrol_new my_control = {
              .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
              .name = "PCM Playback Switch",
              .index = 0,
              .access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
              .private_value = 0xffff,
              .info = my_control_info,
              .get = my_control_get,
              .put = my_control_put
      };


```

`iface` 瀛楁鎸囧畾鎺т欢绫诲瀷 `SNDRV_CTL_ELEM_IFACE_XXX`锛岄€氬父鏄?`MIXER`銆?
瀵逛笉灞炰簬娣烽煶鍣ㄩ€昏緫閮ㄥ垎鐨勫叏灞€鎺т欢浣跨敤 `CARD`銆傚鏋滄帶浠朵笌澹板崱涓婃煇涓壒瀹氳澶囧瘑鍒囩浉鍏筹紝
浣跨敤 `HWDEP`銆乣PCM`銆乣RAWMIDI`銆乣TIMER` 鎴?`SEQUENCER`锛屽苟鐢?`device` 鍜?
`subdevice` 瀛楁鎸囧畾璁惧鍙枫€?

`name` 鏄悕绉版爣璇嗙瀛楃涓层€傝嚜 ALSA 0.9.x 璧凤紝鎺т欢鍚嶇О闈炲父閲嶈锛屽洜涓哄畠鐨勮鑹叉槸
浠庡悕绉颁腑鍒嗙被鍑烘潵鐨勩€傛湁棰勫畾涔夌殑鏍囧噯鎺т欢鍚嶇О銆傜粏鑺傚湪 `Control Names`_ 灏忚妭涓弿杩般€?

`index` 瀛楁淇濆瓨杩欎釜鎺т欢鐨勭储寮曞彿銆傚鏋滄湁鍑犱釜鍚嶇О涓嶅悓鐨勬帶浠讹紝鍙互閫氳繃绱㈠紩鍙锋潵鍖哄垎銆?
褰撳０鍗′笂瀛樺湪澶氫釜缂栬В鐮佸櫒鏃跺氨鏄繖绉嶆儏鍐点€傚鏋滅储寮曚负闆讹紝浣犲彲浠ョ渷鐣ヤ笂闈㈢殑瀹氫箟銆?

`access` 瀛楁鍖呭惈杩欎釜鎺т欢鐨勮闂被鍨嬨€傚湪杩欓噷缁欏嚭浣嶆帺鐮佺殑缁勫悎
`SNDRV_CTL_ELEM_ACCESS_XXX`銆傜粏鑺傚皢鍦?`Access Flags`_ 灏忚妭涓В閲娿€?

`private_value` 瀛楁鍖呭惈杩欎釜璁板綍鐨勪换鎰忛暱鏁村瀷鍊笺€傚綋浣跨敤閫氱敤鐨?`info`銆乣get`
鍜?`put` 鍥炶皟鏃讹紝浣犲彲浠ラ€氳繃杩欎釜瀛楁浼犲€笺€傚鏋滈渶瑕佸嚑涓皬鏁板瓧锛屼綘鍙互灏嗗畠浠寜浣?
缁勫悎銆傛垨鑰咃紝涔熷彲浠ュ湪杩欎釜瀛楁涓瓨鍌ㄦ煇涓褰曠殑鎸囬拡锛堣浆鎹负 unsigned long锛夈€?

`tlv` 瀛楁鍙敤浜庢彁渚涘叧浜庢帶浠剁殑鍏冩暟鎹紱瑙?`Metadata`_ 灏忚妭銆?

鍏朵粬涓変釜鏄?`Control Callbacks`_銆?

### 鎺т欢鍚嶇О


瀹氫箟鎺т欢鍚嶇О鏈変竴浜涙爣鍑嗐€備竴涓帶浠堕€氬父鐢变笁閮ㄥ垎瀹氫箟涓衡€滄簮 鏂瑰悜 鍔熻兘锛圫OURCE
DIRECTION FUNCTION锛夆€濄€?

绗竴锛宍SOURCE`锛屾寚瀹氭帶浠剁殑婧愶紝鏄竴涓鈥淢aster鈥濄€佲€淧CM鈥濄€佲€淐D鈥濆拰鈥淟ine鈥濊繖鏍风殑瀛楃涓层€?
鏈夎澶氶瀹氫箟鐨勬簮銆?

绗簩锛宍DIRECTION`锛屾牴鎹帶浠剁殑鏂瑰悜锛屾槸浠ヤ笅瀛楃涓蹭箣涓€锛氣€淧layback鈥濄€佲€淐apture鈥濄€?
鈥淏ypass Playback鈥濆拰鈥淏ypass Capture鈥濄€傛垨鑰咃紝鍙互鐪佺暐锛屾剰鍛崇潃鎾斁鍜屾崟鑾蜂袱涓柟鍚戙€?

绗笁锛宍FUNCTION`锛屾牴鎹帶浠剁殑鍔熻兘锛屾槸浠ヤ笅瀛楃涓蹭箣涓€锛氣€淪witch鈥濄€佲€淰olume鈥濆拰
鈥淩oute鈥濄€?

鍥犳锛屾帶浠跺悕绉扮殑渚嬪瓙鏈夆€淢aster Capture Switch鈥濇垨鈥淧CM Playback Volume鈥濄€?

鏈変竴浜涗緥澶栵細

#### 鍏ㄥ眬鎹曡幏涓庢挱鏀?


鈥淐apture Source鈥濄€佲€淐apture Switch鈥濆拰鈥淐apture Volume鈥濈敤浜庡叏灞€鎹曡幏锛堣緭鍏ワ級婧愩€?
寮€鍏冲拰闊抽噺銆傜被浼煎湴锛屸€淧layback Switch鈥濆拰鈥淧layback Volume鈥濈敤浜庡叏灞€杈撳嚭澧炵泭寮€鍏冲拰
闊抽噺銆?

#### 闊宠皟鎺у埗


闊宠皟鎺у埗寮€鍏冲拰闊抽噺鎸囧畾涓衡€淭one Control - XXX鈥濓紝渚嬪鈥淭one Control - Switch鈥濄€?
鈥淭one Control - Bass鈥濄€佲€淭one Control - Center鈥濄€?

#### 3D 鎺у埗


3D 鎺у埗寮€鍏冲拰闊抽噺鎸囧畾涓衡€?D Control - XXX鈥濓紝渚嬪鈥?D Control - Switch鈥濄€佲€?D
Control - Center鈥濄€佲€?D Control - Space鈥濄€?

#### Mic 澧炵泭


Mic-boost 寮€鍏宠缃负鈥淢ic Boost鈥濇垨鈥淢ic Boost (6dB)鈥濄€?

鏇寸簿纭殑淇℃伅鍙互鍦?`Documentation/sound/designs/control-names.rst` 涓壘鍒般€?

### 璁块棶鏍囧織


璁块棶鏍囧織鏄寚瀹氫綅鎺╃爜锛屽畠鎸囧畾缁欏畾鎺т欢鐨勮闂被鍨嬨€傞粯璁よ闂被鍨嬫槸
`SNDRV_CTL_ELEM_ACCESS_READWRITE`锛岃繖鎰忓懗鐫€鍏佽瀵硅鎺т欢杩涜璇诲拰鍐欍€傚綋璁块棶鏍囧織
琚渷鐣ワ紙鍗?= 0锛夋椂锛岄粯璁よ瑙嗕负 `READWRITE` 璁块棶銆?

褰撴帶浠舵槸鍙鏃讹紝鏀逛负浼犲叆 `SNDRV_CTL_ELEM_ACCESS_READ`銆傚湪杩欑鎯呭喌涓嬶紝浣犱笉蹇呭畾涔?
`put` 鍥炶皟銆傜被浼煎湴锛屽綋鎺т欢鏄彧鍐欐椂锛堝敖绠¤繖绉嶆儏鍐靛緢灏戣锛夛紝浣犲彲浠ヤ娇鐢?`WRITE`
鏍囧織锛屽苟涓斾笉闇€瑕?`get` 鍥炶皟銆?

濡傛灉鎺т欢鍊奸绻佸彉鍖栵紙渚嬪 VU 琛級锛屽簲缁欏嚭 `VOLATILE` 鏍囧織銆傝繖鎰忓懗鐫€璇ユ帶浠跺彲鑳戒細
鍦ㄦ病鏈?`Change notification`_ 鐨勬儏鍐典笅琚敼鍙樸€傚簲鐢ㄧ▼搴忓簲褰撴寔缁疆璇㈣繖鏍风殑鎺т欢銆?

褰撴帶浠跺彲鑳借鏇存柊锛屼絾褰撳墠瀵逛换浣曚笢瑗块兘娌℃湁褰卞搷鏃讹紝璁剧疆 `INACTIVE` 鏍囧織鍙兘鏄悎閫傜殑銆?
渚嬪锛屽綋娌℃湁 PCM 璁惧鎵撳紑鏃讹紝PCM 鎺т欢搴斿綋鏄笉娲昏穬鐨勩€?

鏈?`LOCK` 鍜?`OWNER` 鏍囧織鍙互鏀瑰彉鍐欐潈闄愩€?

### 鎺т欢鍥炶皟


#### info 鍥炶皟


`info` 鍥炶皟鐢ㄤ簬鑾峰彇鍏充簬杩欎釜鎺т欢鐨勮缁嗕俊鎭€傚畠蹇呴』瀛樺偍缁欏畾鐨?struct
snd_ctl_elem_info 瀵硅薄鐨勫€笺€備緥濡傦紝

```


      static int snd_myctl_mono_info(struct snd_kcontrol *kcontrol,
                              struct snd_ctl_elem_info *uinfo)
      {
              uinfo->type = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
              uinfo->count = 1;
              uinfo->value.integer.min = 0;
              uinfo->value.integer.max = 1;
              return 0;
      }



```

`type` 瀛楁鎸囧畾鎺т欢鐨勭被鍨嬨€傛湁 `BOOLEAN`銆乣INTEGER`銆乣ENUMERATED`銆乣BYTES`銆?
`IEC958` 鍜?`INTEGER64`銆俙count` 瀛楁鎸囧畾杩欎釜鎺т欢涓厓绱犵殑鏁伴噺銆備緥濡傦紝绔嬩綋澹?
闊抽噺浼氭湁 count = 2銆俙value` 瀛楁鏄竴涓仈鍚堜綋锛屽瓨鍌ㄧ殑鍊煎彇鍐充簬绫诲瀷銆傚竷灏斿拰鏁存暟
绫诲瀷鏄浉鍚岀殑銆?

鏋氫妇绫诲瀷涓庡叾浠栫被鍨嬬暐鏈変笉鍚屻€備綘闇€瑕?

```

  static int snd_myctl_enum_info(struct snd_kcontrol *kcontrol,
                          struct snd_ctl_elem_info *uinfo)
  {
          static char *texts[4] = {
                  "First", "Second", "Third", "Fourth"
          };
          uinfo->type = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
          uinfo->count = 1;
          uinfo->value.enumerated.items = 4;
          if (uinfo->value.enumerated.item > 3)
                  uinfo->value.enumerated.item = 3;
          strcpy(uinfo->value.enumerated.name,
                 texts[uinfo->value.enumerated.item]);
          return 0;
  }

```

涓婇潰鐨勫洖璋冨彲浠ョ敤杈呭姪鍑芥暟 `snd_ctl_enum_info()` 绠€鍖栥€傛渶缁堜唬鐮佸涓嬫墍绀恒€?
锛堜綘鍙互鍦ㄧ涓変釜鍙傛暟涓紶鍏?`ARRAY_SIZE(texts)` 鑰屼笉鏄?4锛涜繖鐪嬩釜浜哄枩濂姐€傦級

```

  static int snd_myctl_enum_info(struct snd_kcontrol *kcontrol,
                          struct snd_ctl_elem_info *uinfo)
  {
          static char *texts[4] = {
                  "First", "Second", "Third", "Fourth"
          };
          return snd_ctl_enum_info(uinfo, 1, 4, texts);
  }


```

涓€浜涘父瑙佺殑 info 鍥炶皟鍙緵浣犳柟渚夸娇鐢細`snd_ctl_boolean_mono_info()` 鍜?
`snd_ctl_boolean_stereo_info()`銆傛樉鐒讹紝鍓嶈€呮槸鍗曞０閬撳竷灏旈」鐨?info 鍥炶皟锛屽氨鍍忎笂闈㈢殑
`snd_myctl_mono_info()`锛屽悗鑰呮槸绔嬩綋澹板竷灏旈」鐨?info 鍥炶皟銆?

#### get 鍥炶皟


杩欎釜鍥炶皟鐢ㄤ簬璇诲彇鎺т欢鐨勫綋鍓嶅€硷紝浠ヤ究瀹冨彲浠ヨ繑鍥炵粰鐢ㄦ埛绌洪棿銆?

```

      static int snd_myctl_get(struct snd_kcontrol *kcontrol,
                               struct snd_ctl_elem_value *ucontrol)
      {
              struct mychip *chip = snd_kcontrol_chip(kcontrol);
              ucontrol->value.integer.value[0] = get_some_value(chip);
              return 0;
      }



```

`value` 瀛楁鍙栧喅浜庢帶浠剁被鍨嬩互鍙?info 鍥炶皟銆備緥濡傦紝sb 椹卞姩浣跨敤杩欎釜瀛楁鏉ュ瓨鍌ㄥ瘎瀛樺櫒
鍋忕Щ銆佷綅绉诲拰浣嶆帺鐮併€俙private_value`

```

  .private_value = reg | (shift << 16) | (mask << 24)

```

```

  static int snd_sbmixer_get_single(struct snd_kcontrol *kcontrol,
                                    struct snd_ctl_elem_value *ucontrol)
  {
          int reg = kcontrol->private_value & 0xff;
          int shift = (kcontrol->private_value >> 16) & 0xff;
          int mask = (kcontrol->private_value >> 24) & 0xff;
          ....
  }

```

鍦?`get` 鍥炶皟涓紝濡傛灉鎺т欢鏈夊涓厓绱狅紙鍗?`count > 1`锛夛紝浣犲繀椤诲～鍏呮墍鏈夊厓绱犮€?
鍦ㄤ笂闈㈢殑渚嬪瓙涓紝鐢变簬鍋囪 `count = 1`锛屾垜浠彧濉厖浜嗕竴涓厓绱?
锛坄value.integer.value[^0^]`锛夈€?

#### put 鍥炶皟


杩欎釜鍥炶皟鐢ㄤ簬鍐欏叆鏉ヨ嚜鐢ㄦ埛绌洪棿鐨勫€笺€?

```

      static int snd_myctl_put(struct snd_kcontrol *kcontrol,
                               struct snd_ctl_elem_value *ucontrol)
      {
              struct mychip *chip = snd_kcontrol_chip(kcontrol);
              int changed = 0;
              if (chip->current_value !=
                   ucontrol->value.integer.value[0]) {
                      change_current_value(chip,
                                  ucontrol->value.integer.value[0]);
                      changed = 1;
              }
              return changed;
      }



```

濡備笂鎵€绀猴紝濡傛灉鍊兼敼鍙樹簡锛屼綘蹇呴』杩斿洖 1銆傚鏋滃€兼病鏈夋敼鍙橈紝鍒欒繑鍥?0銆傚鏋滃彂鐢熶换浣曡嚧鍛?
閿欒锛屽儚寰€甯镐竴鏍疯繑鍥炶礋鐨勯敊璇爜銆?

涓?`get` 鍥炶皟涓€鏍凤紝褰撴帶浠舵湁澶氫釜鍏冪礌鏃讹紝鎵€鏈夊厓绱犱篃蹇呴』鍦ㄨ繖涓洖璋冧腑琚眰鍊笺€?

#### 鍥炶皟涓嶆槸鍘熷瓙鐨?


杩欎笁涓洖璋冮兘涓嶆槸鍘熷瓙鐨勩€?

### 鎺т欢鏋勯€犲嚱鏁?


褰撲竴鍒囧氨缁紝鎴戜滑缁堜簬鍙互鍒涘缓涓€涓柊鐨勬帶浠躲€傝鍒涘缓涓€涓帶浠讹紝闇€瑕佽皟鐢ㄤ袱涓嚱鏁帮紝
`snd_ctl_new1()` 鍜?`snd_ctl_add()`銆?

```

  err = snd_ctl_add(card, snd_ctl_new1(&my_control, chip));
  if (err < 0)
          return err;

```

鍏朵腑 `my_control` 鏄笂闈㈠畾涔夌殑 struct snd_kcontrol_new 瀵硅薄锛宑hip 鏄浼犻€掔粰
kcontrol->private_data 鐨勫璞℃寚閽堬紝鍙互鍦ㄥ洖璋冧腑寮曠敤銆?

`snd_ctl_new1()` 鍒嗛厤涓€涓柊鐨?struct snd_kcontrol 瀹炰緥锛宍snd_ctl_add()` 灏嗙粰瀹?
鐨勬帶浠剁粍浠跺垎閰嶇粰澹板崱銆?

### 鍙樻洿閫氱煡


濡傛灉浣犻渶瑕佸湪涓柇渚嬬▼涓彉鏇村拰鏇存柊涓€涓帶浠讹紝

```

  snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, id_pointer);

```

杩欎釜鍑芥暟鎺ュ彈澹板崱鎸囬拡銆佷簨浠舵帺鐮佷互鍙婄敤浜庨€氱煡鐨勬帶浠?id 鎸囬拡銆備簨浠舵帺鐮佹寚瀹氶€氱煡鐨勭被鍨嬶紝
渚嬪锛屽湪涓婇潰鐨勪緥瀛愪腑锛岄€氱煡鎺т欢鍊肩殑鏀瑰彉銆俰d 鎸囬拡鏄閫氱煡鐨?struct snd_ctl_elem_id
鐨勬寚閽堛€備綘鍙互鍦?`es1938.c` 鎴?`es1968.c` 涓壘鍒扮‖浠堕煶閲忎腑鏂殑涓€浜涗緥瀛愩€?

### 鍏冩暟鎹?


瑕佹彁渚涘叧浜庢贩闊冲櫒鎺т欢 dB 鍊肩殑淇℃伅锛屼娇鐢?`<sound/tlv.h>` 涓殑鏌愪釜
`DECLARE_TLV_xxx` 瀹忔潵瀹氫箟涓€涓寘鍚淇℃伅鐨勫彉閲忥紝灏?`tlv.p` 瀛楁璁剧疆涓烘寚鍚戣繖涓?
鍙橀噺锛屽苟鍖呭惈 `SNDRV_CTL_ELEM_ACCESS_TLV_READ` 鏍囧織锛?

```

  static DECLARE_TLV_DB_SCALE(db_scale_my_control, -4050, 150, 0);

  static struct snd_kcontrol_new my_control = {
          ...
          .access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
                    SNDRV_CTL_ELEM_ACCESS_TLV_READ,
          ...
          .tlv.p = db_scale_my_control,
  };


```

`DECLARE_TLV_DB_SCALE()` 瀹忓畾涔夊叧浜庝竴涓贩闊冲櫒鎺т欢鐨勪俊鎭紝鍏朵腑鎺т欢鍊肩殑姣忎竴姝ュ彉鍖?
閮戒細浠ユ亽瀹氱殑 dB 閲忔敼鍙?dB 鍊笺€傜涓€涓弬鏁版槸瑕佸畾涔夌殑鍙橀噺鍚嶃€傜浜屼釜鍙傛暟鏄渶灏忓€硷紝
鍗曚綅涓?0.01 dB銆傜涓変釜鍙傛暟鏄闀匡紝鍗曚綅涓?0.01 dB銆傚鏋滄渶灏忓€煎疄闄呬笂浼氫娇鎺т欢闈欓煶锛?
鍒欏皢绗洓涓弬鏁拌缃负 1銆?

`DECLARE_TLV_DB_LINEAR()` 瀹忓畾涔夊叧浜庝竴涓贩闊冲櫒鎺т欢鐨勪俊鎭紝鍏朵腑鎺т欢鐨勫€肩嚎鎬у湴褰卞搷
杈撳嚭銆傜涓€涓弬鏁版槸瑕佸畾涔夌殑鍙橀噺鍚嶃€傜浜屼釜鍙傛暟鏄渶灏忓€硷紝鍗曚綅涓?0.01 dB銆傜涓変釜鍙傛暟鏄?
鏈€澶у€硷紝鍗曚綅涓?0.01 dB銆傚鏋滄渶灏忓€间娇鎺т欢闈欓煶锛屽垯灏嗙浜屼釜鍙傛暟璁剧疆涓?
`TLV_DB_GAIN_MUTE`銆?

## AC97 缂栬В鐮佸櫒鐨?API


### 姒傝堪


ALSA AC97 缂栬В鐮佸櫒灞傛槸涓€涓畾涔夎壇濂界殑灞傦紝浣犱笉蹇呯紪鍐欏お澶氫唬鐮佹潵鎺у埗瀹冦€傚彧闇€瑕佷綆灞?
鎺у埗渚嬬▼銆侫C97 缂栬В鐮佸櫒 API 瀹氫箟鍦?`<sound/ac97_codec.h>` 涓€?

### 瀹屾暣浠ｇ爜绀轰緥


```

      struct mychip {
              ....
              struct snd_ac97 *ac97;
              ....
      };

      static unsigned short snd_mychip_ac97_read(struct snd_ac97 *ac97,
                                                 unsigned short reg)
      {
              struct mychip *chip = ac97->private_data;
              ....
              /* read a register value here from the codec */
              return the_register_value;
      }

      static void snd_mychip_ac97_write(struct snd_ac97 *ac97,
                                       unsigned short reg, unsigned short val)
      {
              struct mychip *chip = ac97->private_data;
              ....
              /* write the given register value to the codec */
      }

      static int snd_mychip_ac97(struct mychip *chip)
      {
              struct snd_ac97_bus *bus;
              struct snd_ac97_template ac97;
              int err;
              static struct snd_ac97_bus_ops ops = {
                      .write = snd_mychip_ac97_write,
                      .read = snd_mychip_ac97_read,
              };

              err = snd_ac97_bus(chip->card, 0, &ops, NULL, &bus);
              if (err < 0)
                      return err;
              memset(&ac97, 0, sizeof(ac97));
              ac97.private_data = chip;
              return snd_ac97_mixer(bus, &ac97, &chip->ac97);
      }


```

### AC97 鏋勯€犲嚱鏁?


瑕佸垱寤轰竴涓?ac97 瀹炰緥锛岄鍏堣皟鐢?`snd_ac97_bus()`锛?

```

  struct snd_ac97_bus *bus;
  static struct snd_ac97_bus_ops ops = {
        .write = snd_mychip_ac97_write,
        .read = snd_mychip_ac97_read,
  };

  snd_ac97_bus(card, 0, &ops, NULL, &pbus);

```

鎬荤嚎璁板綍鍦ㄦ墍鏈変粠灞炵殑 ac97 瀹炰緥涔嬮棿鍏变韩銆?

鐒跺悗鐢ㄤ竴涓?struct snd_ac97_template 璋冪敤 `snd_ac97_mixer()`锛?

```

  struct snd_ac97_template ac97;
  int err;

  memset(&ac97, 0, sizeof(ac97));
  ac97.private_data = chip;
  snd_ac97_mixer(bus, &ac97, &chip->ac97);

```

鍏朵腑 chip->ac97 鏄寚鍚戞柊鍒涘缓鐨?`ac97_t` 瀹炰緥鐨勬寚閽堛€傚湪杩欑鎯呭喌涓嬶紝鑺墖鎸囬拡琚?
璁剧疆涓虹鏈夋暟鎹紝浠ヤ究璇?鍐欏洖璋冨嚱鏁板彲浠ュ紩鐢ㄨ繖涓姱鐗囧疄渚嬨€傝繖涓疄渚嬩笉涓€瀹氫繚瀛樺湪鑺墖
璁板綍涓€傚鏋滀綘闇€瑕佷粠椹卞姩鏀瑰彉瀵勫瓨鍣ㄥ€硷紝鎴栬€呴渶瑕?ac97 缂栬В鐮佸櫒鐨勬寕璧?鎭㈠锛岃淇濈暀
杩欎釜鎸囬拡浠ヤ紶缁欑浉搴旂殑鍑芥暟銆?

### AC97 鍥炶皟


鏍囧噯鐨勫洖璋冩槸 `read` 鍜?`write`銆傛樉鐒跺畠浠搴斾簬纭欢浣庡眰浠ｇ爜鐨勮鍜屽啓璁块棶鍑芥暟銆?

`read` 鍥炶皟杩斿洖 `read` 鍥炶皟鎸囧畾鐨勫瘎瀛樺櫒鍊硷細

```

  static unsigned short snd_mychip_ac97_read(struct snd_ac97 *ac97,
                                             unsigned short reg)
      {
              struct mychip *chip = ac97->private_data;
              ....
              return the_register_value;
      }

```

杩欓噷锛宑hip 鍙互浠?`ac97->private_data` 杞崲寰楀埌銆?

鍚屾椂锛宍write` 鍥炶皟鐢ㄤ簬璁剧疆瀵勫瓨鍣?

```

  static void snd_mychip_ac97_write(struct snd_ac97 *ac97,
                       unsigned short reg, unsigned short val)


```

杩欎簺鍥炶皟涓庢帶鍒?API 鍥炶皟涓€鏍锋槸闈炲師瀛愮殑銆?

杩樻湁鍏朵粬鍥炶皟锛歚reset`銆乣wait` 鍜?`init`銆?

`reset` 鍥炶皟鐢ㄤ簬閲嶇疆缂栬В鐮佸櫒銆傚鏋滆姱鐗囬渶瑕佷竴绉嶇壒娈婄殑閲嶇疆锛屼綘鍙互瀹氫箟杩欎釜鍥炶皟銆?

`wait` 鍥炶皟鐢ㄤ簬鍦ㄧ紪瑙ｇ爜鍣ㄧ殑鏍囧噯鍒濆鍖栦腑娣诲姞涓€浜涚瓑寰呮椂闂淬€傚鏋滆姱鐗囬渶瑕侀澶栫殑绛夊緟
鏃堕棿锛屽畾涔夎繖涓洖璋冦€?

`init` 鍥炶皟鐢ㄤ簬缂栬В鐮佸櫒鐨勯澶栧垵濮嬪寲銆?

### 鍦ㄩ┍鍔ㄤ腑鏇存柊瀵勫瓨鍣?


濡傛灉浣犻渶瑕佷粠椹卞姩璁块棶缂栬В鐮佸櫒锛屼綘鍙互璋冪敤浠ヤ笅鍑芥暟锛歚snd_ac97_write()`銆?
`snd_ac97_read()`銆乣snd_ac97_update()` 鍜?`snd_ac97_update_bits()`銆?

`snd_ac97_write()` 鍜?`snd_ac97_update()` 鍑芥暟閮界敤浜庣粰缁欏畾瀵勫瓨鍣?
锛坄AC97_XXX`锛夎缃竴涓€笺€傚畠浠箣闂寸殑鍖哄埆鍦ㄤ簬锛宍snd_ac97_update()` 鍦ㄧ粰瀹氬€煎凡缁?
璁剧疆鏃朵笉鍐欏叆锛岃€?`snd_ac97_write()`

```

  snd_ac97_write(ac97, AC97_MASTER, 0x8080);
  snd_ac97_update(ac97, AC97_MASTER, 0x8080);

```

`snd_ac97_read()` 鐢ㄤ簬璇诲彇缁欏畾

```

  value = snd_ac97_read(ac97, AC97_MASTER);

```

`snd_ac97_update_bits()` 鐢ㄤ簬鏇存柊鏌愪簺浣嶏細

```

  snd_ac97_update_bits(ac97, reg, mask, value);

```

姝ゅ锛岃繕鏈変竴涓嚱鏁板湪鏀寔 VRA 鎴?DRA 鏃舵敼鍙橀噰鏍风巼锛堥拡瀵圭粰瀹氱殑瀵勫瓨鍣紝渚嬪
`AC97_PCM_FRONT_DAC_RATE`锛夛細

```

  snd_ac97_set_rate(ac97, AC97_PCM_FRONT_DAC_RATE, 44100);


```

浠ヤ笅瀵勫瓨鍣ㄥ彲鐢ㄤ簬璁剧疆閫熺巼锛歚AC97_PCM_MIC_ADC_RATE`銆乣AC97_PCM_FRONT_DAC_RATE`銆?
`AC97_PCM_LR_ADC_RATE`銆乣AC97_SPDIF`銆傚綋鎸囧畾 `AC97_SPDIF` 鏃讹紝瀵勫瓨鍣ㄥ疄闄呬笂
骞舵病鏈夎鏀瑰彉锛岃€屾槸鐩稿簲鐨?IEC958 鐘舵€佷綅浼氳鏇存柊銆?

### 鏃堕挓璋冩暣


鍦ㄦ煇浜涜姱鐗囦腑锛岀紪瑙ｇ爜鍣ㄧ殑鏃堕挓涓嶆槸 48000锛岃€屾槸浣跨敤 PCI 鏃堕挓锛堜互鑺傜渷涓€涓煶鑻辨櫠鎸紒锛夈€?
鍦ㄨ繖绉嶆儏鍐典笅锛屽皢 `bus->clock` 瀛楁鏇存敼涓虹浉搴旂殑鍊笺€備緥濡傦紝intel8x0 鍜?es1968 椹卞姩
鏈夊畠浠嚜宸辩殑鍑芥暟浠庢椂閽熻鍙栥€?

### Proc 鏂囦欢


ALSA AC97 鎺ュ彛灏嗗垱寤轰竴涓?proc 鏂囦欢锛屽 `/proc/asound/card0/codec97#0/ac97#0-0`
鍜?`ac97#0-0+regs`銆備綘鍙互鍙傝€冭繖浜涙枃浠舵潵鏌ョ湅缂栬В鐮佸櫒鐨勫綋鍓嶇姸鎬佸拰瀵勫瓨鍣ㄣ€?

### 澶氫釜缂栬В鐮佸櫒


褰撳悓涓€寮犲０鍗′笂鏈夊涓紪瑙ｇ爜鍣ㄦ椂锛屼綘闇€瑕佸娆¤皟鐢?`snd_ac97_mixer()`锛屽苟灏?
`ac97.num=1` 鎴栨洿澶с€俙num` 瀛楁鎸囧畾缂栬В鐮佸櫒缂栧彿銆?

濡傛灉浣犺缃簡澶氫釜缂栬В鐮佸櫒锛屼綘瑕佷箞闇€瑕佷负姣忎釜缂栬В鐮佸櫒缂栧啓涓嶅悓鐨勫洖璋冿紝瑕佷箞鍦ㄥ洖璋冧緥绋嬩腑
妫€鏌?`ac97->num`銆?

## MIDI锛圡PU401-UART锛夋帴鍙?


### 姒傝堪


璁稿澹板崱鏈夊唴缃殑 MIDI锛圡PU401-UART锛夋帴鍙ｃ€傚綋澹板崱鏀寔鏍囧噯鐨?MPU401-UART 鎺ュ彛鏃讹紝
寰堝彲鑳戒綘鍙互浣跨敤 ALSA MPU401-UART API銆侻PU401-UART API 瀹氫箟鍦?`<sound/mpu401.h>`
涓€?

鏌愪簺澹拌姱鏈夌被浼间絾鐣ユ湁涓嶅悓鐨?mpu401 瀹炵幇銆備緥濡傦紝emu10k1 鏈夎嚜宸辩殑 mpu401 渚嬬▼銆?

### MIDI 鏋勯€犲嚱鏁?


```

  struct snd_rawmidi *rmidi;
  snd_mpu401_uart_new(card, 0, MPU401_HW_MPU401, port, info_flags,
                      irq, &rmidi);


```

绗竴涓弬鏁版槸澹板崱鎸囬拡锛岀浜屼釜鏄繖涓粍浠剁殑绱㈠紩銆備綘鏈€澶氬彲浠ュ垱寤?8 涓?rawmidi 璁惧銆?

绗笁涓弬鏁版槸纭欢绫诲瀷 `MPU401_HW_XXX`銆傚鏋滀笉鏄壒娈婄殑锛屽彲浠ヤ娇鐢?
`MPU401_HW_MPU401`銆?

绗?4 涓弬鏁版槸 I/O 绔彛鍦板潃銆傝澶氬悜鍚庡吋瀹圭殑 MPU401 鏈変竴涓儚 0x330 杩欐牱鐨?I/O 绔彛銆?
鎴栬€咃紝瀹冨彲鑳芥槸鍏惰嚜韬?PCI I/O 鍖哄煙鐨勪竴閮ㄥ垎銆傝繖鍙栧喅浜庤姱鐗囪璁°€?

绗?5 涓弬鏁版槸鐢ㄤ簬棰濆淇℃伅鐨勪綅鏍囧織銆傚綋涓婇潰鐨?I/O 绔彛鍦板潃鏄?PCI I/O 鍖哄煙鐨勪竴閮ㄥ垎鏃讹紝
MPU401 I/O 绔彛鍙兘宸茬粡琚┍鍔ㄨ嚜韬垎閰嶏紙淇濈暀锛夈€傚湪杩欑鎯呭喌涓嬶紝浼犲叆浣嶆爣蹇?
`MPU401_INFO_INTEGRATED`锛宮pu401-uart 灞傚皢鑷鍒嗛厤 I/O 绔彛銆?

褰撴帶鍒跺櫒鍙敮鎸佽緭鍏ユ垨杈撳嚭 MIDI 娴佹椂锛屽垎鍒紶鍏?`MPU401_INFO_INPUT` 鎴?
`MPU401_INFO_OUTPUT` 浣嶆爣蹇椼€傜劧鍚?rawmidi 瀹炰緥琚垱寤轰负鍗曟祦銆?

`MPU401_INFO_MMIO` 浣嶆爣蹇楃敤浜庡皢璁块棶鏂规硶鏇存敼涓?MMIO锛堥€氳繃 readb 鍜?writeb锛夎€屼笉鏄?
iob 鍜?outb銆傚湪杩欑鎯呭喌涓嬶紝浣犲繀椤诲皢 iomapped 鍦板潃浼犵粰 `snd_mpu401_uart_new()`銆?

褰撹缃簡 `MPU401_INFO_TX_IRQ` 鏃讹紝杈撳嚭娴佷笉鍦ㄩ粯璁や腑鏂鐞嗗嚱鏁颁腑妫€鏌ャ€傞┍鍔ㄩ渶瑕佽嚜宸?
璋冪敤 `snd_mpu401_uart_interrupt_tx()` 鏉ュ湪 irq 澶勭悊鍑芥暟涓惎鍔ㄨ緭鍑烘祦鐨勫鐞嗐€?

濡傛灉 MPU-401 鎺ュ彛涓庡０鍗′笂鐨勫叾浠栭€昏緫璁惧鍏变韩鍏朵腑鏂紝璁剧疆 `MPU401_INFO_IRQ_HOOK`
锛堣 `涓嬫柟 <MIDI Interrupt Handler_>`__锛夈€?

閫氬父锛岀鍙ｅ湴鍧€瀵瑰簲浜庡懡浠ょ鍙ｏ紝绔彛 + 1 瀵瑰簲浜庢暟鎹鍙ｃ€傚鏋滀笉鏄紝浣犲彲浠ョ◢鍚庢墜鍔?
鏇存敼 struct snd_mpu401 鐨?`c