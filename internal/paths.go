package internal

import (
	"os"
	"path"
)

func HomeDir() string {
	return os.Getenv("HOME")
}
func ConfigDir() string {
	return path.Join(HomeDir(), ".config")
}

func DefaultDataDir() string {
	return path.Join(ConfigDir(), "dotfiles", "data")
}
