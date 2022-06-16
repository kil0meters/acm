import dynamic from "next/dynamic";
import { useState } from "react";
import shallow from "zustand/shallow";
import { api_url } from "../../utils/fetcher";
import { useAdminStore, useSession, useStore } from "../../utils/state";
import LoadingButton from "../loading-button";
import { isError, RunnerError } from "../problem/submission/error";
import { Test } from "../problem/submission/tests";
const Editor = dynamic(import("../../components/editor"), { ssr: false });

function TestEditor({ index }: { index: number }): JSX.Element {
  const input = useAdminStore((state) => state.problemTests[index]?.input);
  const expectedOutput = useAdminStore(
    (state) => state.problemTests[index]?.expected_output
  );
  const updateTest = useAdminStore((state) => state.updateProblemTest);

  return (
    <div className="grid grid-cols-2 gap-2">
      <div className="flex flex-col gap-2">
        <span>Input</span>
        <textarea
          className="h-32 resize-none border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
          onChange={(event) =>
            updateTest(index, { input: event.currentTarget.value })
          }
          value={input}
        />
      </div>

      <div className="flex flex-col gap-2">
        <span>Expected Output</span>
        <textarea
          className="h-32 resize-none border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
          onChange={(event) =>
            updateTest(index, { expected_output: event.currentTarget.value })
          }
          value={expectedOutput}
        />
      </div>
    </div>
  );
}

function TestsEditorList(): JSX.Element {
  // only rerender based on the length
  const testCount = useAdminStore((state) => state.problemTests.length);
  const [ token, username ] = useStore((state) => [state.token, state.user!.username], shallow);
  const setError = useSession((state) => state.setError);
  const [pushTest, popTest, updateTest, setTests] = useAdminStore(
    (state) => [state.pushProblemTest, state.popProblemTest, state.updateProblemTest, state.setProblemTests],
    shallow
  );
  const [loading, setLoading] = useState(false);

  const populateTests = async () => {
    setLoading(true);
    const { problemRunner: runner, problemReference: reference, problemTests: tests } = useAdminStore.getState();

    try {
      const res: Test[] | RunnerError = await (await fetch(api_url("/generate-tests"), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`,
        },
        body: JSON.stringify({
          runner,
          reference,
          username,
          inputs: tests.map((test) => test.input),
        })
      })).json();

      if (isError(res)) {
        setError(res.message, true);
      } else {
        setTests(res);
      }

      setLoading(false);
    } catch (e) {
      // TODO: Handle error
      console.log(e);
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col gap-2 p-2 overflow-y-auto">
      {Array(testCount)
        .fill(0)
        .map((_, i) => (
          <TestEditor key={i} index={i} />
        ))}

      <div className="grid grid-cols-3 gap-1 mx-auto max-w-sm w-full">
        <button
          className="py-2 rounded-l-full bg-blue-600 hover:bg-blue-500 text-blue-50 transition-colors text-sm whitespace-nowrap"
          onClick={pushTest}
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
