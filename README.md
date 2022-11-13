# DB migration HW

## AmiGo
Go migration with gORM.

- [ ] migration
- [ ] ORM
- [ ] request-1
- [ ] request-2
- [ ] request-3
- [ ] request-4
- [ ] request-5
- [X] web-server

## KRusty
Rust migration with diesel.

- [X] migration
- [X] ORM
- [ ] request-1
- [ ] request-2
- [X] request-3
- [X] request-4
- [X] request-5
- [X] web-server

In both of the projects there is an implemented web-server for you to have better visual data representation:
- localhost:{PORT} - answers for all 5 tasks
- localhost:{PORT}/first - answers for first task
- localhost:{PORT}/second - answers for second task
- localhost:{PORT}/third - answers for third task
- localhost:{PORT}/forth - answers for forth task
- localhost:{PORT}/fifth - answers for fifth task

### Build:
The easiest way to run this project is to build docker:
`docker-compose up --build -d`

### Ports:
- database is listening on *5432*
- amigo is listening on *3000*
- krusty is listening on *8080*

### Attention!
- Rust binary takes quiet a while (around 2-3 minutes)
- After build there will be database heath checks, need to wait for around 10-20 more seconds before using
