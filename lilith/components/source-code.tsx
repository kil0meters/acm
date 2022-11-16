import { useRef, useEffect } from "react";

export default function SourceCodeBlock({ text }: { text: string }): JSX.Element {
  const preRef = useRef<HTMLPreElement>(null);

  useEffect(() => {
    const darkMode = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;

    const highlight = async () => {
      const monaco = await import("monaco-editor");
      monaco.editor.colorizeElement(preRef.current!, {
        theme: darkMode ? "vs-dark" : "vs"
      });
    }
    highlight();
  }, [preRef])

  return (
    <pre
      lang="cpp"
      ref={preRef}
      className="rounded-md bg-blue-50 bg-neutral-100 dark:bg-stone-900 p-2 overflow-auto border border-blue-200 dark:border-slate-700"
    >
      {text}
    </pre>
  )
}
