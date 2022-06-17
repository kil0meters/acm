import { useContext, useEffect, useState } from "react";
import useSWR from "swr";
import { SUBMISISON_TESTS_QUERY, Test, TestResult } from ".";
import { ProblemIDContext } from "../..";
import { api_url, fetcher } from "../../../../utils/fetcher";
import { useSession, useStore } from "../../../../utils/state";
import Modal from "../../../modal";
import TestResultInfo from "../test-result";

function LoadingTest(): JSX.Element {
  return (
    <div className="animate-pulse flex px-4 justify-center items-center aspect-square bg-neutral-200 dark:bg-slate-700 border border-neutral-400 dark:border-slate-800 b rounded transition-shadow hover:shadow-md hover:ring-2 ring-neutral-400 dark:ring-slate-800">
      <div className="w-full bg-neutral-400 h-4 rounded" />
    </div>
  );
}

function TestEntry({ index, input, expected_output }: Test): JSX.Element {
  const [shown, setShown] = useState(false);

  return (
    <>
      <button
        onClick={() => setShown(true)}
        className="aspect-square bg-neutral-200 dark:bg-slate-700 border border-neutral-400 dark:border-slate-800 b rounded transition-shadow hover:shadow-md hover:ring-2 ring-neutral-400 dark:ring-slate-800"
      >
        Test #{index}
      </button>

      <Modal shown={shown} onClose={() => setShown(false)}>
        <div className="shadow-lg bg-white rounded-md border border-neutral-300 dark:bg-black dark:border-neutral-700 p-4 flex flex-col gap-2">
          <h2 className="text-2xl">
            Test #{index}
          </h2>

          <label>Input</label>

          <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
            <code>{input}</code>
          </pre>

          <label>Expected</label>

          <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
            <code>{expected_output}</code>
          </pre>
        </div>
      </Modal>
    </>
  );
}

function TestResultEntry(test: TestResult): JSX.Element {
  const [shown, setShown] = useState(false);

  const baseStyles =
    "aspect-square border b rounded transition-shadow hover:shadow-md hover:ring-2";

  const flexStyles = test.success
    ? "bg-green-200 dark:bg-green-800 dark:border-green-600 dark:ring-green-600 dark:text-green-100 border-green-400 ring-green-400 text-green-900"
    : "bg-red-200 dark:bg-red-800 dark:border-red-600 dark:ring-red-600 dark:text-red-100 border-red-400 ring-red-400 text-red-900";

  return (
    <>
      <button
        className={`${baseStyles} ${flexStyles}`}
        onClick={() => setShown(true)}
      >
        {`Test #${test.index}`}
      </button>

      <Modal shown={shown} onClose={() => setShown(false)}>
        <div className="bg-white dark:bg-black rounded-md border border-neutral-300 dark:border-neutral-700 p-4">
          <TestResultInfo {...test} />
        </div>
      </Modal>
    </>
  );
}

export default function TestEntries(): JSX.Element {
  let problemId = useContext(ProblemIDContext);
  const submissionId = useSession(
    (state) => problemId && state.submissions[problemId]?.id
  );

  const { data, error } = useSWR<Test[] | TestResult[]>(
    SUBMISISON_TESTS_QUERY,
    () =>
      fetcher(
        api_url(
          submissionId
            ? `/submissions/${submissionId}/tests`
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
            : submissionId
            ? (data as TestResult[]).map((test, i) => (
                <TestResultEntry key={i} {...test} />
              ))
            : data.map((test, i) => <TestEntry key={i} {...test} />)
        }
      </div>
    </div>
  );
}
