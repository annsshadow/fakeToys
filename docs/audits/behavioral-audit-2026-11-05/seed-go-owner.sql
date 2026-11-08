INSERT INTO battle_records (user_id, level_id, result, stars, created_at) VALUES (2, 1, 'win', 3, now());
INSERT INTO orders (user_id, order_no, sku, amount, status, provider, provider_txn, created_at)
VALUES (2, 'AUDIT-ORDER-B', 'sku-audit', 100, 'pending', 'mock', '', now());
SELECT 'battle' AS t, id, user_id FROM battle_records WHERE user_id=2 ORDER BY id DESC LIMIT 1;
SELECT 'order' AS t, id, user_id FROM orders WHERE user_id=2;
