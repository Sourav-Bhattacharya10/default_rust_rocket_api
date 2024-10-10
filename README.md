# Default Hello World API in Rust using rocket framework
This is sample repo for Dockerfile reference of running hello world api

## Change the default address of Rocket
Set this inside Dockerfile
```
ENV ROCKET_ADDRESS=0.0.0.0
```

## Build the Dockerfile
```
sudo docker build -t docker-rust-rocket-api-alpine .
```

## Run the container
```
sudo docker run -d -p 8000:8000 docker-rust-rocket-api-alpine
```
