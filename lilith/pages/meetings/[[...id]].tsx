import Navbar from "../../components/navbar";
import { MeetingContext, Meeting } from "../../components/meetings";
import Activities from "../../components/meetings/activities";
import useSWR from "swr";
import { NextPage } from "next";
import { api_url, fetcher } from "../../utils/fetcher";
import Schedule from "../../components/meetings/schedule";
import MeetingView from "../../components/meetings/meeting";
import { useRouter } from "next/router";
import Error from "next/error";

const Meetings: NextPage = () => {
  const { query, isReady } = useRouter();
  const id = query.id ? query.id : "next";

  const { data: meeting, error } = useSWR<Meeting>(
    isReady ? api_url(`/meetings/${id}`) : null,
    fetcher
  );

  if (error) return <Error statusCode={404} />;

  return (
    <MeetingContext.Provider value={meeting}>
      <Navbar />

      <div className="grid grid-rows-[min-content_1fr] md:grid-cols-[1fr_300px] md:grid-rows-1 max-w-screen-lg mx-auto gap-2 my-2">
        <div className="sm:px-2 flex flex-col gap-2">
          <MeetingView />
          <Activities />
        </div>

        <Schedule />
      </div>
    </MeetingContext.Provider>
  );
};

export default Meetings;
