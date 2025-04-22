package user_management

type Service struct {
	repo *Repository
}

func NewService(r *Repository) *Service {
	return &Service{repo: r}
}

func (s *Service) GetUsers() []User {
	return s.repo.FetchAll()
}

