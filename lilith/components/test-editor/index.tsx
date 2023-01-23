import produce from "immer";
import { useEffect, useState } from "react";
import { FunctionType } from "../problem/submission/tests";

// Any is the easiest way out of this :(
// Give me rust enums please
type GridEditorProps = {
  data: any[][],

  onChange: (grid: any[][]) => void,
};

function GridEditor({ data, onChange }: GridEditorProps) {
  let [grid, setGrid] = useState(data);

  useEffect(() => {
    onChange(grid);
  }, [grid, onChange]);

  // I hate that react makes us do this but what can you do
  const addRow = () => {
    setGrid([
      ...grid,
      Array(grid[0].length).fill("")
    ]);
  };

  const removeRow = () => {
    if (grid.length > 1) {
      let newGrid = [...grid];
      newGrid.pop();
      setGrid(newGrid);
    }
  };

  const addCol = () => {
    let newGrid = [...grid].map((row) => [...row, ""]);
    setGrid(newGrid);
  };

  const removeCol = () => {
    let newGrid = [...grid].map((row) => {
      let newRow = [...row];
      newRow.pop();
      return newRow;
    });

    setGrid(newGrid);
  };

  return (
    <div className="grid grid-cols-[minmax(0,1fr),40px] grid-rows-[minmax(0,1fr),40px]">
      <div className="grid overflow-auto border" style={{
        gridTemplateRows: `repeat(${grid.length}, 1fr)`,
        gridTemplateColumns: `repeat(${grid[0].length}, 1fr)`
      }}>
        {
          grid.map((row, y) =>
            row.map((element, x) =>
              <input
                key={y * row[0].length + x}
                className="border p-2 w-full"
                type="text"
                value={element.toString()}
                onChange={(e) => {
                  let newGrid = [...grid];

                  try {
                    newGrid[y][x] = JSON.parse(e.target.value);
                  } catch {
                    newGrid[y][x] = e.target.value as any;
                  }

                  setGrid(newGrid);
                }}
              />
            )
          )
        }
      </div>

      <div className="flex flex-col my-auto gap-4">
        <button
          onClick={addCol}
          className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full mx-auto w-6 py-2 px-1"
        >+</button>
        <button
          onClick={removeCol}
          className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full mx-auto w-6 py-2 px-1"
        >-</button>
      </div>

      <div className="flex mx-auto my-auto gap-4">
        <button
          onClick={addRow}
          className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
        >+</button>
        <button
          onClick={removeRow}
          className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
        >-</button>
      </div>
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
              <div className="border flex w-full">
                {row.map((element, x) =>
                  <input
                    key={x}
                    className="border p-2 w-full"
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
                  className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
                >+</button>
                <button
                  onClick={() => removeCol(y)}
                  className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
                >-</button>
              </div>
            </div>
          )
        }
      </div>

      <div className="flex mx-auto my-auto gap-4">
        <button
          onClick={addRow}
          className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
        >+</button>
        <button
          onClick={removeRow}
          className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
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
  let [list, setList] = useState(data);

  console.table(list);

  useEffect(() => {
    onChange(list);
  }, [list, onChange]);

  const addElement = () => {
    setList(produce(list, newList => {
      newList.push("");
    }));
  };

  const removeElement = () => {
    if (list.length > 1) {
      setList(produce(list, newList => {
        newList.pop();
      }));
    }
  };

  return (
    <div className="flex flex-col gap-2">
      <div>
        <div className="flex">
          <div className="border flex w-full">
            {list.map((element, i) =>
              <input
                key={i}
                className="border p-2 w-full"
                type="text"
                value={element.toString()}
                onChange={(e) => {
                  setList(produce(list, newList => {
                    try {
                      newList[i] = JSON.parse(e.target.value);
                    } catch {
                      newList[i] = e.target.value as any;
                    }
                  }));
                }}
              />
            )}
          </div>

          <div className="flex mx-auto my-auto gap-2 ml-2">
            <button
              onClick={() => addElement()}
              className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
            >+</button>
            <button
              onClick={() => removeElement()}
              className="bg-neutral-200 hover:bg-neutral-300 transition-colors rounded-full h-6 px-4"
            >-</button>
          </div>
        </div>
      </div>
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
      className="border-2 p-2 w-full mr-40"
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
  arg: FunctionType,
  onChange: (data: FunctionType) => void,
}

// edits a single field of a function argument
function FunctionArgumentEditor({ arg, onChange }: FunctionArgumentEditorProps) {
  let [[type, variant]] = Object.entries(arg);

  if ("Grid" in variant) {
    return (
      <GridEditor data={variant.Grid} onChange={(grid) => {
        // typescript: trust me bro
        onChange({ [type]: { Grid: grid } } as FunctionType);
      }} />
    );
  } else if ("Graph" in variant) {
    return (
      <GraphEditor data={variant.Graph} onChange={(graph) => {
        onChange({ [type]: { Graph: graph } } as FunctionType);
      }} />
    );
  } else if ("List" in variant) {
    return (
      <ListEditor data={variant.List} onChange={(list) => {
        onChange({ [type]: { List: list } } as FunctionType);
      }} />
    );
  } else if ("Single" in variant) {
    return <SingleEditor value={variant.Single} onChange={(single) => {
      onChange({ [type]: { Single: single } } as FunctionType);
    }} />
  }

  return <></>;
}

type TestEditorProps = {
  baseArgs: FunctionType[],

  onChange: (args: FunctionType[]) => void,
}

export function TestEditor({ baseArgs, onChange }: TestEditorProps) {
  let functionArguments = baseArgs;
  onChange(functionArguments);

  const updateArgument = (targetIdx: number) => (data: FunctionType) => {
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

