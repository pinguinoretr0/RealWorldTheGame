package game

import (
	"bufio"
	"os"
	"fmt"
	"strings"
	"math/rand"
	"time"
)

var usrname string = getUsr()

// Intro
func getUsr() string {
	scanner := bufio.NewScanner(os.Stdin)

	fmt.Print("Enter your username:\n(MAX is 10 characters)\n> ")
	scanner.Scan()

	usrname := scanner.Text() // usrname
	usrname= strings.TrimSpace(usrname)

	if len(usrname) > 10 { // check the length of the input.
		fmt.Println("\nYour username exceeds 10 characters.\nTruncated to the first 10 characters.\n")
		usrname = usrname[:10] // limits to 10 characters
	}

	fmt.Printf("Username: %s\n", usrname)
	// Y/n response | default is yes
	for {
		fmt.Print("\nIs this correct? [Y/n]\t> ")
		var response string
		fmt.Scanln(&response)

		response = strings.TrimSpace(response)
		if response == "" || response == "Y" || response == "y" {
			return usrname
		} else if response == "n" || response == "N" {
			// handles no
			fmt.Println("Enter your new username: ")
			return getUsr() // recursive call to get the username again.
		} else {
			fmt.Println("Invalid input. Please enter 'Y' or 'n'.")
		}
	}

	fmt.Println("\nIs this correct?\n[Y/n]\t> ")
	var response string
	fmt.Scanln(&response)

	if strings.TrimSpace(response) == "n" {
		// handles no
		fmt.Println("Enter your new username: ")
		return getUsr() // Recursive call to get the username again.
	}

	return usrname
}

func calIntroDebt(usrname string) {
	// seed the rand with the current time
	rand.Seed(time.Now().UnixNano())

	var rollsList [3]int
	var irsDebt, cartelDebt int

	fmt.Println("Rolling Die...")
	rollsList[0] = rand.Intn(20) + 1
	fmt.Printf("> %s rolled a: %d\n\n", usrname, rollsList[0])

	fmt.Println("Rolling Die...")
	rollsList[1] = rand.Intn(20) + 1
	fmt.Printf("> %s rolled a: %d\n\n", usrname, rollsList[1])

	// get IRS debt | convert to USD
	irsDebt = cryptoToUsd(rollsList[0]*rollsList[1], 0)
	fmt.Printf("%s owes %d USD to the IRS...\n\n", usrname, irsDebt)

	// do third roll | get Cartel debt | convert to USD
	rollsList[2] = rand.Intn(20) + 1
	fmt.Printf("> %s rolled a: %d\n", usrname, rollsList[2])
	cartelDebt = cryptoToUsd(rollsList[2]*3, 0)
	fmt.Printf("%s owes %d USD to the Cartel...\n\n", usrname, cartelDebt)

	// now get the usr's rent
	fmt.Printf("Calculating %s's rent total...\n", usrname)
	rent := calRent(usrname)
	fmt.Printf("%s's rent is %d USD.\n", usrname, rent)
}

func RunIntro() {
	calIntroDebt(usrname)
}
