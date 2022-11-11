import { NextPage } from "next";
import dynamic from "next/dynamic";
import { useRouter } from "next/router";
import { useState } from "react";
import useSWR from "swr";
import shallow from "zustand/shallow";
import LoadingButton from "../../components/loading-button";
import Navbar from "../../components/navbar";
import DescriptionEditor from "../../components/problem-editor/description-editor";
import TestsEditor from "../../components/problem-editor/tests-editor";
import Tabbed from "../../components/tabbed";
import { api_url, fetcher } from "../../utils/fetcher";
import { useAdminStore, useSession, useStore } from "../../utils/state";
import { Competition } from "../competitions";
const Editor = dynamic(import("../../components/editor"), { ssr: false });

function RunnerEditor(): JSX.Element {
  const [runner, setRunner] = useAdminStore(
    (state) => [state.problemRunner, state.setProblemRunner],
    shallow
  );

  return (
    <Editor
      language="cpp"
      value={runner}
      onChange={(value, _event) => setRunner(value)}
    />
  );
}

function TemplateEditor(): JSX.Element {
  const [template, setTemplate] = useAdminStore(
    (state) => [state.problemTemplate, state.setProblemTemplate],
    shallow
  );

  return (
    <Editor
      language="cpp"
      value={template}
      onChange={(value, _event) => setTemplate(value)}
    />
  );
}

function TitleEditor(): JSX.Element {
  const [title, setTitle] = useAdminStore((state) => [state.problemTitle, state.setProblemTitle], shallow);

  return (
    <div className="border-b lg:border-0 bg-white dark:bg-black border-neutral-300 dark:border-neutral-700 flex flex-col gap-2 p-2">
      <label className="font-bold">Title</label>
      <input
        className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
        onChange={(event) => setTitle(event.currentTarget.value)}
        value={title}
        placeholder="Title"
      />
    </div>
  );
}

function Scheduler(): JSX.Element {
  const [dateShown, setDateShown] = useAdminStore((state) => [state.problemDateShown, state.setProblemDateShown]);
  const [publishTime, setPublishTime] = useAdminStore((state) => [state.problemPublishTime, state.setProblemPublishTime]);

  function toggleDateShown() {
    if (dateShown) {
      setPublishTime(undefined);
    }

    setDateShown(!dateShown);
  }

  let time = publishTime ? new Date(publishTime) : new Date();
  time.setMinutes(time.getMinutes() - time.getTimezoneOffset());
  let formattedTime = time.toISOString().substring(0, 16);

  return (
    <div className="px-4 flex gap-4 items-center border-r border-neutral-300 h-full">
      <label>Custom publish time: </label>
      <input type="checkbox" defaultChecked={dateShown} onChange={toggleDateShown} />
      {dateShown && <input
        className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
        type="datetime-local"
        value={formattedTime}
        onChange={(e) => {
          setPublishTime(e.target.value);
        }}
      />}
    </div>
  );
}

function CompetitionPicker(): JSX.Element {
  const { data } = useSWR<Competition[]>(api_url("/competitions"), fetcher);
  const [competitionId, setCompetitionId] = useAdminStore((state) => [state.problemCompetitionId, state.setProblemCompetitionId]);

  return (
    <div className="px-4 border-neutral-300 border-r h-full flex gap-4 items-center">
      <label>Competition: </label>
      <select
        value={competitionId}
        onChange={(e) => {
          let newValue = parseInt(e.target.value);
          setCompetitionId(newValue == -1 ? undefined : newValue);
        }}
        className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
      >
        <option value={-1}>None (default)</option>
        {data && data.map((competition, i) => <option key={i} value={competition.id}>{competition.name}</option>)}
      </select>

    </div>
  );
}

function SubmitButton(): JSX.Element {
  const setError = useSession((state) => state.setError);
  const [loading, setLoading] = useState(false);
  const router = useRouter();
  const clearProblemEditor = useAdminStore((state) => state.clearProblemCreation);

  const submitProblem = async () => {

    const {
      problemTitle: title,
      problemDescription: description,
      problemRunner: runner,
      problemReference: reference,
      problemTemplate: template,
      problemTests: tests,
      problemPublishTime: publish_time,
      problemRuntimeMultiplier: runtime_multiplier,
      problemCompetitionId: competition_id
    } = useAdminStore.getState();

    // TODO: Look into zod validator
    if (!title || !description || !runner || !reference || !template || tests.length === 0) {
      setError("One or more required fields is empty.", true);
      return;
    }

    setLoading(true);
    try {
      const res = await (await fetch(api_url("/problems/new"), {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        credentials: "include",
        body: JSON.stringify({
          title,
          description,
          runner,
          reference,
          template,
          tests,
          publish_time: publish_time ? new Date(publish_time).toISOString().slice(0, -1) : undefined,
          runtime_multiplier,
          competition_id
        }),
      })).json();

      if (res.error) {
        setError(res.error, true);
      } else {
        clearProblemEditor();
        router.push(`/problems/${res.id}`);
      }

    } catch (e) {
      setError("Network error.", true);
    }
    setLoading(false);
  };

  return (
    <LoadingButton
      className="lg:ml-auto rounded-full p-4 bg-green-600 hover:bg-green-500 text-green-50 transition-colors mx-2 mb-8 lg:m-0 lg:rounded-none lg:h-full"
      onClick={submitProblem}
      loading={loading}
    >
      Submit
    </LoadingButton>
  )
}

const ProblemList: NextPage = () => {
  return (
    <div className="grid grid-rows-min-full grid-cols-full w-screen h-screen">
      <Navbar />

      <div className="flex flex-col gap-2 lg:gap-0 lg:grid lg:grid-cols-[450px_minmax(0,1fr)] lg:grid-rows-full-min">
        <div className="grid grid-rows-min-full grid-cols-full gap-2 lg:gap-0 lg:border-r border-neutral-300 dark:border-neutral-700 row-span-2">
          <TitleEditor />
          <DescriptionEditor />
        </div>

        <Tabbed
          className="border-y border-neutral-300 dark:border-neutral-700 lg:border-0"
          titles={["Runner", "Template", "Tests"]}
        >
          <RunnerEditor />
          <TemplateEditor />
          <TestsEditor />
        </Tabbed>

        <div className="border-t border-neutral-300 dark:border-neutral-700 flex flex-col items-center lg:bg-white dark:lg:bg-black lg:flex-row">
          <Scheduler />
          <CompetitionPicker />
          <SubmitButton />
        </div>
      </div>
    </div>
  );
};

export default ProblemList;
