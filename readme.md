https://actix.rs/docs/getting-started/

- tests
curl -i http://localhost:3000/
curl -i http://localhost:3000/json
curl -i http://localhost:3000/hello
curl -i -X POST  http://localhost:3000/params?username=dk
curl -i -X POST -d "{ \"username\" : \"fake\" }" -H "Content-Type: application/json" http://localhost:3000/extractor

- auto reload (dev) https://actix.rs/docs/autoreload/

cargo install systemfd cargo-watch

systemfd --no-pid -s http::3000 -- cargo watch -x run

- post example
curl -X POST -d '{ "id" : 123 }' http://localhost:3000/extractor


### resources
- https://actix.rs/#responders

