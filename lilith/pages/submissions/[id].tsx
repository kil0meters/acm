import { NextPage } from "next";
import Error from "next/error";
import Link from "next/link";
import { useRouter } from "next/router";
import Prism from "prismjs";
import useSWR, { mutate } from "swr";
import Navbar from "../../components/navbar";
import SubmissionFeedback from "../../components/problem/submission";
import { api_url, fetcher } from "../../utils/fetcher";
import { Submission, User, useStore } from "../../utils/state";
import { timeFormat } from "../../utils/time";

function InvalidateButton({ id }: { id?: number }): JSX.Element {
  const submit = async () => {
    await fetch(api_url(`/submissions/${id}/invalidate`), {
      credentials: "include",
    });

    mutate(api_url(`/submissions/${id}`));
  }

  return (
    <button
      onClick={submit}
      className="w-full mt-4 rounded-full bg-red-600 hover:bg-red-700 px-4 py-2 text-red-50 transition-colors">
      Invalidate
    </button>
  )
}

function ValidateButton({ id }: { id?: number }): JSX.Element {
  const submit = async () => {
    await fetch(api_url(`/submissions/${id}/validate`), {
      credentials: "include",
    });

    mutate(api_url(`/submissions/${id}`));
  }

  return (
    <button
      onClick={submit}
      className="w-full mt-4 rounded-full bg-blue-600 hover:bg-blue-700 px-4 py-2 text-red-50 transition-colors">
      Validate
    </button>
  )
}

type UserInfoProps = {
  id: number
};

function UserInfo({ id }: UserInfoProps): JSX.Element {
  const { data, error } = useSWR<User>(
    id ? api_url(`/user/id/${id}`) : null,
    fetcher
  );

  if (error)
    return <>Error</>;

  if (!data)
    return <div>loading</div>;

  return (
    <Link href={`/user/${data.username}`}>
      {data.name}
    </Link>
  );
}

const SubmissionPage: NextPage = () => {
  const router = useRouter();
  const id = router.isReady ? parseInt(router.query.id as string) : undefined;
  const setProblemImpl = useStore((state) => state.setProblemImpl);

  const { data: submission, error } = useSWR<Submission>(
    id ? api_url(`/submissions/${id}`) : null,
    fetcher
  );

  const { data: user, error: _error } = useSWR<User>(
    api_url("/user/me"),
    fetcher, {
    shouldRetryOnError: false,
  });

  if (error)
    return <Error statusCode={404} />;

  if (!submission) {
    return <Navbar />;
  }

  return <>
    <Navbar />

    <div className="grid grid-cols-[minmax(0,1fr)_320px] mt-4 md:flex-row md:gap-4 max-w-screen-lg md:mx-auto md:mt-8">
      <div className="p-4 border-neutral-300 border-b md:border rounded-md flex flex-col gap-4 max-w">
        <div className="flex flex-col gap-1">
          <div className="flex flex-col">
            <h1 className="text-3xl font-extrabold">{"Problem "} {submission.problem_id}</h1>
          </div>
          <span className="text-sm text-neutral-500">
            <UserInfo id={submission.user_id} />
            {" • "}
            {timeFormat(submission.time + 'Z')}
          </span>
        </div>

        <pre
          className="language-cpp rounded-md bg-blue-50 dark:bg-slate-800 p-2 overflow-auto border border-blue-200 dark:border-slate-700"
          dangerouslySetInnerHTML={{
            __html: Prism.highlight(submission.code, Prism.languages.cpp, "cpp"),
          }}
        />

        <div className="mt-auto p-4 rounded-md bg-yellow-300 border-yellow-500 border text-yellow-900">
          <h1 className="font-bold text-xl mb-2">{"Warning"}</h1>
          <p>{"You will lose your current progress."}</p>
        </div>

        <a href={`/problems/${submission.problem_id}`}
          onClick={event => {
            event.preventDefault();
            setProblemImpl(submission.problem_id, submission.code);
            router.push(`/problems/${submission.problem_id}`);
          }}
          className="rounded-full p-2 bg-blue-600 hover:bg-blue-500 text-blue-50 transition-colors text-center">
          {"View in editor"}
        </a>
      </div>
      <div>
        <div className="border rounded-md border-neutral-300 overflow-hidden">
          <SubmissionFeedback inProblemView={false} {...submission} />
        </div>

        {(user && (user.auth == "ADMIN" || user.auth == "OFFICER")) && <>
          <ValidateButton id={id} />
          <InvalidateButton id={id} />
        </>}
      </div>
    </div>
  </>;
};

export default SubmissionPage;
