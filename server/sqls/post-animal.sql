INSERT INTO animal_post
(id,
 post_uid,
 name,
 images,
 content,
 adopted,
 mobile_number,
 description,
 creation_time)
VALUES (default, $1, $2, $3, $4, $5, $6, $7,
        default)