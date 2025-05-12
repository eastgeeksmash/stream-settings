package main

import (
	"context"
	"flag"
	"log/slog"
	"os"
	"os/signal"

	streamsettings "github.com/eastgeeksmash/stream-settings"
)

var (
	flagDiscord  = flag.Bool("discord", false, "Discordのクリーンアップを実行する")
	flagChrome   = flag.Bool("chrome", false, "Chromeのクリーンアップを実行する")
	flagObs      = flag.Bool("obs", false, "OBSのクリーンアップを実行する")
	flagDownload = flag.Bool("download", false, "ダウンロードフォルダのクリーンアップを実行する")
)

func main() {
	flag.Parse()

	cleanupHandlers := []streamsettings.Cleanup{}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	logger := slog.New(slog.NewTextHandler(os.Stdout, nil))

	if *flagDiscord {
		cleanupHandlers = append(cleanupHandlers, streamsettings.NewDiscordCleanup(logger))
	}

	if *flagChrome {
		cleanupHandlers = append(cleanupHandlers, streamsettings.NewChromeCleanup(logger))
	}

	if *flagObs {
		cleanupHandlers = append(cleanupHandlers, streamsettings.NewObsCleanup(logger))
	}

	if *flagDownload {
		cleanupHandlers = append(cleanupHandlers, streamsettings.NewDownloadCleanup(logger))
	}

	logger.Info("クリーンアップを開始します")
	for _, handler := range cleanupHandlers {
		if err := handler.Clean(ctx); err != nil {
			logger.Error("クリーンアップに失敗しました", "error", err)
			os.Exit(1)
		}
	}

	logger.Info("クリーンアップが完了しました")
}
