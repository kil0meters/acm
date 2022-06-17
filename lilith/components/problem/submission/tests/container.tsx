import { useTransition, animated } from "@react-spring/web";
import { useContext, useState } from "react";
import SubmissionFeedback from "..";
import { ProblemIDContext } from "../..";
import { useStore } from "../../../../utils/state";
import TestEntries from "./entries";

export default function TestContainer(): JSX.Element {
  const [isVisible, setIsVisible] = useState<Boolean>(false);
  const id = useContext(ProblemIDContext);
  const submission = useStore((state) => id && state.problems[id]?.submission);

  function toggleVisibility() {
    setIsVisible(!isVisible);
  }

  const collapse = useTransition(isVisible, {
    from: {
      height: "0px",
    },
    enter: {
      height: "300px",
    },
    leave: {
      height: "0px",
    },
  });

  return (
    <div className="flex flex-col">
      {submission && <SubmissionFeedback {...submission} />}

      <div className="flex flex-col border-neutral-300 dark:border-neutral-700 md:m-0 md:border-0">
        <button
          onClick={toggleVisibility}
          className="text-left p-4 bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:bg-neutral-100 cursor-pointer select-none transition-colors border-b border-neutral-300 dark:border-neutral-700"
        >
          {isVisible ? "Hide tests" : "Show tests"}
        </button>

        {collapse(
          (styles, item) =>
            item && (
              <animated.div style={styles}>
                <TestEntries />
              </animated.div>
            )
        )}
      </div>
    </div>
  );
}