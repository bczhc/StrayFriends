SELECT id,
       name,
       email,
       avatar_image_id,
       bio,
       password
FROM "user"
WHERE id = $1