-- 107: BBS 板块图标（picture/section/{id}/icon 落库；十类功能3）
ALTER TABLE bbs_section_info ADD COLUMN IF NOT EXISTS icon TEXT;
