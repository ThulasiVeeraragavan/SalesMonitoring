-- SELECT * FROM total_revenue_by_product('2023-12-01', '2024-12-31');

CREATE OR REPLACE FUNCTION total_revenue_by_product(start_date TEXT, end_date TEXT)
RETURNS TABLE (
    status_id INT,
    product_name TEXT,
    total_revenue DOUBLE PRECISION
)
LANGUAGE plpgsql
AS $$
DECLARE
    valid_start DATE;
    valid_end DATE;
BEGIN
    
    BEGIN
        IF start_date !~ '^\d{4}-\d{2}-\d{2}$' OR end_date !~ '^\d{4}-\d{2}-\d{2}$' THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::TEXT, NULL::DOUBLE PRECISION;
            RETURN;
        END IF;

        valid_start := start_date::DATE;
        valid_end := end_date::DATE;
    EXCEPTION
        WHEN others THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::TEXT, NULL::DOUBLE PRECISION;
            RETURN;
    END;

    
    RETURN QUERY 
    SELECT 
        0 AS status_id, 
        p.name::TEXT AS product_name, 
        COALESCE(SUM(oi.quantity_sold * p.unit_price * (1 - oi.discount))::DOUBLE PRECISION, 0.0::DOUBLE PRECISION) AS total_revenue
    FROM order_items oi
    JOIN orders o ON oi.order_id = o.id
    JOIN products p ON oi.product_id = p.id
    WHERE o.date_of_sale BETWEEN valid_start AND valid_end
    GROUP BY p.name;
END;
$$;