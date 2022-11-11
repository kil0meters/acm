import moment from "moment";
import type { NextPage } from "next";
import { useRouter } from "next/router";
import { useState } from "react";
import Navbar from "../../components/navbar";
import { isServerError, ServerError } from "../../components/problem/submission/error";
import { api_url } from "../../utils/fetcher";
import { useSession } from "../../utils/state";

const NewCompetitionPage: NextPage = () => {
  const [name, setName] = useState("");
  const [start, setStart] = useState(new Date);
  const [end, setEnd] = useState(new Date);
  const router = useRouter();
  const setError = useSession((state) => state.setError);

  const submit = async () => {
    const res: { id: number } | ServerError = await (await fetch(api_url("/competitions/new"), {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",
      body: JSON.stringify({
        name,
        start: start.toISOString().slice(0, -1),
        end: end.toISOString().slice(0, -1),
      }),
    })).json();

    if (isServerError(res)) {
      setError(res.error, true);
    } else {
      router.push(`/competitions/${res.id}`);
    }
  };

  return (
    <>
      <Navbar />

      <div className="flex max-w-screen-md mx-auto flex-col gap-4 mt-4">
        <h1 className="text-3xl font-extrabold">New Competition</h1>

        <div className="w-full p-4 bg-white dark:bg-black dark:border-neutral-700 border-neutral-300 rounded-md mx-auto border flex flex-col gap-2">
          <span>Name</span>
          <input
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
          />

          <span>Start Date</span>
          <input
            type="datetime-local"
            value={moment(start).format("yyyy-MM-DDTHH:mm")}
            onChange={(e) => setStart(new Date(e.target.value))}
            className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
          />

          <span>End Date</span>
          <input
            type="datetime-local"
            value={moment(end).format("yyyy-MM-DDTHH:mm")}
            onChange={(e) => setEnd(new Date(e.target.value))}
            className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
          />

          <button
            onClick={submit}
            className="mt-2 rounded-full bg-green-500 hover:bg-green-700 px-4 py-2 text-green-50 transition-colors">
            Submit
          </button>
        </div>
      </div>

    </>
  );
};

export default NewCompetitionPage;
