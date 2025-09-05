SELECT department, SUM(salary) AS total_salary 
FROM employees 
GROUP BY department;
