export interface RunnerError {
    type: string;
    message: string;
    line?: number;
}

export function isRunnerError(
    result: unknown | RunnerError
): result is RunnerError {
    return (result as RunnerError).type !== undefined;
}

export interface ServerError {
    error: string
}

export function isServerError(
    result: unknown | ServerError
): result is ServerError {
    return (result as ServerError).error !== undefined;
}
