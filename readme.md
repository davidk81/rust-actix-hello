https://actix.rs/docs/getting-started/


- auto reload (dev) https://actix.rs/docs/autoreload/

cargo install systemfd cargo-watch

systemfd --no-pid -s http::3000 -- cargo watch -x run

