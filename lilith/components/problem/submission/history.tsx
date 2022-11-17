import Link from "next/link";
import { useContext, useState } from "react";
import useSWRInfinite from "swr/infinite";
import { ShareButton } from ".";
import { ProblemIDContext } from "..";
import { api_url, fetcher } from "../../../utils/fetcher";
import { Submission, useStore } from "../../../utils/state";
import { timeFormat } from "../../../utils/time";
import LoadingButton from "../../loading-button";
import { isServerError, RunnerError, ServerError } from "./error";

function LoadHistoryButton({ id }: { id: number }): JSX.Element {
  const problemId = useContext(ProblemIDContext)!;
  const [loading, setLoading] = useState(false);
  const setProblemImpl = useStore((store) => store.setProblemImpl);

  const submit = async () => {
    setLoading(true);
    let data: Submission = await (await fetch(api_url(`/submissions/${id}`))).json();
    setProblemImpl(problemId, data.code);
    setLoading(false);
  }

  return (
    <LoadingButton
      className="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 rounded-full font-bold text-blue-50 transition-colors"
      loading={loading}
      onClick={submit}
    >
      {"Load"}
    </LoadingButton>
  );
}


function HistoryEntry({
  id,
  success,
  time,
  error,
  runtime
}: Submission): JSX.Element {
  if (error) {
    return (
      <div className="flex gap-2 items-center bg-red-100 dark:bg-red-900 p-4 border-neutral-300 dark:border-red-700 border-b">
        <span className="text-red-600 dark:text-red-200 font-bold text-lg">
          Error
        </span>
        <span className="ml-auto text-red-600 dark:text-red-200 text-sm">
          {timeFormat(time + 'Z')}
        </span>
        <ShareButton className="px-4 py-2 text-sm bg-neutral-300 hover:bg-neutral-200 rounded-full font-bold transition-colors" path={`/submissions/${id}`} />
        <LoadHistoryButton id={id} />
      </div>
    );
  }

  const fuelCompact = Intl.NumberFormat('en', { notation: "compact" }).format(runtime) + " fuel";
  const fuelLong = Intl.NumberFormat('en', { notation: "standard" }).format(runtime) + " fuel";

  if (success) {
    return (
      <div className="flex gap-2 items-center bg-white dark:bg-black p-4 border-neutral-300 dark:border-emerald-700 border-b">
        <span className="font-bold text-lg text-green-600 dark:text-emerald-200">
          Passed
        </span>

        <span className="text-sm text-green-600" title={fuelLong}>{fuelCompact}</span>
        <span className="ml-auto text-sm text-neutral-500 dark:text-emerald-200">
          {timeFormat(time + 'Z')}
        </span>
        <ShareButton className="px-4 py-2 text-sm bg-neutral-300 hover:bg-neutral-200 rounded-full font-bold transition-colors" path={`/submissions/${id}`} />
        <LoadHistoryButton id={id} />
      </div>
    );
  } else {
    return (
      <div className="flex gap-2 items-center bg-white dark:bg-black p-4 border-neutral-300 dark:border-neutral-700 border-b">
        <span className="text-red-700 dark:text-red-400 font-bold text-lg">
          Failed
        </span>
        <span className="ml-auto text-sm text-neutral-500 dark:text-emerald-200">
          {timeFormat(time + 'Z')}
        </span>
        <ShareButton className="px-4 py-2 text-sm bg-neutral-300 hover:bg-neutral-200 rounded-full font-bold transition-colors" path={`/submissions/${id}`} />
        <LoadHistoryButton id={id} />
      </div>
    );
  }
}

export default function SubmissionHistory(): JSX.Element {
  const id = useContext(ProblemIDContext);

  const { data, error, isValidating, size, setSize } = useSWRInfinite<Submission[]>(
    (pageIndex, previousSubmissions) => {
      if (previousSubmissions && !previousSubmissions.length) return null;
      return api_url(`/problems/${id}/history?offset=${15 * pageIndex}&count=15`);
    },
    fetcher
  );

  if (error) {
    return <div>{error.toString()}</div>;
  }

  if (data && isServerError(data)) {
    return <div>{data.error}</div>
  }

  return (
    <div className="flex flex-col h-full bg-white dark:bg-black overflow-y-auto">
      {data && data.map((submissions) =>
        submissions.map((submission, i) =>
          <HistoryEntry key={i} {...submission} />))
      }

      <LoadingButton
        loading={isValidating}
        className="mx-auto my-4 rounded-full bg-neutral-200 hover:bg-neutral-300 px-6 py-3 transition-colors mx-auto dark:hover:bg-neutral-700 dark:bg-neutral-800"
        onClick={() => setSize(size + 1)}
      >Load more</LoadingButton>
    </div>
  );
}
