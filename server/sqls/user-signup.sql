INSERT INTO "user"
(id,
 name,
 email,
 avatar_image_id,
 bio,
 password,
 gender)
VALUES (default,
        $1,
        $2,
        null,
        null,
        row ($3, $4),
        default)