import { ReactNode, useState } from "react";
import useSWR from "swr";
import { FunctionTypeDiffDisplay, FunctionTypeDisplay, TestResult } from ".";
import { api_url, fetcher } from "../../../../utils/fetcher";
import ErrorBox from "../../../error-box";
import Modal from "../../../modal";
import { GridDiffDisplay, GridDisplay } from "./grid";

export type TestResultDescription = {
  id: number;
  index: number;
  success: boolean;
  hidden: boolean;
  problemId: number;
}

export function TestResultEntry({ id, index, success, hidden, problemId }: TestResultDescription): JSX.Element {
  const [shown, setShown] = useState(false);

  const baseStyles =
    "aspect-square border b rounded transition-shadow hover:shadow-md hover:ring-2";

  const flexStyles = success
    ? "bg-green-200 dark:bg-green-800 dark:border-green-600 dark:ring-green-600 dark:text-green-100 border-green-400 ring-green-400 text-green-900"
    : "bg-red-200 dark:bg-red-800 dark:border-red-600 dark:ring-red-600 dark:text-red-100 border-red-400 ring-red-400 text-red-900";

  return (
    <>
      <button
        className={`${baseStyles} ${flexStyles}`}
        onClick={() => setShown(true)}
      >
        <div className="flex flex-col">
          <span>Test #{index}</span>

          {hidden ? <span className="text-sm">HIDDEN</span> : <></>}
        </div>
      </button>

      <Modal shown={shown} onClose={() => setShown(false)}>
        <TestResultView id={id} index={index} success={success} problemId={problemId} hidden={hidden} />
      </Modal>
    </>
  );
}

function TestResultView({ index, success, problemId }: TestResultDescription): JSX.Element {
  const { data, error } = useSWR<TestResult>(api_url(`/problems/${problemId}/recent-tests/${index}`), fetcher);

  let containerClasses = success ? "outline-green-200" : "outline-red-300";
  let textColor = success ? "text-green-700" : "text-red-900";

  if (error) return (
    <ErrorBox>
      There was a problem fetching this test. (Perhaps it's from an old version of the website?)
    </ErrorBox>
  );

  if (!data) return (
    <div className={`rounded-md outline outline-4 p-4 flex flex-col gap-4 bg-white ${containerClasses}`}>
      <div className="flex">
        <h2 className={`text-2xl font-bold mb-1`}>
          Test #{index}
        </h2>

        <h3 className={`text-xl ml-auto ${textColor}`}>
          {success ? "Passed" : "Failed"}
        </h3>
      </div>

      HIDDEN TEST
    </div>
  );

  return <TestResultInner {...data} />;
}

export function TestResultInner({ index, input, output, expected_output, error, fuel, max_fuel, success }: TestResult) {

  let compact = Intl.NumberFormat('en', { notation: "compact" }).format(fuel) + " fuel";
  let long = Intl.NumberFormat('en', { notation: "standard" }).format(fuel) + " fuel";

  let compact_max = Intl.NumberFormat('en', { notation: "compact" }).format(max_fuel!) + " fuel";
  let long_max = Intl.NumberFormat('en', { notation: "standard" }).format(max_fuel!) + " fuel";

  let containerClasses = success ? "outline-green-200" : "outline-red-300";
  let textColor = success ? "text-green-700" : "text-red-900";

  if (error) {
    return (
      <div className={`rounded-md outline-red-700 outline outline-4 p-4 flex flex-col gap-4 bg-red-600 box-border`}>
        <h2 className="text-red-900 font-extrabold text-2xl">Error</h2>

        <pre className="bg-red-900 text-red-100 border-red-700 border rounded-md overflow-auto p-2">
          <code>{error}</code>
        </pre>
      </div>
    );
  }

  return (
    <div className={`rounded-md outline outline-4 p-4 flex flex-col gap-4 bg-white ${containerClasses}`}>
      <div>
        <div className="flex">
          <h2 className={`text-2xl font-bold mb-1`}>
            Test #{index}
          </h2>

          <h3 className={`text-xl ml-auto ${textColor}`}>
            {success ? "Passed" : "Failed"}
          </h3>
        </div>

        {!max_fuel || <div>
          <span>Max Fuel: </span><span title={long_max}>{compact_max}</span>
        </div>}

        <div>
          <span>Consumed </span><span title={long}>{compact}.</span>
        </div>
      </div>

      <div>
        <label>Input</label>

        <div className="flex flex-col gap-4">
          {input.arguments.map(arg =>
            <FunctionTypeDisplay data={arg} />
          )}
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label>Expected</label>

          <FunctionTypeDisplay data={expected_output} />
        </div>
        <div>
          <label>Output</label>

          <FunctionTypeDiffDisplay output={output} expected={expected_output} />
        </div>
      </div>
    </div>
  );

}
