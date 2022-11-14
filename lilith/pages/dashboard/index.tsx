import type { NextPage } from "next";
import Head from "next/head";
import Link from "next/link";
import { useEffect, useState } from "react";
import useSWR from "swr";
import Navbar from "../../components/navbar";
import { api_url, fetcher } from "../../utils/fetcher";
import { Submission, User } from "../../utils/state";

function Completion({ completion }: { completion: Submission }): JSX.Element {
  const { data: user } = useSWR<User>(
    api_url(`/user/id/${completion.user_id}`),
    fetcher
  );

  return (
    <Link href={`/submissions/${completion.id}`}>
      <a className="p-2 border-b last:border-b-0 border-neutral-300 hover:bg-neutral-50 transition-colors flex flex-col">
        <span className="font-extrabold">Problem {completion.problem_id}</span>

        {user && <span>{user.username}</span>}
      </a>
    </Link>
  );
}

function JobElement({ job }: { job: Job }): JSX.Element {
  const { data: user } = useSWR<User>(
    api_url(`/user/id/${job.user_id}`),
    fetcher
  );

  const animate = job.queue_position ? "animate-pulse" : "";

  return (
    <div className={`p-2 border-b last:border-b-0 border-neutral-300 hover:bg-neutral-50 transition-colors flex flex-col ${animate}`}>
      <span className="font-extrabold">Problem {job.problem_id}</span>

      <div className="grid grid-cols-2">
        <span>job type</span>
        <span>{job.job_type}</span>

        {user && <>
          <span>username</span>
          <span>{user.username}</span>
        </>}

        {job.queue_position && <>
          <span>queue position</span>
          <span>{job.queue_position}</span>
        </>}
      </div>
    </div>
  );
}

function CompletionsList({ completions }: { completions: Submission[] }): JSX.Element {
  if (completions.length == 0) return <></>;

  return (
    <div className="rounded-xl border-neutral-300 bg-white border flex flex-col overflow-hidden">
      {completions.map((completion, i) =>
        <Completion key={i} completion={completion} />
      )}
    </div>
  );
}

function JobsList({ jobs }: { jobs: Job[] }): JSX.Element {
  if (jobs.length == 0) return <></>;

  return (
    <div className="rounded-xl border-neutral-300 bg-white border flex flex-col overflow-hidden">
      {jobs.map((job, i) =>
        <JobElement key={i} job={job} />
      )}
    </div>
  );
}

type Job = {
  job_type: "CustomInput" | "SubmitJob";
  problem_id: number,
  user_id: number,
  queue_position?: number,
};

const DashboardPage: NextPage = () => {
  const [completions, setCompletions] = useState<Submission[]>([]);
  const [pendingJobs, setPendingJobs] = useState<Map<number, Job>>(new Map);
  const [finishedJobs, setFinishedJobs] = useState<Job[]>([]);

  useEffect(() => {
    const client = new WebSocket(process.env.NEXT_PUBLIC_WS_URL!);

    console.log("Creating client");

    // fuck me i'm just using any here and there's nothing you can do to stop
    // me.
    client.addEventListener('message', (event) => {
      let data = JSON.parse(event.data);

      if (data.NewJob) {
        console.log(data.NewJob);
        const newJob: Job = {
          job_type: data.NewJob.job_type,
          problem_id: data.NewJob.problem_id,
          user_id: data.NewJob.user_id,
          queue_position: data.NewJob.queue_position,
        };

        setPendingJobs(oldJobs =>
          new Map(oldJobs.set(data.NewJob.id, newJob))
        );
      } else if (data.FinishedJob) {
        const newJob: Job = {
          job_type: data.FinishedJob.job_type,
          problem_id: data.FinishedJob.problem_id,
          user_id: data.FinishedJob.user_id,
        };

        setPendingJobs(oldJobs => {
          oldJobs.delete(data.FinishedJob.id);
          return new Map(oldJobs);
        });
        setFinishedJobs(oldJobs => [newJob, ...oldJobs]);

      } else if (data.NewCompletion) {
        setCompletions(oldCompletions => [data.NewCompletion, ...oldCompletions]);
      }
    });

    return () => {
      console.log("Destroying client");
      client.close();
    };
  }, []);

  return (
    <div>
      <Navbar />

      <Head>
        <title>Admin Dashboard</title>
      </Head>

      <div className="grid grid-cols-3 gap-4 p-4">
        <div className="flex flex-col gap-4">
          <h1 className="font-extrabold text-2xl">Pending Jobs</h1>

          <JobsList jobs={Array.from(pendingJobs.values())} />
        </div>
        <div className="flex flex-col gap-4">
          <h1 className="font-extrabold text-2xl">Finished Jobs</h1>

          <JobsList jobs={finishedJobs} />
        </div>
        <div className="flex flex-col gap-4">
          <h1 className="font-extrabold text-2xl">Completions</h1>

          <CompletionsList completions={completions} />
        </div>
      </div>

    </div>
  );
};

export default DashboardPage;
