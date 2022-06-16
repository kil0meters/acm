import { MeetingContext } from ".";
import { timeFormat } from "../../utils/time";
import { useContext } from "react";
import Countdown from "../countdown";

export default function MeetingView(): JSX.Element {
  const meeting = useContext(MeetingContext);

  function MeetingLoading(): JSX.Element {
    return (
      <div className="sm:px-2 flex flex-col gap-2 animate-pulse">
        <h1 className="h-8 rounded bg-neutral-300 w-[50%]" />
        <span className="h-4 rounded bg-neutral-300 w-48" />

        <div className="mx-auto h-12 bg-neutral-300 w-72 rounded"></div>
      </div>
    );
  }

  if (!meeting) return <MeetingLoading />;

  const { title, meeting_time } = meeting;

  return (
    <>
      <div className="px-2 sm:p-0">
        <h1 className="text-3xl font-extrabold">{title}</h1>
        <span className="text-neutral-600 dark:text-neutral-400 text-sm">
          {timeFormat(meeting_time)}
        </span>
      </div>

      <Countdown to={new Date(meeting_time)} />
    </>
  );
}
