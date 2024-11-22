SELECT a.title, au.name 
FROM articles a 
JOIN authors au ON a.author_id = au.id;
