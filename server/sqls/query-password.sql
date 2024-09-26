SELECT (password).blake3, (password).salt
FROM "user"
WHERE email = $1