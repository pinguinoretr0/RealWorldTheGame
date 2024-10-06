package ui

import (
  "log"
  "errors"

  "rwg/utils/game"

  "github.com/charmbracelet/huh"
)

// RWG Global Options
var (
  gamemode  int
  name      string
  desc      string
  e bool
)

func ProfileManager(x int) *game.Player {
  profileManager := huh.NewForm(
    huh.NewGroup(
      huh.NewInput().
        Title("Enter Your Username:").
          Value(&name).
          Validate(func(str string) error {
              if game.CheckUser(str) != false {
                return errors.New("Username cannot contian: Uppercases, Spaces, nor Glyphs")
              }

              if str == "Frank" {
                  return errors.New("Sorry, we don’t serve customers named Frank.")
              }
              return nil
          }),

      huh.NewText().
        Title("Player Description:").
        CharLimit(400).
        Value(&desc),
    ),
  )

  wifiChecker := huh.NewForm(
    huh.NewGroup(
      huh.NewConfirm().
        Title("Do you have an existing user?").
        Value(&e),
    ),
  )

  switch x {
  case 0:
    err := profileManager.Run()
    if err != nil {
      log.Fatal(err)
    }
    player := game.CreatePlayer(name, desc)
    return &player
  case 1:
    err := wifiChecker.Run()
    if err != nil {
      log.Fatal(err)
    }
    return nil
  }
  return nil
}
