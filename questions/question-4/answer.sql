SELECT
    first_name,
    last_name,
    allergies
FROM patients
WHERE
    city = 'Hamilton'
  and allergies is not null


