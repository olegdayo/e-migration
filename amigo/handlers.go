package main

import (
	"encoding/json"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"log"
	"net/http"
)

func (s *Server) setRouter() *chi.Mux {
	r := chi.NewRouter()
	r.Use(middleware.Logger)

	r.Get(
		"/",
		func(w http.ResponseWriter, _ *http.Request) {
			answers, err := json.Marshal(s.answers)
			if err != nil {
				log.Fatalf("Cannot marshal answers into JSON: %s\n", err.Error())
			}

			_, err = w.Write(answers)

			if err != nil {
				log.Fatalf("During all tasks error ocurred: %s\n", err.Error())
			}
		},
	)
	r.Get(
		"/first",
		func(w http.ResponseWriter, _ *http.Request) {
			answers, err := json.Marshal(s.answers.First)
			if err != nil {
				log.Fatalf("Cannot marshal answers into JSON: %s\n", err.Error())
			}

			_, err = w.Write(answers)

			if err != nil {
				log.Fatalf("During all tasks error ocurred: %s\n", err.Error())
			}
		},
	)
	r.Get(
		"/second",
		func(w http.ResponseWriter, _ *http.Request) {
			answers, err := json.Marshal(s.answers.Second)
			if err != nil {
				log.Fatalf("Cannot marshal answers into JSON: %s\n", err.Error())
			}

			_, err = w.Write(answers)

			if err != nil {
				log.Fatalf("During all tasks error ocurred: %s\n", err.Error())
			}
		},
	)
	r.Get(
		"/third",
		func(w http.ResponseWriter, _ *http.Request) {
			answers, err := json.Marshal(s.answers.Third)
			if err != nil {
				log.Fatalf("Cannot marshal answers into JSON: %s\n", err.Error())
			}

			_, err = w.Write(answers)

			if err != nil {
				log.Fatalf("During all tasks error ocurred: %s\n", err.Error())
			}
		},
	)
	r.Get(
		"/forth",
		func(w http.ResponseWriter, _ *http.Request) {
			answers, err := json.Marshal(s.answers.Forth)
			if err != nil {
				log.Fatalf("Cannot marshal answers into JSON: %s\n", err.Error())
			}

			_, err = w.Write(answers)

			if err != nil {
				log.Fatalf("During all tasks error ocurred: %s\n", err.Error())
			}
		},
	)
	r.Get(
		"/fifth",
		func(w http.ResponseWriter, _ *http.Request) {
			answers, err := json.Marshal(s.answers.Fifth)
			if err != nil {
				log.Fatalf("Cannot marshal answers into JSON: %s\n", err.Error())
			}

			_, err = w.Write(answers)

			if err != nil {
				log.Fatalf("During all tasks error ocurred: %s\n", err.Error())
			}
		},
	)

	return r
}
