import { NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import shallow from "zustand/shallow";
import Navbar from "../../components/navbar";
import { api_url } from "../../utils/fetcher";
import { useAdminStore, useSession, useStore } from "../../utils/state";

export interface Activity {
  title: string,
  description: string,
  activity_type: "SOLO" | "PAIR" | "LECT"
}

function ActivitiesEditor(): JSX.Element {
  const [pushActivity, popActivity] = useAdminStore((state) => [state.pushMeetingActivity, state.popMeetingActivity], shallow);
  const activityCount = useAdminStore((state) => state.meetingActivities.length);

  const formClasses = "border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300";

  function ActivityEditor({ index }: { index: number }): JSX.Element {
    const { title, description, activity_type: activityType } = useAdminStore((state) => state.meetingActivities[index]!);
    const updateActivity = useAdminStore((state) => state.updateMeetingActivity);

    return <>
      {index === 0 && <h2 className="text-lg font-bold">Activities</h2>}

      <div className="bg-white dark:bg-black border-y md:border border-neutral-300 dark:border-neutral-700 p-2 md:rounded-md grid grid-cols-2 gap-2">
        <div className="flex flex-col">
          <label>{"Name"}</label>
          <input
            onChange={event => updateActivity(index, { title: event.currentTarget.value })}
            value={title}
            className={formClasses}
          />
        </div>

        <div className="flex flex-col">
          <label>{"Type"}</label>
          <select
            className={formClasses}
            value={activityType}
            // @ts-ignore
            onChange={event => updateActivity(index, { activity_type: event.currentTarget.value })}
          >
            <option value="LECT">{"Lecture"}</option>
            <option value="PAIR">{"Pair Programming"}</option>
            <option value="SOLO">{"Solo Competition"}</option>
          </select>
        </div>

        <div className="flex flex-col col-span-2">
          <label>{"Description"}</label>
          <textarea
            onChange={event => updateActivity(index, { description: event.currentTarget.value })}
            value={description}
            className={formClasses}
          />
        </div>
      </div>
    </>;
  }

  return <>
    {Array(activityCount).fill(0).map((_, i) => (
      <ActivityEditor key={i} index={i} />
    ))}

    <div className="flex gap-2 mx-2 md:m-0">
      <button
        className="transition-shadow rounded-md border hover:ring px-4 py-2 border-blue-700 bg-blue-500 ring-blue-700 text-blue-50"
        onClick={pushActivity}>
        Add activity
      </button>
      <button
        className="transition-shadow rounded-md border hover:ring px-4 py-2 border-red-700 bg-red-500 ring-red-700 text-red-50"
        onClick={popActivity}>
        Remove activity
      </button>
    </div>
  </>;
}

// {"title":"sdfgsd","description":"fgsdfgsdfg","meeting_time":"2022-06-30T19:10","activities":[{"title":"dfgsdfg","description":"sdfgs","activity_type":"SOLO"}]}

const MeetingEditor: NextPage = () => {
  const formClasses = "border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300";
  const [isComponentMounted, setIsComponentMounted] = useState(false);
  useEffect(() => setIsComponentMounted(true), []);
  const setError = useSession((state) => state.setError);
  const router = useRouter();

  const submit = async () => {
    try {
      const {
        meetingTitle: title,
        meetingDescription: description,
        meetingTime: meeting_time,
        meetingActivities: activities,
      } = useAdminStore.getState();

      const body = JSON.stringify({
        title,
        description,
        meeting_time,
        activities,
      });

      console.log(body);

      const res = await (await fetch(api_url("/meetings/edit"), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body,
      })).json();

      if (res.error) {
        setError(res.error, true);
      } else {
        router.push(`/meetings/${res.id}`)
      }
    }
    catch (e) {
      setError("Network error.", true);
    }
  };

  function TitleForm(): JSX.Element {
    const [meetingTitle, setMeetingTitle] = useAdminStore((state) => [state.meetingTitle, state.setMeetingTitle], shallow)

    return (
      <div className="flex flex-col">
        <label>Title</label>
        <input
          className={formClasses}
          value={meetingTitle}
          onChange={(event) => setMeetingTitle(event.currentTarget.value)}
        />
      </div>
    );
  }

  function TimeForm(): JSX.Element {
    const [meetingTime, setMeetingTime] = useAdminStore((state) => [state.meetingTime, state.setMeetingTime], shallow)

    return (
      <div className="flex flex-col">
        <label>{"Meeting Time"}</label>
        <input
          className={formClasses}
          type="datetime-local"
          step={1}
          value={meetingTime}
          onChange={(event) => setMeetingTime(event.currentTarget.value)}
        />
      </div>
    );
  }

  function DescriptionForm(): JSX.Element {
    const [meetingDescription, setMeetingDescription] = useAdminStore((state) => [state.meetingDescription, state.setMeetingDescription], shallow)

    return (
      <div className="flex flex-col col-span-2">
        <label>{"Description"}</label>
        <textarea
          className={formClasses}
          value={meetingDescription}
          onChange={(event) => setMeetingDescription(event.currentTarget.value)}
        />
      </div>
    );
  }

  return (
    <>
      <Navbar />

      <div className="max-w-screen-md mx-auto my-2 flex flex-col gap-2">
        <div className="flex items-center mx-2 md:m-0">
          <h1 className="text-2xl font-bold">{"New Meeting"}</h1>
          <button onClick={submit} className="ml-auto bg-green-700 hover:bg-green-500 text-green-50 transition-colors rounded-full px-4 py-2 text-sm">{"Submit"}</button>
        </div>

        <div className="bg-white dark:bg-black border-y md:border border-neutral-300 dark:border-neutral-700 p-2 md:rounded-md grid grid-cols-2 gap-2">
          <TitleForm />
          <TimeForm />
          <DescriptionForm />
        </div>

        {isComponentMounted && <ActivitiesEditor />}
      </div>
    </>
  );
}

export default MeetingEditor;
