import { api_url } from "./fetcher";

export type JobStatus<T, E> = {
  id: number,
  queue_position: number,

  response?: T,
  error?: E,
};

export async function monitorJob<T, E>(job: JobStatus<T, E>, token: string, updateQueuePosition: (pos: number) => void): Promise<[T?, E?]> {
  // Max timeout = 50s
  for (let i = 0; i < 100; i++) {
    let res = await fetch(api_url(`/run/check/${job.id}`), {
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      }
    });

    let job_status: JobStatus<T, E> = await res.json();

    if (job_status.response) {
      return [job_status.response, undefined];
    }

    if (job_status.error) {
      return [undefined, job_status.error];
    }

    updateQueuePosition(job_status.queue_position);

    // wait 500ms before the next iteration
    await new Promise(resolve => setTimeout(resolve, 500));
  }

  throw Error("ree");
}