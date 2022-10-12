import "../styles/globals.css";
import "prismjs/themes/prism.css";
import "katex/dist/katex.css";
// import "prismjs/themes/prism-dark.css";
import type { AppProps } from "next/app";
import Prism from "prismjs";
import "prismjs/components/prism-c.js";
import "prismjs/components/prism-cpp.js";
import ErrorBox from "../components/error-box";
import Modal from "../components/modal";
import { useSession } from "../utils/state";
import shallow from "zustand/shallow";
import { marked } from "marked";
import Head from "next/head";

// TODO: Defer loading of prism and marked until needed.
marked.setOptions({
  highlight: (code, lang) => {
    if (Prism.languages[lang]) {
      return Prism.highlight(code, Prism.languages[lang], lang);
    } else {
      return code;
    }
  },
});

function ErrorDisplay(): JSX.Element {
  const [error, shown, setError] = useSession(
    (state) => [state.error, state.errorShown, state.setError],
    shallow
  );

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
