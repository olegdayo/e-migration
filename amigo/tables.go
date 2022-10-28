package main

import (
	"gorm.io/gorm"
	"time"
)

type Countries struct {
	gorm.Model
	Name       string
	CountryID  string `gorm:"primaryKey"`
	AreaSQKM   int
	Population int
}

type Olympics struct {
	gorm.Model
	OlympicID string `gorm:"primaryKey"`
	CountryID string
	City      string
	Year      string
	StartDate string
	EndDate   string
}

type Players struct {
	gorm.Model
	Name      string
	PlayerID  string `gorm:"primaryKey"`
	CountryID string
	BirthDate time.Time
}

type Events struct {
	gorm.Model
	EventID          string `gorm:"primaryKey"`
	Name             string
	EventType        string
	OlympicID        string
	IsTeamEvent      bool
	NumPlayersInTeam int
	ResultNotedIn    string
}

type Results struct {
	gorm.Model
	EventID  string `gorm:"primaryKey"`
	PlayerID string `gorm:"primaryKey"`
	Medal    string
	Result   float64
}
