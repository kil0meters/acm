import produce from "immer";
import { NextPage } from "next";
import dynamic from "next/dynamic";
import Head from "next/head";
import { useRouter } from "next/router";
import { useState } from "react";
import useSWR from "swr";
import shallow from "zustand/shallow";
import LoadingButton from "../../components/loading-button";
import Navbar from "../../components/navbar";
import DescriptionEditor from "../../components/problem-editor/description-editor";
import TestsEditor from "../../components/problem-editor/tests-editor";
import { FunctionValue, FunctionTypeDisplay, FunctionType } from "../../components/problem/submission/tests";
import Tabbed from "../../components/tabbed";
import { api_url, fetcher } from "../../utils/fetcher";
import { useAdminStore, useSession, useStore } from "../../utils/state";
import { Competition } from "../competitions";
const Editor = dynamic(import("../../components/editor"), { ssr: false });

function ArgumentsEditor(): JSX.Element {
    const [testFormat, setTestFormat] = useAdminStore(
        (state) => [state.problemTestFormat, state.setProblemTestFormat],
        shallow
    );

    const [containerType, setContainerType] = useState("Single");
    const [dataType, setDataType] = useState("Int");

    const getItem = () => {
        let defaultValue: any;
        if (dataType == "Int" || dataType == "Long" || dataType == "Float" || dataType == "Double") {
            defaultValue = 0;
        } else if (dataType == "String" || dataType == "Char") {
            defaultValue = "a";
        }

        let item: any = {};
        if (containerType == "Grid" || containerType == "Graph" || containerType == "List") {
            item[containerType] = [];
        }
        else {
            item[containerType] = defaultValue;
        }

        let outerItem: any = {};
        outerItem[dataType] = item;

        return outerItem as FunctionValue;
    }

    const getItemType = () => {
        let item: any = {};
        item[dataType] = containerType;

        return item as FunctionType;
    }

    const addItem = () => {
        setTestFormat(produce(testFormat, (format) => {
            format.arguments.push(getItem());
        }));
    };

    const setOutput = () => {
        setTestFormat(produce(testFormat, (format) => {
            format.return_type = getItemType();
        }));
    };

    const removeItem = () => {
        setTestFormat(produce(testFormat, (format) => {
            format.arguments.pop();
        }));
    };

    return (
        <div className="p-4 flex flex-col gap-4">
            <div className="flex gap-2">
                <span className="my-auto">fname: </span>
                <input value={testFormat.name}
                    onChange={(e) => {
                        setTestFormat(produce(testFormat, (format) => {
                            format.name = e.target.value;
                        }));
                    }}
                    className="p-2 bg-white border border-neutral-300 rounded-md"></input>
            </div>

            <div className="grid grid-cols-2 border gap-2 bg-white p-4 rounded-md border-neutral-300 w-auto">
                <span>Container:</span>
                <select
                    value={containerType}
                    onChange={e => setContainerType(e.currentTarget.value)}
                >
                    <option>Grid</option>
                    <option>Graph</option>
                    <option>List</option>
                    <option>Single</option>
                </select>

                <span>Data Type:</span>
                <select
                    value={dataType}
                    onChange={e => setDataType(e.currentTarget.value)}
                >
                    <option>Int</option>
                    <option>Long</option>
                    <option>Float</option>
                    <option>Double</option>
                    <option>String</option>
                    <option>Char</option>
                </select>

                <div className="col-span-2 flex">
                    <button
                        onClick={addItem}
                        className="border-green-300 py-1 px-6 text-sm bg-green-700 hover:bg-green-600 transition-colors text-green-50 rounded-full border mx-auto">Add</button>
                    <button
                        onClick={setOutput}
                        className="border-green-300 py-1 px-6 text-sm bg-green-700 hover:bg-green-600 transition-colors text-green-50 rounded-full border mx-auto">Output</button>
                    <button
                        onClick={removeItem}
                        className="border-red-300 py-1 px-6 text-sm bg-red-700 hover:bg-red-600 transition-colors text-red-50 rounded-full border mx-auto">Remove</button>
                </div>
            </div>

            <div className="flex flex-col gap-4">
                <span className="text-2xl">Output:</span>
                <div>{JSON.stringify(testFormat.return_type)}</div>

                <span className="text-2xl">Input:</span>
                <div className="flex flex-col gap-2">
                    {testFormat.arguments.map((data, i) =>
                        <FunctionTypeDisplay key={i} data={data} />
                    )}
                </div>
            </div>
        </div>
    );
}

function TemplateEditor(): JSX.Element {
    const [template, setTemplate] = useAdminStore(
        (state) => [state.problemTemplate, state.setProblemTemplate],
        shallow
    );

    return (
        <div className="grid grid-rows-2 grid-cols-1 xl:grid-rows-1 xl:grid-cols-3">
            <div className="xl:col-span-2 border-b border-neutral-300 dark:border-neutral-700 xl:border-b-0 xl:border-r">
                <Editor
                    language="cpp"
                    value={template}
                    onChange={(value, _event) => setTemplate(value)}
                />
            </div>
            <ArgumentsEditor />
        </div>
    );
}

