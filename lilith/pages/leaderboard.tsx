import { NextPage } from "next";
import Head from "next/head";
import Link from "next/link";
import useSWR from "swr";
import Navbar from "../components/navbar";
import { api_url, fetcher } from "../utils/fetcher";

type LeaderboardItem = {
  username: string;
  name: string;
  count: number;
};

type LeaderboardEntryProps = {
  index: number;
  username: string;
  name: string;
  count: number;
};

const Leaderboard: NextPage = () => {
  const { data, error } = useSWR<LeaderboardItem[]>(
    api_url("/leaderboard/first-place"),
    fetcher
  );

  function LeaderboardEntry({
    name,
    username,
    index,
    count,
  }: LeaderboardEntryProps): JSX.Element {
    return (
      <Link href={`/user/${username}`}>
        <a className="border-b border-neutral-300 dark:border-neutral-700 p-4 last:border-b-0 flex flex-row gap-4 hover:bg-neutral-100 dark:bg-black dark:hover:bg-neutral-800 transition-colors">
          <div className="bg-blue-700 text-neutral-50 flex items-center justify-center rounded-full w-9 h-9 text-xl font-bold self-center">
            {index}
          </div>
          <div className="flex flex-col">
            <span className="text-xl font-bold">{name}</span>
            <span className="text-neutral-500 dark:text-neutral-400">
              {username}
            </span>
          </div>
          <span className="ml-auto bg-yellow-300 text-yellow-800 rounded-full px-4 h-9 self-center flex items-center">
            {count} ★
          </span>
        </a>
      </Link>
    );
  }

  function LoadingLeaderboardEntry(): JSX.Element {
    return (
      <div className="border-b border-neutral-300 dark:border-neutral-700 last:border-b-0 flex flex-col p-4 gap-4">
        <div className="w-32 h-5 animate-pulse rounded bg-neutral-300" />
        <div className="w-24 h-4 animate-pulse rounded bg-neutral-300" />
      </div>
    );
  }

  if (error) return <div>Failed to load</div>;

  return (
    <>
      <Navbar />

      <Head>
        <title>Leaderboard</title>
      </Head>

      <div className="max-w-screen-md mx-auto mb-12">
        <h1 className="text-3xl font-extrabold p-2">{"Leaderboard"}</h1>

        <div className="flex flex-col border-y sm:rounded-md sm:border sm:m-2 md:m-0 border-neutral-300 dark:border-neutral-700 bg-white overflow-hidden">
          {!data
            ? Array(3)
              .fill(0)
              .map((_, i) => <LoadingLeaderboardEntry key={i} />)
            : data.map((entry, i) => (
              <LeaderboardEntry key={i} index={i + 1} {...entry} />
            ))}
        </div>
      </div>
    </>
  );
};

export default Leaderboard;
