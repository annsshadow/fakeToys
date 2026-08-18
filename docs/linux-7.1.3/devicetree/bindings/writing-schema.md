
## 鐢?json-schema 缂栧啓 Devicetree 缁戝畾


璁惧鏍戯紙Devicetree锛夌粦瀹氫娇鐢?json-schema 璇嶆眹鏉ョ紪鍐欍€傛ā寮忥紙Schema锛夋枃浠堕噰鐢?YAML 鐨?
涓€涓笌 JSON 鍏煎鐨勫瓙闆嗙紪鍐欍€備箣鎵€浠ヤ娇鐢?YAML 鑰岄潪 JSON锛屾槸鍥犱负瀹冭璁や负鏇存槗璇伙紝骞朵笖
鍏锋湁涓€浜涗紭鍔匡紝渚嬪鍏佽娉ㄩ噴锛堜互 '#' 涓哄墠缂€锛夈€?

鍙﹁鍙傞槄 example-schema銆?

### Schema 鍐呭


姣忎釜妯″紡鏂囨。閮芥槸涓€涓粨鏋勫寲鐨?json-schema锛岀敱涓€缁勯《灞傚睘鎬у畾涔夈€傞€氬父锛屾瘡涓枃浠跺畾涔変竴涓?
缁戝畾銆傛墍浣跨敤鐨勯《灞?json-schema 灞炴€у涓嬶細

$id
  涓€涓?json-schema 鍞竴鏍囪瘑绗﹀瓧绗︿覆銆傝瀛楃涓插繀椤绘槸涓€涓湁鏁堢殑 URI锛岄€氬父鍖呭惈缁戝畾鐨?
  鏂囦欢鍚嶅拰璺緞銆傚浜?DT schema锛屽畠蹇呴』浠?"http://devicetree.org/schemas/" 寮€澶淬€傝 URL
  鐢ㄤ簬鏋勯€犲 schema 鐨?"$ref" 灞炴€т腑鎸囧畾鐨勫叾浠栨枃浠剁殑寮曠敤銆傚甫鏈夊墠瀵?'/' 鐨?$ref 鍊间細琚?
  鍔犱笂涓绘満鍚嶃€備粎鍖呭惈鐩稿璺緞鎴栨枃浠跺悕鐨?$ref 鍊间細琚姞涓婂綋鍓?schema 鏂囦欢 '$id' 鍊肩殑涓绘満鍚?
  鍜岃矾寰勯儴鍒嗐€傚嵆浣垮浜庢湰鍦版枃浠朵篃浣跨敤 URL锛屼絾瀹為檯鍙兘骞朵笉瀛樺湪浣嶄簬杩欎簺浣嶇疆鐨勬枃浠躲€?

$schema
  鎸囨槑璇?schema 鏂囦欢鎵€閬靛惊鐨勫厓妯″紡锛坢eta-schema锛夈€?

title
  涓€琛屾弿杩帮紝璇存槑缁戝畾 schema 涓墍鎻忚堪纭欢鐨勫唴瀹广€?

maintainers
  DT 鐗规湁鐨勫睘鎬с€傚寘鍚竴涓垨澶氫釜缁存姢璇ョ粦瀹氱殑缁存姢鑰呯殑鐢靛瓙閭欢鍦板潃鍒楄〃銆?

