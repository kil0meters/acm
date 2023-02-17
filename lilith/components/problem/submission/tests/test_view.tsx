import { useState } from "react";
import useSWR from "swr";
import { api_url, fetcher } from "../../../../utils/fetcher";
import Modal from "../../../modal";
import { FunctionValue, FunctionTypeDisplay, Test } from ".";
import { GridDisplay } from "./grid";
import ErrorBox from "../../../error-box";

export type TestDescription = {
    id: number;
    index: number;
    problemId: number;
    hidden: boolean;
}

export function TestEntry({ id, index, hidden, problemId }: TestDescription): JSX.Element {
    const [shown, setShown] = useState(false);

    return (
        <>
            <button
                onClick={() => setShown(true)}
                className="aspect-square bg-neutral-200 dark:bg-slate-700 border border-neutral-400 dark:border-slate-800 b rounded transition-shadow hover:shadow-md hover:ring-2 ring-neutral-400 dark:ring-slate-800"
            >
                <div className="flex flex-col">
                    <span>Test #{index}</span>

                    {hidden ? <span className="text-sm">HIDDEN</span> : <></>}
                </div>
            </button>

            <Modal shown={shown} onClose={() => setShown(false)}>
                <TestView id={id} index={index} hidden={hidden} problemId={problemId} />
            </Modal>
        </>
    );
}

function TestView({ index, problemId }: TestDescription): JSX.Element {
    const { data, error } = useSWR<Test>(api_url(`/problems/${problemId}/tests/${index}`), fetcher);

    if (error) return (
        <ErrorBox>
            There was a problem fetching this test.
        </ErrorBox>
    );

    if (!data) return (
        <div className="shadow-lg bg-white rounded-md border border-neutral-300 dark:bg-black dark:border-neutral-700 p-4 flex flex-col gap-2">
            <h2 className="text-2xl">
                Test #{index}
            </h2>

            HIDDEN TEST
        </div>
    );

    let compact = Intl.NumberFormat('en', { notation: "compact" }).format(data.max_fuel!);
    let long = Intl.NumberFormat('en', { notation: "standard" }).format(data.max_fuel!);

    return (
        <div className="shadow-lg bg-white rounded-md border border-neutral-300 dark:bg-black dark:border-neutral-700 p-4 flex flex-col gap-2">
            <h2 className="text-2xl">
                Test #{index}
            </h2>

            {!data.max_fuel || <div>
                <span>Max Fuel: </span><span title={long}>{compact}</span>
            </div>}

            <label>Input</label>

            <div className="flex flex-col gap-4">
                {data.input.arguments.map((arg, i) =>
                    <FunctionTypeDisplay key={i} data={arg} />
                )}
            </div>

            <label>Expected</label>

            <FunctionTypeDisplay data={data.expected_output} />
        </div>
    );
}
