UPDATE "user"
SET name            = $2,
    avatar_image_id = $3,
    gender          = row ($4::"GenderType", $5),
    bio             = $6
WHERE id = $1