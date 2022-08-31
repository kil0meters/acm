export default function Footer(): JSX.Element {
  return (
    <div className="h-52 w-full bg-neutral-100 border-neutral-300 dark:border-neutral-600 border-t flex items-center justify-center dark:bg-neutral-800 mt-auto">
      <div className="flex flex-col gap-2 justify-center">
      <span className="select-none">Check out our <a className="text-blue-500 hover:text-blue-700 hover:underline transition-colors" href="https://discord.gg/ZNUxzVketd">Discord</a> server!</span>

      <span className="select-none text-center">made with 🐌</span>
      </div>
    </div>
  );
}
