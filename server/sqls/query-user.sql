SELECT id,
       name,
       email,
       avatar_image_id,
       bio,
       password,
       (gender).type  as gender_type,
       (gender).other as gender_other,
       admin
FROM "user"
WHERE id = $1