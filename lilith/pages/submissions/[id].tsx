import { NextPage } from "next";
import Error from "next/error";
import Head from "next/head";
import Link from "next/link";
import { useRouter } from "next/router";
import useSWR, { mutate } from "swr";
import Navbar from "../../components/navbar";
import SubmissionFeedback from "../../components/problem/submission";
import { api_url, fetcher } from "../../utils/fetcher";
import { Submission, User, useStore } from "../../utils/state";
import { timeFormat } from "../../utils/time";
import { Problem } from "../problems";
import { useEffect, useRef } from "react";
import SourceCodeBlock from "../../components/source-code";

function InvalidateButton({ id }: { id?: number }): JSX.Element {
    const submit = async () => {
        await fetch(api_url(`/submissions/${id}/invalidate`), {
            credentials: "include",
        });

        mutate(api_url(`/submissions/${id}`));
    }

    return (
        <div className="w-full px-4 lg:px-0">
            <button
                onClick={submit}
                className="w-full mt-4 rounded-full bg-red-600 hover:bg-red-700 px-4 py-2 text-red-50 transition-colors">
                Invalidate
            </button>
        </div>
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
        <div className="w-full px-4 lg:px-0">
            <button
                onClick={submit}
                className="w-full mt-4 rounded-full bg-blue-600 hover:bg-blue-700 px-4 py-2 text-red-50 transition-colors">
                Validate
            </button>
        </div>
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

function ProblemTitle({ id }: { id: number }): JSX.Element {
    const { data } = useSWR<Problem>(
        api_url(`/problems/${id}`),
        fetcher
    );

    if (!data) return (
        <div className="animate-pulse bg-neutral-300 dark:bg-neutral-700 h-9 w-[75%] rounded-md"></div>
    );

    return (
        <h1 className="text-3xl font-extrabold">{data.title}</h1>
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

        <Head>
            <title>Submission {id}</title>
        </Head>

        <div className="grid grid-cols-[minmax(0,1fr)] grid-rows-min-full gap-4 lg:grid-cols-[minmax(0,1fr)_320px] mt-4 md:flex-row max-w-screen-lg md:mx-auto md:mt-8">
            <div className="p-4 border-neutral-300 bg-white dark:bg-black dark:border-neutral-700 border-y lg:border lg:rounded-md flex flex-col gap-4 max-w row-start-2 lg:row-start-1">
                <div className="flex flex-col gap-1">
                    <div className="flex flex-col">
                        <ProblemTitle id={submission.problem_id} />
                    </div>
                    <span className="text-sm text-neutral-500">
                        <UserInfo id={submission.user_id} />
                        {" • "}
                        {timeFormat(submission.time + 'Z')}
                    </span>
                </div>

                <SourceCodeBlock text={submission.code} />

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
            <div className="col-row-1 relative">
                <div className="sticky top-20">
                    <div className="border-y lg:border lg:rounded-md border-neutral-300 dark:border-neutral-700 overflow-hidden">
                        <SubmissionFeedback inProblemView={false} {...submission} />
                    </div>

                    {(user && (user.auth == "ADMIN" || user.auth == "OFFICER")) && <>
                        <ValidateButton id={id} />
                        <InvalidateButton id={id} />
                    </>}
                </div>
            </div>
        </div>
    </>;
};

export default SubmissionPage;
