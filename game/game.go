package game

import (
	"math/rand"
	"time"
	"fmt"
)

// Game
func calRent(usrname string) int {
	// seed the rand with the current time
	rand.Seed(time.Now().UnixNano())

	var card1, card2, card3, rent int

	fmt.Println("Generating Rent Cards...\n")
	card1 = rand.Intn(10) + 1
	fmt.Printf("> %s's first card has a value of: %d\n", usrname, card1)
	card2 = rand.Intn(10) + 1
	fmt.Printf("> %s's second card has a value of: %d\n", usrname, card2)
	card3 = rand.Intn(10) + 1
	fmt.Printf("> %s's third card has a value of: %d\n", usrname, card3)

	// calculate rent and convert to USD
	rent = cryptoToUsd(card1 + card2 + card3, 1)
	return rent
}
