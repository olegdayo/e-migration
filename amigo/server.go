package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

type Server struct {
	http.Server
	answers *Answers
}

func NewServer(answers *Answers) (s *Server) {
	s = new(Server)

	port, ok := os.LookupEnv("AMIGO_PORT")
	if !ok {
		port = "8080"
	}

	s.Addr = fmt.Sprintf(":%s", port)
	s.answers = answers
	s.Handler = s.setRouter()

	return
}

func (s *Server) Start() {
	err := s.ListenAndServe()
	if err != nil {
		log.Fatalf("Server running error: %s\n", err.Error())
	}
}
