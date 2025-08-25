import { ComponentProps } from "react";
import { twMerge } from "tailwind-merge";

export function Button({ className, ...props }: ComponentProps<"button">) {
    return (
        <button className={twMerge(
            "border border-neutral-200 rounded-sm bg-white/50 hover:bg-neutral-100/50 shadow-xs",
            "px-2 py-1 text-sm",
            "flex items-center justify-center",
            "min-w-32",
            "transition-colors duration-100",
            className,
        )} {...props} />
    );
}
