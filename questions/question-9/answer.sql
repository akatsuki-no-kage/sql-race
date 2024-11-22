SELECT *
FROM movies 
WHERE rating > (SELECT AVG(rating) FROM movies);
