SELECT id,
       name,
       email,
       avatar,
       bio,
       password
FROM "user"
WHERE id = $1