function TitleEditor(): JSX.Element {
    const [title, setTitle] = useAdminStore((state) => [state.problemTitle, state.setProblemTitle], shallow);

    return (
        <div className="border-b lg:border-0 bg-white dark:bg-black border-neutral-300 dark:border-neutral-700 flex flex-col gap-2 p-2">
            <label className="font-bold">Title</label>
            <input
                className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
                onChange={(event) => setTitle(event.currentTarget.value)}
                value={title}
                placeholder="Title"
            />
        </div>
    );
}

function Scheduler(): JSX.Element {
    const [dateShown, setDateShown] = useAdminStore((state) => [state.problemDateShown, state.setProblemDateShown]);
    const [publishTime, setPublishTime] = useAdminStore((state) => [state.problemPublishTime, state.setProblemPublishTime]);

    function toggleDateShown() {
        if (dateShown) {
            setPublishTime(undefined);
        }

        setDateShown(!dateShown);
    }

    let time = publishTime ? new Date(publishTime) : new Date();
    time.setMinutes(time.getMinutes() - time.getTimezoneOffset());
    let formattedTime = time.toISOString().substring(0, 16);

    return (
        <div className="px-4 flex gap-4 items-center border-r border-neutral-300 h-full">
            <label>Custom publish time: </label>
            <input type="checkbox" defaultChecked={dateShown} onChange={toggleDateShown} />
            {dateShown && <input
                className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
                type="datetime-local"
                value={formattedTime}
                onChange={(e) => {
                    setPublishTime(e.target.value);
                }}
            />}
        </div>
    );
}

function CompetitionPicker(): JSX.Element {
    const { data } = useSWR<Competition[]>(api_url("/competitions"), fetcher);
    const [competitionId, setCompetitionId] = useAdminStore((state) => [state.problemCompetitionId, state.setProblemCompetitionId]);

    return (
        <div className="px-4 border-neutral-300 border-r h-full flex gap-4 items-center">
            <label>Competition: </label>
            <select
                value={competitionId}
                onChange={(e) => {
                    let newValue = parseInt(e.target.value);
                    setCompetitionId(newValue == -1 ? undefined : newValue);
                }}
                className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
            >
                <option value={-1}>None (default)</option>
                {data && data.map((competition, i) => <option key={i} value={competition.id}>{competition.name}</option>)}
            </select>

        </div>
    );
}

function SubmitButton(): JSX.Element {
    const setError = useSession((state) => state.setError);
    const [loading, setLoading] = useState(false);
    const router = useRouter();
    const clearProblemEditor = useAdminStore((state) => state.clearProblemCreation);

    const submitProblem = async () => {
        const {
            problemTitle: title,
            problemDescription: description,
            problemReference: reference,
            problemTemplate: template,
            problemTests: tests,
            problemPublishTime: publish_time,
            problemRuntimeMultiplier: runtime_multiplier,
            problemCompetitionId: competition_id
        } = useAdminStore.getState();

        // TODO: Look into zod validator
        if (!title || !description || !reference || !template || tests.length === 0) {
            setError("One or more required fields is empty.", true);
            return;
        }

        setLoading(true);
        try {
            const res = await (await fetch(api_url("/problems/new"), {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                credentials: "include",
                body: JSON.stringify({
                    title,
                    description,
                    reference,
                    template,
                    tests,
                    publish_time: publish_time ? new Date(publish_time).toISOString().slice(0, -1) : undefined,
                    runtime_multiplier,
                    competition_id
                }),
            })).json();

            if (res.error) {
                setError(res.error, true);
            } else {
                clearProblemEditor();
                router.push(`/problems/${res.id}`);
            }

        } catch (e) {
            console.log(e);
            setError("Network error.", true);
        }
        setLoading(false);
    };

    return (
        <LoadingButton
            className="lg:ml-auto rounded-full p-4 bg-green-600 hover:bg-green-500 text-green-50 transition-colors mx-2 mb-8 lg:m-0 lg:rounded-none lg:h-full"
            onClick={submitProblem}
            loading={loading}
        >
            Submit
        </LoadingButton>
    )
}

const ProblemEditorPage: NextPage = () => {
    return (
        <div className="grid grid-rows-min-full grid-cols-full w-screen h-screen">
            <Navbar />

            <Head>
                <title>Problem Editor</title>
            </Head>

            <div className="flex flex-col gap-2 lg:gap-0 lg:grid lg:grid-cols-[450px_minmax(0,1fr)] lg:grid-rows-full-min">
                <div className="grid grid-rows-min-full grid-cols-full gap-2 lg:gap-0 lg:border-r border-neutral-300 dark:border-neutral-700 row-span-2">
                    <TitleEditor />
                    <DescriptionEditor />
                </div>

                <Tabbed
                    className="border-y border-neutral-300 dark:border-neutral-700 lg:border-0"
                    titles={["Template", "Tests"]}
                >
                    <TemplateEditor />
                    <TestsEditor />
                </Tabbed>

                <div className="border-t border-neutral-300 dark:border-neutral-700 flex flex-col items-center lg:bg-white dark:lg:bg-black lg:flex-row">
                    <Scheduler />
                    <CompetitionPicker />
                    <SubmitButton />
                </div>
            </div>
        </div>
    );
};

export default ProblemEditorPage;
