import { NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { api_url } from "../../utils/fetcher";
import { useStore } from "../../utils/state";

const DiscordAuth: NextPage = () => {
  const logIn = useStore((state) => state.logIn);
  const router = useRouter();

  useEffect(() => {
    const fragment = new URLSearchParams(window.location.hash.slice(1));
    const [accessToken, tokenType, expiresIn] = [fragment.get('access_token'), fragment.get('token_type'), fragment.get('expires_in')];

    if (!accessToken || !tokenType || !expiresIn) {
      router.replace("/");
      return;
    }

    fetch(api_url("/auth/discord"), {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        access_token: accessToken,
        token_type: tokenType,
        expires_in: +expiresIn
      })
    })
      .then((res) => res.json())
      .then(res => {
        logIn(res.user, res.discord_token, res.token)
        router.replace("/")
      })
    .catch(() => router.replace("/"));
  }, []);

  return <></>;
};

export default DiscordAuth;
