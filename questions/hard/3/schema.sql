CREATE TABLE authors (
    id INTEGER PRIMARY KEY,
    name TEXT
);

CREATE TABLE articles (
    id INTEGER PRIMARY KEY,
    title TEXT,
    author_id INTEGER
);

INSERT INTO authors (id, name) VALUES
(1, 'Author A'),
(2, 'Author B'),
(3, 'Author C'),
(4, 'Author D'),
(5, 'Author E'),
(6, 'Author F'),
(7, 'Author G'),
(8, 'Author H'),
(9, 'Author I'),
(10, 'Author J'),
(11, 'Author K'),
(12, 'Author L'),
(13, 'Author M'),
(14, 'Author N'),
(15, 'Author O'),
(16, 'Author P'),
(17, 'Author Q'),
(18, 'Author R'),
(19, 'Author S'),
(20, 'Author T');

INSERT INTO articles (id, title, author_id) VALUES
(1, 'Article 1', 1),
(2, 'Article 2', 2),
(3, 'Article 3', 3),
(4, 'Article 4', 4),
(5, 'Article 5', 5),
(6, 'Article 6', 6),
(7, 'Article 7', 7),
(8, 'Article 8', 8),
(9, 'Article 9', 9),
(10, 'Article 10', 10),
(11, 'Article 11', 11),
(12, 'Article 12', 12),
(13, 'Article 13', 13),
(14, 'Article 14', 14),
(15, 'Article 15', 15),
(16, 'Article 16', 16),
(17, 'Article 17', 17),
(18, 'Article 18', 18),
(19, 'Article 19', 19),
(20, 'Article 20', 20);
