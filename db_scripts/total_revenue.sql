-- SELECT * FROM total_revenue('2023-01-01', '2024-12-31');

CREATE OR REPLACE FUNCTION total_revenue(start_date TEXT, end_date TEXT)
RETURNS TABLE (status_id INT, total_revenue DOUBLE PRECISION)
LANGUAGE plpgsql
AS $$
DECLARE
    valid_start DATE;
    valid_end DATE;
    total DOUBLE PRECISION;
BEGIN
    
    BEGIN
        IF start_date !~ '^\d{4}-\d{2}-\d{2}$' OR end_date !~ '^\d{4}-\d{2}-\d{2}$' THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::DOUBLE PRECISION AS total_revenue;
            RETURN;
        END IF;

        valid_start := start_date::DATE;
        valid_end := end_date::DATE;
    EXCEPTION
        WHEN others THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::DOUBLE PRECISION AS total_revenue;
            RETURN;
    END;

    
    SELECT COALESCE(SUM(oi.quantity_sold * p.unit_price * (1 - oi.discount)), 0.0)
    INTO total
    FROM order_items oi
    JOIN orders o ON oi.order_id = o.id
    JOIN products p ON oi.product_id = p.id
    WHERE o.date_of_sale BETWEEN valid_start AND valid_end;

    
    RETURN QUERY SELECT 0 AS status_id, total;
END;
$$;