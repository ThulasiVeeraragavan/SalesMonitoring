-- SELECT * FROM total_customers('2023-01-01', '2024-12-31');
CREATE OR REPLACE FUNCTION total_customers(start_date TEXT, end_date TEXT)
RETURNS TABLE (status_id INT, total_customers INT)
LANGUAGE plpgsql
AS $$
DECLARE
    valid_start DATE;
    valid_end DATE;
    total INT;
BEGIN
    
    BEGIN
        IF start_date !~ '^\d{4}-\d{2}-\d{2}$' OR end_date !~ '^\d{4}-\d{2}-\d{2}$' THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::INT AS total_customers;
            RETURN;
        END IF;

        valid_start := start_date::DATE;
        valid_end := end_date::DATE;
    EXCEPTION
        WHEN others THEN
            RETURN QUERY SELECT 1 AS status_id, NULL::INT AS total_customers;
            RETURN;
    END;

    
    SELECT COUNT(DISTINCT o.customer_id) INTO total
    FROM orders o
    WHERE o.date_of_sale BETWEEN valid_start AND valid_end;

    
    RETURN QUERY SELECT 0 AS status_id, total;
END;
$$;