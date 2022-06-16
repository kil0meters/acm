import dynamic from "next/dynamic";
import Error from "next/error";
import { createContext } from "react";
import useSWR from "swr";
import { api_url, fetcher } from "../../utils/fetcher";
import { useStore } from "../../utils/state";
import Tabbed from "../tabbed";
import CodeRunner from "./code-runner";
import Description from "./description";
import SubmissionHistory from "./submission/history";
import TestContainer from "./submission/tests/container";
const Editor = dynamic(import("../../components/editor"), { ssr: false });

export const ProblemContext = createContext<Problem | undefined>(undefined);
export const ProblemIDContext = createContext<number | undefined>(undefined);

type ProblemViewProps = {
  id?: number,
};

type Problem = {
  id: number;
  title: string;
  description: string;
  template: string;
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
        id ? state.problems[id]?.implementation : undefined
      ) ??
      data?.template ??
      "";

    const setProblemImpl = useStore((state) => state.setProblemImpl);

    return (
      <div className="bg-white dark:bg-neutral-900 h-full">
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

  if (error) return <Error statusCode={404} />;

  return (
    <ProblemIDContext.Provider value={id}>
      <ProblemContext.Provider value={data}>
        <div className="md:grid md:grid-cols-[400px_minmax(0,1fr)] lg:grid-cols-[500px_minmax(0,1fr)] md:grid-rows-full-min md:h-full">
          <div className="md:border-r border-neutral-300 dark:border-neutral-700 pt-2 md:p-0 row-span-2 flex flex-col">
            <TestContainer />

            <Tabbed
              titles={["Description", "History"]}
              className="h-full overflow-y-auto"
            >
              <Description />
              <SubmissionHistory />
            </Tabbed>
          </div>

          <ProblemEditorWrapper />
          <CodeRunner />
        </div>
      </ProblemContext.Provider>
    </ProblemIDContext.Provider>
  );
}
