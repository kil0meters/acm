import { NextPage } from "next";
import Navbar from "../../components/navbar";
import { createContext, useContext, useEffect, useState } from "react";
import Error from "next/error";
import Countdown from "../../components/countdown";
import { User, useSession } from "../../utils/state";
import Link from "next/link";
import { Problem, ProblemList } from "../problems";
import { useRouter } from "next/router";
import { api_url, fetcher } from "../../utils/fetcher";
import useSWR, { mutate } from "swr";
import Modal from "../../components/modal";
import { isServerError, ServerError } from "../../components/problem/submission/error";
import Head from "next/head";

function TeamDisplay({ id }: { id: number }) {
    const { data: team, error: _error } = useSWR<Team>(
        api_url(`/competitions/0/teams/${id}`),
        fetcher
    );

    if (!team) return <></>;

    return (
        <div className="border border-neutral-300 dark:border-neutral-700 rounded-md bg-white dark:bg-black flex flex-col overflow-hidden">
            <h1 className="bg-neutral-50 dark:bg-neutral-800 border-b last:border-b-0 border-neutral-300 dark:border-neutral-700 p-2 text-xl font-extrabold">
                {team.name}
            </h1>

            {team.members.map((user, i) => <TeamUser key={i} {...user} />)}
        </div>
    );
}

function CompetitionLeaderboardEntry({ id, name, score }: CompetitionLeaderboardEntryProps) {
    const [shown, setShown] = useState(false);

    return (
        <div className="border-neutral-300 border-b dark:border-neutral-700 last:border-b-0">
            <button
                onClick={() => setShown(true)}
                className="w-full dark:border-neutral-700 p-4 flex flex-row items-center gap-4 hover:bg-neutral-100 dark:bg-black dark:hover:bg-neutral-800 transition-colors">
                <div className="flex flex-col">
                    <span className="text-xl font-bold whitespace-nowrap">{name}</span>
                </div>
                {score > 0 && <span className="ml-auto whitespace-nowrap bg-yellow-300 text-yellow-800 rounded-full px-4 h-9 self-center flex items-center">
                    {score} ★
                </span>}

            </button>

            <Modal shown={shown} onClose={() => setShown(false)}>
                <TeamDisplay id={id} />
            </Modal>
        </div>
    );
}

type Team = {
    id: number,
    name: string,
    members: User[]
};

type CompetitionLeaderboardEntryProps = {
    id: number,
    name: string,
    score: number,
}

function CompetitionLeaderboard() {
    const id = useContext(CompetitionIDContext);
    const setError = useSession((state) => state.setError);

    const { data: user, error: _error } = useSWR<User>(
        api_url("/user/me"),
        fetcher, {
        shouldRetryOnError: false,
    });

    function NewTeamButton(): JSX.Element {
        const [shown, setShown] = useState(false);
        const [name, setName] = useState("");

        const submit = async () => {
            const res: {} | ServerError = await (await fetch(api_url(`/competitions/${id}/teams/new`), {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                credentials: "include",
                body: JSON.stringify({
                    name,
                }),
            })).json();

            if (isServerError(res)) {
                setError(res.error, true);
            } else {
                setShown(false);
                mutate(api_url(`/competitions/${id}/leaderboard`));
            }
        };

        return (
            <>
                <button
                    onClick={() => setShown(true)}
                    className="ml-auto my-auto text-green-50 text-sm font-bold rounded-full bg-green-700 hover:bg-green-500 transition-colors px-4 py-2 mr-2 lg:mr-0"
                >
                    New Team
                </button>

                <Modal shown={shown} onClose={() => setShown(false)}>
                    <div className="border bg-white p-4 border-neutral-300 rounded-md flex flex-col gap-4 dark:bg-black dark:border-neutral-700">
                        <span>Name</span>
                        <input
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                            className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
                        />

                        <div>
                            <button
                                onClick={submit}
                                className="text-green-50 text-sm font-bold rounded-full bg-green-700 hover:bg-green-500 transition-colors px-4 py-2 mr-4 md:mr-0"
                            >
                                New Team
                            </button>
                        </div>
                    </div>
                </Modal>
            </>
        );
    }


    function CompetitionLeaderboardList(): JSX.Element {
        const { data } = useSWR<CompetitionLeaderboardEntryProps[]>(
            api_url(`/competitions/${id}/leaderboard`),
            fetcher
        );

        if (!data) {
            return <></>;
        }

        return (
            <div className="border-y sm:rounded-md sm:border sm:m-2 lg:m-0 border-neutral-300 dark:border-neutral-700 bg-white overflow-hidden">
                <div>
                    {data.map((item, i) => (
                        <CompetitionLeaderboardEntry key={i} {...item} />
                    ))}
                </div>
            </div>
        );
    }

    return (
        <div>
            <div className="flex">
                <h1 className="text-2xl font-extrabold p-2 lg:px-0">Rankings</h1>

                {user && (user.auth === "OFFICER" || user.auth === "ADMIN") && <NewTeamButton />}
            </div>
            <CompetitionLeaderboardList />
        </div>
    );
};

