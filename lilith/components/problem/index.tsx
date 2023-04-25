import dynamic from "next/dynamic";
import Error from "next/error";
import Head from "next/head";
import Link from "next/link";
import { createContext } from "react";
import useSWR from "swr";
import { Competition } from "../../pages/competitions";
import { api_url, fetcher } from "../../utils/fetcher";
import { useStore } from "../../utils/state";
import Tabbed from "../tabbed";
import CodeRunner from "./code-runner";
import Description from "./description";
import ProblemLeaderboard from "./leaderboard";
import SubmissionHistory from "./submission/history";
import TestContainer from "./submission/tests/container";
const Editor = dynamic(import("../../components/editor"), { ssr: false });

export const ProblemContext = createContext<Problem | undefined>(undefined);
export const ProblemIDContext = createContext<number | undefined>(undefined);

type ProblemViewProps = {
    id?: number,
    competitionId?: number
};

type Problem = {
    id: number;
    title: string;
    description: string;
    template: string;
    competition_id?: number;
};

export default function ProblemView({ id }: ProblemViewProps): JSX.Element {
    const { data, error } = useSWR<Problem>(
        id ? api_url(`/problems/${id}`) : null,
        fetcher
    );

    function ProblemEditorWrapper(): JSX.Element {
        // bad
        let content =
            useStore((state) =>
                id ? state.problemImpls[id] : undefined
            ) ??
            data?.template ??
            "";

        const setProblemImpl = useStore((state) => state.setProblemImpl);

        return (
            <div className="bg-white dark:bg-neutral-900 h-full ring-1 md:ring-0 ring-neutral-300 dark:ring-neutral-700">
                <Editor
                    language="cpp"
                    value={content}
                    onChange={(text, _event) => {
                        if (id) setProblemImpl(id, text);
                    }}
                />
            </div>
        );
    }

    function BackToCompetition({ competitionId }: { competitionId?: number }): JSX.Element {
        const { data, error } = useSWR<Competition>(
            competitionId ? api_url(`/competitions/${competitionId}`) : null,
            fetcher
        );

        if (!competitionId || error || !data) return <></>;

        return (
            <div className="border-neutral-300 dark:border-neutral-700 border-b p-2">
                <h1 className="font-bold text-lg">{data.name}</h1>

                <Link href={`/competitions/${competitionId}`}>
                    <a className="text-blue-700 hover:underline dark:text-blue-500">
                        ‹ Back to overview.
                    </a>
                </Link>
            </div>
        );
    }

    if (error) return <Error statusCode={404} />;

    return (
        <ProblemIDContext.Provider value={id}>
            <ProblemContext.Provider value={data}>
                <Head>
                    <title>{data?.title}</title>
                </Head>

                <div className="grid grid-rows-[min-content_min-content_40vh_minmax(0,1fr)] md:grid-cols-[400px_minmax(0,1fr)] lg:grid-cols-[500px_minmax(0,1fr)] md:grid-rows-full-min h-full">
                    <div className="md:border-r border-neutral-300 dark:border-neutral-700 row-span-2 flex flex-col border-b md:border-b-0">
                        {data?.competition_id && <BackToCompetition competitionId={data?.competition_id} />}

                        <TestContainer />

                        <Tabbed
                            titles={["Description", "History", "Leaderboard"]}
                            className="h-full overflow-y-auto"
                        >
                            <Description />
                            <SubmissionHistory />
                            <ProblemLeaderboard />
                        </Tabbed>
                    </div>

                    <ProblemEditorWrapper />
                    <CodeRunner />
                </div>
            </ProblemContext.Provider>
        </ProblemIDContext.Provider>
    );
}
