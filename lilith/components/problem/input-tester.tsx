import { Dispatch, SetStateAction, useContext, useState } from "react";
import { ProblemIDContext } from ".";
import { api_url } from "../../utils/fetcher";
import { JobStatus, monitorJob } from "../../utils/job";
import { useSession, useStore } from "../../utils/state";
import LoadingButton from "../loading-button";
import QueueStatus from "../queue-status";
import ErrorDisplay, { isRunnerError, RunnerError } from "./submission/error";
import TestResultInfo, { TestResult } from "./submission/test-result";

export default function InputTester(): JSX.Element {
  const [loading, setLoading] = useState(false);
  const [input, setInput] = useState("");
  const [queuePosition, setQueuePosition] = useState(0);
  const [testResult, setTestResult] = useState<TestResult | RunnerError | null>(
    null
  );

  const setError = useSession((state) => state.setError);
  const token = useStore((state) => state.token);
  const problem_id = useContext(ProblemIDContext);
  const implementation = useStore((state) =>
    problem_id ? state.problemImpls[problem_id] : undefined
  );

  const testInput = async () => {
    if (!token) {
      setError("You must be logged in to run a test", true);
      return;
    }

    setLoading(true);
    try {
      let res = await fetch(api_url("/run/custom"), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          problem_id,
          input,
          implementation,
        }),
      });

      let job: JobStatus<TestResult, RunnerError> = await res.json();
      let [data, err] = await monitorJob(job, token, (n) => setQueuePosition(n));

      if (err) {
        console.log(err);
        setTestResult(err);
      }

      if (data) {
        setTestResult(data);
      }
    }
    catch (e) {
      console.log(e);
      setError("Network error.", true);
    }
    finally {
      setLoading(false);
    }
  };

  return (
    <div className="border-t border-neutral-300 dark:border-neutral-700 bg-white dark:bg-black flex flex-col lg:flex-row min-h-0">
      <div className="flex flex-col gap-2 lg:w-96 p-4">
        <label>{"Input"}</label>
        <textarea
          onChange={(event) => {
            setInput((event.target as HTMLTextAreaElement).value);
          }}
          className="rounded border border-neutral-300 dark:border-neutral-700 bg-neutral-100 dark:bg-neutral-900 outline-0 transition-shadow focus:ring-2 ring-neutral-300 dark:ring-neutral-700 resize-none p-2 lg:flex-auto"
        ></textarea>

        <div className="flex flex-col sm:flex-row sm:items-center gap-2">
          <LoadingButton
            className="px-4 py-2 rounded-full bg-blue-700 hover:bg-blue-500 transition-colors text-sm text-blue-100 mr-auto"
            loading={loading}
            onClick={() => testInput()}
          >
            Run
          </LoadingButton>

          {loading && <QueueStatus queuePosition={queuePosition} />}
        </div>
      </div>

      <div className="lg:w-96 lg:h-80 overflow-y-auto m-x lg:ml-0">
        {testResult &&
          (isRunnerError(testResult) ? (
            <ErrorDisplay {...testResult} />
          ) : (
            <div className="my-4">
              <TestResultInfo {...testResult} />
            </div>
          ))}
      </div>
    </div>
  );
}
