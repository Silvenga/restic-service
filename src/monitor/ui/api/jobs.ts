import { RestEndpoint } from "@data-client/rest";
import { Entity, schema } from "@data-client/endpoint";

export class ResticJobConfig extends Entity {
    cron = "";
    repository = "";

    static key = "ResticJobConfig";
}

export class ResticJob extends Entity {
    job_id = "";
    job = ResticJobConfig.fromJS();

    pk() {
        return this.job_id;
    }

    static key = "ResticJob";
    static schema = {
        job: ResticJobConfig,
    };
}

export const getJobIds = new RestEndpoint({
    urlPrefix: "http://127.0.0.1:42038",
    path: "/api/v1/jobs",
    schema: ["string"],
});

export const getJob = new RestEndpoint({
    urlPrefix: "http://127.0.0.1:42038",
    path: "/api/v1/jobs/:jobId",
    schema: ResticJob,
});

export const queueJob = new RestEndpoint({
    urlPrefix: "http://127.0.0.1:42038",
    path: "/api/v1/jobs/:jobId/queue",
    schema: ResticJob,
    method: "POST",
});
