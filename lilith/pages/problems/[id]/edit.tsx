import type { NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect, useRef, useState } from "react";
import { api_url, fetcher } from "../../../utils/fetcher";
import useSWR from "swr";
import { Problem } from "..";
import dynamic from "next/dynamic";
import Navbar from "../../../components/navbar";
import { marked } from "marked";
import renderLatex from "../../../utils/latex";
import Tabbed from "../../../components/tabbed";
import Head from "next/head";
const Editor = dynamic(import("../../../components/editor"), { ssr: false });

function RenderedDescription({ description }: { description: string }): JSX.Element {
    const content = useRef<HTMLDivElement>(null);

    const descriptionHtml = marked.parse(description);

    useEffect(() => {
        if (content.current) {
            renderLatex(content.current);
        }
    }, [descriptionHtml])

    return (
        <div
            ref={content}
            className="prose prose-neutral dark:prose-invert p-4 max-h-full overflow-auto border-neutral-300 dark:border-neutral-700 border-x"
            dangerouslySetInnerHTML={{ __html: descriptionHtml }}
        />
    )
}

const ProblemEditor: NextPage = () => {
    const { query, isReady } = useRouter();
    const id = isReady ? parseInt(query.id as string) : undefined;
    const [difficulty, setDifficulty] = useState("Easy");
    const [description, setDescription] = useState("");
    const [template, setTemplate] = useState("");
    const [title, setTitle] = useState("");
    const [runtimeMultiplier, setRuntimeMultiplier] = useState(1.0);
    const [visible, setVisible] = useState(false);
    const router = useRouter();

    const { data } = useSWR<Problem>(
        id ? api_url(`/problems/${id}`) : null,
        fetcher
    );

    useEffect(() => {
        if (data) {
            setDescription(data.description);
            setTitle(data.title);
            setTemplate(data.template);
            setVisible(data.visible);
            setDifficulty(data.difficulty!);
            setRuntimeMultiplier(data.runtime_multiplier);
        }
    }, [data]);

    const updateDifficulty = (e: any) => {
        setDifficulty(e.target.value);
    };

    const submit = async () => {
        if (!id) return;

        await fetch(api_url(`/problems/${id}/edit`), {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",
            body: JSON.stringify({
                title,
                description,
                template,
                visible,
                difficulty,
                runtime_multiplier: runtimeMultiplier
            })
        });

        router.push(`/problems/${id}`);
    };

    if (!data) return <></>;

    return (
        <div className="h-screen w-screen grid grid-rows-min-full grid-cols-full">
            <Navbar />

            <Head>
                <title>Editing {data.title}</title>
            </Head>

            <div className="grid grid-cols-4 grid-rows-[minmax(0,1fr)] h-full">
                <div className="col-span-2 grid grid-rows-2">
                    <div className="border-neutral-300 dark:border-neutral-700 border-b">
                        <Editor language="markdown" value={description} onChange={(text) => setDescription(text)} />
                    </div>
                    <Editor language="cpp" value={template} onChange={(text) => setTemplate(text)} />
                </div>

                <RenderedDescription description={description} />

                <div className="flex flex-col">
                    <div className="p-4">
                        <h1 className="font-bold mb-2">Title</h1>
                        <input
                            className="w-full border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
                            onChange={(event) => setTitle(event.currentTarget.value)}
                            value={title}
                            placeholder="Title"
                        />
                    </div>

                    <div className="p-4">
                        <h1 className="font-bold mb-2">Tags</h1>
                    </div>

                    <fieldset className="p-4">
                        <h1 className="font-bold mb-2">Difficulty</h1>

                        <div className="flex gap-2">
                            <input
                                id="easy"
                                name="difficulty"
                                value="Easy"
                                type="radio"
                                onChange={updateDifficulty}
                                checked={difficulty == "Easy"}
                            />
                            <label htmlFor="easy">Easy</label>
                        </div>

                        <div className="flex gap-2">
                            <input
                                id="medium"
                                name="difficulty"
                                value="Medium"
                                type="radio"
                                onChange={updateDifficulty}
                                checked={difficulty == "Medium"}
                            />
                            <label htmlFor="medium">Medium</label>
                        </div>

                        <div className="flex gap-2">
                            <input
                                id="hard"
                                name="difficulty"
                                value="Hard"
                                type="radio"
                                onChange={updateDifficulty}
                                checked={difficulty == "Hard"}
                            />
                            <label htmlFor="hard">Hard</label>
                        </div>
                    </fieldset>

                    <div className="p-4">
                        <h1 className="font-bold mb-2">Visibility</h1>

                        <div className="flex gap-2">
                            <input type="checkbox" id="visible" checked={visible} onClick={() => setVisible(!visible)} />
                            <label htmlFor="visible">Visible</label>
                        </div>

                    </div>

                    <div className="p-4">
                        <label className="font-bold mb-2">Runtime Multiplier</label>

                        <div className="flex gap-2 align-end">
                            <output className="self-center">{
                                Intl.NumberFormat('en-US', {
                                    minimumFractionDigits: 1
                                }).format(runtimeMultiplier)
                            }</output>
                            <input
                                type="range"
                                min="1"
                                max="5"
                                step="0.1"
                                value={runtimeMultiplier}
                                onInput={(e) => setRuntimeMultiplier(parseFloat((e.target as HTMLInputElement).value))}
                            />
                        </div>
                    </div>

                    <button onClick={submit} className="mt-auto mb-4 bg-blue-600 text-blue-50 py-2 mx-4 rounded-full hover:bg-blue-500 transition-colors">
                        Update
                    </button>
                </div>
            </div>
        </div>
    );
}

export default ProblemEditor;
