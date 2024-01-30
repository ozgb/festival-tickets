DROP TRIGGER IF EXISTS check_order_limit ON orders;

DROP FUNCTION IF EXISTS update_order_stats;

DROP TABLE IF EXISTS order_stats;

ALTER TABLE IF EXISTS orders
DROP COLUMN IF EXISTS duration_days;

ALTER TABLE IF EXISTS orders
RENAME TO tickets_purchased;
