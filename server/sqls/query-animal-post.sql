SELECT u.name                                     as username,
       u.avatar_image_id                          as user_avatar_image_id,
       ap.name                                    as name,
       ap.description                             as description,
       ap.content                                 as content,
       ap.images                                  as image_id_list,
       extract(epoch from ap.creation_time)::int8 as creation_time,
       ap.id                                      as post_id
from animal_post as ap
         LEFT JOIN "user" as u
                   ON ap.post_uid = u.id
where ap.id = $1