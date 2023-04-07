import type { NextPage } from "next";
import Navbar from "../../components/navbar";
import useSWRInfinite from "swr/infinite";
import { marked } from "marked";
import Link from "next/link";
import { api_url, fetcher } from "../../utils/fetcher";
import { User, useStore } from "../../utils/state";
import { useContext, useEffect, useRef, useState } from "react";
import renderLatex from "../../utils/latex";
import { CompetitionIDContext } from "../competitions/[id]";
import useSWR from "swr";
import LoadingButton from "../../components/loading-button";
import Head from "next/head";

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

export type Problem = {
    id: number;
    title: string;

    // markdown formatted
    description: string;

    template: string;

    visible: boolean;
    difficulty: string;
};

type ProblemStatus = "Complete" | "InProgress" | "NotStarted";

function ProblemTeamStatus({ problem_id }: { problem_id: number }): JSX.Element {

    const competition_id = useContext(CompetitionIDContext);

    const { data, error } = useSWR<ProblemStatus>(
        api_url(`/competitions/${competition_id}/problem-status/${problem_id}`),
        fetcher
    );

    if (!data || error) return <></>;

    if (data == "InProgress") {
        return (
            <div className="ml-auto bg-blue-700 text-blue-50 rounded-full px-4 py-2 text-sm">
                In Progress
            </div>
        );
    } else if (data == "Complete") {
        return (
            <div className="ml-auto bg-green-700 text-blue-50 rounded-full px-4 py-2 text-sm">
                Completed
            </div>
        );
    } else {
        return <></>;
    }
}

interface ProblemListingProps extends Problem {
    show_team_status?: boolean;
};

function DifficultyBadge({ difficulty }: { difficulty: string }): JSX.Element {
    if (difficulty == "Easy") {
        return <span className="bg-green-600 text-green-50 rounded-full px-4 py-2 text-sm">Easy</span>;
    }

    if (difficulty == "Medium") {
        return <span className="bg-yellow-600 text-yellow-50 rounded-full px-4 py-2 text-sm">Medium</span>;
    }

    if (difficulty == "Hard") {
        return <span className="bg-red-600 text-red-50 rounded-full px-4 py-2 text-sm">Hard</span>;
    }

    return <>{difficulty}</>;
}

