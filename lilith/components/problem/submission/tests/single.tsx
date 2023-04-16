export function SingleDisplay({ data, dataType }: { data: string, dataType: string }): JSX.Element {
    return (
        <div>
            <span className="text-sm">{dataType}:</span>
            <pre className="bg-blue-50 border-blue-200 rounded p-2 overflow-auto border dark:border-slate-700 dark:bg-slate-800">
                <code>
                    {data}
                </code>
            </pre>
        </div>
    );
}

export function SingleDiffDisplay({ dataType, output, expected }: { dataType: string, output: string, expected: string }): JSX.Element {
    return (
        <div>
            <span className="text-sm">{dataType}:</span>
            <pre className="bg-blue-50 border-blue-200 dark:border-slate-700 dark:bg-slate-800 rounded p-2 border overflow-auto">
                {Array.from(output).map((c, i) => expected[i] === c
                    ? <code key={i}>{c}</code>
                    : <code key={i} className="bg-red-300">{c}</code>
                )}
            </pre>
        </div>
    );
}
