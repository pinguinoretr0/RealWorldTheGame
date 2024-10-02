package main

import (
  "fmt"
  "strings"
  "math/rand"

// cobra cli

  "github.com/aquilax/truncate"
)

func getUsr() string {
  for {
    name := ""

    fmt.Println("Enter your username:\n(MAX is 10 characters)")
    fmt.Scan(&name)

    if len(name) > 10 {
      fmt.Println("Username exceeded 10 characters;\nTruncating to the first 10")
      truncatedName := truncate.Truncate(name, 10, "", truncate.PositionEnd)
      fmt.Println("Received: ", strings.ToLower(truncatedName), "\nIs this ok?")

      if runCliInput(1) {
        return strings.ToLower(truncatedName)
      }
    } else {
      fmt.Println("You entered:", strings.ToLower(name), "\nIs this ok?")
      if runCliInput(1) {
        return strings.ToLower(name)
      }
    }
  }
}

/* X - definition & use cases
1 - Handle Username
*/
func runCliInput(x int) bool {
  for {
    input := ""
    fmt.Scan(&input)

    switch strings.ToLower(input) {
    case "yes", "y":
      switch x {
      case 1:
        return true
      }
    case "no", "n":
      switch x {
      case 1:
        return false
      }
    default:
      fmt.Println("Invalid input, please enter 'yes' or 'no'")
    }
  }
}

func main() {
  fmt.Println("Hello from Nix!")
  fmt.Println(rand.Intn(3))
  username := getUsr()
  fmt.Println("Hello", username, "from the Main function")
}
