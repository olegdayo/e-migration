FROM golang:latest
WORKDIR /amigo
COPY . .

RUN go build -o gotchaPage *.go
CMD ["./gotchaPage"]
