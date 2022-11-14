import { useState } from "react";
import { Submission, useSession } from "../../../utils/state";

export default function SubmissionFeedback({
  inProblemView,
  id,
  error,
  success,
  runtime,
}: Submission & { inProblemView: boolean }): JSX.Element {
  function CloseButton({
    className
  }: {
    className: string;
  }) {
    const setSubmissionShown = useSession((session) => session.setSubmissionShown);

    return (
      <button className={className} onClick={() => setSubmissionShown(true)}>
        ⨯
      </button>
    )
  }

  function ShareButton({
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

  if (error) {
    const buttonClass = "bg-red-700 dark:bg-red-800 hover:bg-red-600 dark:hover:bg-red-900 text-red-50 rounded-full px-4 py-2 text-sm transition-colors";

    return (
      <div className="bg-red-500 dark:bg-red-700 text-red-50 p-4 flex flex-col gap-2 h-full">
        <div className="flex items-start">
          <h1 className="text-2xl font-bold">error.</h1>

          {inProblemView &&
            <div className="ml-auto">
              <ShareButton
                path={`/submissions/${id}`}
                className={buttonClass}
              />
              <CloseButton className={`${buttonClass} ml-2`} />
            </div>
          }
        </div>

        <pre className="bg-red-700 dark:bg-red-800 overflow-x-auto p-2 rounded max-h-64">
          <code>{error}</code>
        </pre>
      </div>
    );
  }

  let fuel = <div>
    <span>Consumed </span>
    <span title={Intl.NumberFormat('en', { notation: "standard" }).format(runtime) + " fuel"}>
      {Intl.NumberFormat('en', { notation: "compact" }).format(runtime)} fuel.
    </span>
  </div>

  if (success) {
    return (
      <div className="flex-col flex p-4 bg-green-500 dark:bg-green-800 text-green-50 h-full">
        <div className="flex items-start">
          <span className="font-bold text-2xl">Congratulations!</span>
          {inProblemView && <ShareButton
            path={`/submissions/${id}`}
            className="bg-green-700 hover:bg-green-600 text-green-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
          />}
        </div>
        <span>Your code passed all of the supplied tests.</span>
        {fuel}
      </div>
    );
  } else {
    return (
      <div className="flex-col flex p-4 bg-white dark:bg-black h-full">
        <div className="flex items-start">
          <span className="text-red-500 font-bold text-2xl">{"Failed"}</span>
          {inProblemView && <ShareButton
            path={`/submissions/${id}`}
            className="bg-neutral-600 hover:bg-neutral-500 text-neutral-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
          />}
        </div>

        <span>Your code did not pass all of the tests.</span>
        {fuel}

      </div>
    );
  }
}
