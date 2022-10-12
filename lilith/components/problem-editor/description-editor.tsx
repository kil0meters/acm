import { marked } from "marked";
import dynamic from "next/dynamic";
import { useState } from "react";
import shallow from "zustand/shallow";
import { useAdminStore } from "../../utils/state";
const Editor = dynamic(import("../../components/editor"), { ssr: false });

export default function DescriptionEditor(): JSX.Element {
  const [description, setDescription] = useAdminStore(
    (state) => [state.problemDescription, state.setProblemDescription],
    shallow
  );
  const [preview, setPreview] = useState(false);

  return (
    <div className="grid grid-rows-min-full grid-cols-full border-y border-neutral-300 dark:border-neutral-700 bg-white dark:bg-black">
      <div className="flex items-center border-b border-neutral-300 dark:border-neutral-700 p-2">
        <span className="font-bold">Problem Description</span>
        <button
          className="ml-auto rounded-full bg-blue-700 hover:bg-blue-500 text-sm font-bold text-blue-50 transition-colors px-4 py-2"
          onClick={() => setPreview(!preview)}
        >
          {preview ? "hide preview" : "show preview"}
        </button>
      </div>

      {preview ? (
        <div
          className="prose prose-neutral p-2 min-h-[40vh] dark:prose-invert overflow-auto"
          dangerouslySetInnerHTML={{ __html: marked.parse(description) }}
        />
      ) : (
        <Editor
          className="min-h-[40vh]"
          language="cpp"
          value={description}
          onChange={(value, _event) => setDescription(value)}
        />
      )}
    </div>
  );
}
