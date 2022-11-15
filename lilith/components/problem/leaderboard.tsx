import Link from "next/link";
import { useContext } from "react";
import useSWR from "swr";
import { ProblemIDContext } from ".";
import { api_url, fetcher } from "../../utils/fetcher";

type ProblemLeaderboardItem = {
  submission_id: number,
  runtime: number,
  name: String,
  username: String,
}

function ProblemLeaderboardElement({
  submission_id,
  runtime,
  username,
  name,
  index
}: ProblemLeaderboardItem & { index: number }): JSX.Element {
  let bgColor = "bg-white dark:bg-neutral-800 border-neutral-300 hover:bg-neutral-50 hover:dark:bg-neutral-700 dark:border-neutral-700";

  if (index == 0) bgColor = "bg-yellow-400 hover:bg-yellow-300 border-yellow-500 text-yellow-900";
  if (index == 1) bgColor = "bg-slate-300 hover:bg-slate-200 border-slate-400 text-slate-800";
  if (index == 2) bgColor = "bg-orange-500 hover:bg-orange-400 border-orange-700 text-amber-900";

  const fuelCompact = Intl.NumberFormat('en', { notation: "compact" }).format(runtime) + " fuel";
  const fuelLong = Intl.NumberFormat('en', { notation: "standard" }).format(runtime) + " fuel";

  return (
    <Link href={`/submissions/${submission_id}`}>
      <a className={`transition-colors p-2 bg-silver border-b ${bgColor} flex`}>
        <div className="flex flex-col">
          <span className="font-extrabold">{name}</span>
          <span>{username}</span>
        </div>
        <span title={fuelLong} className="font-bold my-auto ml-auto">{fuelCompact}</span>
      </a>
    </Link>
  );
}

export default function ProblemLeaderboard(): JSX.Element {
  const problemId = useContext(ProblemIDContext);

  const { data } = useSWR<ProblemLeaderboardItem[]>(
    problemId ? api_url(`/problems/${problemId}/leaderboard`) : null,
    fetcher
  );

  if (!data) return <></>;

  return (
    <div className="flex flex-col">
      {data.map((item, i) => <ProblemLeaderboardElement {...item} index={i} key={i} />)}
    </div>
  );
}
