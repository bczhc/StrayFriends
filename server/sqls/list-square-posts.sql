select id,
       post_uid,
       content,
       images,
       extract(epoch from creation_time)::int8 as creation_time
from square_post
order by creation_time desc
offset $1 limit $2