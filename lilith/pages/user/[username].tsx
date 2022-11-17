import type { NextPage } from "next";
import { useRouter } from "next/router";
import useSWR, { useSWRConfig } from "swr";
import Navbar from "../../components/navbar";
import Error from "next/error";
import { api_url, fetcher } from "../../utils/fetcher";
import Link from "next/link";
import { User, useSession, useStore } from "../../utils/state";
import useSWRInfinite from "swr/infinite";
import { useState } from "react";
import LoadingButton from "../../components/loading-button";
import Head from "next/head";
import SourceCodeBlock from "../../components/source-code";
import { timeFormat } from "../../utils/time";

type Submission = {
  id: number;
  problem_id: number;
  problem_title: string;
  success: boolean;
  time: string;
  runtime: number;
  code: string;
};

function StarCount({ id }: { id?: number }): JSX.Element {
  const { data } = useSWR<{ count: number }>(
    id ? api_url(`/user/star-count/${id}`) : null,
    fetcher
  );

  if (!data || (data && data.count == 0)) return <></>;

  return (
    <div className="bg-yellow-300 text-yellow-800 rounded-full px-4 h-9 self-center flex items-center">
      {data.count} ★
    </div>
  )
}

function RecentSubmissions({ username }: { username: string }): JSX.Element {
  const { data: submissions, error, isValidating, size, setSize } = useSWRInfinite<Submission[]>(
    (pageIndex, previousSubmissions) => {
      if (previousSubmissions && !previousSubmissions.length) return null;
      return api_url(`/user/username/${username}/submissions?offset=${pageIndex * 10}&count=10`);
    },
    fetcher
  );

  function SubmissionEntry({
    id,
    success,
    runtime,
    problem_title,
    time,
    code,
  }: Submission): JSX.Element {
    let compact = Intl.NumberFormat('en', { notation: "compact" }).format(runtime) + " fuel";
    let long = Intl.NumberFormat('en', { notation: "standard" }).format(runtime) + " fuel";

    return (
      <div className="border-y border-neutral-300 dark:border-neutral-700 bg-white dark:bg-black sm:rounded-md sm:m-2 md:m-0 sm:border p-4 flex flex-col gap-4">
        <div className="flex gap-2">
          <div>
            <h1 className="text-2xl font-extrabold">{problem_title}</h1>
            <div className="flex items-center gap-2 text-neutral-500">
              {success ? (
                <>
                  <span className="font-bold text-green-600 text-lg">
                    Passed
                  </span>
                  {" • "}
                  <span title={long}>
                    {compact}
                  </span>
                  {" • "}
                  <span>
                    {timeFormat(time + 'Z')}
                  </span>
                </>
              ) : (
                <>
                  <span className="font-bold text-red-600 text-lg">
                    Failed
                  </span>
                  {" • "}
                  <span>
                    {timeFormat(time + 'Z')}
                  </span>
                </>
              )}
            </div>
          </div>

          <Link href={`/submissions/${id}`}>
            <a className="ml-auto self-center bg-blue-700 hover:bg-blue-500 transition-colors text-blue-50 px-3 py-2 text-sm rounded-full font-bold">
              View Submission
            </a>
          </Link>
        </div>

        <SourceCodeBlock text={code} />
      </div>
    );
  }

  if (error) return <div>Error...</div>;

  if (!submissions) return <div>Loading...</div>;

  return (
    <div className="flex flex-col gap-4">
      <h2 className="text-2xl font-extrabold pt-4 px-4 lg:p-0">
        Recent Submissions
      </h2>

      {submissions.map((submissions2) =>
        submissions2.map((submission, i) => (
          <SubmissionEntry key={i} {...submission} />
        ))
      )}

      <LoadingButton
        loading={isValidating}
        className="rounded-full bg-neutral-200 hover:bg-neutral-300 px-6 py-3 transition-colors mx-auto dark:hover:bg-neutral-700 dark:bg-neutral-800"
        onClick={() => setSize(size + 1)}
      >Load more</LoadingButton>
    </div>
  );
}