function TeamUser({ id, username, name }: User): JSX.Element {
    return (
        <Link href={`/user/${username}`}>
            <a className="p-2 flex flex-col border-b last:border-b-0 border-neutral-300 dark:border-neutral-700 dark:bg-black hover:bg-neutral-50 dark:hover:bg-neutral-900 transition-colors">
                <span className="font-bold">{name}</span>
                <span className="text-sm text-neutral-700 dark:text-neutral-400">{username}</span>
            </a>
        </Link>
    );
}

function TeamJoiner(): JSX.Element {
    const competition_id = useContext(CompetitionIDContext);
    const [shown, setShown] = useState(false);

    const { data, error: __error } = useSWR<Team[]>(
        api_url(`/competitions/${competition_id}/teams/joinable`),
        fetcher
    );


    function JoinableTeamEntry({ id, name, members }: Team): JSX.Element {
        const join_team = async () => {
            await fetch(api_url(`/competitions/${id}/teams/join`), {
                method: "POST",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    team_id: id,
                })
            });

            mutate(api_url(`/competitions/${competition_id}/teams/me`));
        };

        return (
            <button
                className="w-full hover:bg-neutral-50 dark:bg-black dark:hover:bg-neutral-800 dark:border-neutral-700 text-left transition-colors p-4 border-b last:border-b-0 border-neutral-300 flex-col"
                onClick={join_team}
            >
                <div className="flex items-center w-full">
                    <h2 className="font-extrabold text-lg">{name}</h2>
                    <span className="row-span-2 ml-auto">{members.length}/3</span>
                </div>

                <ul className="flex flex-col row-start-2 row-end-2 list-disc list-inside">
                    {members.map((member, i) => <li key={i}>{member.name}</li>)}
                </ul>
            </button>
        );
    }

    if (!data) return <></>;

    return (
        <div className="flex flex-col gap-2">
            <button
                className="text-green-50 text-sm font-bold rounded-full bg-green-700 hover:bg-green-500 transition-colors px-4 py-2 mx-2 lg:mx-0"
                onClick={() => setShown(true)}>Register</button>

            <Modal shown={shown} onClose={() => setShown(false)}>
                <div className="bg-white border-neutral-300 rounded-md border overflow-hidden dark:border-neutral-700">
                    <h1 className="border-b border-neutral-300 p-4 w-full font-bold bg-neutral-50 dark:bg-neutral-800 dark:border-neutral-700">Click on a team to join it!</h1>
                    {data.map((team, i) => <JoinableTeamEntry key={i} {...team} />)}
                </div>
            </Modal>
        </div>
    );
}

