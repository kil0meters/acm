import "../styles/globals.css";
import "katex/dist/katex.css";
import type { AppProps } from "next/app";
import ErrorBox from "../components/error-box";
import Modal from "../components/modal";
import { useSession } from "../utils/state";
import shallow from "zustand/shallow";
import Head from "next/head";

// marked.setOptions({
//   highlight: (code, lang, callback) => {
//     const highlight = async () => {
//       const monaco = await import("monaco-editor");
//       monaco.editor.colorize(code, lang, {}).then((code) => {
//         callback!(undefined, code);
//       })
//     }
//     highlight();
//   },
// });

function ErrorDisplay(): JSX.Element {
    const [error, shown, setError] = useSession(
        (state) => [state.error, state.errorShown, state.setError],
        shallow
    );

    console.log(`displaying error: ${JSON.stringify(error)}`);

    return (
        <Modal shown={shown} onClose={() => setError(error, false)}>
            <ErrorBox>{error}</ErrorBox>
        </Modal>
    );
}

export default function MyApp({ Component, pageProps }: AppProps) {
    return (
        <>
            <Head>
                <link rel="icon" type="image/png" href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABAQMAAAAl21bKAAAAA1BMVEV0Ysuv/+9LAAAACklEQVQI12NgAAAAAgAB4iG8MwAAAABJRU5ErkJggg==" />
            </Head>
            <Component {...pageProps} />
            <ErrorDisplay />
        </>
    );
}
