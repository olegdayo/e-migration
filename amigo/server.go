package main

import (
	"fmt"
	"log"
	"net/http"
)

const (
	Port uint16 = 8080
)

type Server struct {
	http.Server
	answers *Answers
}

func NewServer(answers *Answers) (s *Server) {
	s = new(Server)

	s.Addr = fmt.Sprintf(":%d", Port)
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
