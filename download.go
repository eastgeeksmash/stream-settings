package streamsettings

import (
	"context"
	"log/slog"
	"os"
	"path/filepath"

	"golang.org/x/xerrors"
)

type downloadCleanup struct {
	logger *slog.Logger
}

func NewDownloadCleanup(logger *slog.Logger) Cleanup {
	return &downloadCleanup{logger: logger}
}

func (d *downloadCleanup) Clean(ctx context.Context) error {
	username := os.Getenv("USERNAME")
	if username == "" {
		d.logger.Warn("USERNAMEが見つかりません")
		return nil
	}

	downloadPath := filepath.Join("C:", "Users", username, "Downloads")
	files, err := filepath.Glob(filepath.Join(downloadPath, "*.*"))
	if err != nil {
		return xerrors.Errorf("ダウンロードフォルダのファイル検索エラー: %w", err)
	}

	for _, file := range files {
		if err := os.Remove(file); err != nil {
			return xerrors.Errorf("ファイル削除エラー: %w", err)
		}
	}

	return nil
}
