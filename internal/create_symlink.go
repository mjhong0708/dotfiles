package internal

import (
	"errors"
	"os"
	"strings"
	"time"

	"github.com/charmbracelet/log"
)

var logger = log.NewWithOptions(os.Stdout, log.Options{Level: log.InfoLevel, ReportTimestamp: true, TimeFormat: time.TimeOnly})

type FileAlreadyExistsError struct {
	Path string
}

func (e *FileAlreadyExistsError) Error() string {
	return "file already exists: " + e.Path
}

type SymlinkAlreadyExistsError struct {
	Path string
}

func (e *SymlinkAlreadyExistsError) Error() string {
	return "symlink already exists: " + e.Path
}

type UnexpectedSymlinkError struct {
	OriginalError error
	Path          string
	TargetPath    string
}

func (e *UnexpectedSymlinkError) Error() string {
	msg := "unexpected error while creating symlink for "
	msg += e.Path + " -> " + e.TargetPath
	msg += ": " + e.OriginalError.Error()
	return msg
}

func CreateSymlinkAndBackup(target string, linkPath string) error {
	// Try to create the symlink
	err := tryCreateSymlink(target, linkPath)
	if err == nil {

		return nil
	}

	switch e := err.(type) {
	case *FileAlreadyExistsError:
		// Create backup and retry - add timestamp to backup name
		backupPath := linkPath + ".backup." + getTimestamp()
		renameErr := os.Rename(linkPath, backupPath)
		if renameErr != nil {
			return renameErr
		}
		retryErr := tryCreateSymlink(target, linkPath)
		if retryErr != nil {
			if restoreErr := os.Rename(backupPath, linkPath); restoreErr != nil {
				return &UnexpectedSymlinkError{
					OriginalError: errors.Join(retryErr, restoreErr),
					Path:          linkPath,
					TargetPath:    target,
				}
			}
			return retryErr
		}

		logger.Info("Backed up existing file and created symlink", "link", linkPath, "target", target, "backup", backupPath)
		return nil
	case *SymlinkAlreadyExistsError:
		return &UnexpectedSymlinkError{
			OriginalError: e,
			Path:          linkPath,
			TargetPath:    target,
		}
	default:
		return err
	}
}

func tryCreateSymlink(target string, linkPath string) error {
	// Attempt to create the symlink first to avoid TOCTTOU on linkPath
	if err := os.Symlink(target, linkPath); err != nil {
		if errors.Is(err, os.ErrExist) {
			fi, lerr := os.Lstat(linkPath)
			if lerr != nil {
				return &UnexpectedSymlinkError{OriginalError: lerr, Path: linkPath, TargetPath: target}
			}
			if fi.Mode()&os.ModeSymlink != 0 {
				existingTarget, rerr := os.Readlink(linkPath)
				if rerr != nil {
					return &UnexpectedSymlinkError{OriginalError: rerr, Path: linkPath, TargetPath: target}
				}
				if existingTarget == target {
					logger.Info("Symlink already exists and points to the correct target", "link", linkPath, "target", target)
					return nil
				}
				return &SymlinkAlreadyExistsError{Path: linkPath}
			}
			return &FileAlreadyExistsError{Path: linkPath}
		}
		return &UnexpectedSymlinkError{OriginalError: err, Path: linkPath, TargetPath: target}
	}
	logger.Info("Created symlink", "link", linkPath, "target", target)
	return nil
}

func getTimestamp() string {
	return strings.Join(strings.Split(time.Now().Format(time.DateTime), " "), "_")
}
