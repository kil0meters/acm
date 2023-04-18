import { useEffect, useRef, useState } from "react";
import renderLatex from "../../../utils/latex";
import { AsymptoticComplexity, Submission, useSession } from "../../../utils/state";

export function ShareButton({
    className,
    path,
}: {
    className: string;
    path: string;
}): JSX.Element {
    const [buttonText, setButtonText] = useState("Share");

    const onClick = async () => {
        const url = `${location.protocol}//${location.host}${path}`;
        const contentType = "text/plain";

        navigator.clipboard.write([
            new ClipboardItem({
                [contentType]: new Blob([url], { type: contentType }),
            }),
        ]);

        setButtonText("Copied!");
        setTimeout(() => {
            setButtonText("Share");
        }, 1000);
    };

    return (
        <button onClick={onClick} className={className}>
            {buttonText}
        </button>
    );
}

function AsymptoticComplexityDisplay({ complexity }: { complexity: AsymptoticComplexity }) {
    const element = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (element.current) {
            renderLatex(element.current);
        }
    });

    let content: string;

    if (complexity == "LOG") {
        content = "$\\mathrm{O}(\\log n)$";
    } else if (complexity == "SQRT") {
        content = "$\\mathrm{O}(\\sqrt{n})$";
    } else if (complexity == "LINEAR") {
        content = "$\\mathrm{O}(n)$";
    } else if (complexity == "CONSTANT") {
        content = "$\\mathrm{O}(1)$";
    } else if (complexity == "QUADRATIC") {
        content = "$\\mathrm{O}(n^2)$";
    } else if (complexity == "LOG_LINEAR") {
        content = "$\\mathrm{O}(n \\log n)$";
    } else if (complexity == "EXPONENTIAL") {
        content = "$\\mathrm{O}(2^n)$";
    } else {
        content = "";
    }

    return <span ref={element}>{content}</span>;
}

function CloseButton({ className }: { className?: string }) {
    const setSubmissionShown = useSession((session) => session.setSubmissionShown);

    return <button
        className={`rounded-full aspect-square p-4 ${className}`}
        onClick={() => setSubmissionShown(false)
        }>
        ⨯
    </button >
}

type Diagnostic = {
    line: number;
    diagnostic_type: "Error" | "Warning" | "Note";
    col: number;
    message: string;
};

function DiagnosticDisplay(diagnostic: Diagnostic) {
    let diagnostic_color: string;
    let diagnostic_text: string;

    if (diagnostic.diagnostic_type == "Error") {
        diagnostic_text = "ERROR";
        diagnostic_color = "text-red-600";
    } else if (diagnostic.diagnostic_type == "Warning") {
        diagnostic_text = "WARN";
        diagnostic_color = "text-yellow-500";
    } else {
        diagnostic_text = "NOTE";
        diagnostic_color = "text-cyan-500";
    }

    return <>
        <div className="bg-neutral-50 dark:bg-neutral-900 text-neutral-900 dark:text-neutral-50 p-1 border-r border-b last-of-type:border-b-0 border-neutral-300 dark:border-neutral-700 flex">
            <span className={`ml-auto font-bold font-mono ${diagnostic_color}`}>
                {diagnostic_text}
            </span>
            &nbsp;
            <span className="font-mono">
                {diagnostic.line}:{diagnostic.col}
            </span>
        </div>
        <code className="break-all bg-white dark:bg-black text-neutral-900 dark:text-neutral-50 p-1 border-b last-of-type:border-b-0 border-neutral-300 dark:border-neutral-700">{diagnostic.message}</code>
    </>;
}

export function DiagnosticsDisplay({ error }: { error: string }) {
    try {
        let diagnostics = JSON.parse(error) as Diagnostic[];

        return (
            <div className="grid grid-cols-min-full bg-red-600 border-red-700 border-t dark:bg-red-800 overflow-x-auto max-h-64">
                {diagnostics.map((diagnostic, i) => <DiagnosticDisplay key={i} {...diagnostic} />)}
            </div>
        );
    } catch (e) {
        return (
            <pre className="bg-red-700 dark:bg-red-800 overflow-x-auto p-2 max-h-64">
                <code>{error}</code>
            </pre>
        );
    }
}

function SubmissionFeedbackError({ id, error, inProblemView }: { id: number, error: string, inProblemView: boolean }) {
    const buttonClass = "bg-red-700 dark:bg-red-800 hover:bg-red-600 dark:hover:bg-red-900 text-red-50 rounded-full px-4 py-2 text-sm transition-colors";

    return (
        <div className="bg-red-500 dark:bg-red-700 text-red-50 flex flex-col h-full">
            <div className="flex items-start p-4">
                <h1 className="text-2xl font-bold my-auto">Error</h1>

                {inProblemView &&
                    <div className="ml-auto flex gap-2">
                        <ShareButton
                            path={`/submissions/${id}`}
                            className={buttonClass}
                        />
                        <CloseButton className={buttonClass} />
                    </div>
                }
            </div>

            <DiagnosticsDisplay error={error} />
        </div>
    );
}

export default function SubmissionFeedback({
    inProblemView,
    id,
    error,
    success,
    runtime,
    complexity
}: Submission & { inProblemView: boolean }): JSX.Element {
    if (error) {
        return <SubmissionFeedbackError id={id} inProblemView={inProblemView} error={error} />
    }

    let fuel = <div>
        <span>Consumed </span>
        <span title={Intl.NumberFormat('en', { notation: "standard" }).format(runtime) + " fuel"}>
            {Intl.NumberFormat('en', { notation: "compact" }).format(runtime)} fuel.
        </span>
    </div>

    if (success) {
        const buttonClass = "bg-green-700 hover:bg-green-600 text-green-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors";

        return (
            <div className="flex-col flex p-4 bg-green-500 dark:bg-green-800 text-green-50 h-full gap-2">
                <div className="flex items-start">
                    <span className="font-bold text-2xl my-auto">Congratulations!</span>

                    {inProblemView &&
                        <div className="ml-auto flex gap-2">
                            <ShareButton
                                path={`/submissions/${id}`}
                                className={buttonClass}
                            />
                            <CloseButton className={buttonClass} />
                        </div>
                    }
                </div>
                <span>The code passed all of the supplied tests.</span>
                {complexity && <span>
                    Estimated Time Complexity: <AsymptoticComplexityDisplay complexity={complexity} />
                </span>}
                <span>{fuel}</span>
            </div>
        );
    } else {
        const buttonClass = "bg-neutral-600 hover:bg-neutral-500 text-neutral-50 rounded-full px-4 py-2 text-sm transition-colors";

        return (
            <div className="flex-col flex p-4 bg-white dark:bg-black h-full" >
                <div className="flex items-start">
                    <span className="text-red-500 font-bold text-2xl my-auto">Failed</span>
                    {inProblemView &&
                        <div className="ml-auto flex gap-2">
                            <ShareButton

                                path={`/submissions/${id}`}
                                className={buttonClass}
                            />
                            <CloseButton className={buttonClass} />
                        </div>
                    }
                </div>

                <span>The code did not pass all of the tests.</span>
                {fuel}

            </div>
        );
    }
}
