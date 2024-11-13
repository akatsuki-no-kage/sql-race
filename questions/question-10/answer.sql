select n, case when (p IS NULL) then "Root" when (n not in (select distinct(p) from BST as b)) IS NULL then "Leaf" else "Inner" end from BST order by N;
