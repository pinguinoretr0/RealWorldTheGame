package main

import (
  "fmt"
  "strings"
  "math/rand"
  "time"

// cobra cli

  "github.com/aquilax/truncate"
)

func getUsr() string {
  for {
    name := ""

    fmt.Println("Enter your username:\n(MAX is 10 characters)")
    fmt.Scan(&name)

    if len(name) > 10 {
      fmt.Println("\nUsername exceeded 10 characters;\nTruncating to the first 10...")
      truncatedName := truncate.Truncate(name, 10, "", truncate.PositionEnd)
      fmt.Printf("Received: '%s' Is this ok? [y/n]\n\n", strings.ToLower(truncatedName))

      if runCliInput(1) {
        return strings.ToLower(truncatedName)
      }
    } else {
      fmt.Printf("\nYou entered: '%s' Is this ok? [y/n]\n", strings.ToLower(name))
      if runCliInput(1) {
        return strings.ToLower(name)
      }
    }
  }
}

func cryptoToUsd(num int, label int) int {
  var er int
  switch label {
  case 0: // BTC
    er = 6500
  case 1: // ETH
    er = 4891
  case 2: // XMR
    er = 517
  case 3: // LTC
    er = 413
  }
  return num * er
}

func calRent(n string) {
  c1 := rand.Intn(10) + 1
  c2 := rand.Intn(10) + 1
  c3 := rand.Intn(10) + 1

  fmt.Println("Generating Rent...")
  fmt.Printf("> %s first card has a value of: %d\n", n, c1)
  fmt.Printf("> %s second card has a value of: %d\n", n, c2)
  fmt.Printf("> %s third card has a value of: %d\n\n", n, c3)

  rent := cryptoToUsd(c1 + c2 + c3, 1)
  fmt.Printf("> %s owes %d today!\n", n, rent)
}

// n = short for 'name'
func calIntroDebt(n string) {
  rollsList := make([]int, 0)
  var irsDebt, cartelDebt int

  fmt.Println("Rolling Die...");
  rollsList = append(rollsList, rand.Intn(20) + 1)
  fmt.Printf("> %s rolled a %d!\n", n, rollsList[0])

  fmt.Println("Rolling Die...");
  rollsList = append(rollsList, rand.Intn(20) + 1)
  fmt.Printf("> %s rolled a %d!\n\n", n, rollsList[1])

  irsDebt = cryptoToUsd(rollsList[0] * rollsList[1], 0)
  fmt.Printf("> %s owes %d to the IRS!\n\n", n, irsDebt)

  fmt.Println("Rolling Die...")
  rollsList = append(rollsList, rand.Intn(20) + 1)
  fmt.Printf("> %s rolled a %d!\n\n", n, rollsList[2])

  cartelDebt = cryptoToUsd(rollsList[2] * 3, 0)
  fmt.Printf("> %s owes %d to the Cartel!\n\n", n, cartelDebt)

  calRent(n)
}

/* X - definition & use cases
1 - Handle Username (Print statement for a new line)
*/
func runCliInput(x int) bool {
  for {
    input := ""
    fmt.Scan(&input)

    switch strings.ToLower(input) {
    case "yes", "y":
      switch x {
      case 1:
        fmt.Println("")
        return true
      }
    case "no", "n":
      switch x {
      case 1:
        fmt.Println("")
        return false
      }
    default:
      fmt.Println("Invalid input, please enter 'yes' or 'no'")
    }
  }
}

func main() {
  username := getUsr()
  rand.Seed(time.Now().UnixNano()) // seed globally
  calIntroDebt(username)
}
