import dynamic from "next/dynamic";
const GraphDisplay = dynamic(import("./graph"), { ssr: false });
import { GridDiffDisplay, GridDisplay } from "./grid";
import { ListDiffDisplay, ListDisplay } from "./list";
import { SingleDiffDisplay, SingleDisplay } from "./single";

export const SUBMISSION_TESTS_QUERY = "submissions-tests";

export interface Test {
  id: number;
  index: number;
  input: WasmFunctionArgs;
  expected_output: FunctionType;
  max_fuel?: number;
  error?: string;
}

export type WasmFunctionArgs = {
  name: string;
  arguments: FunctionType[]
}

export type FunctionType =
  { String: ContainerVariant<string> } |
  { Char: ContainerVariant<string> } |
  { Int: ContainerVariant<number> } |
  { Long: ContainerVariant<number> } |
  { Float: ContainerVariant<number> } |
  { Double: ContainerVariant<number> } |
  { Bool: ContainerVariant<boolean> }

export type ContainerVariant<T> =
  { Grid: T[][] } |
  { Graph: T[][] } |
  { List: T[] } |
  { Single: T }

export interface TestResult extends Test {
  output: FunctionType;
  success: boolean;
  fuel: number;
}

export function FunctionTypeDisplay({ data }: { data: FunctionType }): JSX.Element {
  // KILL JAVASCRIPT I SWEAR TO GOD
  let [[_type, variant]] = Object.entries(data);

  if ("Grid" in variant) {
    return <GridDisplay data={variant.Grid} />;
  } else if ("Graph" in variant) {
    return <GraphDisplay data={variant.Graph} />;
  } else if ("List" in variant) {
    return <ListDisplay data={variant.List} />;
  } else if ("Single" in variant) {
    return <SingleDisplay data={variant.Single.toString()} />;
  }

  return <></>;
}

export function FunctionTypeDiffDisplay({ output, expected }: { output: FunctionType, expected: FunctionType }): JSX.Element {
  let [[_type, outputVariant]] = Object.entries(output);
  let [[__type, expectedVariant]] = Object.entries(expected);

  if ("Grid" in outputVariant && "Grid" in expectedVariant) {
    return <GridDiffDisplay output={outputVariant.Grid} expected={expectedVariant.Grid} />;
  }
  else if ("Graph" in outputVariant && "Graph" in expectedVariant) {
    return <div>NOT IMPLEMENTED</div>
  }
  else if ("List" in outputVariant && "List" in expectedVariant) {
    return <ListDiffDisplay output={outputVariant.List} expected={expectedVariant.List} />;
  }
  else if ("Single" in outputVariant && "Single" in expectedVariant) {
    return <SingleDiffDisplay output={outputVariant.Single.toString()} expected={expectedVariant.Single.toString()} />;
  }

  return <></>;
}

