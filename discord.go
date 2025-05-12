package streamsettings

import (
	"context"
	"log/slog"
)

type discordCeanup struct {
	logger *slog.Logger
}

func NewDiscordCleanup(logger *slog.Logger) Cleanup {
	return &discordCeanup{logger: logger}
}

func (d *discordCeanup) Clean(ctx context.Context) error {
	// TODO: クリーンアップ処理の記述

	// TODO: DiscordのプロセスをKillする

	// del "C:\Users\%USERNAME%\AppData\Roaming\Discord\Local Storage\leveldb\*.*" /q
	return nil
}
