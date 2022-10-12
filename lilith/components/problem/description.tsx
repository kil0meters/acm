import { marked } from "marked";
import { useContext, useEffect, useRef } from "react";
import { ProblemContext } from ".";
import renderMathInElement from "katex/contrib/auto-render";
import renderLatex from "../../utils/latex";

export default function Description(): JSX.Element {
  const problem = useContext(ProblemContext);
  const content = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (content.current) {
      renderLatex(content.current);
    }
  });

  if (!problem) {
    return (
      <div className="animate-pulse p-4">
        <h1 className="rounded bg-neutral-200 w-64 h-10 mb-4"></h1>
        <p className="rounded bg-neutral-200 w-full h-3 mb-2"></p>
        <p className="rounded bg-neutral-200 w-full h-3 mb-2"></p>
        <p className="rounded bg-neutral-200 w-full h-3 mb-2"></p>
        <p className="rounded bg-neutral-200 w-[70%] h-3"></p>
      </div>
    );
  }

  let description = marked.parse(problem.description);

  return (
    <div className="grow bg-white dark:bg-black p-4 h-full max-h-full overflow-y-auto">
      <h1 className="text-4xl font-extrabold mb-4">{problem.title}</h1>

      <div
        ref={content}
        className="prose prose-neutral dark:prose-invert"
        dangerouslySetInnerHTML={{ __html: description }}
      />
    </div>
  );
}
