package api

import (
	"github.com/gin-gonic/gin"
	"noobit/utils"
)

type Server struct {
	listAddr string
}

func NewServer(env *utils.Env) Server {
	return Server{
		listAddr: env.ListenAddr,
	}
}

func (s *Server) Start() error {
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
			"addr":    s.listAddr,
		})
	})
	return r.Run(s.listAddr)
}
