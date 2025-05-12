package streamsettings

import (
	"context"
	"log/slog"
	"os/exec"

	"golang.org/x/xerrors"
)

type chromeCleanup struct {
	logger *slog.Logger
}

func NewChromeCleanup(logger *slog.Logger) Cleanup {
	return &chromeCleanup{logger: logger}
}

func (c *chromeCleanup) Clean(ctx context.Context) error {
	// TODO: クリーンアップ処理の記述

	// REM Googleアカウントからログアウト
	// Googleアカウントからログアウトするためのコマンドを実行
	cmd := exec.Command("open", "http://accounts.google.com/logout")
	if err := cmd.Run(); err != nil {
		c.logger.Error("Googleアカウントからのログアウトに失敗しました", "error", err)
		return xerrors.Errorf("Googleアカウントからのログアウトに失敗しました: %w", err)
	}
	c.logger.Info("Googleアカウントからのログアウト処理を実行しました")
	return nil
}
