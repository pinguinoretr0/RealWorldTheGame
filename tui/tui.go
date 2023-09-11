package tui

import (
	"fmt"
	"os"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea" // main TUI lib
	"github.com/charmbracelet/lipgloss"

	//	gl "github.com/charmbracelet/glamour"   // for viewing manual in-game
)

var (
	docStyle = lipgloss.NewStyle().Margin(3, 2)
	menuStyle = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#FFFDF5")).
			Background(lipgloss.Color("#25A065")).
			Padding(0, 1)
	optsNormStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#2629fc"))
	optsFocusedStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#1bf5b7"))
)

// declare menu & customize here
type item struct {
	title, desc string
	focused bool
	//	action Input
}

func (i item) Title() string       { return i.title }
func (i item) Description() string { return i.desc }
func (i item) FilterValue() string { return i.title }

// customize the rendering for each item based on its focus
func (i item) Render(focused bool) string {
	if focused {
		return optsFocusedStyle.Render(fmt.Sprintf("%s - %s", i.title, i.desc))
	}
	return optsNormStyle.Render(fmt.Sprintf("%s - %s", i.title, i.desc))
}

type model struct {
	list list.Model
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		if msg.String() == "ctrl+c" {
			return m, tea.Quit
		}
	case tea.WindowSizeMsg:
		h, v := docStyle.GetFrameSize()
		m.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	m.list, cmd = m.list.Update(msg)


	return m, cmd
}

func (m model) View() string {
	return docStyle.Render(m.list.View())
}

func RunMainMenu() {
	items := []list.Item{
		item{title: "Start Game", desc: "Start Real World: The Game"},
		item{title: "Manual", desc: "Read RWG Manual"},
		item{title: "High Scores", desc: "Check the high scores of your local machine"},
		item{title: "Settings", desc: "Tweak the game to your liking"},
		//item{title: "...", desc: "..."},
	}

	m := model{list: list.New(items, list.NewDefaultDelegate(), 0, 0)}
	m.list.Title = "Real World: The Game"
	m.list.Styles.Title = menuStyle

	mainmenu := tea.NewProgram(m, tea.WithAltScreen())

	if _, err := mainmenu.Run(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}
