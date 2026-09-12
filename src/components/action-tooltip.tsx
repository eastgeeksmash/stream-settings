import type { ReactNode } from "react"
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip"

type ActionTooltipProps = {
  text: string
  children: ReactNode
}

export function ActionTooltip({ text, children }: ActionTooltipProps) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span className="block w-full min-w-0">{children}</span>
      </TooltipTrigger>
      <TooltipContent
        side="top"
        align="start"
        className="max-w-xs text-left leading-relaxed"
      >
        {text}
      </TooltipContent>
    </Tooltip>
  )
}
