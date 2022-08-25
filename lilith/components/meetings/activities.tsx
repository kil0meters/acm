import { useContext, useState } from "react";
import useSWR from "swr";
import { MeetingContext } from ".";
import { api_url, fetcher } from "../../utils/fetcher";
import Modal from "../modal";

type Activity = {
  title: string;
  description: string;
  activity_type: "SOLO" | "PAIR" | "LECT";
};

function ActivityEntry({ title, description, activity_type }: Activity): JSX.Element {
  let [shown, setShown] = useState(false);

  return <>
    <button onClick={() => setShown(true)} className="text-left bg-white dark:bg-black border sm:rounded-md border-neutral-300 dark:border-neutral-700 p-2 transition-all hover:shadow-md">
      <h3 className="text-lg font-bold">{title}</h3>

      <span className="text-neutral-500 dark:text-neutral-400">
        {description}
      </span>

    </button>

    <Modal shown={shown} onClose={() => setShown(false)}>
      <div className="bg-white dark:bg-black border-neutral-300 border dark:border-neutral-700 rounded-lg shadow-md overflow-hidden">
        <div className="p-4 bg-neutral-100 border-b border-neutral-300 flex gap-4 items-center">
          <h1 className="font-extrabold text-3xl">{title}</h1> <span className="rounded-full p-2 px-3 bg-slate-800 text-slate-50 text-xs">{activity_type}</span>
        </div>

        <div className="p-2 flex flex-col gap-2">
          <p>{description}</p>

          <button className="text-lg font-bold bg-green-500 p-4 rounded text-neutral-700">Sign up</button>
        </div>
      </div>
    </Modal>
  </>;

}

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
          {activities.map((activity, i) => <ActivityEntry key={i} {...activity} />)}
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
