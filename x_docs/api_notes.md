## API Notes

This page contains notes and examples of using the API.

<br/>

#### Login

The login API operation can be used like this:

```shell
curl -X POST -H "Content-Type: application/json" \
    -d '{ "email": "admin@local", "password": "admin" }' \
    http://localhost:9011/auth/login
```

and it responds with HTTP status code `200` and a body like this (formatted for human readability):

```json
{
	"session": "de7875ef-6cc5-433b-b397-c688dcc6633f",
	"expires_in_seconds": 86400,
	"user": {
		"id": "nrWKA48AeU",
		"email": "admin@local",
		"username": "admin",
		"bio": "",
		"is_anonymous": false,
		"permissions": ["Admin::Read", "Admin::Write"]
	}
}
```

If the credentials (email and/or password) are incorrect, it responds with HTTP status code `401` (Unauthorized) and the body `{"error":"wrong credentials"}`.

<br/>

#### Logout

The logout API operation can be used like this:

```shell
curl -X POST -H "Authorization: 7ebf679c-7519-4a61-927e-b0597b3b8184" \
    http://localhost:9011/auth/logout
```

and it responds with HTTP status code `204` (No Content).

<br/>
