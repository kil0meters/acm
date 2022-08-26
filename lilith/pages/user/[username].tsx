import type { NextPage } from "next";
import { useRouter } from "next/router";
import useSWR from "swr";
import Navbar from "../../components/navbar";
import Error from "next/error";
import { api_url, fetcher } from "../../utils/fetcher";
import Link from "next/link";
import Prism from "prismjs";
import { User } from "../../utils/state";

type Submission = {
  problem_id: number;
  success: boolean;
  runtime: number;
  code: string;
};

function RecentSubmissions({ username }: { username: string }): JSX.Element {
  const { data: submissions, error } = useSWR<Submission[]>(
    api_url(`/user/username/${username}/submissions`),
    fetcher
  );

  function SubmissionEntry({
    success,
    runtime,
    problem_id,
    code,
  }: Submission): JSX.Element {
    return (
      <div className="border-y border-neutral-300 dark:border-neutral-700 bg-white dark:bg-black sm:rounded-md sm:m-2 md:m-0 sm:border p-4 flex flex-col gap-4">
        <div className="flex gap-2">
          {success ? (
            <>
              <span className="font-bold text-green-600 text-2xl self-center">
                Passed
              </span>
              <span className="text-green-600 self-center text-sm">
                {runtime}ms
              </span>
            </>
          ) : (
            <span className="font-bold text-red-600 text-2xl self-center">
              Failed
            </span>
          )}

          <Link href={`/problems/${problem_id}`}>
            <a className="ml-auto self-center bg-blue-700 hover:bg-blue-500 transition-colors text-blue-50 px-3 py-2 text-sm rounded-full font-bold">
              View Problem
            </a>
          </Link>
        </div>

        <pre
          className="language-cpp rounded-md bg-blue-50 dark:bg-slate-800 p-2 overflow-auto max-h-72 border border-blue-200 dark:border-slate-700"
          dangerouslySetInnerHTML={{
            __html: Prism.highlight(code, Prism.languages.cpp, "cpp"),
          }}
        />
      </div>
    );
  }

  if (error) return <div>Error...</div>;

  if (!submissions) return <div>Loading...</div>;

  return (
    <div className="flex flex-col gap-4">
      <h2 className="text-2xl font-bold pt-4 px-4 lg:p-0">
        Recent Submissions
      </h2>

      {submissions.map((submission, i) => (
        <SubmissionEntry key={i} {...submission} />
      ))}
    </div>
  );
}

const UserPage: NextPage = () => {
  const { query, isReady } = useRouter();
  const username = query.username;

  const { data: user, error } = useSWR<User>(
    isReady ? api_url(`/user/username/${username}`) : null,
    fetcher
  );

  function UserInfo({ name, username, auth }: User): JSX.Element {
    return (
      <div className="flex flex-col gap-2 p-4 lg:p-0">
        <h1 className="text-2xl font-bold">{name}</h1>
        <h3 className="text-neutral-500 dark:text-neutral-400">{username}</h3>

        <span className="rounded-full px-4 p-2 bg-neutral-600 text-neutral-50 self-start text-sm">
          {auth[0] + auth.slice(1).toLowerCase()}
        </span>
      </div>
    );
  }

  function UserLoading(): JSX.Element {
    return (
      <div className="flex flex-col gap-2 p-4 lg:p-0">
        <h1 className="rounded bg-neutral-300 animate-pulse w-32 h-6 my-1" />
        <h3 className="rounded bg-neutral-300 animate-pulse w-48 h-4 my-1" />
      </div>
    );
  }

  if (error) return <Error statusCode={404} />;

  if (!user) return <div>Loading</div>;

  return (
    <>
      <Navbar />

      <div className="grid grid-rows-min-full grid-cols-[minmax(0,1fr)] lg:grid-rows-1 lg:grid-flow-col lg:gap-4 lg:p-4 lg:grid-cols-[300px_minmax(0,1fr)] max-w-screen-md lg:max-w-screen-lg mx-auto">
        {!user ? <UserLoading /> : <UserInfo {...user} />}

        <RecentSubmissions username={username as string} />
      </div>
    </>
  );
};

export default UserPage;
