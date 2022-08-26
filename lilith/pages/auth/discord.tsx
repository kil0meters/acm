import { NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { api_url } from "../../utils/fetcher";
import { useStore } from "../../utils/state";

const DiscordAuth: NextPage = () => {
  const logIn = useStore((state) => state.logIn);
  const router = useRouter();
  const redirect_uri = process.env.NODE_ENV == "production"
    ? "https://acm.meters.sh/auth/discord"
    : "http://localhost:3000/auth/discord";

  useEffect(() => {
    const fragment = new URLSearchParams(window.location.search);
    const code = fragment.get('code');

    if (!code) {
      router.replace("/");
      return;
    }

    fetch(api_url("/auth/discord"), {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        code,
        redirect_uri
      })
    })
      .then((res) => res.json())
      .then(res => {
        logIn(res.user, res.token)
        router.replace("/")
      })
    .catch(() => router.replace("/"));
  }, []);

  return <></>;
};

export default DiscordAuth;
