import dynamic from "next/dynamic";
const GraphDisplay = dynamic(import("./graph"), { ssr: false });
import { GridDiffDisplay, GridDisplay } from "./grid";
import { ListDiffDisplay, ListDisplay } from "./list";
import { SingleDiffDisplay, SingleDisplay } from "./single";

export const SUBMISSION_TESTS_QUERY = "submissions-tests";

export interface Test {
    id: number;
    index: number;
    input: WasmFunctionCall;
    expected_output: FunctionValue;
    max_fuel?: number;
    error?: string;
}

export type WasmFunctionCall = {
    name: string;
    arguments: FunctionValue[];
    return_type: FunctionType;
}

export type FunctionValue =
    { String: ContainerVariant<string> } |
    { Char: ContainerVariant<string> } |
    { Int: ContainerVariant<number> } |
    { Long: ContainerVariant<number> } |
    { Float: ContainerVariant<number> } |
    { Double: ContainerVariant<number> } |
    { Bool: ContainerVariant<boolean> }

export type FunctionType =
    { String: ContainerVariantType } |
    { Char: ContainerVariantType } |
    { Int: ContainerVariantType } |
    { Long: ContainerVariantType } |
    { Float: ContainerVariantType } |
    { Double: ContainerVariantType } |
    { Bool: ContainerVariantType }

export type ContainerVariant<T> =
    { Grid: T[][] } |
    { Graph: T[][] } |
    { List: T[] } |
    { Single: T }

export type ContainerVariantType = "Grid" | "Graph" | "List" | "Single";

export interface TestResult extends Test {
    output: FunctionValue;
    success: boolean;
    fuel: number;
}

export function FunctionTypeDisplay({ data }: { data: FunctionValue }): JSX.Element {
    // KILL JAVASCRIPT I SWEAR TO GOD
    let [[type, variant]] = Object.entries(data);

    if ("Grid" in variant) {
        return <GridDisplay dataType={type} data={variant.Grid} />;
    } else if ("Graph" in variant) {
        return <GraphDisplay dataType={type} data={variant.Graph} />;
    } else if ("List" in variant) {
        return <ListDisplay dataType={type} data={variant.List} />;
    } else if ("Single" in variant) {
        return <SingleDisplay dataType={type} data={variant.Single.toString()} />;
    }

    return <></>;
}

export function FunctionTypeDiffDisplay({ output, expected }: { output: FunctionValue, expected: FunctionValue }): JSX.Element {
    let [[type, outputVariant]] = Object.entries(output);
    let [[_type, expectedVariant]] = Object.entries(expected);

    if ("Grid" in outputVariant && "Grid" in expectedVariant) {
        return <GridDiffDisplay dataType={type} output={outputVariant.Grid} expected={expectedVariant.Grid} />;
    }
    else if ("Graph" in outputVariant && "Graph" in expectedVariant) {
        // currently we just display the output graph normally
        return <GraphDisplay dataType={type} data={outputVariant.Graph} />
    }
    else if ("List" in outputVariant && "List" in expectedVariant) {
        return <ListDiffDisplay dataType={type} output={outputVariant.List} expected={expectedVariant.List} />;
    }
    else if ("Single" in outputVariant && "Single" in expectedVariant) {
        return <SingleDiffDisplay dataType={type} output={outputVariant.Single.toString()} expected={expectedVariant.Single.toString()} />;
    }

    return <></>;
}

