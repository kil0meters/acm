import { TestResult } from "./tests";

export default function TestResultInfo({
  success,
  runtime,
  input,
  expected_output,
  max_runtime,
  error,
  output,
}: TestResult): JSX.Element {
  let compact = Intl.NumberFormat('en', { notation: "compact" }).format(runtime) + " fuel";
  let long = Intl.NumberFormat('en', { notation: "standard" }).format(runtime) + " fuel";

  let compact_max = Intl.NumberFormat('en', { notation: "compact" }).format(max_runtime!) + " fuel";
  let long_max = Intl.NumberFormat('en', { notation: "standard" }).format(max_runtime!) + " fuel";

  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-2">
        {success ? (
          <>
            <span className="text-green-600 text-2xl">Passed</span>
            <span className="text-green-600" title={long}>
              {compact}
            </span>
          </>
        ) : (
          error ?
            <div>
              <div className="text-red-600 text-2xl">Error</div>
              <span className="text-red-600">{error}</span>
            </div>
            :
            <>
              <span className="text-red-600 text-2xl">Failed</span>
              <span className="text-red-600" title={long}>
                {compact}
              </span>
            </>
        )}
      </div>

      {!max_runtime || <div>
        <span>Max Fuel: </span><span title={long_max}>{compact_max}</span>
      </div>}

      <label>Input</label>

      <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
        <code>{input}</code>
      </pre>

      <label>Expected</label>

      <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
        <code>{expected_output}</code>
      </pre>

      {!error && <>
        <label>Output</label>

        <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
          <code>{output}</code>
        </pre>
      </>}
    </div >
  );
}
