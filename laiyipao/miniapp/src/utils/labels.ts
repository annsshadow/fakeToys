/**
 * 界面用词表（2026-10-21 P6-5）。
 *
 * # 为什么要有这个文件
 *
 * 此前「货币名」这个映射在**三个页面各写一份**
 * （`battle.vue` / `me.vue` / `signin.vue`），三份的内容**已经不一样了**：
 * 一份多一个道具名、一份少一个。
 *
 * 后果不是「重复代码不好看」，而是：改一个货币名要改三处，
 * 漏一处就出现「同一个东西在两个页面叫两个名字」——
 * 而玩家会以为那是**两种东西**。
 *
 * 同样的问题还有一处更隐蔽的：服务端返回的**英文 key** 直接上屏
 * （宝石品质 `gray` / `blue`、词条 `attack` / `crit`），
 * 玩家看到的是 `gray` 而不是「灰」。未知值一律回落成
 * 「原始 key」—— 那比中文差，但**至少不撒谎**（它确实不认识）。
 */

/** 货币 / 道具的中文名。 */
const CURRENCY_NAME: Record<string, string> = {
  coin: '金币',
  gem: '钻石',
  energy: '体力',
  keys: '钥匙',
  revive_token: '复活币',
  mastery_reset: '专精重铸券',
  gem_wash_token: '宝石洗练券',
}

/**
 * 货币 / 道具的中文名。
 *
 * 未知值**原样返回**（不吞掉服务端新增的种类）——
 * 吞掉的话，新加一种道具在界面上会显示成空白，
 * 而空白看起来像「数量是 0」。
 */
export function currencyName(k: string): string {
  return CURRENCY_NAME[k] ?? k
}

/** 宝石品质的中文名（key 与服务端 `domain.GemQualities` 一致）。 */
const QUALITY_NAME: Record<string, string> = {
  gray: '灰',
  blue: '蓝',
  purple: '紫',
  red: '红',
  gold: '金',
}

/** 宝石品质的中文名；未知品质原样返回（不要吞掉后端新增的档位）。 */
export function qualityName(q: string): string {
  return QUALITY_NAME[q] ?? q
}

/** 宝石词条的中文名。 */
const AFFIX_NAME: Record<string, string> = {
  attack: '攻击',
  crit: '暴击',
  crit_multiplier: '暴击倍率',
  element_coef: '元素系数',
  reaction_mult: '反应倍率',
  element_cap: '元素层数',
  heat_cap: '热量上限',
  armor: '护甲',
}

/** 宝石词条的中文名；未知词条原样返回。 */
export function affixName(a: string): string {
  return AFFIX_NAME[a] ?? a
}