description
  鍙€夈€備竴涓琛屾枃鏈潡锛屽寘鍚叧浜庤纭欢鐨勪换浣曡缁嗕俊鎭€傚畠搴斿寘鍚濡傝妯″潡鎴栬澶囩殑浣滅敤銆?
  璁惧鎵€閬靛惊鐨勬爣鍑嗭紝浠ュ強鎸囧悜鏁版嵁鎵嬪唽浠ヨ幏鍙栨洿澶氫俊鎭殑閾炬帴绛夊唴瀹广€?

  YAML 鏍煎紡鏈夊嚑绉嶅畾涔夋枃鏈潡鏍煎紡鐨勯€夐」銆傝繖浜涢€夐」鐢遍敭鍚庨潰鐨勬寚绀虹瀛楃鎺у埗锛堜緥濡?
  "description: \|"锛夈€傚簲浣跨敤鏂囨湰鍧楁墍闇€鐨勬渶灏忔牸寮忋€傛牸寮忔帶鍒朵笉浠呬細褰卞搷 YAML 鏄惁鑳借
  姝ｇ‘瑙ｆ瀽锛岃€屼笖鍦ㄥ皢鏂囨湰鍧楁覆鏌撲负鍏朵粬褰㈠紡鏃朵篃寰堥噸瑕併€傞€夐」濡備笅銆?

  娌℃湁浠讳綍鎸囩ず绗︾殑榛樿鏄祦寮忥紙flowed锛夌殑绾爣閲忥紙plain scalar锛夐鏍硷紝浼氬幓鎺夊崟鎹㈣绗﹀拰
  鍓嶅绌虹櫧銆傛钀界敱绌鸿锛堝嵆鍙屾崲琛岀锛夊垎闅斻€傝繖绉嶉鏍间笉鑳藉寘鍚?": "锛屽洜涓哄畠浼氳瑙ｉ噴涓洪敭銆?
  浠讳綍 " #" 搴忓垪閮戒細琚В閲婁负娉ㄩ噴銆傚鍏朵粬瀛楃涔熸湁鏇村闄愬埗銆傚ぇ澶氭暟闄愬埗鏄叧浜庨瀛楃鍙互
  鏄粈涔堛€?

  绗簩绉嶉鏍兼槸鎶樺彔锛坒olded锛夛紝鐢?">" 瀛楃鎸囩ず銆傞櫎浜嗗湪鍙屾崲琛岀澶勪繚鐣欐崲琛屽锛屾姌鍙犻鏍艰繕
  淇濈暀瓒呭嚭棣栬缂╄繘鐨勫墠瀵肩┖鐧姐€傜缉杩涜涓婄殑鎹㈣绗︿篃浼氳淇濈暀銆?

  绗笁绉嶉鏍兼槸瀛楅潰锛坙iteral锛夛紝鐢?"\|" 瀛楃鎸囩ず銆傚瓧闈㈤鏍间繚鐣欐墍鏈夋崲琛岀鍜岀┖鐧斤紙瓒呭嚭
  棣栬缂╄繘鐨勯儴鍒嗭級銆?

  浠ヤ笂骞堕潪瀵?YAML 鏂囨湰鍧楃殑瀹屾暣鎻忚堪銆傚叧浜庡琛?YAML 鏂囨湰鍧楃殑鏇村缁嗚妭鍙互鍦ㄧ綉涓婃壘鍒帮細

  https://yaml-multiline.info/

  https://www.yaml.info/learn/quote.html

select
  鍙€夈€備竴涓?json-schema锛岀敤浜庡尮閰嶈搴旂敤璇?schema 鐨勮妭鐐广€傞粯璁ゆ儏鍐典笅锛屽湪娌℃湁 'select'
  鏃讹紝鑺傜偣浼氫緷鎹叾鍙兘鐨?compatible 瀛楃涓插€兼垨鑺傜偣鍚嶈繘琛屽尮閰嶃€傚ぇ澶氭暟缁戝畾涓嶉渶瑕?select銆?

allOf
  鍙€夈€傝鍖呭惈鐨勫叾浠?schema 鐨勫垪琛ㄣ€傜敤浜庡寘鍚缁戝畾鎵€閬靛惊鐨勫叾浠?schema銆傝繖浜涘彲浠ユ槸鏌愮被
  璁惧锛堜緥濡?I2C 鎴?SPI 鎺у埗鍣級鐨?schema銆?

properties
  涓€缁勫瓙 schema锛屽畾涔夎缁戝畾鐨勬墍鏈?DT 灞炴€с€傚叿浣撶殑 schema 璇硶鍙栧喅浜庡睘鎬ф槸宸茬煡鐨勫叕鍏?
  灞炴€э紙渚嬪 'interrupts'锛夎繕鏄粦瀹?鍘傚晢鐗瑰畾鐨勫睘鎬с€?

