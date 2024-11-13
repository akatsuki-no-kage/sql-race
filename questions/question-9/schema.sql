CREATE TABLE patients (
  patient_id INTEGER PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  gender CHAR(1),
  birth_date DATE,
  city TEXT,
  province_id CHAR(2),
  allergies TEXT,
  height INT,
  weight INT,
  FOREIGN KEY (province_id) REFERENCES provinces (province_id)
);
CREATE TABLE provinces (
   province_id CHAR(2) PRIMARY KEY,
   province_name TEXT NOT NULL
);

INSERT INTO provinces (province_id, province_name) VALUES
('ON', 'Ontario'),
('QC', 'Quebec'),
('BC', 'British Columbia'),
('AB', 'Alberta'),
('NS', 'Nova Scotia'),
('NB', 'New Brunswick'),
('PE', 'Prince Edward Island'),
('NL', 'Newfoundland and Labrador'),
('MB', 'Manitoba'),
('SK', 'Saskatchewan');



INSERT INTO patients (patient_id, first_name, last_name, gender, birth_date, city, province_id, allergies, height, weight) VALUES
(1, 'Michael', 'Taylor', 'M', '1980', 'Toronto', 'ON', 'Peanuts', 180, 75),
(2, 'Sarah', 'Martinez', 'F', '1992', 'Montreal', 'QC', 'None', 165, 60),
(3, 'David', 'Davis', 'M', '1978', 'Vancouver', 'BC', 'Dust', 170, 80),
(4, 'Emily', 'Miller', 'F', '2001', 'Calgary', 'AB', 'None', 155, 55),
(5, 'Daniel', 'Wilson', 'M', '1995', 'Ottawa', 'ON', 'Penicillin', 175, 70),
(6, 'Olivia', 'Moore', 'F', '1990', 'Edmonton', 'AB', 'None', 160, 65),
(7, 'James', 'Anderson', 'M', '1988', 'Halifax', 'NS', 'Shellfish', 185, 90),
(8, 'Charlotte', 'Thomas', 'F', '1994', 'Winnipeg', 'MB', 'None', 160, 58),
(9, 'Benjamin', 'Jackson', 'M', '1980', 'Regina', 'SK', 'Latex', 180, 85),
(10, 'Lucas', 'White', 'M', '2003', 'Quebec City', 'QC', 'None', 170, 65),
(11, 'Amelia', 'Harris', 'F', '1987', 'Toronto', 'ON', 'None', 162, 63),
(12, 'Ethan', 'Clark', 'M', '2000', 'Montreal', 'QC', 'None', 172, 78),
(13, 'Mason', 'Lewis', 'M', '1997', 'Vancouver', 'BC', 'None', 178, 82),
(14, 'Harper', 'Young', 'F', '1991', 'Calgary', 'AB', 'Cats', 167, 63),
(15, 'Alexander', 'Walker', 'M', '1983', 'Toronto', 'ON', 'Peanuts', 180, 77),
(16, 'Avery', 'Hall', 'F', '1998', 'Edmonton', 'AB', 'None', 158, 56),
(17, 'Gabriel', 'Allen', 'M', '1993', 'Halifax', 'NS', 'None', 176, 80),
(18, 'Sophia', 'Scott', 'F', '1996', 'Winnipeg', 'MB', 'Dairy', 164, 62),
(19, 'Jacob', 'Adams', 'M', '1989', 'Toronto', 'SK', 'None', 175, 70),
(20, 'Zoe', 'Baker', 'F', '2002', 'Quebec City', 'QC', 'None', 160, 55),
(21, 'William', 'Gonzalez', 'M', '1994', 'Toronto', 'ON', 'None', 179, 74),
(22, 'Lily', 'Perez', 'F', '1986', 'Montreal', 'QC', 'Dust', 167, 61),
(23, 'Jackson', 'Roberts', 'M', '1991', 'Vancouver', 'BC', 'None', 180, 83),
(24, 'Eleanor', 'Kim', 'F', '1995', 'Calgary', 'AB', 'None', 162, 59),
(25, 'Matthew', 'Evans', 'M', '1984', 'Ottawa', 'ON', 'Latex', 182, 88),
(26, 'Ella', 'Nelson', 'F', '1990', 'Edmonton', 'AB', 'None', 168, 63),
(27, 'Henry', 'Carter', 'M', '1982', 'Halifax', 'NS', 'None', 174, 76),
(28, 'Grace', 'Mitchell', 'F', '1999', 'Winnipeg', 'MB', 'None', 161, 57),
(29, 'Jackson', 'Perez', 'M', '1995', 'Regina', 'SK', 'None', 183, 85),
(30, 'Luna', 'Rogers', 'F', '2004', 'Quebec City', 'QC', 'None', 155, 50);

