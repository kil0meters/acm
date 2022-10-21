import useSWR from "swr";
import { Meeting } from ".";
import { api_url, fetcher } from "../../utils/fetcher";
import Link from "next/link";
import { timeFormat } from "../../utils/time";
import { useEffect, useState } from "react";
import { useStore } from "../../utils/state";

export default function Schedule(): JSX.Element {
  const { data, error } = useSWR<Meeting[]>(api_url("/meetings"), fetcher);
  const [isComponentMounted, setIsComponentMounted] = useState(false);
  // const auth = useStore((state) => state.user?.auth);
  const auth = "OFFICER";
  useEffect(() => setIsComponentMounted(true), []);

  function LoadingScheduleItem(): JSX.Element {
    return (
      <div className="border-neutral-300 border-b last:border-0 flex flex-col gap-2 p-2">
        <div className="animate-pulse h-4 rounded bg-neutral-300 w-32" />
        <div className="animate-pulse h-4 rounded bg-neutral-300 w-24" />
      </div>
    );
  }

  function ScheduleItem({ id, title, meeting_time }: Meeting): JSX.Element {
    return (
      <Link href={`/meetings/${id}`}>
        <a className="border-neutral-300 border-b last:border-0 flex flex-col gap-1 p-2 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors">
          <h3 className="font-bold">{title}</h3>
          <span className="text-neutral-600 dark:text-neutral-400 text-sm">
            {timeFormat(meeting_time)}
          </span>
        </a>
      </Link>
    );
  }

  function ScheduleItems({ meetings }: { meetings: Meeting[] | undefined }): JSX.Element {
    if (!meetings) {
      return <>
        <LoadingScheduleItem />
        <LoadingScheduleItem />
        <LoadingScheduleItem />
      </>;
    }

    if (meetings.length === 0) {
      return (
        <div className="p-2">
          {"It looks like there aren't any meetings planned at the moment. Try checking back later."}
        </div>
      );
    }

    return <>
      {meetings.map((meeting, i) => <ScheduleItem key={i} {...meeting} />)}
    </>;
  }

  if (error) return <div>Failed to load schedule</div>;

  return (
    <div className="sm:px-2 flex flex-col gap-2">
      <h2 className="text-2xl font-bold px-2 sm:p-0">Schedule</h2>

      <div className="bg-white dark:bg-black sm:rounded-md border-y sm:border border-neutral-300 dark:border-neutral-700 flex flex-col overflow-hidden">
        <ScheduleItems meetings={data} />
      </div>

      {isComponentMounted && (auth === "OFFICER" || auth === "ADMIN") && (
        <Link href="/meetings/new">
          <a className="text-center rounded-full bg-green-700 hover:bg-green-500 transition-colors text-green-50 py-2 text-sm">
            Add
          </a>
        </Link>
      )}
    </div>
  );
}
