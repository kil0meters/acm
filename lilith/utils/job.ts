import { api_url } from "./fetcher";

export type JobStatus<T, E> = {
    id: number,
    queue_position: number,

    response?: T,
    error?: E,
};

export async function monitorJob<T, E>(job: JobStatus<T, E>, updateQueuePosition: (pos: number) => void): Promise<[T?, E?]> {
    let oldQueuePosition = 0;

    if (job.error) {
        return [undefined, job.error];
    }

    // Max timeout = 50s
    for (let i = 0; i < 500; i++) {
        let res = await fetch(api_url(`/run/check/${job.id}`), {
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",
        });

        let job_status: JobStatus<T, E> = await res.json();

        if (job_status.response) {
            return [job_status.response, undefined];
        }

        if (job_status.error) {
            return [undefined, job_status.error];
        }

        // Reset queue timeout if queue position updated
        if (oldQueuePosition != job_status.queue_position)
            i = 0;

        updateQueuePosition(job_status.queue_position);

        // wait 1s before the next iteration
        await new Promise(resolve => setTimeout(resolve, 500));
    }

    throw Error("Test took too long");
}
