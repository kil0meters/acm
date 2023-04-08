import dynamic from "next/dynamic";
import { useState } from "react";
import useSWR from "swr";
import shallow from "zustand/shallow";
import { api_url, fetcher } from "../../utils/fetcher";
import { JobStatus, monitorJob } from "../../utils/job";
import { useAdminStore, User, useSession, useStore } from "../../utils/state";
import LoadingButton from "../loading-button";
import { isRunnerError, isServerError, RunnerError, ServerError } from "../problem/submission/error";
import { FunctionTypeDisplay, Test } from "../problem/submission/tests";
import QueueStatus from "../queue-status";
import { TestEditor } from "../test-editor";
const Editor = dynamic(import("../../components/editor"), { ssr: false });

function TestEditorWithFuel({ index }: { index: number }): JSX.Element {
    const state = useAdminStore.getState();
    const [input, expectedOutput, max_fuel] = [
        state.problemTests[index]?.input,
        state.problemTests[index]?.expected_output,
        state.problemTests[index]?.max_fuel
    ];

    const updateTest = useAdminStore((state) => state.updateProblemTest);

    let long = Intl.NumberFormat('en', { notation: "standard" }).format(max_fuel!);

    return (
        <div className="bg-white border-neutral-300 border p-2 flex flex-col gap-2">
            <div>
                <span className="font-bold">Input</span>
                <TestEditor baseArgs={input.arguments} onChange={(args) => {
                    // console.log(args);
                    updateTest(index, {
                        input: { name: input.name, arguments: args, return_type: input.return_type }
                    })
                }} />
            </div>

            <div className="flex flex-col">
                <span className="font-bold">Output</span>
                <span className="text-sm">Consumed {long} fuel</span>
                <FunctionTypeDisplay data={expectedOutput} />
            </div>
        </div>
    );
}

function AdvancedSettings(): JSX.Element {
    let [runtimeMultiplier, setRuntimeMultiplier] = useAdminStore((state) => [state.problemRuntimeMultiplier, state.setProblemRuntimeMultiplier]);

    const state = useAdminStore.getState();
    let testIndex = state.problemTests.length;
    let pushTest = state.pushProblemTest;

    return (
        <details className="open:bg-blue">
            <summary className="font-bold text-xl cursor-pointer select-none">Advanced</summary>
            <div className="grid grid-cols-[1fr_min-content] gap-2">
                <label>Runtime Multiplier</label>

                <div className="flex gap-2 align-end">
                    <output className="self-center">{
                        Intl.NumberFormat('en-US', {
                            minimumFractionDigits: 1
                        }).format(runtimeMultiplier)
                    }</output>
                    <input
                        type="range"
                        min="1"
                        max="5"
                        step="0.1"
                        defaultValue={runtimeMultiplier}
                        onInput={(e) => setRuntimeMultiplier(parseFloat((e.target as HTMLInputElement).value))}
                    />
                </div>

                {/*<label>Import tests</label>

                <input type="file" multiple onInput={async (e) => {
                    let input = e.target as HTMLInputElement;

                    for (let file of input.files!) {
                        pushTest({
                            id: 0,
                            index: testIndex++,
                            input: await file.text(),
                            expected_output: "",
                            max_runtime: 0,
                        })
                    }

                    input.value = "";
                }} />*/}
            </div>
        </details>
    );
}

function TestsEditorList(): JSX.Element {
    // only rerender based on the length
    const testCount = useAdminStore((state) => state.problemTests.length);
    const setError = useSession((state) => state.setError);
    const [pushTest, popTest, testFormat, setTests] = useAdminStore(
        (state) => [
            state.pushProblemTest,
            state.popProblemTest,
            state.problemTestFormat,
            state.setProblemTests
        ],
        shallow
    );
    const [loading, setLoading] = useState(false);
    const [queuePosition, setQueuePosition] = useState(0);

    const { data: user, error: _error } = useSWR<User>(
        api_url("/user/me"),
        fetcher, {
        shouldRetryOnError: false,
    });

    const populateTests = async () => {
        setLoading(true);
        const {
            problemReference: reference,
            problemTests: tests,
        } = useAdminStore.getState();

        try {
            const res = await fetch(api_url("/run/generate-tests"), {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify({
                    reference,
                    user_id: user!.id,
                    inputs: tests.map((test) => test.input),
                })
            });

            let job: JobStatus<Test[], RunnerError | ServerError> = await res.json();
            let [data, err] = await monitorJob(job, (n) => setQueuePosition(n));

            if (data) {
                setTests(data);
            }

            if (err) {
                if (isRunnerError(err)) {
                    setError(err.message, true);
                } else {
                    setError(err.error, true);
                };
            }
        } catch (e) {
            console.log(e);
            setError("Network error.", true);
        }
        finally {
            setLoading(false);
        }
    };

    console.log(testCount);

    return (
        <div className="flex flex-col gap-2 p-2 overflow-y-auto">
            {Array(testCount)
                .fill(0)
                .map((_, i) => (
                    <TestEditorWithFuel key={i} index={i} />
                ))}

            <div className="grid grid-cols-3 gap-1 mx-auto max-w-sm w-full">
                <button
                    className="py-2 rounded-l-full bg-blue-600 hover:bg-blue-500 text-blue-50 transition-colors text-sm whitespace-nowrap"
                    onClick={() => {
                        pushTest({
                            id: 0,
                            index: testCount,
                            input: structuredClone(testFormat),
                            expected_output: { Int: { Single: 0 } }
                        })
                    }}
                >
                    Add
                </button>
                <button
                    className="py-2 bg-red-600 hover:bg-red-500 text-red-50 transition-colors text-sm whitespace-nowrap"
                    onClick={popTest}
                >
                    Remove
                </button>
                <LoadingButton
                    className="px-4 py-2 rounded-r-full bg-neutral-600 hover:bg-neutral-500 text-neutral-50 transition-colors text-sm justify-center whitespace-nowrap"
                    onClick={populateTests}
                    loading={loading}
                >
                    Populate
                </LoadingButton>
            </div>

            {loading && <QueueStatus className="mx-auto" queuePosition={queuePosition} />}

            <AdvancedSettings />
        </div>
    );
}

export default function TestsEditor(): JSX.Element {
    const [reference, setReference] = useAdminStore(
        (state) => [state.problemReference, state.setProlbemReference],
        shallow
    );

    return (
        <div className="grid grid-rows-2 grid-cols-1 xl:grid-rows-1 xl:grid-cols-3">
            <div className="xl:col-span-2 border-b border-neutral-300 dark:border-neutral-700 xl:border-b-0 xl:border-r">
                <Editor
                    language="cpp"
                    value={reference}
                    onChange={(value, _event) => setReference(value)}
                />
            </div>
            <TestsEditorList />
        </div>
    );
}
