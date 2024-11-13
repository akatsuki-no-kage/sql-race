SELECT
    first_name,
    last_name,
    allergies
FROM patients
WHERE
    city = 'Toronto'
  and allergies is not null