涓€涓睘鎬т篃鍙互瀹氫箟涓€涓瓙 DT 鑺傜偣锛屽叾涓嬪畾涔夊瓙灞炴€с€?

鍏充簬 properties 閮ㄥ垎鐨勬洿澶氱粏鑺傦紝璇峰弬闃?'Property Schema' 涓€鑺傘€?

patternProperties
  鍙€夈€備笌 'properties' 绫讳技锛屼絾鍚嶇О鏄鍒欒〃杈惧紡銆?

required
  鏉ヨ嚜 'properties' 鑺傘€佸繀椤诲缁堝瓨鍦ㄧ殑 DT 灞炴€у垪琛ㄣ€?

additionalProperties / unevaluatedProperties
  鎺у埗 schema 濡備綍楠岃瘉鏈鏈?schema 鐨?'properties' 鎴?'patternProperties' 鍖归厤鍒扮殑
  灞炴€х殑鍏抽敭瀛椼€傛瘡涓?schema 閮藉簲鍦ㄩ《灞傞儴鍒嗘伆濂藉寘鍚繖浜涘叧閿瓧涔嬩竴锛屽嵆 additionalProperties
  鎴?unevaluatedProperties銆傚祵濂楄妭鐐癸紙鍗充綔涓哄璞＄殑灞炴€э級涔熷簲鍖呭惈涓€涓€?

  - additionalProperties: false
      鏈€甯歌鐨勬儏鍐碉紝鍗充笉寮曠敤棰濆鐨?schema锛屾垨鑰呮湰缁戝畾鍏佽鏉ヨ嚜鍏朵粬琚紩鐢?schema 鐨勫睘鎬?
      鐨勫瓙闆嗐€?

  - unevaluatedProperties: false
      褰撴湰缁戝畾寮曠敤浜嗗叾浠?schema锛屽苟涓斿簲鍏佽鍏舵墍鏈夊睘鎬ф椂浣跨敤銆?

  - additionalProperties: true
      - 椤跺眰閮ㄥ垎锛?
        缃曡鎯呭喌锛岀敤浜庡疄鐜颁竴缁勫叕鍏卞睘鎬х殑 schema銆傛绫?schema 搴旇鍏朵粬 schema 寮曠敤锛屽悗鑰?
        鍐嶄娇鐢?'unevaluatedProperties: false'銆傞€氬父鏄€荤嚎鎴栧叕鍏遍儴鍒嗙殑 schema銆?
      - 宓屽鑺傜偣锛?
        褰撲粎鍒楀嚭宓屽鑺傜偣鐨勬湡鏈?compatible锛屽苟涓斿瓨鍦ㄥ彟涓€涓尮閰嶈 compatible銆佷互涓婅堪涓ょ
        鎯呭喌涔嬩竴锛?false'锛夌粨灏剧殑 schema 鏃躲€?

examples
  鍙€夈€傚疄鐜颁竴涓垨澶氫釜浠呭寘鍚湰缁戝畾鐨?DTS 鐗囨鐨勫垪琛ㄣ€傜ず渚嬩笉搴斿寘鍚笉鐩稿叧鐨勮澶囪妭鐐癸紝渚嬪
  鍦?provider 缁戝畾涓殑 consumer 鑺傜偣锛屾垨鍏朵粬閫氳繃 phandle 寮曠敤鐨勮妭鐐广€?
  娉ㄦ剰锛歒AML 涓嶅厑璁镐娇鐢ㄥ墠瀵煎埗琛ㄧ锛屽洜姝ゅ繀椤绘敼鐢ㄧ┖鏍笺€?

闄ら潪鍙︽湁璇存槑锛屾墍鏈夊睘鎬ч兘鏄繀闇€鐨勩€?

### 灞炴€?Schema锛圥roperty Schema锛?


