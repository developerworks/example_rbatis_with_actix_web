## Integration rbatis 4 with actix web 4

1. Create mysql table with `user.sql` in project root
2. Run `cargo run`
3. Create user
```
curl -X "POST" "http://127.0.0.1:9991/user/save" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "name": "user created"
}'
```

4. Update user

```
curl -X "PATCH" "http://127.0.0.1:9991/user/1" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "id": 1,
  "name": "test updated"
}'
```