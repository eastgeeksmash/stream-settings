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
	flagObs = flag.Bool("obs", false, "OBSのセットアップを実行する")
)

func main() {
	flag.Parse()

	setupHandlers := []streamsettings.Setup{}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	logger := slog.New(slog.NewTextHandler(os.Stdout, nil))

	if *flagObs {
		setupHandlers = append(setupHandlers, streamsettings.NewObsSetup(logger))
	}

	logger.Info("セットアップを開始します")
	for _, handler := range setupHandlers {
		if err := handler.Setup(ctx); err != nil {
			logger.Error("セットアップに失敗しました", "error", err)
			os.Exit(1)
		}
	}

	logger.Info("セットアップが完了しました")
}
