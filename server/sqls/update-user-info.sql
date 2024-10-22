UPDATE "user"
SET name            = $2,
    avatar_image_id = $3,
    -- TODO: sqlx throws `type 'gender' does not exist` error
    --  maybe a case issue? Investigation is needed.
    --  the workaround is to flatten them.
    password        = row ($4, $5),
    gender          = row ($6::"GenderType", $7),
    bio             = $8
WHERE id = $1