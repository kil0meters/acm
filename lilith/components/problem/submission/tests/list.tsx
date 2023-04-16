import { ReactNode } from "react";

export function ListDisplay<T extends ReactNode>({ dataType, data }: { dataType: string, data: T[] }): JSX.Element {
    return (
        <div>
            <span className="text-sm">{dataType} list:</span>
            <div
                className="rounded border w-min bg-blue-50 border-blue-200 max-w-full max-h-[30vh] overflow-auto flex dark:border-slate-700 dark:bg-slate-800">
                {data.length == 0
                    ? <span className="whitespace-nowrap px-4 py-2">[ EMPTY LIST ]</span>
                    : data.map((element, i) => <span key={i} className="text-center px-4 py-2 border-l first:border-0 border-blue-200 dark:border-slate-700">{element}</span>)
                }
            </div>
        </div>
    );
}

export function ListDiffDisplay<T extends ReactNode>({ dataType, output, expected }: { dataType: string, output: T[], expected: T[] }): JSX.Element {
    return (
        <div>
            <span className="text-sm">{dataType} list:</span>
            <div
                className="rounded border w-min bg-blue-50 border-blue-200 dark:border-slate-700 dark:bg-slate-800 max-w-full max-h-[30vh] overflow-auto flex">
                {output.length === 0
                    ? <span className="text-center px-4 py-2 border-l first:border-0 border-blue-200 bg-red-500 whitespace-nowrap">[ EMPTY LIST ]</span>
                    : output.map((element, i) => {
                        if (expected[i] === undefined || expected[i] !== element) {
                            return <span key={i} className="text-center px-4 py-2 border-l first:border-0 border-blue-200 bg-red-500">output[i]</span>;
                        } else {
                            return <span key={i} className="text-center px-4 py-2 border-l first:border-0 border-blue-200 dark:border-slate-700">{output[i]}</span>
                        }
                    })}
            </div>
        </div>
    )
}
