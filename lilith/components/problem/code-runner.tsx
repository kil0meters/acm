import { useContext, useState } from "react";
import { useSWRConfig } from "swr";
import { ProblemIDContext } from ".";
import { api_url } from "../../utils/fetcher";
import { Submission, useSession, useStore } from "../../utils/state";
import LoadingButton from "../loading-button";
import InputTester from "./input-tester";
import { SUBMISISON_TESTS_QUERY } from "./submission/tests";

export default function CodeRunner(): JSX.Element {
  const [dockerShown, setDockerShown] = useState(false);

  function SubmitButton(): JSX.Element {
    const token = useStore((state) => state.token);
    const setProblemSubmission = useStore(
      (state) => state.setProblemSubmission
    );
    const id = useContext(ProblemIDContext)!;
    const implementation = useStore(
      (state) => id && state.problems[id]?.implementation
    );
    const setError = useSession((session) => session.setError);
    const [loading, setLoading] = useState(false);
    const { mutate } = useSWRConfig();

    const submitProblem = async () => {
      if (!token) {
        setError("You must be logged in to submit a problem.", true);
        return;
      }

      if (!implementation) {
        setError("You must modify the answer before submitting.", true);
        return;
      }

      setLoading(true);

      fetch(api_url("/submit-problem"), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          problem_id: id,
          implementation,
        }),
      })
        .then((res) => res.json())
        .then((res: Submission) => {
          setProblemSubmission(id, res);
          setLoading(false);

          // make sure we queue after the submission id is updated
          setTimeout(() => mutate(SUBMISISON_TESTS_QUERY), 0);
        })
        .catch(() => {
          // TODO: Handle network errors.
          setLoading(false);
        });
    };

    return (
      <LoadingButton
        onClick={() => submitProblem()}
        loading={loading}
        className="p-4 border-l border-neutral-300 dark:border-neutral-700 bg-green-500 dark:bg-green-600 hover:bg-green-400 dark:hover:bg-green-500 transition-colors text-white"
      >
        Submit
      </LoadingButton>
    );
  }

  return (
    <div className="sticky md:static bottom-0">
      {dockerShown && <InputTester />}

      <div className="flex bg-white dark:bg-black border-t border-neutral-300 dark:border-neutral-700">
        <button
          className="mr-auto p-4 border-r border-neutral-300 dark:border-neutral-700 hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors"
          onClick={() => setDockerShown(!dockerShown)}
        >
          {dockerShown ? "Hide console" : "Show console"}
        </button>

        <SubmitButton />
      </div>
    </div>
  );
}
