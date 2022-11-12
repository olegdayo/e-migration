package main

import (
	"errors"
	"fmt"
	"github.com/joho/godotenv"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"log"
	"os"
)

func createConnection() (*gorm.DB, error) {
	url, ok := os.LookupEnv("AMIGO_DATABASE_URL")

	if ok != true {
		return nil, errors.New("cannot find database url in environment variables")
	}

	log.Printf("DB URL: %s", url)
	return gorm.Open(postgres.Open(url), &gorm.Config{})
}

func init() {
	if err := godotenv.Load(); err != nil {
		panic("No .env file found")
	}
}

func main() {
	db, err := createConnection()

	if err != nil {
		panic(err)
	}

	log.Println(db)

	answers := NewAnswers()
	answers.GetAnswers()
	fmt.Println(answers)

	s := NewServer(answers)
	fmt.Println(s)
	s.Start()
}
