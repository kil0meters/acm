import type { NextPage } from "next";
import Link from "next/link";
import useSWR from "swr";
import Navbar from "../../components/navbar";
import { api_url, fetcher } from "../../utils/fetcher";
import { User } from "../../utils/state";

export type Competition = {
  id: number,
  name: string,
  start: string,
  end: string,
};

function CompetitionListItem({ id, name, start }: Competition): JSX.Element {
  return (
    <Link href={`/competitions/${id}`}>
      <a className="border-neutral-300 p-4 border rounded-md bg-white dark:bg-black dark:border-neutral-700 dark:hover:bg-neutral-900 flex flex-col gap-2 hover:shadow-md transition-all">
        <h1 className="text-2xl font-bold">{name}</h1>
        <span className="text-neutral-700 dark:text-neutral-400">{new Intl.DateTimeFormat('en-US', { dateStyle: 'long', timeStyle: undefined }).format(new Date(start))}</span>
      </a>
    </Link>
  );
}

export function CompetitionGrid(): JSX.Element {
  const { data, error } = useSWR<Competition[]>(api_url("/competitions"), fetcher);

  if (error)
    return <></>;

  if (!data)
    return <></>;

  return (
    <div className="grid grid-cols-2 gap-4">

      {data.map((competition, i) => <CompetitionListItem key={i} {...competition} />)}
    </div>
  );
}

const CompetitionsPage: NextPage = () => {
  const { data: user, error: _error } = useSWR<User>(
    api_url("/user/me"),
    fetcher, {
    shouldRetryOnError: false,
  });

  return (
    <>
      <Navbar />

      <div className="flex flex-col max-w-screen-md mx-auto my-4 gap-4">
        <div className="flex">
          <h1 className="text-3xl font-extrabold">Competitions</h1>

          {user && (user.auth === "OFFICER" || user.auth === "ADMIN") && (
            <Link href="/competitions/new">
              <a className="ml-auto text-green-50 text-sm font-bold rounded-full bg-green-700 hover:bg-green-500 transition-colors px-4 py-2 mr-4 md:mr-0">
                New Competition
              </a>
            </Link>
          )}
        </div>

        <CompetitionGrid />
      </div>
    </>
  );
};

export default CompetitionsPage;
