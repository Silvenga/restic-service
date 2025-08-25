import { Chrome } from "../frame/Chrome.tsx";
import { Jobs } from "./jobs/Jobs.tsx";

export function StatusWindow() {
    return (
        <Chrome className="container mx-auto flex flex-col" hideInsteadOfClose>
            <Jobs />
        </Chrome>
    );
}
