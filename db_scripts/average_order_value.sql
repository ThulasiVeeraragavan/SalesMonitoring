-- SELECT * FROM average_order_value('2023-01-01', '2024-12-31');

CREATE OR REPLACE FUNCTION average_order_value(start_date TEXT, end_date TEXT)
RETURNS TABLE (status_id INT, result_value DOUBLE PRECISION)
LANGUAGE plpgsql
AS $$
DECLARE
    valid_start DATE;
    valid_end DATE;
    avg_value DOUBLE PRECISION;
BEGIN
    
    BEGIN
        IF start_date !~ '^\d{4}-\d{2}-\d{2}$' OR end_date !~ '^\d{4}-\d{2}-\d{2}$' THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::DOUBLE PRECISION AS result_value;
            RETURN;
        END IF;

        valid_start := start_date::DATE;
        valid_end := end_date::DATE;
    EXCEPTION
        WHEN others THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::DOUBLE PRECISION AS result_value;
            RETURN;
    END;

    
    SELECT COALESCE(AVG(oi.quantity_sold * p.unit_price * (1 - oi.discount)), 0.0)::DOUBLE PRECISION
    INTO avg_value
    FROM order_items oi
    JOIN orders o ON oi.order_id = o.id
    JOIN products p ON oi.product_id = p.id
    WHERE o.date_of_sale BETWEEN valid_start AND valid_end;

    
    RETURN QUERY SELECT 0 AS status_id, avg_value;
END;
$$;