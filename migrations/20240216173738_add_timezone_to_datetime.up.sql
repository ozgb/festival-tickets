ALTER TABLE IF EXISTS orders
ALTER COLUMN reserved_until TYPE timestamp with time zone;

ALTER TABLE IF EXISTS orders
ALTER COLUMN purchased_at TYPE timestamp with time zone;
