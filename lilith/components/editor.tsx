import { useEffect, useRef, useState } from "react";

import * as monaco from "monaco-editor";
// @ts-ignore
import { initVimMode } from "monaco-vim";
import { useStore } from "../utils/state";

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
    const [vimEnabled, editorTheme, editorFontSize] = useStore((state) => [state.vimEnabled, state.editorTheme, state.editorFontSize]);

    let theme: string;

    if (editorTheme == "system") {
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            theme = "vs-dark";
        } else {
            theme = "vs";
        }
    } else if (editorTheme == "light") {
        theme = "vs";
    } else {
        theme = "vs-dark";
    }

    useEffect(() => {
        if (editor && editor.getValue() != value) {
            editor.setValue(value);
        }
    }, [value]);

    useEffect(() => {
        const editor = monaco.editor.create(editorRef.current!, {
            fontSize: editorFontSize,
            language,
            theme,
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

        let vimMode: any = null;

        if (vimEnabled) {
            vimMode = initVimMode(editor, statusBarRef.current);
        }

        return () => {
            editor.dispose();
            vimMode?.dispose();

            const model = editor.getModel();
            if (model) {
                model.dispose();
            }

            if (subscription) {
                subscription.dispose();
            }
        };
    }, [vimEnabled, editorTheme, editorFontSize]);

    return (
        <div className="h-full grid grid-rows-full-min grid-cols-full">
            <div ref={editorRef} />
            {vimEnabled && <div className="border-neutral-300 dark:border-neutral-700 border-t font-mono" ref={statusBarRef} />}
        </div>
    );
}
