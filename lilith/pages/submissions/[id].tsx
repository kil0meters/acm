import { NextPage } from "next";
import Error from "next/error";
import Link from "next/link";
import { useRouter } from "next/router";
import Prism from "prismjs";
import useSWR from "swr";
import Navbar from "../../components/navbar";
import SubmissionFeedback from "../../components/problem/submission";
import { api_url, fetcher } from "../../utils/fetcher";
import { Submission, useStore } from "../../utils/state";
import { timeFormat } from "../../utils/time";

const SubmissionPage: NextPage = () => {
  const router = useRouter();
  const id = router.isReady ? parseInt(router.query.id as string) : undefined;
  const setProblemImpl = useStore((state) => state.setProblemImpl);

  const { data: submission, error } = useSWR<Submission>(
    id ? api_url(`/submissions/${id}`) : null,
    fetcher
  );

  if (error)
    return <Error statusCode={404} />;

  if (!submission) {
    return <Navbar />;
  }

  return <>
    <Navbar />

    <div className="bg-white md:rounded-xl border border-neutral-300 flex flex-col mt-4 md:grid md:grid-cols-5 overflow-hidden max-w-screen-md md:mx-auto md:mt-8">
      <div className="p-4 col-span-3 border-neutral-300 border-b md:border-b-0 md:border-r flex flex-col gap-4">
        <div className="flex flex-col gap-1">
          <h1 className="text-3xl font-extrabold">{ "Problem " } { submission.problem_id }</h1>
          <span className="text-sm text-neutral-500">
            <Link href="/users/kilometers">
              { "Miles Benton" }
            </Link>
            {" • "}
            { timeFormat(submission.time) }
          </span>
        </div>

        <pre
          className="language-cpp rounded-md bg-blue-50 dark:bg-slate-800 p-2 overflow-auto border border-blue-200 dark:border-slate-700"
          dangerouslySetInnerHTML={{
            __html: Prism.highlight(submission.code, Prism.languages.cpp, "cpp"),
          }}
        />

        <div className="mt-auto p-4 rounded-md bg-yellow-300 border-yellow-500 border text-yellow-900">
          <h1 className="font-bold text-xl mb-2">{ "Warning" }</h1>
          <p>{ "You will lose your current progress." }</p>
        </div>

        <a href={`/problems/${submission.problem_id}`}
          onClick={event => {
            event.preventDefault();
            setProblemImpl(submission.problem_id, submission.code);
            router.push(`/problems/${submission.problem_id}`);
          }}
          className="rounded-full p-2 bg-blue-600 hover:bg-blue-500 text-blue-50 transition-colors text-center">
          { "View in editor" }
        </a>
      </div>
      <div className="col-span-2 border-b border-neutral-300 md:border-0">
        <SubmissionFeedback {...submission} />
      </div>
    </div>
  </>;
};

export default SubmissionPage;
