import type { ReactNode } from "react"
import { ThemeProvider as NextThemesProvider, useTheme } from "next-themes"

type ThemeProviderProps = {
  children: ReactNode
  defaultTheme?: "dark" | "light" | "system"
  storageKey?: string
}

export function ThemeProvider({
  children,
  defaultTheme = "system",
  storageKey = "vite-ui-theme",
}: ThemeProviderProps) {
  return (
    <NextThemesProvider
      attribute="class"
      defaultTheme={defaultTheme}
      enableSystem
      disableTransitionOnChange
      storageKey={storageKey}
    >
      {children}
    </NextThemesProvider>
  )
}

export { useTheme }
