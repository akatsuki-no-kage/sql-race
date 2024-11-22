CREATE TABLE orders (
    id INTEGER PRIMARY KEY,
    customer TEXT,
    status TEXT
);

INSERT INTO orders (id, customer, status) VALUES
(1, 'Alice', 'Shipped'),
(2, 'Bob', 'Pending'),
(3, 'Alice', 'Pending'),
(4, 'Carol', 'Shipped'),
(5, 'Dave', 'Cancelled'),
(6, 'Eve', 'Shipped'),
(7, 'Frank', 'Pending'),
(8, 'Grace', 'Shipped'),
(9, 'Heidi', 'Pending'),
(10, 'Ivan', 'Cancelled'),
(11, 'Judy', 'Pending'),
(12, 'Kathy', 'Shipped'),
(13, 'Leo', 'Cancelled'),
(14, 'Mona', 'Shipped'),
(15, 'Nina', 'Pending'),
(16, 'Oscar', 'Shipped'),
(17, 'Paul', 'Cancelled'),
(18, 'Quinn', 'Pending'),
(19, 'Rita', 'Shipped'),
(20, 'Steve', 'Cancelled');
