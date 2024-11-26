SELECT category, SUM(amount) AS total_amount 
FROM transactions 
GROUP BY category 
HAVING SUM(amount) > 200;
