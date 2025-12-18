## API Notes

This page contains notes and examples of using the API.

<br/>

#### Login

The login API operation can be used like this:

```shell
curl -X POST -H "Content-Type: application/json" \
    -d '{"email": "admin@local", "password": "admin"}' \
    http://localhost:3000/auth/login
```

and it responds with HTTP status code `200` and a body like this:
`{"session":"7ebf679c-7519-4a61-927e-b0597b3b8184","expires_in_seconds":86400}`

If the credentials (email and/or password) are incorrect, it responds with HTTP status code `401` (Unauthorized) and the body `{"error":"wrong credentials"}`.

<br/>

#### Logout

The logout API operation can be used like this:

```shell
curl -X POST -H "Authorization: 7ebf679c-7519-4a61-927e-b0597b3b8184" \
    http://localhost:3000/auth/logout
```

and it responds with HTTP status code `204` (No Content).

<br/>
