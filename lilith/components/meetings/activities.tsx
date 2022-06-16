import { useContext } from "react";
import useSWR from "swr";
import { MeetingContext } from ".";
import { api_url, fetcher } from "../../utils/fetcher";

type Activity = {
  title: string;
  description: string;
};

export default function Activities(): JSX.Element {
  function ActivitiesLoading(): JSX.Element {
    return (
      <>
        <div className="animate-pulse h-6 bg-neutral-300 w-32 rounded"></div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
          <div className="rounded-md border-neutral-300 border p-2 bg-white flex flex-col gap-2">
            <div className="animate-pulse h-6 bg-neutral-300 w-32 rounded"></div>
            <div className="animate-pulse h-4 bg-neutral-300 w-full rounded"></div>
          </div>
        </div>
      </>
    );
  }

  function ActivitiesInner({ id }: { id: number }): JSX.Element {
    const { data: activities, error } = useSWR<Activity[]>(
      api_url(`/meetings/${id}/activities`),
      fetcher
    );

    if (error) return <div>Failed to fetch activities</div>;
    if (!activities) return <ActivitiesLoading />;

    return (
      <>
        <h2 className="font-bold text-xl px-2 sm:p-0">Activities</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
          {activities.map(({ title, description }, i) => (
            <div
              key={i}
              className="bg-white dark:bg-black border sm:rounded-md border-neutral-300 dark:border-neutral-700 p-2"
            >
              <h3 className="text-lg font-bold">{title}</h3>

              <span className="text-neutral-500 dark:text-neutral-400">
                {description}
              </span>
            </div>
          ))}
        </div>
      </>
    );
  }

  const meeting = useContext(MeetingContext);
  if (!meeting) {
    return <ActivitiesLoading />;
  } else {
    return <ActivitiesInner id={meeting.id} />;
  }
}
