import { ReactNode } from "react";

export function GridDisplay<T extends ReactNode>({ data }: { data: T[][] }): JSX.Element {
  return (
    <div className="overflow-hidden rounded border border-blue-200 w-min max-w-full">
      <div className="overflow-auto">
        <tbody
          className="border w-min bg-blue-50 border-blue-200 max-w-full max-h-[30vh] overflow-auto">
          {data.map(row =>
            <tr className="border-b border-blue-200">
              {row.map(element => <td className="text-center px-2 py-[0.125rem] border-l border-t border-blue-200 first-of-type:border-l-0 [tr:first-of-type>&]:border-t-0">{element}</td>)}
            </tr>
          )}
        </tbody>
      </div>
    </div>
  )
}

export function GridDiffDisplay<T extends ReactNode>({ output, expected }: { output: T[][], expected: T[][] }): JSX.Element {
  return (
    <div className="overflow-hidden rounded border border-blue-200 w-min max-w-full">
      <div className="overflow-auto">
        <tbody
          className="border w-min bg-blue-50 border-blue-200 max-w-full max-h-[30vh] overflow-auto">
          {output.map((row, y) =>
            <tr className="border-b border-blue-200">
              {row.map((element, x) =>
                expected[y] && expected[y][x] && expected[y][x] === element
                  ? <td className="text-center px-2 py-[0.125rem] border-l border-t border-blue-200 first-of-type:border-l-0 [tr:first-of-type>&]:border-t-0">{element}</td>
                  : <td className="text-center px-2 py-[0.125rem] border-l border-t first-of-type:border-l-0 [tr:first-of-type>&]:border-t-0 bg-red-300 text-red-900 border-red-400">{element}</td>
              )}
            </tr>
          )}
        </tbody>
      </div>
    </div>
  )
}
