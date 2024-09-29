package main

import (
	"fmt"
  "strings"
  "math/rand"
)

func getUsr() {
  name := ""

  fmt.Println("Enter your username:\n(MAX is 10 characters)")
  fmt.Scan(&name)
  username := strings.ToLower(name)
  fmt.Println("You entered:", username)


}

func main() {
	fmt.Println("Hello from Nix!")
  fmt.Println(rand.Intn(3))
  getUsr()
}
