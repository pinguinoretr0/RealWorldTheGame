package game

import (
	"os"
	"fmt"
)

// Market
/* Taking the currencies' exchange rates
   then multiplying that by the crypto you have
   will result with the value in USD;
   (Taken from each currencies' peak) */

func cryptoToUsd(x, c int) int {
	var er int

	switch c {
	case 0:
		er = 6500 // BTC
	case 1:
		er = 4891 // ETH
	case 2:
		er = 517 // XMR
	case 3:
		er = 413 // LTC
	default:
		fmt.Println("Go -> USD | ERROR!")
		os.Exit(0)
	}

	return x * er
}
