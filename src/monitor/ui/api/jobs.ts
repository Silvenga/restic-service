import { RestEndpoint } from "@data-client/rest";
import { Entity } from "@data-client/endpoint";

export class ResticJob extends Entity {
    job_id = "";
    job = {
        cron: "",
        repository: "",
    };

    pk() {
        return this.job_id;
    }

    static key = "ResticJob";
}

const jobsEndpoint = new RestEndpoint({
    urlPrefix: "http://127.0.0.1:42038",
    path: "/api/v1/jobs",
});

export const getJobIds = jobsEndpoint.extend({
    schema: ["string"],
    pollFrequency: 5000,
});

export const getJob = jobsEndpoint.extend({
    path: "/api/v1/jobs/:jobId",
    schema: ResticJob,
    pollFrequency: 5000,
});

export const queueJob = new RestEndpoint({
    urlPrefix: "http://127.0.0.1:42038",
    path: "/api/v1/jobs/:id/queue",
    method: "POST",
    sideEffect: true,
    body: undefined,
});
