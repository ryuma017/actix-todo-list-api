# ToDo list API
This repository is a sample of a Todo application's back-end API that running on AWS Lambda.


# Routes

### `GET /health_check` - Get the health check working

Request:

```
GET /health_check HTTP/1.1
Host: example.jp
```

Response:

```
HTTP/1.1 200 OK
content-length: 0
```

### `GET /` - Get status

Request:

```
GET / HTTP/1.1
```

Response:

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "status": "Up"
}
```

### `GET /tasks` - Get all todo lists

Request:

```
GET /tasks HTTP/1.1
```

Response:

```
HTTP/1.1 200 OK
Content-Type: application/json
[
  {
    "id": 1,
    "title": "Learning Rust"
  },
  {
    "id": 2,
    "title": "Learning AWS"
  },
  ...
]
```

### `GET /tasks/{id}` - Get a desired todo list

Request:

```
GET /tasks/1 HTTP/1.1
```

Response:

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Rust",
}
```

### `POST /tasks` - Create a todo list

Request:

```
POST /tasks HTTP/1.1
Content-Type: application/json
{
  "title": "Learning Rust",
}
```

Response:

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Rust",
}
```

### PUT `/todos/{id}` -> Update a desired todo list

Request:

```
PUT /tasks HTTP/1.1
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Python",
}
```

Response:

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Python",
}
```

### DELETE `/tasks/{id}` -> Delete a desired todo list

Request:

```
DELETE /tasks/{id} HTTP/1.1
Content-Type: application/json
{
  "id": 2
}
```

Response:

```
HTTP/1.1 200 OK
```
