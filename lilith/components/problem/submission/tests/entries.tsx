import { useContext, useEffect, useState } from "react";
import useSWR, { mutate } from "swr";
import { SUBMISSION_TESTS_QUERY, Test, TestResult } from ".";
import { ProblemIDContext } from "../..";
import { api_url, fetcher } from "../../../../utils/fetcher";
import { Submission, useSession, useStore } from "../../../../utils/state";
import { TestResultDescription, TestResultEntry } from "./test_result_view";
import { TestDescription, TestEntry } from "./test_view";

function LoadingTest(): JSX.Element {
  return (
    <div className="animate-pulse flex px-4 justify-center items-center aspect-square bg-neutral-200 dark:bg-slate-700 border border-neutral-400 dark:border-slate-800 b rounded transition-shadow hover:shadow-md hover:ring-2 ring-neutral-400 dark:ring-slate-800">
      <div className="w-full bg-neutral-400 h-4 rounded" />
    </div>
  );
}

export default function TestEntries(): JSX.Element {
  let problemId = useContext(ProblemIDContext);

  const { data: submission } = useSWR<Submission>(
    problemId ? api_url(`/problems/${problemId}/recent-submission`) : null,
    fetcher
  );

  useEffect(() => {
    mutate(SUBMISSION_TESTS_QUERY);
  }, [submission])

  const { data, error } = useSWR<TestDescription[] | TestResultDescription[]>(
    SUBMISSION_TESTS_QUERY,
    () =>
      fetcher(
        api_url(
          (submission && !submission.error)
            ? `/problems/${problemId}/recent-tests`
            : `/problems/${problemId}/tests`
        )
      )
  );

  if (error) return <div>Failed to fetch tests</div>;

  return (
    <div className="overflow-auto border-neutral-300 dark:border-neutral-700 border-b h-full">
      <div className="grid grid-cols-3 lg:grid-cols-4 p-2 gap-2">
        {
          // gotta love double nested ternaries
          !data
            ? Array(5)
              .fill(0)
              .map((_, i) => <LoadingTest key={i} />)
            : (submission && !submission.error)
              ? (data as TestResultDescription[]).map((test, i) => (
                <TestResultEntry key={i} id={test.id} index={test.index} success={test.success} hidden={test.hidden} problemId={problemId!} />
              ))
              : data.map((test, i) => <TestEntry key={i} id={test.id} index={test.index} hidden={test.hidden} problemId={problemId!} />)
        }
      </div>
    </div>
  );
}
