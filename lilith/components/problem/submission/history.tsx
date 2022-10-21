import { useContext } from "react";
import useSWR from "swr";
import { ProblemIDContext } from "..";
import { api_url } from "../../../utils/fetcher";
import { Submission, useStore } from "../../../utils/state";
import { timeFormat } from "../../../utils/time";
import { isServerError, RunnerError, ServerError } from "./error";

export default function SubmissionHistory(): JSX.Element {
  const id = useContext(ProblemIDContext);

  const { data, error } = useSWR<Submission[] | ServerError>(
    id ? api_url(`/problems/${id}/history`) : null,

    async (url: string) => {
      return fetch(url, {
        credentials: "include"
      }).then((res) => res.json());
    }
  );

  function HistoryEntry({
    error,
    success,
    code,
    time,
  }: Submission): JSX.Element {
    const setProblemImpl = useStore((store) => store.setProblemImpl);
    const problem_id = useContext(ProblemIDContext);

    const btn = (
      <button
        className="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 rounded-full font-bold text-blue-50 transition-colors"
        onClick={() => {
          if (problem_id) setProblemImpl(problem_id, code);
        }}
      >
        {"Load"}
      </button>
    );

    if (error) {
      return (
        <div className="flex gap-2 items-center bg-red-100 dark:bg-red-900 p-4 border-neutral-300 dark:border-red-700 border-b">
          <span className="text-red-600 dark:text-red-200 font-bold text-lg">
            Error
          </span>
          <span className="ml-auto text-red-600 dark:text-red-200 text-sm">
            {timeFormat(time)}
          </span>
          {btn}
        </div>
      );
    }

    if (success) {
      return (
        <div className="flex gap-2 items-center bg-green-100 dark:bg-emerald-900 p-4 border-neutral-300 dark:border-emerald-700 border-b">
          <span className="font-bold text-lg text-green-600 dark:text-emerald-200">
            Passed
          </span>
          <span className="ml-auto text-sm text-green-600 dark:text-emerald-200">
            {timeFormat(time)}
          </span>
          {btn}
        </div>
      );
    } else {
      return (
        <div className="flex gap-2 items-center bg-neutral-50 dark:bg-neutral-900 p-4 border-neutral-300 dark:border-neutral-700 border-b">
          <span className="text-red-600 dark:text-red-400 font-bold text-lg">
            Failed
          </span>
          <span className="ml-auto text-neutral-400 dark:text-red-300 text-sm">
            {timeFormat(time)}
          </span>
          {btn}
        </div>
      );
    }
  }

  if (error) {
    return <div>{error.toString()}</div>;
  }

  if (data && isServerError(data)) {
    return <div>{data.error}</div>
  }

  return (
    <div className="h-full bg-white dark:bg-black overflow-y-auto">
      {data &&
        data.map((submission, i) => <HistoryEntry key={i} {...submission} />)}
    </div>
  );
}
