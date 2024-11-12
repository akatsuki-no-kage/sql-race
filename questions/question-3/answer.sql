SELECT
    allergies,
    COUNT(*) AS total_diagnosis
FROM patients
WHERE
    allergies IS NOT NULL
GROUP BY allergies
ORDER BY total_diagnosis DESC