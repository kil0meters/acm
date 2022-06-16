import { useState } from "react";
import { Submission } from "../../../utils/state";

export default function SubmissionFeedback({
  id,
  error,
  success,
  runtime,
}: Submission): JSX.Element {
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
    return (
      <div className="bg-red-500 dark:bg-red-700 text-red-50 p-4 flex flex-col gap-2 h-full">
        <div className="flex items-start">
          <h1 className="text-2xl font-bold">error.</h1>
          <ShareButton
            path={`/submissions/${id}`}
            className="bg-red-700 dark:bg-red-800 hover:bg-red-600 dark:hover:bg-red-900 text-red-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
          />
        </div>

        <pre className="bg-red-700 dark:bg-red-800 overflow-x-auto p-2 rounded">
          <code>{error}</code>
        </pre>
      </div>
    );
  }

  if (success) {
    return (
      <div className="flex-col flex p-4 bg-green-500 dark:bg-green-800 text-green-50 h-full">
        <div className="flex items-start">
          <span className="font-bold text-2xl">Congratulations!</span>
          <ShareButton
            path={`/submissions/${id}`}
            className="bg-green-700 hover:bg-green-600 text-green-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
          />
        </div>
        <span>Your code passed all of the supplied tests.</span>
        <span>
          {"Ran in "} {runtime} {" ms."}
        </span>
      </div>
    );
  } else {
    return (
      <div className="flex-col flex p-4 bg-white dark:bg-black h-full">
        <div className="flex items-start">
          <span className="text-red-500 font-bold text-2xl">{"Failed"}</span>
          <ShareButton
            path={`/submissions/${id}`}
            className="bg-neutral-600 hover:bg-neutral-500 text-neutral-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
          />
        </div>

        <span>Your code did not pass all of the tests.</span>
        <span>
          {"Ran in "} {runtime} {" ms."}
        </span>
      </div>
    );
  }
}
