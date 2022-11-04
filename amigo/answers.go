package main

const (
	MinLength = 1
)

type Answers struct {
	First  []int `json:"first"`
	Second []int `json:"second"`
	Third  []int `json:"third"`
	Forth  []int `json:"forth"`
	Fifth  []int `json:"fifth"`
}

func NewAnswers() *Answers {
	return &Answers{
		First:  make([]int, MinLength),
		Second: make([]int, MinLength),
		Third:  make([]int, MinLength),
		Forth:  make([]int, MinLength),
		Fifth:  make([]int, MinLength),
	}
}

func (a *Answers) GetAnswers() {
	a.First = []int{1}
	a.Second = []int{2}
	a.Third = []int{3}
	a.Forth = []int{4}
	a.Fifth = []int{5}
}
