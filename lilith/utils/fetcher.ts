export const fetcher = async (url: string) => {
  const res = await fetch(url);

  if (!res.ok)
    throw new Error("failed to make request");

  return await res.json();
}

export function api_url(url: string): string {
  return process.env.NEXT_PUBLIC_API_URL + url;
}
