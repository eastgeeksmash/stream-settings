import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core"
import { Button } from "@/components/ui/button"
import { formatInvokeError, showErrorToast } from "@/lib/invoke"

export function AdminWarning() {
  const [open, setOpen] = useState(false)
  const [relaunching, setRelaunching] = useState(false)

  useEffect(() => {
    invoke<boolean>("is_elevated")
      .then((elevated) => {
        setOpen(!elevated)
      })
      .catch(() => {
        // ブラウザプレビューなど、Tauri 以外では出さない
      })
  }, [])

  const relaunch = async () => {
    setRelaunching(true)
    try {
      await invoke("relaunch_as_admin")
    } catch (error) {
      showErrorToast(`管理者として再起動できませんでした: ${formatInvokeError(error)}`)
      setRelaunching(false)
    }
  }

  if (!open) {
    return null
  }

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-6"
      role="presentation"
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-labelledby="admin-warning-title"
        className="w-full max-w-md rounded-md border bg-background p-6 shadow-xs"
      >
        <h2 id="admin-warning-title" className="text-lg font-bold">
          管理者として実行してください
        </h2>
        <p className="mt-3 text-sm leading-relaxed text-muted-foreground">
          電源や更新など、多くの設定を変えるには管理者権限が必要です。このまま続けると、一部の操作が失敗することがあります。
        </p>
        <div className="mt-6 flex justify-end gap-2">
          <Button type="button" variant="outline" onClick={() => setOpen(false)}>
            閉じる
          </Button>
          <Button type="button" disabled={relaunching} onClick={() => void relaunch()}>
            管理者として再起動
          </Button>
        </div>
      </div>
    </div>
  )
}
