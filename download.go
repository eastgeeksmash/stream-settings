package streamsettings

import (
	"context"
	"log/slog"
)

type downloadCleanup struct {
	logger *slog.Logger
}

func NewDownloadCleanup(logger *slog.Logger) Cleanup {
	return &downloadCleanup{logger: logger}
}

func (d *downloadCleanup) Clean(ctx context.Context) error {
	// TODO: クリーンアップ処理の記述

	// REM ダウンロードフォルダ内のファイルとフォルダを全削除
	// forfiles /p "C:\Users\%USERNAME%\Downloads" /s /m *.* /c "cmd /c Del @path"
	return nil
}
