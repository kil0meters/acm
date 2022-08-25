import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useStore } from "../utils/state";

type NavbarLinkProps = {
  className?: string;
  href: string;
  children: React.ReactNode;
  onClick?: React.MouseEventHandler<HTMLAnchorElement>;
};

function NavbarLink({
  className,
  href,
  children,
  onClick,
}: NavbarLinkProps): JSX.Element {
  const defaultStyles =
    "font-bold text-lg self-start md:self-center hover:text-neutral-600 dark:hover:text-neutral-400 transition-colors ease-in-out md:block";

  return (
    <Link href={href}>
      <a onClick={onClick} className={className + " " + defaultStyles}>
        {children}
      </a>
    </Link>
  );
}

export default function Navbar(): JSX.Element {
  const [hiddenStyle, setHiddenStyle] = useState("hidden");
  const username = useStore((state) => state.user?.username);
  const [isComponentMounted, setIsComponentMounted] = useState(false);
  const logOut = useStore((store) => store.logOut);
  const router = useRouter();

  useEffect(() => setIsComponentMounted(true), []);

  function handleClick() {
    if (hiddenStyle === "") {
      setHiddenStyle("hidden");
    } else {
      setHiddenStyle("");
    }
  }

  let sidebar = undefined;

  const oauth_url = process.env.NODE_ENV == "development"
    ? "https://discord.com/api/oauth2/authorize?client_id=984742374112624690&redirect_uri=http%3A%2F%2Flocalhost%3A3000%2Fauth%2Fdiscord&response_type=token&scope=identify"
    : "https://discord.com/api/oauth2/authorize?client_id=984742374112624690&redirect_uri=https%3A%2F%2Facm.meters.sh%2Fauth%2Fdiscord&response_type=token&scope=identify";

  if (isComponentMounted) {
    if (!username) {
      sidebar = (
        <>
          <NavbarLink
            className={`md:ml-auto bg-[#5865F2] text-white hover:text-white px-4 py-1 rounded hover:bg-[#6f7af2] ${hiddenStyle}`}
            href={oauth_url}>
            Log in with Discord
          </NavbarLink>
        </>
      );
    } else {
      sidebar = (
        <>
          <NavbarLink
            className={`md:ml-auto ${hiddenStyle}`}
            href={`/user/${username}`}
          >
            {"Account"}
          </NavbarLink>
          <NavbarLink
            className={hiddenStyle}
            href="#"
            onClick={(event) => {
              event.preventDefault();
              logOut();
              router.push("/");
            }}
          >
            Sign out
          </NavbarLink>
        </>
      );
    }
  }

  return (
    <div className="sticky top-0 z-50 w-full">
      <div className="p-4 flex flex-col gap-4 md:flex-row bg-white/90 dark:bg-black/90 backdrop-blur-lg border-neutral-300 dark:border-neutral-700 border-b">
        <div className="flex">
          <Link href="/">
            <a className="font-extrabold text-2xl hover:text-neutral-600 transition-colors ease-in-out flex items-center dark:hover:text-neutral-400">
              Chico ACM
            </a>
          </Link>

          <button
            onClick={handleClick}
            className="md:hidden ml-auto rounded-full p-2 px-5 bg-blue-700 text-blue-50 hover:bg-blue-500 transition-colors"
          >
            Menu
          </button>
        </div>

        <NavbarLink className={hiddenStyle} href="/problems">
          Problems
        </NavbarLink>
        {/*
        We're hiding this for now, waiting for the feature to be more mature.

        <NavbarLink className={hiddenStyle} href="/meetings">
           Meetings
        </NavbarLink>
        */}
        <NavbarLink className={hiddenStyle} href="/leaderboard">
          Leaderboard
        </NavbarLink>

        {sidebar}
      </div>
    </div>
  );
}
