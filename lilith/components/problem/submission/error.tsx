export interface RunnerError {
  type: string;
  message: string;
  line?: number;
}

export function isError(
  result: unknown | RunnerError
): result is RunnerError {
  return (result as RunnerError).type !== undefined;
}

export default function ErrorDisplay({ message }: RunnerError): JSX.Element {
  return (
    <div className="bg-red-500 text-red-50 p-4 flex flex-col gap-2 rounded-md border-red-600 dark:border-red-500 dark:bg-red-700 border">
      <h1 className="text-2xl font-bold">{"error."}</h1>

      <pre className="bg-red-700 dark:bg-red-800 overflow-auto p-2 rounded">
        <code>{message}</code>
      </pre>
    </div>
  );
}
