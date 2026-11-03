-- O10（2026-11-03）：x_general_assemble_office.word_flag 列类型修正 boolean → text。
-- 背景：036 迁移定义 word_flag VARCHAR（o2 语义：html→word 转换任务的查询凭据，uuid 文本）；
-- 101 迁移误以 BOOLEAN DEFAULT false 补列，handler 插 uuid 字符串必炸
-- （WrongType/22P02 → 500，IDOR 矩阵 O10 实证：POST /api/general/assemble/control/office/html/to/word）。
-- 改列类型而非改 handler：boolean 在类型层面无法承载「唯一查询凭据」语义。
-- 回滚见 116_*_rollback.sql；注意回滚要求列中无非 boolean 文本（有 uuid 数据时回滚会失败——
-- 该列本就只为承载 uuid 而设，回滚前应清空或导出）。
ALTER TABLE "x_general_assemble_office" ALTER COLUMN "word_flag" DROP DEFAULT;
ALTER TABLE "x_general_assemble_office" ALTER COLUMN "word_flag" TYPE TEXT USING "word_flag"::text;
