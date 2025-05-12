package streamsettings

import (
	"context"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"

	"golang.org/x/xerrors"
)

type discordCeanup struct {
	logger *slog.Logger
}

func NewDiscordCleanup(logger *slog.Logger) Cleanup {
	return &discordCeanup{logger: logger}
}

func (d *discordCeanup) Clean(ctx context.Context) error {
	// TODO: DiscordのプロセスをKillする
	// Discordプロセスを強制終了
	cmd := exec.Command("taskkill", "/F", "/IM", "Discord.exe", "/T")
	output, err := cmd.CombinedOutput()
	if err != nil {
		d.logger.Warn("Discordプロセスの終了に失敗しました", "error", err, "output", string(output))
		// プロセス終了の失敗はクリーンアップ全体を中断する理由にはならないため、エラーは返さない
	} else {
		d.logger.Info("Discordプロセスを終了しました")
	}

	// Discordのローカルストレージをクリーンアップ
	username := os.Getenv("USERNAME")
	if username == "" {
		d.logger.Warn("USERNAMEが見つかりません")
		return nil
	}

	discordPath := filepath.Join("C:", "Users", username, "AppData", "Roaming", "Discord", "Local Storage", "leveldb")
	files, err := filepath.Glob(filepath.Join(discordPath, "*.*"))
	if err != nil {
		return xerrors.Errorf("Discordのファイル検索エラー: %w", err)
	}

	for _, file := range files {
		if err := os.Remove(file); err != nil {
			return xerrors.Errorf("ファイル削除エラー: %w", err)
		}
	}
	return nil
}
