import { useTheme } from "@/components/theme-provider"
import { Toaster as Sonner, ToasterProps } from "sonner"

const Toaster = ({ ...props }: ToasterProps) => {
  const { resolvedTheme } = useTheme()
  const theme = resolvedTheme === "dark" || resolvedTheme === "light" ? resolvedTheme : "system"

  return (
    <Sonner
      theme={theme as ToasterProps["theme"]}
      className="toaster group"
      closeButton
      swipeDirections={[]}
      toastOptions={{
        classNames: {
          toast: "select-text cursor-text",
          title: "select-text whitespace-pre-wrap break-all",
          description: "select-text whitespace-pre-wrap break-all",
        },
      }}
      style={
        {
          "--normal-bg": "var(--popover)",
          "--normal-text": "var(--popover-foreground)",
          "--normal-border": "var(--border)",
        } as React.CSSProperties
      }
      {...props}
    />
  )
}

export { Toaster }
