package ui

import (
  _ "embed"
  "fmt"
  "strings"
  "log"
  "os"
  "time"

  "github.com/charmbracelet/glamour"
  tea "github.com/charmbracelet/bubbletea"
  "github.com/charmbracelet/bubbles/progress"
  "github.com/charmbracelet/lipgloss"

  // use 'spinner' for online matchmaking UI
)

// Loading Screen
type tickMsg time.Time

type altScrModel struct {
  altscreen bool
  quitting  bool
}

type loadingModel struct {
	percent  float64
	progress progress.Model
}

const (
	padding  = 2
	maxWidth = 80
)

/* Manual.md
TODO: make a TermManual.md file with only the info need if playing via term & \
then make CardManual.md for original contents
*/
//go:embed doc/Manual.md
var fileByte []byte

// Global UI Options
var (
  // Loading Screen
  loadStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#626262")).Render
)

func (m altScrModel) Init() tea.Cmd {
	return nil
}

func (m altScrModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "q", "ctrl+c":
			m.quitting = true
			return m, tea.Quit
		}
	}
	return m, nil
}

func (m altScrModel) View() string {
	if m.quitting {
		return "Bye!\n"
	}
	return "Entering alternate screen mode...\nPress 'q' or 'ctrl+c' to exit."
}

func EnableAltScreen() {
	m := altScrModel{altscreen: true}

	p := tea.NewProgram(m, tea.WithAltScreen())

	if err := p.Start(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}

// ------------------------------------------------------------------------------------------------------------------

func RunMainScreen() { // navigation around game (ran after intro)
  EnableAltScreen()
}

func RunLoadingScreen() {
	prog := progress.New(progress.WithScaledGradient("#c21313", "#f5c66e"))

	if _, err := tea.NewProgram(loadingModel{progress: prog}).Run(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}

func (m loadingModel) Init() tea.Cmd {
	return tickCmd()
}

func (m loadingModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		return m, tea.Quit

	case tea.WindowSizeMsg:
		m.progress.Width = msg.Width - padding*2 - 4
		if m.progress.Width > maxWidth {
			m.progress.Width = maxWidth
		}
		return m, nil

	case tickMsg:
		m.percent += 0.45
		if m.percent > 1.0 {
			m.percent = 1.0
			return m, tea.Quit
		}
		return m, tickCmd()

	default:
		return m, nil
	}
}

func (m loadingModel) View() string {
	pad := strings.Repeat(" ", padding)
	return "\n" +
		pad + m.progress.ViewAs(m.percent) + "\n\n" +
		pad + loadStyle("Press any key to quit")
}

func tickCmd() tea.Cmd {
	return tea.Tick(time.Second, func(t time.Time) tea.Msg {
		return tickMsg(t)
	})
}

// Display Manual
// TODO: display in sections & make scrollable
func DisplayManual() {
  out, err := glamour.Render(string(fileByte), "dark")
  if err != nil {
    log.Fatal(err)
  }

  fmt.Print(out)
}