const UserPage: NextPage = () => {
  const { query, isReady } = useRouter();
  const username = query.username;
  const [editingProfile, setEditingProfile] = useState(false);

  const { data: user, error } = useSWR<User>(
    isReady ? api_url(`/user/username/${username}`) : null,
    fetcher
  );

  function UserInfo({ name, username, auth }: User): JSX.Element {
    const { data: currentUser, error: _error } = useSWR<User>(
      api_url("/user/me"),
      fetcher, {
      shouldRetryOnError: false,
    });

    const showEditButton = currentUser?.username == username || currentUser?.auth == "ADMIN";

    return (
      <div className="flex flex-col gap-4 p-4 lg:p-0">
        <div>
          <h1 className="text-2xl font-extrabold">{name}</h1>
          <h3 className="text-neutral-500 dark:text-neutral-400">{username}</h3>
        </div>

        <div className="flex gap-2">
          <span className="rounded-full px-4 p-2 bg-neutral-600 text-neutral-50 self-start text-sm">
            {auth[0] + auth.slice(1).toLowerCase()}
          </span>

          <StarCount id={user?.id} />
        </div>

        {showEditButton && <button
          onClick={() => setEditingProfile(true)}
          className="rounded outline outline-gray-300 bg-gray-200 dark:bg-gray-700 dark:outline-gray-500 dark:hover:bg-gray-600 py-2 w-full text-center mt-4 hover:bg-gray-100 transition-colors">
          Edit profile
        </button>}
      </div>
    );
  }

  function UserEditor({ id, name, username, auth }: User): JSX.Element {
    const [newUsername, setNewUsername] = useState(username);
    const [newName, setNewName] = useState(name);
    const [newAuth, setNewAuth] = useState(auth);
    const { mutate } = useSWRConfig();
    const setError = useSession((state) => state.setError);
    const router = useRouter();

    const { data: currentUser, error: _error } = useSWR<User>(
      api_url("/user/me"),
      fetcher, {
      shouldRetryOnError: false,
    });

    // console.log(currentUser);
    console.log("Hello");

    const formClasses = "border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300";

    function submitUserEdit() {
      fetch(api_url(`/user/edit/${id}`), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify({
          new_username: newUsername,
          new_name: newName,
          new_auth: newAuth,
        }),
      })
        .then(res => res.json())
        .then(data => {
          if (data.error) {
            setError("Error updating profile", true);
          }

          if (username == newUsername) {
            router.replace(`/user/${username}`);
          } else {
            router.push(`/user/${newUsername}`);
          }

          mutate(api_url(`/user/username/${newUsername}`));
          setEditingProfile(false);
        })
        .catch(() => {
          setError("Network error", true);
        });
    }

    return (
      <div className="flex flex-col gap-2 p-4 lg:p-0">
        <div className="flex flex-col gap-2">
          <label>Name</label>
          <input
            value={newName}
            onChange={e => setNewName(e.target.value)}
            className={formClasses}
            minLength={1}
            maxLength={16}
          />
        </div>

        <div className="flex flex-col gap-2">
          <label>Username</label>
          <input
            value={newUsername}
            onChange={e => setNewUsername(e.target.value)}
            className={formClasses}
            pattern="[a-zA-Z0-9]+"
            minLength={1}
            maxLength={16}
          />
        </div>

        {currentUser?.auth == "ADMIN" && <div className="flex flex-col">
          <label>Auth</label>
          <select
            className={formClasses}
            value={newAuth}
            // @ts-ignore
            onChange={e => setNewAuth(e.currentTarget.value)}
          >
            <option value="ADMIN">Admin</option>
            <option value="OFFICER">Officer</option>
            <option value="MEMBER">Member</option>
          </select>
        </div>}

        <button
          onClick={() => submitUserEdit()}
          className="rounded outline outline-green-400 bg-green-500 py-2 w-full text-center mt-4 hover:bg-green-600 transition-colors">
          Save changes
        </button>
      </div>
    );
  }

  function UserSidebar(user: User): JSX.Element {
    return <>
      {editingProfile ? <UserEditor {...user} /> : <UserInfo {...user} />}
    </>
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

      <Head>
        <title>{user.name} - {user.username}</title>
      </Head>

      <div className="grid grid-rows-min-full grid-cols-[minmax(0,1fr)] lg:grid-rows-1 lg:grid-flow-col lg:gap-4 lg:p-4 lg:grid-cols-[300px_minmax(0,1fr)] max-w-screen-md lg:max-w-screen-lg mx-auto">
        {!user ? <UserLoading /> : <UserSidebar {...user} />}

        <RecentSubmissions username={username as string} />
      </div>
    </>
  );
};

export default UserPage;
