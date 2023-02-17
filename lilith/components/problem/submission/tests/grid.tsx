import { ReactNode } from "react";

export function GridDisplay<T extends ReactNode>({ dataType, data }: { dataType: string, data: T[][] }): JSX.Element {
    return (
        <div>
            <span className="border-neutral-700 text-sm">{dataType} grid:</span>
            <div className="overflow-hidden rounded border border-blue-200 dark:border-slate-700 w-min max-w-full">
                <div className="overflow-auto">
                    <tbody
                        className="border w-min bg-blue-50 dark:bg-slate-800 dark:border-slate-700 border-blue-200 max-w-full max-h-[30vh] overflow-auto">
                        {data.map((row, y) =>
                            <tr key={y} className="border-b border-blue-200">
                                {row.map((element, x) =>
                                    <td key={x} className="text-center px-2 py-[0.125rem] border-l border-t border-blue-200 dark:border-slate-700 first-of-type:border-l-0 [tr:first-of-type>&]:border-t-0">{element}</td>
                                )}
                            </tr>
                        )}
                    </tbody>
                </div>
            </div>
        </div>
    )
}

export function GridDiffDisplay<T extends ReactNode>({ dataType, output, expected }: { dataType: string, output: T[][], expected: T[][] }): JSX.Element {
    return (

        <div>
            <span className="border-neutral-700 text-sm">{dataType} grid:</span>
            <div className="overflow-hidden rounded border border-blue-200 dark:border-slate-700 w-min max-w-full">
                <div className="overflow-auto">
                    <tbody
                        className="border w-min bg-blue-50 dark:bg-slate-800 dark:border-slate-700 border-blue-200 max-w-full max-h-[30vh] overflow-auto">
                        {output.map((row, y) =>
                            <tr key={y} className="border-b border-blue-200">
                                {row.map((element, x) =>
                                    expected[y] && expected[y][x] && expected[y][x] === element
                                        ? <td key={x} className="text-center px-2 py-[0.125rem] border-l border-t border-blue-200 dark:border-slate-700 first-of-type:border-l-0 [tr:first-of-type>&]:border-t-0">{element}</td>
                                        : <td key={x} className="text-center px-2 py-[0.125rem] border-l border-t first-of-type:border-l-0 [tr:first-of-type>&]:border-t-0 bg-red-300 text-red-900 border-red-400">{element}</td>
                                )}
                            </tr>
                        )}
                    </tbody>
                </div>
            </div>
        </div>
    )
}
