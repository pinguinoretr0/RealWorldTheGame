package ui

import (
  _ "embed"
  "fmt"
  "strings"
  "os"
  "time"

  "github.com/charmbracelet/glamour"
  tea "github.com/charmbracelet/bubbletea"
  "github.com/charmbracelet/bubbles/progress"
  "github.com/charmbracelet/lipgloss"
)

// Loading Screen
type tickMsg time.Time

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
  helpStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#626262")).Render
)

// Display Manual
func DisplayManual() {
  out, err := glamour.Render(string(fileByte), "dark")
  if err == nil {
    fmt.Print(out)
  }
}

func RunLoadingScreen() {
	prog := progress.New(progress.WithScaledGradient("#c21313", "#f5c66e"))

	if _, err := tea.NewProgram(loadingModel{progress: prog}).Run(); err != nil {
		fmt.Println("Oh no!", err)
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
		pad + helpStyle("Press any key to quit")
}

func tickCmd() tea.Cmd {
	return tea.Tick(time.Second, func(t time.Time) tea.Msg {
		return tickMsg(t)
	})
}
