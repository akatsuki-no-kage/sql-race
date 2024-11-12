select
    first_name,
    last_name,
    count(*) as num_of_duplicates
from patients
group by
    first_name,
    last_name
having count(*) > 1