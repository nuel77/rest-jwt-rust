# Rest-jwt-rust

A simple CRUD backend app using Actix-web, Diesel and JWT
## How to run

### Docker

- Enter into project directory
- Run `docker-compose -f docker-compose.yml up`

## APIs

### Address: **`localhost:8080`**

### `GET /api/ping`: Ping

```bash
curl --location '0.0.0.0:8080/ping' \
--data ''
```

- Response:
    - 200 OK

      ```text
      pong!
      ```

### `POST /auth/register`: Register

```bash
curl --location '0.0.0.0:8080/auth/register' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "bob@gmail.com",
    "password": "bob"
}'
```

- Request body:

  ```text
  {
     "email": string,
     "password": string  
  }
  ```

- Response
    - 200 OK

  ```json
  {
     "message": "ok",
     "data": ""
  }
  ```
    - 400 Bad Request

  ```json
  {
     "message": "User already exists",
     "data": ""
  }

  ```

### `POST /api/auth/login`: Login

```bash
curl --location '0.0.0.0:8080/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "alice@gmail.com",
    "password": "alice"
}'
```

- Request body:

  ```text
  {
     "email": string,
     "password": string   
  }
  ```

- Response
    - 200 OK

  ```text
  {
     "message": "ok",
     "data": {
       "token": string      // bearer token
       "token_type": string // token type
     }
  }
  ```

### `GET /transfer`: Transfer

```bash
curl --location --request GET '0.0.0.0:8080/transfer/' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE3MjExMzY3NjYsImV4cCI6MTcyMTc0MTU2NiwiZW1haWwiOiJhbGljZUBnbWFpbC5jb20iLCJsb2dpbl9zZXNzaW9uIjoiMDM0ZmFkZTAtNTEyMS00ZjkxLWE4OTktMzgzNWQ0YjM3M2EzIn0.XsRYtDr1t6m5_uctXg8jNPNGKopoJvFcW5GQIboRFK4' \
--data-raw '{
    "email": "alice@gmail.com"
}'
```

- Request body:

  ```text
  {
     "email": string,
  }
  ```

- Response
    - 200 OK

  ```text
  {
     "message": "ok",
     "data": {
            "id": i32, // tx id
            "from_user": i32, // from user id
            "to_user": i32, // to user id
            "amount": i32 // amount transferred
     }
  }
  ```

### `POST /transfer/create`: Transfer

```bash
curl --location '0.0.0.0:8080/transfer/create' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE3MjExMzY3NjYsImV4cCI6MTcyMTc0MTU2NiwiZW1haWwiOiJhbGljZUBnbWFpbC5jb20iLCJsb2dpbl9zZXNzaW9uIjoiMDM0ZmFkZTAtNTEyMS00ZjkxLWE4OTktMzgzNWQ0YjM3M2EzIn0.XsRYtDr1t6m5_uctXg8jNPNGKopoJvFcW5GQIboRFK4' \
--data-raw '{
    "from_user":"alice@gmail.com",
    "to_user": "bob@gmail.com",
    "amount": 5
}'
```

- Request body:

  ```text
  {
     "from_user": string,
     "to_user": string,
     "amount": i32,
  }
  ```

- Response
    - 200 OK

  ```text
  {
     "message": "ok",
     "data": ""
  }
  ```