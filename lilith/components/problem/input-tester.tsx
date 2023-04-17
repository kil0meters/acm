import produce from "immer";
import { Dispatch, SetStateAction, useContext, useState } from "react";
import useSWR from "swr";
import { ProblemIDContext } from ".";
import { api_url, fetcher } from "../../utils/fetcher";
import { JobStatus, monitorJob } from "../../utils/job";
import { useSession, useStore } from "../../utils/state";
import ErrorBox from "../error-box";
import LoadingButton from "../loading-button";
import QueueStatus from "../queue-status";
import { TestEditor } from "../test-editor";
import { DiagnosticsDisplay } from "./submission";
import { isRunnerError, RunnerError } from "./submission/error";
import { Test, TestResult } from "./submission/tests";
import { TestResultInner } from "./submission/tests/test_result_view";

type CustomInputResponse = {
    result: TestResult;
    output: string;
};

export default function InputTester() {
    const [loading, setLoading] = useState(false);
    const [queuePosition, setQueuePosition] = useState(0);
    const [testResult, setTestResult] = useState<CustomInputResponse | null>(
        null
    );
    const [resultError, setResultError] = useState<string | null>(null);

    const setError = useSession((state) => state.setError);
    const problem_id = useContext(ProblemIDContext);
    const implementation = useStore((state) =>
        problem_id ? state.problemImpls[problem_id] : undefined
    );

    let { data, error } = useSWR<Test>(problem_id ? api_url(`/problems/${problem_id}/tests/0`) : null, fetcher);

    if (!data || error) return null;

    let input = data.input;

    const testInput = async () => {
        if (!implementation) {
            setError("You must modify the answer before submitting.", true);
            return;
        }

        setLoading(true);
        try {
            let res = await fetch(api_url("/run/custom"), {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify({
                    problem_id,
                    input,
                    implementation,
                }),
            });

            let job: JobStatus<CustomInputResponse, string> = await res.json();

            let [data, err] = await monitorJob(job, (n) => setQueuePosition(n));

            if (err) {
                setResultError(err);
                setTestResult(null);
            }

            if (data) {
                setTestResult(data);
                setResultError(null);
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
        <div className="border-t max-h-[40vh] lg:max-h-full border-neutral-300 dark:border-neutral-700 bg-white dark:bg-black flex flex-col lg:flex-row min-h-0 overflow-y-auto lg:overflow-y-visible">
            <div className="flex flex-col gap-2 lg:w-96 p-4 lg:h-80 lg:overflow-y-auto">
                <b>Input</b>

                {problem_id && <TestEditor baseArgs={data.input.arguments} onChange={(args) => {
                    if (input) {
                        input.arguments = args;
                    }
                }} />}

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

            <div className="lg:h-80 lg:w-96 lg:overflow-y-auto m-x lg:ml-0 px-4 lg:pl-0">
                {resultError &&
                    <div className="bg-red-500 dark:bg-red-700 text-red-50 flex flex-col h-full border-x border-red-700">
                        <h1 className="text-2xl font-bold my-auto p-4">Error</h1>
                        <DiagnosticsDisplay error={resultError} />
                        <div className="flex-grow border-t border-neutral-300 bg-white" />
                    </div>
                }

                {testResult &&
                    <div className="my-4 mx-1">
                        <TestResultInner {...testResult.result} />
                    </div>
                }
            </div>

            {testResult !== null && !isRunnerError(testResult) && testResult.output && testResult.output.length > 0 &&
                <div className="py-2 lg:w-96 flex flex-col gap-2 px-4 lg:px-0 w-full lg:h-80">
                    <span className="font-bold">Debug Output</span>
                    <pre
                        className="rounded-md bg-blue-50 dark:bg-stone-900 p-2 overflow-auto border border-blue-200 dark:border-slate-700"
                    >
                        {testResult.output}
                    </pre>
                </div>
            }
        </div>
    );
}
