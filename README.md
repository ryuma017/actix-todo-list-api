# ToDo list API
This repository is a sample of a Todo application's back-end API that running on AWS Lambda.


\> **Requirements**:
- Lang -> Rust
- Build for Amazon Linux 2 runtimes
- Deploy with AWS SAM CLI


# Routes

## `GET /health_check` - Get the health check working

**Request**:

```
GET /health_check HTTP/1.1
Host: example.jp
```

**Response**:

```
HTTP/1.1 200 OK
content-length: 0
```

## `GET /` - Get status

**Request**:

```
GET / HTTP/1.1
```

**Response**:

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "status": "Up"
}
```

## `GET /todos` - Get all todo lists

**Request**:

```
GET /todos HTTP/1.1
```

**Response**:

```
HTTP/1.1 200 OK
Content-Type: application/json
[
  {
    "id": 1,
    "title": "Learning Rust"
    "description": "Vec, HashMap, BtreeMap"
  },
  {
    "id": 2,
    "title": "Learning AWS"
    "description": "VPC, Gateway, Lambda function"
  },
  ...
]
```

## `GET /todos/{id}` - Get a desired todo list

**Request**:

```
GET /todos/1 HTTP/1.1
```

**Response**:

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Rust"
  "description": "Vec, HashMap, BtreeMap"
}
```

## `POST /todos` - Create a todo list

**Request**:

```
POST /todos HTTP/1.1
Content-Type: application/json
{
  "title": "Learning Rust"
  "description": "Vec, HashMap, BtreeMap"
}
```

**Response**:

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Rust"
  "description": "Vec, HashMap, BtreeMap"
}
```

## PUT `/todos/{id}` -> Update a desired todo list

**Request**

```
POST /todos HTTP/1.1
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Python"
  "description": "Fast API"
}
```

**Response**

```
HTTP/1.1 200 OK
Content-Type: application/json
{
  "id": 1,
  "title": "Learning Python"
  "description": "Fast API"
}
```

## DELEAT `/todos/{id}` -> Delete a desired todo list

**Request**

```
POST /todos HTTP/1.1
Content-Type: application/json
{
  "id": 2
}
```

**Response**

```
HTTP/1.1 200 OK
```
