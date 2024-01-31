-- Delete trigger and function
DROP TRIGGER IF EXISTS subtract_order_count ON orders;
DROP FUNCTION IF EXISTS subtract_order_count_on_delete;
