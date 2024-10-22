UPDATE "user"
-- TODO: sqlx throws `type 'gender' does not exist` error
--  maybe a case issue? Investigation is needed.
--  the workaround is to flatten them.
SET password = row ($2, $3)
WHERE id = $1