function TeamStatus({ editable }: { editable: boolean }): JSX.Element {
    let id = useContext(CompetitionIDContext);

    let { data: team, error: _error } = useSWR<Team>(
        api_url(`/competitions/${id}/teams/me`),
        fetcher
    )

    const leave = async () => {
        await fetch(api_url(`/competitions/${id}/teams/leave`), {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            credentials: "include",
        });

        mutate(api_url(`/competitions/${id}/teams/me`));
    };

    if (!team) return <>{editable && <TeamJoiner />}</>;

    return (
        <div>
            <div className="border-y sm:rounded-md sm:border sm:m-2 lg:m-0 border-neutral-300 dark:border-neutral-700 bg-white overflow-hidden">
                <h1 className="text-lg font-extrabold p-2 border-b border-neutral-300 bg-neutral-50 dark:border-neutral-700 dark:bg-neutral-800">{team.name}</h1>

                {team.members.map((user, i) => <TeamUser key={i} {...user} />)}
                {editable && <button
                    onClick={leave}
                    className="p-2 bg-red-500 text-center w-full text-white hover:bg-red-700 transition-colors"
                >
                    Leave
                </button>}
            </div>
        </div>
    );
}

type Competition = {
    id: number,
    name: string,
    start: string,
    end: string
};

function CompetitionProblemList(): JSX.Element {
    const id = useContext(CompetitionIDContext);

    const { data: problems } = useSWR<Problem[]>(
        id ? api_url(`/problems?competition_id=${id}`) : null,
        fetcher
    );

    if (!problems) return <></>;

    return (
        <div className="flex flex-col gap-4 mb-12">
            <h1 className="text-2xl font-extrabold p-2 lg:px-0">Problems</h1>

            <ProblemList problems={problems} show_team_status={true} show_difficulty={false} />
        </div>
    );
}

export const CompetitionIDContext = createContext<number>(0);

const CompetitionPage: NextPage = () => {
    const { query, isReady } = useRouter();
    const id = isReady ? parseInt(query.id as string) : undefined;
    const router = useRouter();

    const { data: competition, error } = useSWR<Competition>(
        id ? api_url(`/competitions/${id}`) : null,
        fetcher
    );

    const start = new Date(competition?.start + 'Z').getTime();
    const end = new Date(competition?.end + 'Z').getTime();
    const now = new Date().getTime();

    const ended = end - now < 0;
    const started = start - now < 0;

    if (error) return <Error statusCode={404} />;

    if (!competition)
        return <><Navbar /></>;

    return (
        <CompetitionIDContext.Provider value={competition.id}>
            <Navbar />

            <Head>
                <title>{competition.name}</title>
            </Head>

            <div className="max-w-screen-xl lg:px-4 mx-auto flex flex-col mt-4 ">
                <div className="grid grid-rows-min-full grid-cols-1 lg:grid-rows-1 lg:grid-flow-col gap-4 lg:grid-cols-[2fr_1fr]">
                    <div className="flex flex-col gap-4 lg:col--2">
                        <TeamStatus editable={!ended} />

                        {!ended && (
                            started ?
                                <>
                                    <h1 className="text-2xl font-extrabold p-2 lg:px-0">Time Remaining</h1>
                                    <div>
                                        <Countdown to={new Date(competition.end + 'Z')} onFinal={() => router.replace(router.asPath)} />
                                    </div>
                                </>
                                :
                                <>
                                    <h1 className="text-2xl font-extrabold p-2 lg:px-0">Countdown</h1>
                                    <div>
                                        <Countdown to={new Date(competition.start + 'Z')} onFinal={() => router.replace(router.asPath)} />
                                    </div>
                                </>
                        )}


                        <CompetitionLeaderboard />
                    </div>

                    <div className="lg:col-start-1">
                        <h1 className="text-3xl font-extrabold p-2 lg:px-0">{competition.name}</h1>
                        <CompetitionProblemList />
                    </div>
                </div>
            </div>
        </CompetitionIDContext.Provider>
    );
}

export default CompetitionPage;
