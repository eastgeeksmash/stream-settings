import Moon from "lucide-react/dist/esm/icons/moon"
import Sun from "lucide-react/dist/esm/icons/sun"
import { useTheme } from "./theme-provider"

const themeSun = (
  <Sun className="h-5 w-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
)
const themeMoon = (
  <Moon className="absolute h-5 w-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
)

export function ThemeToggle() {
  const { resolvedTheme, setTheme } = useTheme()

  return (
    <button
      onClick={() => setTheme(resolvedTheme === "dark" ? "light" : "dark")}
      className="rounded-md p-2 hover:bg-slate-100 dark:hover:bg-slate-800"
    >
      {themeSun}
      {themeMoon}
      <span className="sr-only">テーマを切り替え</span>
    </button>
  )
} 