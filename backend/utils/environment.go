package utils

import (
	"github.com/joho/godotenv"
	"os"
)

type Env struct {
	ConnStr    string
	ListenAddr string
}

func GetEnv() Env {
	env := godotenv.Load(".env")
	if env != nil {
		panic(env)
	}
	return Env{
		ConnStr:    os.Getenv("DB_CONNECTION"),
		ListenAddr: os.Getenv("LISTEN_ADDR"),
	}
}
