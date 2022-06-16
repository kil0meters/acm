import { useEffect, useRef, useState } from "react";

import * as monaco from "monaco-editor";
// @ts-ignore
import { initVimMode } from "monaco-vim";

type EditorProps = {
  language: "cpp" | "markdown";
  onChange: (
    text: string,
    event: monaco.editor.IModelContentChangedEvent
  ) => void;
  className?: string;
  value: string;
};

export default function Editor({
  onChange,
  value,
  language,
  className,
}: EditorProps): JSX.Element {
  const [editor, setEditor] = useState<
    monaco.editor.IStandaloneCodeEditor | undefined
  >(undefined);
  const editorRef = useRef<HTMLDivElement>(null);
  const statusBarRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (editor && editor.getValue() != value) {
      editor.setValue(value);
    }
  }, [value]);

  useEffect(() => {
    const editor = monaco.editor.create(editorRef.current!, {
      fontSize: 18,
      language,
      value,
      cursorSmoothCaretAnimation: true,
      extraEditorClassName: `h-full ${className}`,
      automaticLayout: true,
      minimap: {
        enabled: false,
      },
    });

    setEditor(editor);

    const subscription = editor.onDidChangeModelContent((event) => {
      onChange(editor.getValue(), event);
    });

    const vimMode = initVimMode(editor, statusBarRef.current);

    return () => {
      editor.dispose();
      vimMode.dispose();

      const model = editor.getModel();
      if (model) {
        model.dispose();
      }

      if (subscription) {
        subscription.dispose();
      }
    };
  }, []);

  return (
    <div className="h-full grid grid-rows-full-min grid-cols-full">
      <div ref={editorRef} />
      <div ref={statusBarRef} />
    </div>
  );
}
