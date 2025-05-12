package streamsettings

import (
	"context"
	"log/slog"
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
	// start http://accounts.google.com/logout
	return nil
}
