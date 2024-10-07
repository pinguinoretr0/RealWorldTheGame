package main

import (
  "os"
  "log"
  "fmt"
  "time"
  "math/rand"

// cobra cli

  "rwg/utils/game"

  "github.com/charmbracelet/huh"
)

/* IDEAS
  LeaderBoard -> Display Online Stats & option to upload online?
  User profile -> CAN be displayed online

*/

// RWG Global Options
var gamemode  int

/* NOTES:
  Form = Starts a scene/screen
  Select = option list (can bind to var)
  MultiSelect ^ (.Selected() = default, CAN be limited)
  Group = Screen
  NewText = text (binded to var?)
  Confirm = options to select
*/

func launchMainMenu() {
  mainMenu := huh.NewForm(
    huh.NewGroup(
        huh.NewSelect[int]().
          Title("Real World: The Game").
            Options(
                huh.NewOption("Play Game", 0),
                huh.NewOption("Load Game", 1),
                huh.NewOption("Online", 2),
                huh.NewOption("Settings", 3),
                huh.NewOption("Exit", 4),
            ).
            Value(&gamemode),
    ),
)

  err := mainMenu.Run()
  if err != nil {
    log.Fatal(err)
  }

  switch gamemode {
  case 0:
    player := game.ProfileManager(0)
    if player != nil {
      game.CalIntroDebt(player.Username)
    }
  case 1:
    fmt.Println("TOML intergration coming soon...")
  case 2:
    game.ProfileManager(1)
  case 3:
    // settings function
  case 4:
    fmt.Println("Real World: The Game has been exited...")
    os.Exit(0)
  }
}

func main() {
  rand.Seed(time.Now().UnixNano())
  launchMainMenu()
}
