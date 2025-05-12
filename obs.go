package streamsettings

import (
	"context"
	"log/slog"
)

type obsCleanup struct {
	logger *slog.Logger
}

func NewObsCleanup(logger *slog.Logger) Cleanup {
	return &obsCleanup{logger: logger}
}

func (o *obsCleanup) Clean(ctx context.Context) error {
	// TODO: クリーンアップ処理の記述
	return nil
}

type obsSetup struct {
	logger *slog.Logger
}

func NewObsSetup(logger *slog.Logger) Setup {
	return &obsSetup{logger: logger}
}

func (o *obsSetup) Setup(ctx context.Context) error {
	// TODO: セットアップ処理の記述
	return nil
}
