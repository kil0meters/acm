export function SingleDisplay({ data }: { data: string }): JSX.Element {
  return (
    <pre className="bg-blue-50 border-blue-200 rounded p-2 overflow-auto border">
      <code>
        {data}
      </code>
    </pre>
  );
}

export function SingleDiffDisplay({ output, expected }: { output: string, expected: string }): JSX.Element {
  return (
    <pre className="bg-blue-50 border-blue-200 rounded p-2 border overflow-auto">
      {Array.from(output).map((c, i) => expected[i] === c
        ? <code>{c}</code>
        : <code className="bg-red-300">{c}</code>
      )}
    </pre>
  );
}
