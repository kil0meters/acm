import { useContext, useState } from "react";
import { useSWRConfig } from "swr";
import { ProblemIDContext } from ".";
import { api_url } from "../../utils/fetcher";
import { JobStatus, monitorJob } from "../../utils/job";
import { Submission, useSession, useStore } from "../../utils/state";
import EditorPreferences from "../editor-preferences";
import LoadingButton from "../loading-button";
import Modal from "../modal";
import QueueStatus from "../queue-status";
import InputTester from "./input-tester";
import { ServerError } from "./submission/error";
import { SUBMISSION_TESTS_QUERY } from "./submission/tests";

export default function CodeRunner(): JSX.Element {
  const [dockerShown, setDockerShown] = useState(false);
  const [settingsShown, setSettingsShown] = useState(false);

  function SubmitButton(): JSX.Element {
    const setSubmission = useSession(
      (state) => state.setSubmission
    );
    const id = useContext(ProblemIDContext)!;
    const implementation = useStore(
      (state) => id && state.problemImpls[id]
    );
    const setError = useSession((session) => session.setError);
    const [loading, setLoading] = useState(false);
    const [queuePosition, setQueuePosition] = useState(0);
    const { mutate } = useSWRConfig();

    const submitProblem = async () => {
      if (!implementation) {
        setError("You must modify the answer before submitting.", true);
        return;
      }

      setLoading(true);

      awful_bad_hacky_code: try {
        let res = await fetch(api_url("/run/submit"), {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          credentials: "include",
          body: JSON.stringify({
            problem_id: id,
            implementation,
          }),
        });

        let job: JobStatus<Submission, ServerError> = await res.json();

        // yeah, I know typescript, how could you tell
        if ((job as unknown as ServerError).error) {
          setError((job as unknown as ServerError).error, true);
          break awful_bad_hacky_code;
        }

        let [data, err] = await monitorJob(job, (n) => setQueuePosition(n));

        if (data) {
          setSubmission(id, data);
          setTimeout(() => mutate(SUBMISSION_TESTS_QUERY), 0);
        }

        if (err) {
          console.log(err);
          setError(err.error, true);
        }
      }
      catch (e) {
        console.log(e);
        setError("Network error.", true);
      }
      finally {
        setLoading(false);
      }
    };

    return (
      <div>
        {loading &&
          <QueueStatus className="mr-4" queuePosition={queuePosition} />
        }
        <LoadingButton
          onClick={() => submitProblem()}
          loading={loading}
          className="p-4 border-l h-full border-neutral-300 dark:border-neutral-700 bg-green-500 dark:bg-green-600 hover:bg-green-400 dark:hover:bg-green-500 transition-colors text-white"
        >
          Submit
        </LoadingButton>
      </div>
    );
  }

  return (
    <div className="sticky md:static bottom-0">
      <Modal shown={settingsShown} onClose={() => setSettingsShown(false)}>
        <EditorPreferences />
      </Modal>
      {dockerShown && <InputTester />}

      <div className="flex bg-white dark:bg-black border-t border-neutral-300 dark:border-neutral-700">
        <button
          className="p-4 border-r border-neutral-300 dark:border-neutral-700 hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors"
          onClick={() => setDockerShown(!dockerShown)}
        >
          {dockerShown ? "Hide console" : "Show console"}
        </button>

        <button
          className="mr-auto p-4 border-r border-neutral-300 dark:border-neutral-700 hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors"
          onClick={() => setSettingsShown(!settingsShown)}
        >
          Settings
        </button>

        <SubmitButton />
      </div>
    </div>
  );
}
