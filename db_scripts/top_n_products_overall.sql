-- SELECT * FROM top_n_products_overall(3, '2023-01-01', '2024-12-31');

CREATE OR REPLACE FUNCTION top_n_products_overall(n INT, start_date TEXT, end_date TEXT)
RETURNS TABLE (status_id INT, product_name TEXT, total_quantity_sold BIGINT)
LANGUAGE plpgsql
AS $$
DECLARE
    valid_start DATE;
    valid_end DATE;
BEGIN
    
    BEGIN
        IF start_date !~ '^\d{4}-\d{2}-\d{2}$' OR end_date !~ '^\d{4}-\d{2}-\d{2}$' THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::TEXT, NULL::BIGINT;
            RETURN;
        END IF;

        valid_start := start_date::DATE;
        valid_end := end_date::DATE;
    EXCEPTION
        WHEN others THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::TEXT, NULL::BIGINT;
            RETURN;
    END;

    
    RETURN QUERY
    SELECT 
        0 AS status_id, 
        p.name::TEXT AS product_name, 
        SUM(oi.quantity_sold)::BIGINT AS total_quantity_sold
    FROM order_items oi
    JOIN orders o ON oi.order_id = o.id
    JOIN products p ON oi.product_id = p.id
    WHERE o.date_of_sale BETWEEN valid_start AND valid_end
    GROUP BY p.name
    ORDER BY total_quantity_sold DESC
    LIMIT n;
END;
$$;