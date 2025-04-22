package user_management

type Repository struct {
	db any // Replace with actual DB if needed
}

func NewRepository(db any) *Repository {
	return &Repository{db: db}
}

func (r *Repository) FetchAll() []User {
	return []User{
		{ID: 1, Name: "John Doe", Role: "Student"},
		{ID: 2, Name: "Jane Tutor", Role: "Tutor"},
	}
}

type User struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
	Role string `json:"role"`
}

