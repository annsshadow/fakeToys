# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L342 / A349①：`not_a_commit` 整档逐枚点名（不再只按区数总量）。

L299 落地②时只把「9 位 + 同行含批/提交」的批次子集按哈希钉住 254 枚，
其余 273 处仍按区总量钉 —— 一枚写错一位的非批次哈希会安静落进这一档。
本轮按题 7「接受名册重钉成本」把整档唯一哈希全部点名：
错一位必产生新哈希 ⇒ 名册精确相等当场红；新增一枚也要按现量重钉并留归因。

零 parametrize（l97 / l98 把全仓 parametrize 三面钉成现量数）。
"""

from tests.unit.test_ledger_hash_reachability_l270 import (
    MEASURED_DEFECTS, NOTCOMMIT, VOLUMES, batch_citation_rows, defect_cells, sites,
)
from tests.unit.test_backlog_mark_vocab_l253 import load_volumes

MEASURED_ALL_HASHES = 423
MEASURED_NOTCOMMIT_SITES = 730

NOTCOMMIT_ALL_HASHES = frozenset({
    "0101470c5", "015b2ca2e", "0255b005c", "0341cafaf",
    "0385ceb67", "043ee69d3", "0451031d9", "049dea678",
    "04d55eb46080", "06e253bc9", "06f5383e3", "0778cb240",
    "08b9c9537", "08bc4f0c8", "08c0617d4", "0a19fe46c",
    "0a5b59c09", "0a9c210c5", "0b32da27f", "0ba6943e13a7",
    "0bf96fbec", "0c61ba3dd", "0c9a34fa1", "0d2cace8",
    "0d5040f24", "0ecb19f7bec9", "0ed4e1749dc6", "0f9d59d75ccc",
    "0fa56aed881e", "1094b0696", "111d63f1c", "11257a75a",
    "117517d22", "13e78b589e8d", "13ec11eaa", "140de575e9e9",
    "147fbb613", "15c340e80", "167e165c9c10", "16934dc430c2",
    "169e1d87ac76", "16a5dabc9", "16b5bf4c6", "197f5aec113d",
    "19c96b420", "1a1a203c4", "1a431da73", "1a82af467",
    "1aac3b35d", "1b74bab3b", "1c5cdb0b6", "1ca3dd319",
    "1d62df0d9", "1e4df8881", "1e590559b", "1e8e130bd",
    "1ebd2c8ae", "2020a211", "215b306d5", "216108e948e7",
    "21745677c1d0", "219495b4a44c", "21c3ae1f5", "222d45a39",
    "229b545fb", "22e0f51f3", "24d6c3de5", "24faf37ff",
    "250806723", "277277f30", "2788ef66b", "27e15fea",
    "280cd1d88cf8", "284d89e29", "28e3a1264", "29b715600",
    "2a04e1ed3", "2a393842b1cb", "2a9fdcd44", "2b2d8fb03",
    "2d71358df", "2eb7c8c07", "2f14ab265", "2f994fbc7",
    "2fc72b8f2", "2fd83c9b2", "30311ea7d", "30d3bcfb9eb7",
    "31fd3f1fe", "32028f73b", "334e94a40", "33f98dfae",
    "34b525ee5", "34c3e89f9", "34fca5785", "3661c9874",
    "37948d428", "38d4879af", "396e4baf6", "39e968d47",
    "39eaa30b0", "3b0d513a6", "3bf7100a34ec", "3c4dee1c99b2",
    "40b93423c159", "40d2270b8", "41024a400", "41c7e6c13665",
    "42f8a95a1", "43fb0e31d", "4495c585f", "453030931858",
    "4548815bc", "45b32e651d", "45b32e651dad", "462dd405f",
    "46b82e07b", "4860e64a2", "4863e1357", "486c4f6ec",
    "498c9253dd4f", "49b70b3801", "49c0ca965", "4a445da5d",
    "4aaea95124e1", "4acf1434f", "4b3854f47", "4be6d91ba",
    "4c979939e", "4d155cfc4", "4d177e3f3", "4d29342cf",
    "4d434fcec", "4d456d471", "4ea67aaac", "4eff241a3",
    "4fe3fff419ec", "502f83dd7", "50da13633", "51096d9fc",
    "518d4d056", "51ca546db", "5261e0140", "541cec6274",
    "5461ba6b1", "54df44599ab4", "5559e8768", "5655dd35f",
    "565d57aa3", "566e590a6084d71336b4d4f387497b6d7d60631d", "56c4565ff", "57113a4e3",
    "587022b11", "58a1f9987", "58b568468a3c4978", "58cc1fe18c4c00e9",
    "59bb04cac", "5aa1cca7d46c", "5be3d9224", "5c0481611",
    "5c9294ed3", "5d37639ee", "5d80610c6", "5d821c32b",
    "5e45ece2b", "603c88d49", "615ff1df2", "61d075bf3",
    "633574b31", "63ef9368e", "63f39ebd5", "6611607fa",
    "669e0132b", "6856f28a", "68b929bb8", "68e3ff6d8b5ca4e8",
    "6b44aa3a37d0", "6b64d3842", "6bf1f34ca", "6bf5bf1564b4",
    "6c141d7d5", "6c51decca", "6c53d986c", "6d800cb2f",
    "6e2edfb92", "6e63becb3", "6e9bea578", "6f405bf0099ef1e9",
    "6f62ece26", "72b63c432", "72e691b210df", "736bcc9cc",
    "760c46e80", "76aeace0d4096f26", "7744abbc6", "798206eed79354a5",
    "79fcb31f6", "7a6d246f0cd5", "7ab30f256", "7c5666a90",
    "7cca1596a", "7d4d8e362", "7da7190d8", "7e2fe5aa6",
    "7eec9cd84b", "7eec9cd84b21", "7f292d8d2", "7f5bda3fc",
    "8041df804", "807a2beec", "80f36337e", "80f8bb5de",
    "8148f288c", "820821fda", "82ba55ab2", "835d15d4d",
    "86b23a45a", "87174e26b", "88837aa2a", "890965d32",
    "89877d7bdbbf", "8a034b445ab3", "8b17a662", "8b457088",
    "8c7f0201", "8c85ab0b3", "8d1cfb2fb", "8e240357d",
    "8e3356d89", "8e41467a1", "8e6b478a4", "8ed565753",
    "8ede5113e", "8f6256d1b6e7", "90aa931b8", "922689156e3e",
    "92394836f", "925610098", "92729896b", "930e31f74",
    "932c5885a", "94ef1729f8", "94f7f05cc", "94f7f09a2",
    "95ac0a1aa3", "969852845", "97e0f1617f2f", "980cd313b",
    "9abc72c67", "9af6ded88", "9ba59b493", "9c745e187",
    "9c9a13ec1", "9ca6f7849", "9d186e424", "9d417eda7",
    "9dd0cfa6d", "9dfbe3aa7", "9e8362dbf195607a", "9eabfb404c76",
    "9eb75f6a8", "9ec8d806b", "9ed11b369", "9fe7ba89e",
    "a0a2bc5c3", "a18c8567c", "a21320ec6", "a2136c9ea",
    "a266f62f7", "a561c0835", "a6e4748c6", "a76ae612b",
    "a811fbe91", "a96b2c5c9928", "a97d57710", "aafbcbcc4",
    "ab03fbec93fb", "ad3661fdd", "ad498ced5", "adc0df075",
    "ae96baa52", "af03b67b3", "af899524b644", "afc702eff",
    "b0d77707eb37", "b13e265ad", "b14a118c6", "b18bb3ad8bf3",
    "b268e921bc94b8f0", "b2af89396", "b318cf20a", "b52b39780",
    "b5472adc9", "b5ebb1e8b", "b62cd47d5be7", "b63648018",
    "b70266b2641268065c82a28184cce353bd2d2932", "b7b7f0b94", "b7f1b6f17", "b85aacf32",
    "b8de51fc6bf0", "b949f7929", "b9d3ad7b1", "bc6224ee9961",
    "bc6f4ee4f", "bccfeff85", "bd8b808b0", "bdbbcb59e3",
    "bdc8c35ff659", "be06c5dfe", "be42558ffa1f", "be6c8ac9275a4737",
    "bec20fa65e", "bf2641aab", "bfa38c4f", "c019dd097d",
    "c08944aa0", "c0c3089f7", "c15781904", "c1734be35",
    "c17d1224", "c17d122404c30688de4f69a767f6be3e84262152", "c2290f1eb", "c2735dd69",
    "c2d369fc4742", "c3183fc0d", "c3a2b9441", "c52e40256f07",
    "c6209ab67", "c654ecceb", "c7162ccd81b9", "c772f9a5b",
    "c97219355", "cc1a80b8", "cd2391d3b", "cd47d9845",
    "cd56b4498", "cdf5310c9", "ce4b6b390", "ce69f40c16c3",
    "ce9b3c1ba", "cf35b5984", "cfffcc919", "d01722c9e",
    "d11053a30", "d171524a0", "d17cd5b56", "d3659f6251",
    "d3659f6251d5", "d3ab8c78f", "d510340f8", "d55087a6a7b0",
    "d5601369b", "d5617fdaf08d", "d5a990e6ad", "d90148b38",
    "d91e20803", "d9578f87b", "d9c65f255a2a20a4", "d9ead53868047ee9",
    "da10bb97b864", "dae90fec", "dae90fec858bfe0fa0fc88f8bef31621cdc42dc9", "db2292677b3f",
    "db2fd4838", "dbdec3aed485", "dd4c1b500", "deb74c37b",
    "deb9bb3c9", "df54edca9a61", "dfb9ab0ae", "dff6c552a",
    "e0457184e0", "e0457184e038", "e0de32b94", "e1cf45f7df",
    "e1cf45f7df50", "e2665d612", "e3baad45a", "e3cef17cc",
    "e4ce1ae36", "e50a3726d", "e50deb0f5", "e563850d3",
    "e5f5ed044a9f", "e6393e95b", "e6718f828", "e6c14dfaf",
    "e79538569", "e7b41d52b3fe", "e7c39adc1", "e7dfa43ab",
    "e7e051279", "e864debe0", "e958d03c0", "e9722a89d092ebf6",
    "eb8edaea", "eb8edaeac2dcc3550b096092f0cc4b45b9c1532a", "ec0668115", "ed2409452",
    "ed27cc4cc", "edb26ddbe", "ee133fc40", "ee3b12404",
    "eedd6f0b0425", "f1374a942", "f2263a78f", "f531e62e3",
    "f5ae37705", "f74d41c19", "f7f428134", "f9b5d001d90e",
    "fb0ba6e91", "fbc455bf4", "fbf5052ec", "fc143249a",
    "fc1c8f8bc02c", "fd0b6c672", "fdc8a5e4a", "fdfd19f2f",
    "fe5836f8f", "febbe275b", "ff6a4b09438a6b40cc209012152d94f6566788b6",
})


def test_the_whole_tier_is_named_hex_by_hex():
    """整档双向精确相等：多一枚（写错一位/新写一笔）少一枚都红"""
    got = {r["hex"] for r in sites() if r["tier"] == NOTCOMMIT}
    assert got == set(NOTCOMMIT_ALL_HASHES), (
        "新冒出来的", sorted(got - set(NOTCOMMIT_ALL_HASHES)),
        "消失的", sorted(set(NOTCOMMIT_ALL_HASHES) - got))


def test_the_roster_and_site_scales_are_pinned():
    assert len(NOTCOMMIT_ALL_HASHES) == MEASURED_ALL_HASHES
    assert len([r for r in sites() if r["tier"] == NOTCOMMIT]) == MEASURED_NOTCOMMIT_SITES


def test_a_fabricated_hash_is_caught_by_the_full_roster():
    """注入臂：编一枚 11 位哈希 ⇒ 整档名册当场点名它（② 时代的盲区已收掉）"""
    volumes = load_volumes()
    lines = volumes[VOLUMES[1]]
    volumes[VOLUMES[1]] = lines[:2] + ["- [x] **L900** 编造批 `fffffffffff` 一枚"] + lines[2:]
    try:
        got = sites(volumes)
        fresh = [r for r in got if r["hex"] == "fffffffffff"]
        assert [r["tier"] for r in fresh] == [NOTCOMMIT]
        assert defect_cells(got) == MEASURED_DEFECTS  # 缺陷格仍空，靠名册抓
        whole = {r["hex"] for r in got if r["tier"] == NOTCOMMIT}
        assert whole - set(NOTCOMMIT_ALL_HASHES) == {"fffffffffff"}
    finally:
        volumes[VOLUMES[1]] = lines


def test_the_batch_subset_is_still_a_subset_of_the_full_roster():
    """② 的 254 枚必须仍被 ① 全档名册覆盖，两代判据不许分家"""
    from tests.unit.test_ledger_hash_reachability_l270 import NOTCOMMIT_BATCH_HASHES
    assert set(NOTCOMMIT_BATCH_HASHES) <= set(NOTCOMMIT_ALL_HASHES)
    assert {r["hex"] for r in batch_citation_rows(sites())} <= set(NOTCOMMIT_ALL_HASHES)

