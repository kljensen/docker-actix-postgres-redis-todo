# Docker actix postgres redis todo example

This is a Dockerized version of the 
[actix todo example](https://github.com/actix/examples/tree/master/todo).

Uses diesel.


To run it, do

```
docker-compose up
```

This is up-to-date as of the following releases

```
actix-web = "2.0.0-rc"
actix-files = "0.2.0"
actix-session = "0.3.0"
actix-redis = {version = "0.8.0", features = ["web"]}
actix-rt = "1.0.0"
```
