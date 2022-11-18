import { useState } from "react";

type TabbedProps = {
  titles: string[];
  children: JSX.Element[];
  className?: string;
};

export default function Tabbed({
  titles,
  children,
  className,
}: TabbedProps): JSX.Element {
  const [focusedWindow, setFocusedWindow] = useState(0);

  return (
    <div
      className={`bg-white grid grid-rows-min-full grid-cols-full dark:bg-black ${className}`}
    >
      <div className="flex border-neutral-300 dark:border-neutral-700 border-b">
        {titles.map((title, index) => {
          let focused =
            index == focusedWindow
              ? "bg-neutral-300 hover:bg-neutral-100 dark:bg-neutral-700 dark:hover:bg-neutral-600 "
              : "bg-neutral-200 hover:bg-neutral-50 dark:bg-neutral-800 dark:hover:bg-neutral-700 ";

          return (
            <button
              key={index}
              className={`px-4 py-2 transition-colors border-neutral-300 dark:border-neutral-700 border-r ${focused}`}
              onClick={() => setFocusedWindow(index)}
            >
              {title}
            </button>
          );
        })}
      </div>

      {children[focusedWindow]}
    </div>
  );
}
