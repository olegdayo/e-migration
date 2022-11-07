# DB migration HW

## AmiGo
Go migration with gORM.

- [ ] migration
- [ ] request-1
- [ ] request-2
- [ ] request-3
- [ ] request-4
- [ ] request-5
- [X] web-server

To run Go server:
```
cd amigo
go run .
```

## KRusty
Rust migration with diesel.

- [X] migration
- [ ] request-1
- [ ] request-2
- [ ] request-3
- [ ] request-4
- [ ] request-5
- [X] web-server

To run Rust server:
```
cd krusty
cargo run
```

In both of the projects there is an implemented web-server for you to have better visual data representation:
- localhost:8080/ - answers for all 5 tasks
- localhost:8080/first - answers for first task
- localhost:8080/second - answers for second task
- localhost:8080/third - answers for third task
- localhost:8080/forth - answers for forth task
- localhost:8080/fifth - answers for fifth task
