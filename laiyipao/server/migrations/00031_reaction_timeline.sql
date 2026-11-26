-- Copyright (C) 2026 annsshadow
-- SPDX-License-Identifier: AGPL-3.0-or-later

-- R15：battle_records 新增逐次反应时间线（仅展示，非权威结算数据）。
--
-- 与 elements_used/reactions_used 分列存储：时间线只作战报回顾，
-- 不参与结算裁决/score/loot/星级/replay_hash，也不进运营反应分布聚合。
-- NOT NULL DEFAULT '[]' 保证存量行读到的是诚实空数组（旧版本结算），
-- 不做任何编造。Up 幂等，Down 只删本列。

-- +goose Up

ALTER TABLE battle_records
    ADD COLUMN IF NOT EXISTS reaction_timeline JSONB NOT NULL DEFAULT '[]'::jsonb;

-- +goose Down

ALTER TABLE battle_records
    DROP COLUMN IF EXISTS reaction_timeline;
