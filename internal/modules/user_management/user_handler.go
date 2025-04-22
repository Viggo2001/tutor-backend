package user_management

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

type Handler struct {
	service *Service
}

func NewHandler(s *Service) *Handler {
	return &Handler{service: s}
}

func (h *Handler) GetAllUsers(c *gin.Context) {
	users := h.service.GetUsers()
	c.JSON(http.StatusOK, users)
}

