import produce from "immer";
import { useEffect, useState } from "react";
import { FunctionValue } from "../problem/submission/tests";

// Any is the easiest way out of this :(
// Give me rust enums please
type GridEditorProps = {
    data: any[][],

    onChange: (grid: any[][]) => void,
};

function GridEditor({ data, onChange }: GridEditorProps) {
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

    }, [gridString]);

    return (
        <div>
            <textarea value={gridString} className={`p-2 outline w-full font-mono focus:outline-4 rounded-md ${isValid ? "outline-green-500" : "outline-red-500"}`} onChange={(e) => {
                setGridString(e.currentTarget.value);
            }} />
        </div>
    );
}

type GraphEditorProps = {
    data: any[][],

    onChange: (grid: any[][]) => void,
};

function GraphEditor({ data, onChange }: GraphEditorProps) {
    let [graph, setGraph] = useState(data);

    useEffect(() => {
        onChange(graph);
    }, [graph, onChange]);

    const addRow = () => {
        setGraph(produce(graph, newGraph => {
            newGraph.push(Array(graph[0].length).fill(""));
        }));
    };

    const removeRow = () => {
        if (graph.length > 1) {
            setGraph(produce(graph, newGraph => {
                newGraph.pop();
            }));
        }
    };

    const addCol = (row: number) => {
        setGraph(produce(graph, newGraph => {
            newGraph[row].push("");
        }));
    };

    const removeCol = (row: number) => {
        if (graph[row].length > 1) {
            setGraph(produce(graph, newGraph => {
                newGraph[row].pop();
            }));
        }
    };

    return (
        <div className="flex flex-col gap-2">
            <div>
                {
                    graph.map((row, y) =>
                        <div key={y} className="flex">
                            <span className="my-auto mr-2 font-mono">{y}:</span>
                            <div className="border flex w-full border-blue-200 dark:border-slate-700">
                                {row.map((element, x) =>
                                    <input
                                        key={x}
                                        className="border p-2 w-full bg-blue-50 dark:bg-slate-800 border-blue-200 dark:border-slate-700"
                                        type="text"
                                        value={element.toString()}
                                        onChange={(e) => {
                                            setGraph(produce(graph, newGraph => {
                                                try {
                                                    newGraph[y][x] = JSON.parse(e.target.value);
                                                } catch {
                                                    newGraph[y][x] = e.target.value as any;
                                                }
                                            }));
                                        }}
                                    />
                                )}
                            </div>

                            <div className="flex mx-auto my-auto gap-2 ml-2">
                                <button
                                    onClick={() => addCol(y)}
                                    className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4 dark:bg-neutral-700 dark:hover:bg-neutral-600"
                                >+</button>
                                <button
                                    onClick={() => removeCol(y)}
                                    className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4 dark:bg-neutral-700 dark:hover:bg-neutral-600"
                                >-</button>
                            </div>
                        </div>
                    )
                }
            </div>

            <div className="flex mx-auto my-auto gap-4">
                <button
                    onClick={addRow}
                    className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4 dark:bg-neutral-700 dark:hover:bg-neutral-600"
                >+</button>
                <button
                    onClick={removeRow}
                    className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4 dark:bg-neutral-700 dark:hover:bg-neutral-600"
                >-</button>
            </div>
        </div>
    );
}

type ListEditorProps = {
    data: any[],

    onChange: (list: any[]) => void,
};

function ListEditor({ data, onChange }: ListEditorProps) {
    const [list, setList] = useState(data);
    const [listString, setListString] = useState("[]");
    const [isValid, setIsValid] = useState(false);

    useEffect(() => {
        onChange(list);
    }, [list, onChange]);

    useEffect(() => {
        try {
            setList(JSON.parse(listString));
            setIsValid(true);
        }
        catch (_e) {
            setIsValid(false);
        }

    }, [listString]);

    return (
        <div>
            <input value={listString} className={`outline p-2 focus:outline-4 font-mono rounded-md ${isValid ? "outline-green-500" : "outline-red-500"}`} onChange={(e) => {
                setListString(e.currentTarget.value);
            }} />
        </div>
    );
}

type SingleEditorProps = {
    value: any,

    onChange: (grid: any) => void,
}

function SingleEditor({ value, onChange }: SingleEditorProps) {
    let [data, setData] = useState(value);

    useEffect(() => {
        onChange(data);
    }, [data, onChange]);

    return (
        <input
            className="outline focus:outline-4 p-2 w-full rounded-md outline-green-500"
            value={data.toString()}
            onChange={(e) => {
                try {
                    setData(JSON.parse(e.target.value));
                } catch {
                    setData(e.target.value);
                }
            }} />
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
            <GridEditor data={variant.Grid} onChange={(grid) => {
                // typescript: trust me bro
                onChange({ [type]: { Grid: grid } } as FunctionValue);
            }} />
        );
    } else if ("Graph" in variant) {
        return (
            <GraphEditor data={variant.Graph} onChange={(graph) => {
                onChange({ [type]: { Graph: graph } } as FunctionValue);
            }} />
        );
    } else if ("List" in variant) {
        return (
            <ListEditor data={variant.List} onChange={(list) => {
                onChange({ [type]: { List: list } } as FunctionValue);
            }} />
        );
    } else if ("Single" in variant) {
        return <SingleEditor value={variant.Single} onChange={(single) => {
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

