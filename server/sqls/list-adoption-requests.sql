select af.id              as request_id,
       af.post_uid        as post_uid,
       af.animal_post_id  as animal_post_id,
       af.request_details as request_details,
       af.phone_number    as mobile_number
from adoption_filing as af
offset $1 limit $2
