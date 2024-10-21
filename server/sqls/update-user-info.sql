UPDATE "user"
SET name            = $2,
    avatar_image_id = $3,
    password        = $4,
    gender          = $5
WHERE id = $1