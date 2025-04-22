package main

import (
	"github.com/gin-gonic/gin"
	"tutor/main/internal/modules/user_management"
	"tutor/main/internal/db"
)

func main() {
	// Setup Gin
	r := gin.Default()

	// Connect to DB (mock or real)
	database := db.NewPostgresDB()

	// Initialize modules
	userRepo := user_management.NewRepository(database)
	userService := user_management.NewService(userRepo)
	userHandler := user_management.NewHandler(userService)

	// Routes
	api := r.Group("/api")
	{
		api.GET("/users", userHandler.GetAllUsers)
	}

	// Run server
	r.Run(":8080")
}