function ProblemListing({ id, title, description, show_team_status, difficulty }: ProblemListingProps): JSX.Element {
    let desc = marked.parse(description);
    const content = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (content.current) {
            renderLatex(content.current);
        }
    });

    return (
        <Link href={`/problems/${id}`}>
            <a className="sm:rounded-md border-neutral-300 dark:border-neutral-700 border-y sm:border sm:mx-2 md:m-0 bg-white dark:bg-black dark:hover:bg-neutral-800 p-4 hover:shadow-md max-h-52 hover:max-h-64 overflow-hidden transition-all">
                <div className="flex items-center mb-4">
                    <h1 className="text-2xl font-extrabold">{title}</h1>

                    <div className="flex ml-auto gap-4">
                        <DifficultyBadge difficulty={difficulty} />
                        {show_team_status && <ProblemTeamStatus problem_id={id} />}
                    </div>
                </div>

                <div
                    ref={content}
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

export function ProblemList({ problems, show_team_status }: { problems: Problem[], show_team_status?: boolean }): JSX.Element {
    return (
        <>
            {problems.map(({ id, title, description, difficulty, visible }) => (
                <ProblemListing
                    key={id}
                    id={id}
                    title={title}
                    template={""}
                    description={description}
                    show_team_status={show_team_status}
                    difficulty={difficulty}
                    visible={visible}
                />
            ))}
        </>
    );
}

const ProblemListPage: NextPage = () => {
    const [difficulty, setDifficulty] = useState(0);
    const [showCompetitionProblems, setShowCompetitionProblems] = useState(false);
    const [sortBy, setSortBy] = useState("Newest");

    const { data, error, isValidating, size, setSize } = useSWRInfinite<Problem[]>(
        (pageIndex, previousProblems) => {
            if (previousProblems && !previousProblems.length) return null;
            return api_url(`/problems?offset=${7 * pageIndex}&count=7&difficulty=${difficulty}&show_competition_problems=${showCompetitionProblems}&sort_by=${sortBy}`);
        },
        fetcher
    );
    const [isComponentMounted, setIsComponentMounted] = useState(false);
    const [showFilters, setShowFilters] = useState(false);

    const { data: user } = useSWR<User>(
        api_url("/user/me"),
        fetcher, {
        shouldRetryOnError: false,
    });

    useEffect(() => setIsComponentMounted(true), []);

    if (error) return <div>Error</div>;

    return (
        <>
            <Navbar />

            <Head>
                <title>Problems</title>
            </Head>

            <div className="max-w-screen-md mx-auto my-4 flex flex-col gap-4">
                <div className="flex items-center">
                    <button onClick={() => setShowFilters(!showFilters)} className="bg-neutral-100 dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:bg-neutral-200 px-4 py-2 rounded-full transition-colors">
                        Filters
                    </button>
                    {isComponentMounted && user && (user.auth === "OFFICER" || user.auth === "ADMIN") && (
                        <Link href="/problems/new">
                            <a className="ml-auto text-green-50 text-sm font-bold rounded-full bg-green-700 hover:bg-green-500 transition-colors px-4 py-2 mr-4 md:mr-0">
                                New Problem
                            </a>
                        </Link>
                    )}
                </div>

                {showFilters && <div className="bg-neutral-100 dark:bg-neutral-800 rounded-xl p-4 flex flex-col gap-4 sm:flex-row sm:gap-24">
                    <div className="flex flex-col">
                        <span className="font-bold mb-2">Difficulty</span>

                        <div className="flex items-center gap-2">
                            <input id="easy" type="checkbox" value={difficulty & 1} onChange={() => setDifficulty(difficulty ^ 1)} />
                            <label htmlFor="easy">Easy</label>
                        </div>

                        <div className="flex items-center gap-2">
                            <input id="medium" type="checkbox" value={difficulty & 2} onChange={() => setDifficulty(difficulty ^ 2)} />
                            <label htmlFor="medium">Medium</label>
                        </div>

                        <div className="flex items-center gap-2">
                            <input id="hard" type="checkbox" value={difficulty & 4} onChange={() => setDifficulty(difficulty ^ 4)} />
                            <label htmlFor="medium">Hard</label>
                        </div>
                    </div>

                    <div>
                        <span className="font-bold">Misc</span>

                        <div className="flex items-center gap-2">
                            <input type="checkbox" name="time" checked={showCompetitionProblems} onChange={() => setShowCompetitionProblems(!showCompetitionProblems)} />
                            Show competition problems
                        </div>
                    </div>

                    <div>
                        <span className="font-bold">Sort By</span>

                        <div className="flex items-center gap-2">
                            <input id="newest" type="radio" name="sort-by" checked={sortBy == "Newest"} onChange={() => setSortBy("Newest")} />
                            <label htmlFor="oldest">Newest</label>
                        </div>

                        <div className="flex items-center gap-2">
                            <input id="oldest" type="radio" name="sort-by" checked={sortBy == "Oldest"} onChange={() => setSortBy("Oldest")} />
                            <label htmlFor="oldest">Oldest</label>
                        </div>
                    </div>
                </div>}

                {!data ? <ListLoading /> : data.map((problems, i) => <ProblemList key={i} problems={problems} />)}

                <LoadingButton
                    loading={isValidating}
                    className="rounded-full bg-neutral-200 hover:bg-neutral-300 px-6 py-3 transition-colors mx-auto dark:hover:bg-neutral-700 dark:bg-neutral-800"
                    onClick={() => setSize(size + 1)}
                >Load more</LoadingButton>
            </div>
        </>
    );
};

export default ProblemListPage;