schema 鐨?'properties' 閮ㄥ垎鍖呭惈鏌愪釜缁戝畾鐨勬墍鏈?DT 灞炴€с€傛瘡涓睘鎬у寘鍚竴缁勪娇鐢ㄨ灞炴€?
json-schema 璇嶆眹鐨勭害鏉熴€傚睘鎬?schema 鐢ㄤ簬瀵?DT 鏂囦欢杩涜楠岃瘉銆?

瀵逛簬鍏叡灞炴€э紝鍙渶瑕佸畾涔夊叕鍏辩粦瀹?schema 鏈兜鐩栫殑棰濆绾︽潫锛屼緥濡傛湁澶氬皯涓€兼槸鏈夋晥鐨勬垨
鍝簺鍙兘鐨勫€兼槸鏈夋晥鐨勩€?

鍘傚晢鐗瑰畾鐨勫睘鎬ч€氬父闇€瑕佹洿璇︾粏鐨?schema銆傞櫎甯冨皵灞炴€у锛屽畠浠簲寮曠敤 schemas/types.yaml 涓殑
鏌愪釜绫诲瀷銆傚缁堥渶瑕佷竴涓?"description" 灞炴€с€?

璁惧鏍?schema 涓?dtc 浜х敓鐨?YAML 缂栫爜鐨?DT 鏁版嵁骞朵笉瀹屽叏鍖归厤銆傚畠浠绠€鍖栦簡锛屼互浣垮叾鏇?
绱у噾骞堕伩鍏嶅ぇ閲忔牱鏉裤€傚伐鍏蜂細澶勭悊 schema 鏂囦欢浠ョ敓鎴愮敤浜庨獙璇佺殑鏈€缁?schema銆傜洰鍓嶅伐鍏锋墽琛?
涓ょ杞崲銆?

json-schema 涓暟缁勭殑榛樿鎯呭喌鏄彉闀跨殑锛屽苟涓斿厑璁告瘮鏄惧紡瀹氫箟鐨勬洿澶氱殑鏉＄洰銆傝繖鍙互閫氳繃瀹氫箟
'minItems'銆?maxItems' 鍜?'additionalItems' 鏉ラ檺鍒躲€傜劧鑰岋紝瀵逛簬璁惧鏍?Schema锛屽湪澶у鏁?
鎯呭喌涓嬮渶瑕佸浐瀹氬ぇ灏忥紝鍥犳杩欎簺灞炴€т細鏍规嵁 'items' 鍒楄〃涓殑鏉＄洰鏁伴噺娣诲姞銆?

YAML 璁惧鏍戞牸寮忚繕灏嗘墍鏈夊瓧绗︿覆鍊煎彉涓烘暟缁勩€佸皢鏍囬噺鍊煎彉涓虹煩闃碉紙浠ヤ究瀹氫箟鍒嗙粍锛夛紝鍗充娇鍙湁
鍗曚釜鍊兼椂涔熸槸濡傛銆俿chema 涓殑鍗曚釜鏉＄洰浼氳淇浠ュ尮閰嶈繖绉嶇紪鐮併€?

褰撶粦瀹氳鐩栧涓湪鏌愪簺灞炴€т笂涓嶅悓鐨勭浉浼艰澶囨椂锛屽簲瀵规瘡涓澶囩殑杩欎簺灞炴€у姞浠ョ害鏉熴€傝繖閫氬父
鎰忓懗鐫€锛?

 - 鍦ㄩ《灞?'properties' 涓畾涔夊叿鏈夋渶瀹芥硾绾︽潫鐨勫睘鎬с€?
 - 鍦?'if:then:' 鍧椾腑锛岃繘涓€姝ユ敹绐勮繖浜涘睘鎬х殑绾︽潫銆?
 - 涓嶈鍦?'if:then:' 鍧楀唴瀹氫箟灞炴€э紙娉ㄦ剰 'additionalItems' 涔熶笉鍏佽閭ｆ牱鍋氾級銆?

