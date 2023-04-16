import produce from "immer";
import { useEffect, useState } from "react";
import { FunctionValue } from "../problem/submission/tests";

// Any is the easiest way out of this :(
// Give me rust enums please
type GridEditorProps = {
    data: any[][],
    type: string,

    onChange: (grid: any[][]) => void,
};

function GridEditor({ type, data, onChange }: GridEditorProps) {
    const [gridString, setGridString] = useState(JSON.stringify(data));
    const [isValid, setIsValid] = useState(false);

    useEffect(() => {
        try {
            let grid = JSON.parse(gridString);
            setIsValid(true);
            onChange(grid);
        }
        catch (_e) {
            setIsValid(false);
        }

    }, [gridString, onChange]);

    return (
        <div>
            <span className="text-sm">{type} Grid:</span>
            <textarea value={gridString} className={`p-2 outline w-full font-mono focus:outline-4 rounded-md dark:bg-neutral-900 ${isValid ? "outline-green-500" : "outline-red-500"}`} onChange={(e) => {
                setGridString(e.currentTarget.value);
            }} />
        </div>
    );
}

type GraphEditorProps = {
    data: any[][],
    type: string,

    onChange: (grid: any[][]) => void,
};

function GraphEditor({ type, data, onChange }: GraphEditorProps) {
    const [graphString, setGraphString] = useState(JSON.stringify(data));
    const [isValid, setIsValid] = useState(false);

    useEffect(() => {
        try {
            let graph = JSON.parse(graphString);
            setIsValid(true);
            onChange(graph);
        }
        catch (_e) {
            setIsValid(false);
        }

    }, [graphString, onChange]);

    return (
        <div>
            <span className="text-sm">{type} Graph (adjacency list):</span>
            <textarea value={graphString} className={`p-2 outline w-full font-mono focus:outline-4 rounded-md dark:bg-neutral-900 ${isValid ? "outline-green-500" : "outline-red-500"}`} onChange={(e) => {
                setGraphString(e.currentTarget.value);
            }} />
        </div>
    );
}

type ListEditorProps = {
    data: any[],
    type: string,

    onChange: (list: any[]) => void,
};

function ListEditor({ type, data, onChange }: ListEditorProps) {
    const [listString, setListString] = useState(JSON.stringify(data));
    const [isValid, setIsValid] = useState(false);

    useEffect(() => {
        try {
            let list = JSON.parse(listString);
            setIsValid(true);
            onChange(list);
        }
        catch (_e) {
            setIsValid(false);
        }

    }, [listString, onChange]);

    return (
        <div>
            <span className="text-sm">{type} List:</span>
            <input value={listString} className={`w-full outline p-2 focus:outline-4 font-mono rounded-md dark:bg-neutral-900  ${isValid ? "outline-green-500" : "outline-red-500"}`} onChange={(e) => {
                setListString(e.currentTarget.value);
            }} />
        </div>
    );
}

type SingleEditorProps = {
    value: any,
    type: string,

    onChange: (grid: any) => void,
}

function SingleEditor({ type, value, onChange }: SingleEditorProps) {
    let [data, setData] = useState(value);

    useEffect(() => {
        onChange(data);
    }, [data, onChange]);

    return (
        <div>
            <span className="text-sm">{type}:</span>
            <input
                className="outline focus:outline-4 p-2 w-full rounded-md outline-green-500 dark:bg-neutral-900"
                value={data.toString()}
                onChange={(e) => {
                    try {
                        setData(JSON.parse(e.target.value));
                    } catch {
                        setData(e.target.value);
                    }
                }} />
        </div>
    );
}

type FunctionArgumentEditorProps = {
    arg: FunctionValue,
    onChange: (data: FunctionValue) => void,
}

// edits a single field of a function argument
function FunctionArgumentEditor({ arg, onChange }: FunctionArgumentEditorProps) {
    let [[type, variant]] = Object.entries(arg);

    if ("Grid" in variant) {
        return (
            <GridEditor type={type} data={variant.Grid} onChange={(grid) => {
                // typescript: trust me bro
                onChange({ [type]: { Grid: grid } } as FunctionValue);
            }} />
        );
    } else if ("Graph" in variant) {
        return (
            <GraphEditor type={type} data={variant.Graph} onChange={(graph) => {
                onChange({ [type]: { Graph: graph } } as FunctionValue);
            }} />
        );
    } else if ("List" in variant) {
        return (
            <ListEditor type={type} data={variant.List} onChange={(list) => {
                onChange({ [type]: { List: list } } as FunctionValue);
            }} />
        );
    } else if ("Single" in variant) {
        return <SingleEditor type={type} value={variant.Single} onChange={(single) => {
            onChange({ [type]: { Single: single } } as FunctionValue);
        }} />
    }

    return <></>;
}

type TestEditorProps = {
    baseArgs: FunctionValue[],

    onChange: (args: FunctionValue[]) => void,
}

export function TestEditor({ baseArgs, onChange }: TestEditorProps) {
    let functionArguments = baseArgs;

    const updateArgument = (targetIdx: number) => (data: FunctionValue) => {
        let nextArgs = functionArguments.map((c, i) => {
            if (i === targetIdx) {
                return data;
            } else {
                return c;
            }
        });

        functionArguments = nextArgs;

        onChange(functionArguments);
    };

    return (
        <div className="flex flex-col gap-2">
            {functionArguments.map((argument, i) => <FunctionArgumentEditor key={i} arg={argument} onChange={updateArgument(i)} />)}
        </div>
    );
}

