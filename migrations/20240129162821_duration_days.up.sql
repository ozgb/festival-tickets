ALTER TABLE tickets_purchased
RENAME TO orders;

ALTER TABLE orders
ADD duration_days smallint CHECK (duration_days = 3 OR duration_days = 4);

CREATE TABLE order_stats (
    duration_days integer PRIMARY KEY,
    order_limit integer NOT NULL,
    order_count integer DEFAULT 0
);

INSERT INTO order_stats (duration_days, order_limit, order_count)
VALUES (3, 300, 0)
ON CONFLICT (duration_days) DO NOTHING;

INSERT INTO order_stats (duration_days, order_limit, order_count)
VALUES (4, 200, 0)
ON CONFLICT (duration_days) DO NOTHING;


CREATE FUNCTION update_order_stats()
    RETURNS TRIGGER
    LANGUAGE PLPGSQL
AS $$
DECLARE
    cur_order_limit order_stats.order_limit%type;
    cur_order_count order_stats.order_count%type;
BEGIN
    SELECT os.order_limit, os.order_count
    FROM order_stats as os
    WHERE os.duration_days = NEW.duration_days
    INTO cur_order_limit, cur_order_count;

    if NEW.duration_days = 3 OR NEW.duration_days = 4 THEN
        IF cur_order_count >= cur_order_limit THEN
            RAISE EXCEPTION 'Order limit = % reached for duration_days = %', cur_order_limit, NEW.duration_days;
        END IF;

        UPDATE order_stats
        SET order_count = order_count + 1
        WHERE duration_days = NEW.duration_days;
    ELSE
        RAISE EXCEPTION 'Invalid duration_days = %. Should be 3 or 4.', NEW.duration_days;
    END IF;

    RETURN NEW;
END;
$$;

CREATE TRIGGER check_order_limit
BEFORE INSERT ON orders
FOR EACH ROW EXECUTE FUNCTION update_order_stats();
