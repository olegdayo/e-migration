package main

import "gorm.io/gorm"

type Countries struct {
	gorm.Model
	Name       string `gorm:"name"`
	CountryID  string `gorm:"country_id"`
	AreaSQKM   int    `gorm:"area_skqm"`
	Population int    `gorm:"population"`
}

type Olympics struct {
	gorm.Model
	OlympicID string `gorm:"olympic_id"`
	CountryID string `gorm:"country_id"`
	City      string `gorm:"city"`
	Year      string `gorm:"year"`
	StartDate string `gorm:"startdate"`
	EndDate   string `gorm:"enddate"`
}

type Players struct {
	gorm.Model
	Name      string `gorm:"name"`
	PlayerID  string `gorm:"player_id"`
	CountryID string `gorm`
}

type Events struct {
	gorm.Model
}

type Results struct {
	gorm.Model
}
