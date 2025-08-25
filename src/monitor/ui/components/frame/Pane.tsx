import { PropsWithChildren } from "react";
import { twMerge } from "tailwind-merge";

type PaneProps = {
    className?: string;
}

export function Pane({ children, className }: PropsWithChildren<PaneProps>) {
    return (
        <div className={twMerge("border border-neutral-200 rounded-md bg-neutral-50/50", className)}>
            {children}
        </div>
    );
}
