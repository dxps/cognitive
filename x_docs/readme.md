## Docs

### API usage examples

#### Login

The login API operation can be used like this:

```shell
curl -X POST -H "Content-Type: application/json" \
    -d '{"email": "admin@local", "password": "admin"}' \
    http://localhost:3000/auth/login
```

and it responds with HTTP status code `200` and a body like this:
`{"session":"43ea61d0-f5fc-4fb9-9e81-4f7f6dea9106","token_type":"Bearer","expires_in":3600}`

#### Logout

The logout API operation can be used like this:

```shell
curl -X POST -H "Authorization: 43ea61d0-f5fc-4fb9-9e81-4f7f6dea9106" \
    http://localhost:3000/auth/logout
```

and it responds with HTTP status code `204` (No Content).
