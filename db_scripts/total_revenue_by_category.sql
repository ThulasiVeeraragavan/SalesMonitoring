-- SELECT * FROM total_revenue_by_category('2023-01-01', '2023-12-31');

CREATE OR REPLACE FUNCTION total_revenue_by_category(start_date TEXT, end_date TEXT)
RETURNS TABLE (
    status_id INT,
    category TEXT,
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
        p.category::TEXT AS category,
        COALESCE(SUM(oi.quantity_sold * p.unit_price * (1 - oi.discount)), 0.0)::DOUBLE PRECISION AS total_revenue
    FROM products p
    LEFT JOIN order_items oi ON p.id = oi.product_id
    LEFT JOIN orders o ON oi.order_id = o.id AND o.date_of_sale BETWEEN valid_start AND valid_end
    GROUP BY p.category
    ORDER BY p.category; 
END;
$$;

-- SELECT DISTINCT category FROM products;

-- SELECT 
--         0 AS status_id, 
--         p.category::TEXT AS category,
--         COALESCE(SUM(oi.quantity_sold * p.unit_price * (1 - oi.discount)), 0.0)::DOUBLE PRECISION AS total_revenue
--     FROM products p
--     LEFT JOIN order_items oi ON p.id = oi.product_id
--     LEFT JOIN orders o ON oi.order_id = o.id AND o.date_of_sale BETWEEN '2023-01-01' AND '2023-12-31'
--     GROUP BY p.category
--     ORDER BY p.category; 