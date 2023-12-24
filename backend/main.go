package main

import (
	"fmt"
	"noobit/api"
	"noobit/utils"
)

func main() {
	env := utils.GetEnv()
	server := api.NewServer(&env)
	fmt.Println("Starting server on", env.ListenAddr)
	err := server.Start()
	if err != nil {
		panic(err)
	}
}
