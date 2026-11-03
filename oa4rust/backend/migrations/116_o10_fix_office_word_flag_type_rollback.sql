-- 116_o10_fix_office_word_flag_type 的回滚。
-- 前提：列中无非 boolean 文本（'true'/'false'/'t'/'f' 之外的值会让 USING 转换报错）。
ALTER TABLE "x_general_assemble_office" ALTER COLUMN "word_flag" TYPE BOOLEAN USING ("word_flag"::boolean);
ALTER TABLE "x_general_assemble_office" ALTER COLUMN "word_flag" SET DEFAULT false;
