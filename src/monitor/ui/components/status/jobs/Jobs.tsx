import { useLive } from "@data-client/react";
import { Pane } from "../../frame/Pane.tsx";
import { Job } from "./Job.tsx";
import { MdSchedule } from "react-icons/md";
import { getJobIds } from "../../../api/jobs.ts";

export function Jobs() {

    return (
        <div className="p-6 flex flex-col space-y-6">

            <h2 className="sr-only">Active job</h2>

            <Pane className="p-3 min-h-16 flex items-center justify-center">
                <p className="text-gray-600 text-sm">No Active Job</p>
            </Pane>

            <h2 className="font-semibold mb-2 mt-3 flex items-center text-neutral-700">
                <MdSchedule className="me-1 size-5" />
                Inactive Jobs
            </h2>

            <div className="flex flex-col space-y-1">
                <JobList />
            </div>
        </div>
    );
}

function JobList() {
    const jobIds = useLive(getJobIds);
    return (
        <>
            {jobIds.map((jobId) => (
                <Job key={jobId} jobId={jobId} />
            ))}
        </>
    );
}