### 浠ｇ爜椋庢牸锛圕oding style锛?


浣跨敤 YAML 浠ｇ爜椋庢牸锛堜袱绌烘牸缂╄繘锛夈€傚浜?schema 涓殑 DTS 绀轰緥锛屽缓璁娇鐢ㄥ洓绌烘牸缂╄繘銆?

灏?'properties' 鍜?'required' 鑺備腑鐨勬潯鐩寜鐩稿悓椤哄簭鎺掑垪锛屼娇鐢?
Documentation/devicetree/bindings/dts-coding-style.rst 涓殑椋庢牸銆?

### 娴嬭瘯


#### 渚濊禆


蹇呴』瀹夎 DT schema 椤圭洰锛屼互渚块獙璇?DT schema 缁戝畾鏂囨。骞朵娇鐢?DT schema 楠岃瘉 DTS 鏂囦欢銆?
DT schema
```
    pip3 install dtschema
```
娉ㄦ剰锛?dtschema' 鐨勫畨瑁呴渶瑕?'swig' 鍜?Python 寮€鍙戞枃浠?
```
    apt install swig python3-dev
```
浼氬畨瑁呭嚑涓彲鎵ц鏂囦欢锛坉t-doc-validate銆乨t-mk-schema銆乨t-validate锛夈€傝纭繚瀹冧滑鍦ㄤ綘鐨?
PATH 涓紙榛樿涓?~/.local/bin锛夈€?

杩樺缓璁畨瑁?yamllint锛堝湪瀛樺湪鏃剁敱 dtschema 浣跨敤锛夈€?

#### 杩愯妫€鏌?


DT schema 缁戝畾鏂囨。蹇呴』浣跨敤鍏冩ā寮忥紙meta-schema锛屽嵆 schema 鐨?schema锛夎繘琛岄獙璇侊紝浠ョ‘淇?
瀹冧滑鏃㈡槸鏈夋晥鐨?json-schema锛屼篃鏄湁鏁堢殑缁戝畾 schema銆傛墍鏈?DT 缁戝畾鏂囨。閮藉彲浠ラ€氳繃
```
    make dt_binding_check
```
```
    make sram/sram.yaml
```
```
    make dtbs_check
```
娉ㄦ剰锛宍dtbs_check` 浼氳烦杩囦换浣曟湁閿欒鐨勭粦瀹?schema 鏂囦欢銆傚繀椤讳娇鐢?`dt_binding_check` 鎵嶈兘
鑾峰緱缁戝畾 schema 鏂囦欢涓殑鎵€鏈夐獙璇侀敊璇€?
```
    make dt_binding_check dtbs_check
```
涔熷彲浠ュ皢杩愯涓婅堪鍛戒护涓庝竴閮ㄥ垎鍖归厤鐨?schema 鏂囦欢缁撳悎璧锋潵锛屾柟娉曟槸鎶?`DT_SCHEMA_FILES`
鍙橀噺璁剧疆涓轰竴涓垨澶氫釜鐗瑰畾鐨?schema 鏂囦欢鎴栨ā寮忥紙鍥哄畾瀛楃涓茬殑閮ㄥ垎鍖归厤锛夈€傛瘡涓枃浠舵垨妯″紡
搴斾互 ':' 鍒嗛殧銆?
```
    make dt_binding_check DT_SCHEMA_FILES=trivial-devices.yaml
    make dt_binding_check DT_SCHEMA_FILES=trivial-devices.yaml:rtc.yaml
    make dt_binding_check DT_SCHEMA_FILES=/gpio/
    make dtbs_check DT_SCHEMA_FILES=trivial-devices.yaml


```
### json-schema 璧勬簮


`JSON-Schema Specifications <http://json-schema.org/>`_

`Using JSON Schema Book <http://usingjsonschema.com/>`_

### 甯︽敞閲婄殑绀轰緥 Schema


涔熷彲浣滀负鍗曠嫭鐨勬枃浠惰幏鍙栵細`example-schema.yaml`
