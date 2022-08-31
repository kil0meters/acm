import { useStore } from "../utils/state";

export default function EditorPreferences() {
  const [vimEnabled, setVimEnabled, editorTheme, setEditorTheme] = useStore((state) => [state.vimEnabled, state.setVimEnabled, state.editorTheme, state.setEditorTheme]);

  return(
    <div className="bg-white dark:bg-black p-4 rounded shadow-md border border-neutral-300 dark:border-neutral-700">
      <h1 className="font-extrabold text-2xl mb-4">Settings</h1>

      <div className="grid grid-cols-full-min">
        <span>Vim Mode</span>
        <input
          className="justify-self-end"
          type="checkbox"
          checked={vimEnabled}
          onChange={() => setVimEnabled(!vimEnabled)}
        />

        <span className="self-center">Theme</span>
        <select
          className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
          value={editorTheme}
          // @ts-ignore
          onChange={event => setEditorTheme(event.currentTarget.value)}
        >
          <option value="system">System</option>
          <option value="light">Light</option>
          <option value="dark">Dark</option>
        </select>
      </div>
    </div>
  );
}
