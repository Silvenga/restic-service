import { Pane } from "../../frame/Pane.tsx";
import { Button } from "../../frame/Button.tsx";
import { VscArchive } from "react-icons/vsc";
import { TbPlayerPlay } from "react-icons/tb";
import { useSuspense } from "@data-client/react";
import { getJob } from "../../../api/jobs.ts";

type JobProps = {
    jobId: string;
}

export function Job({ jobId }: JobProps) {

    const { job } = useSuspense(getJob, { jobId });

    return (
        <Pane className="p-3 flex items-center">

            <VscArchive className="me-2 size-6 text-neutral-600/80" />
            <div>
                <h3 className="capitalize">{jobId}</h3>
                <p className="text-xs">
                    <span className="text-gray-600">Next run: </span>
                    <span className="text-gray-800 font-medium">{job.cron}</span>
                </p>
            </div>

            <Button className="flex items-center ms-auto">
                <TbPlayerPlay className="text-green-600 me-2" />
                Run
            </Button>
        </Pane>
    );
}
