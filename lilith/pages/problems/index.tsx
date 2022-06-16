import type { NextPage } from "next";
import Navbar from "../../components/navbar";
import useSWR from "swr";
import { marked } from "marked";
import Link from "next/link";
import { api_url } from "../../utils/fetcher";
import { useStore } from "../../utils/state";
import { useEffect, useState } from "react";

const fetcher = (url: string) => fetch(url).then((res) => res.json());

function ProblemLoading(): JSX.Element {
  return (
    <div className="animate-fade-in sm:rounded-md border-neutral-300 dark:border-neutral-700 border-y sm:border sm:mx-2 md:m-0 bg-white dark:bg-black dark:hover:bg-neutral-800 p-4 hover:shadow-md max-h-52 hover:max-h-64 overflow-hidden transition-all">
      <div className="animate-pulse">
        <h1 className="rounded bg-neutral-200 w-64 text-2xl h-6 mb-4"></h1>
        <p className="rounded bg-neutral-200 w-full h-3 mb-2"></p>
        <p className="rounded bg-neutral-200 w-full h-3 mb-2"></p>
        <p className="rounded bg-neutral-200 w-full h-3"></p>
      </div>
    </div>
  );
}

type Problem = {
  id: number;
  title: string;

  // markdown formatted
  description: string;
};

function ProblemListing({ id, title, description }: Problem): JSX.Element {
  let desc = marked.parse(description);

  return (
    <Link href={`/problems/${id}`}>
      <a className="sm:rounded-md border-neutral-300 dark:border-neutral-700 border-y sm:border sm:mx-2 md:m-0 bg-white dark:bg-black dark:hover:bg-neutral-800 p-4 hover:shadow-md max-h-52 hover:max-h-64 overflow-hidden transition-all">
        <h1 className="text-2xl font-extrabold mb-4">{title}</h1>

        <div
          className="prose prose-neutral dark:prose-invert"
          dangerouslySetInnerHTML={{ __html: desc }}
        />
      </a>
    </Link>
  );
}

function ListLoading(): JSX.Element {
  return (
    <>
      <ProblemLoading />
      <ProblemLoading />
      <ProblemLoading />
      <ProblemLoading />
    </>
  );
}

function ListContent({ problems }: { problems: Problem[] }): JSX.Element {
  return (
    <>
      {problems.map(({ id, title, description }) => (
        <ProblemListing
          key={id}
          id={id}
          title={title}
          description={description}
        />
      ))}
    </>
  );
}

const ProblemList: NextPage = () => {
  const auth = useStore((state) => state.user?.auth);
  const { data, error } = useSWR<Problem[]>(api_url("/problems"), fetcher);
  const [isComponentMounted, setIsComponentMounted] = useState(false);

  useEffect(() => setIsComponentMounted(true), []);

  if (error) return <div>Error</div>;

  return (
    <>
      <Navbar />

      <div className="max-w-screen-md mx-auto my-4 flex flex-col gap-4">
        {isComponentMounted && (auth === "OFFICER" || auth === "ADMIN") && (
            <Link href="/problems/new">
              <a className="ml-auto text-green-50 text-sm font-bold rounded-full bg-green-700 hover:bg-green-500 transition-colors px-4 py-2 mr-4 md:mr-0">
                New Problem
              </a>
            </Link>
          )}

        {!data ? <ListLoading /> : <ListContent problems={data} />}
      </div>
    </>
  );
};

export default ProblemList;
