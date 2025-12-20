/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"dotfiles/internal"
	"os"
	"path"
	"time"

	"github.com/charmbracelet/log"
	"github.com/spf13/cobra"
)

var logger = log.NewWithOptions(os.Stdout, log.Options{Level: log.InfoLevel, ReportTimestamp: true, TimeFormat: time.TimeOnly})

var installCmd = &cobra.Command{
	Use:   "install",
	Short: "Install the configs specified in the dotfiles",
	Long: `Install the configuration files specified in the dotfiles repository to their appropriate locations.
This command will create symlinks from the dotfiles to the target locations.`,
	Run: func(cmd *cobra.Command, args []string) {
		home := internal.HomeDir()
		configDir := internal.ConfigDir()
		root, _ := cmd.Flags().GetString("root")
		dataDir := internal.DefaultDataDir()
		if root != "" {
			dataDir = path.Join(root, "data", "home")
		} else {
			dataDir = path.Join(dataDir, "home")
		}

		logger.Info("Installing dotfiles from:", "dataDir", dataDir)
		dryRun, _ := cmd.Flags().GetBool("dry-run")
		if dryRun {
			logger.Info("Performing a dry run. No changes will be made.")
		}

		// Create symbolic links for every item in dataDir except .config
		createSymlinks(dataDir, home, []string{".config"}, dryRun)
		// Create symbolic links for every item in dataDir/.config
		createSymlinks(path.Join(dataDir, ".config"), configDir, []string{}, dryRun)

		// Hook help message
		logger.Info("To enable shell hooks, add the following to your shell configuration file:")
		logger.Info("eval \"$(dotfiles hook)\"")
	},
}

func createSymlinks(targetDir string, linkDir string, excludeList []string, dryRun bool) {
	// Always exclude
	allEntries, err := os.ReadDir(targetDir)
	if err != nil {
		logger.Fatal("Failed to read directory", "directory", targetDir, "error", err)
	}
	excludeMap := make(map[string]bool)
	for _, exclude := range excludeList {
		excludeMap[exclude] = true
	}
	for _, entry := range allEntries {
		if excludeMap[entry.Name()] {
			continue
		}
		targetPath := path.Join(targetDir, entry.Name())
		linkPath := path.Join(linkDir, entry.Name())
		if dryRun {
			logger.Info("Dry run: would create symlink", "target", targetPath, "link", linkPath)
			continue
		}
		err := internal.CreateSymlinkAndBackup(targetPath, linkPath)
		if err != nil {
			logger.Error("Failed to create symlink", "target", targetPath, "link", linkPath, "error", err)
		}
	}
}

func init() {
	rootCmd.AddCommand(installCmd)
	installCmd.Flags().BoolP("dry-run", "d", false, "Perform a dry run without making any changes")
}
