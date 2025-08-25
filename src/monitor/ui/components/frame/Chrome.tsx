import { PropsWithChildren, Suspense, useCallback } from "react";
import { twMerge } from "tailwind-merge";
import { getCurrentWindow } from "@tauri-apps/api/window";

type ChromeProps = {
    minimize?: boolean;
    maximize?: boolean;
    close?: boolean;
    title?: string;
    className?: string;
    hideInsteadOfClose?: boolean;
}

export function Chrome(props: PropsWithChildren<ChromeProps>) {
    const {
        children,
        close = true,
        title,
        className,
        hideInsteadOfClose = false,
    } = props;

    const onClose = useCallback(() => {
        const appWindow = getCurrentWindow();
        if (hideInsteadOfClose) {
            void appWindow.hide();
        } else {
            void appWindow.close();
        }
    }, [hideInsteadOfClose]);

    return (
        <div className="w-screen h-screen overscroll-none relative select-none">
            <div className="fixed h-[32px] w-full flex justify-between select-none z-10">
                <div className="drag-region grow flex items-center">
                    <span className="mx-3">{title}</span>
                </div>
                <div className="flex">
                    {close && (
                        <button className={twMerge(
                            "hover:bg-[#c42b1c] w-[46px]",
                            "hover:text-white",
                            "flex items-center justify-center",
                        )} title="Close" aria-label="Close" onClick={onClose}>
                            <span className="size-[11px] text-[11px] leading-[11px] ps-0.25 pt-0.25">&#xe8bb;</span>
                        </button>
                    )}
                </div>
            </div>
            <div className={twMerge("absolute inset-0 top-[32px] overflow-auto", className)}>
                <Suspense>
                    {children}
                </Suspense>
            </div>
        </div>
    );
}
