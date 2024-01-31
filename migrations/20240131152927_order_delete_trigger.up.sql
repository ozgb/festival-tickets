CREATE FUNCTION subtract_order_count_on_delete()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS $$
BEGIN
    UPDATE order_stats
    SET order_count = order_count - 1
    WHERE duration_days = NEW.duration_days;

    RETURN OLD;
END;
$$;

CREATE TRIGGER subtract_order_count
BEFORE DELETE ON orders
FOR EACH ROW EXECUTE FUNCTION subtract_order_count_on_delete();
