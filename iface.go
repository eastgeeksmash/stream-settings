package streamsettings

import "context"

type Cleanup interface {
	Clean(ctx context.Context) error // クリーンアップ処理を担当するメソッド
}

type Setup interface {
	Setup(ctx context.Context) error // セットアップ処理を担当するメソッド
}

// セットアップ処理も入れる
