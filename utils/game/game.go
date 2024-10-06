package game

import (
  "fmt"
  "strings"
  "math/rand"
  "unicode"
)

type Player struct {
  Username    string
  Description string
//  highScore   int
//  bestRun     int
}

// intro.go
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
func CalIntroDebt(n string) {
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

func CheckUser(s string) bool {
  var hasUpper, hasSpace, hasGlyph bool

  for _, r := range s {
    switch {
    case unicode.IsUpper(r):
      hasUpper = true
    case unicode.IsSpace(r):
      hasSpace = true
    case unicode.IsSymbol(r):
      hasGlyph = true
    case unicode.IsPunct(r):
      hasGlyph = true
    }

    if hasUpper || hasSpace || hasGlyph {
      return true
    }
  }
  return false
}

func CreatePlayer(n string, d string) Player {
  return Player{Username: n, Description: d}
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
