package main

import (
	"fmt"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func main() {
	dsn := `host=localhost
			user=sidorenkov_204
			password=sidorenkov_204
			dbname=sidorenkov_204
			port=5432
			sslmode=disable`

	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		panic(err)
	}

	fmt.Println(db)
}
