export interface TestResult {
  success: boolean;
  input: string;
  output: string;
  expected_output: string;
  runtime: number;
}

export default function TestResultInfo({
  success,
  runtime,
  input,
  expected_output,
  output,
}: TestResult): JSX.Element {
  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-2">
        {success ? (
          <>
            <span className="text-green-600 text-2xl">Passed</span>
            <span className="text-green-600">
              {runtime / 1000000}ms
            </span>
          </>
        ) : (
          <>
            <span className="text-red-600 text-2xl">Failed</span>
            <span className="text-red-600">
              {runtime / 1000000}ms
            </span>
          </>
        )}
      </div>

      <label>Input</label>

      <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
        <code>{input}</code>
      </pre>

      <label>Expected</label>

      <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
        <code>{expected_output}</code>
      </pre>

      <label>Output</label>

      <pre className="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
        <code>{output}</code>
      </pre>
    </div>
  );
}
