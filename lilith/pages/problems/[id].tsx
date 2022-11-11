import type { NextPage } from "next";
import Navbar from "../../components/navbar";
import { useRouter } from "next/router";
import ProblemView from "../../components/problem";

const ProblemPage: NextPage = () => {
  const { query, isReady } = useRouter();
  const id = isReady ? parseInt(query.id as string) : undefined;

  return (
    <div className="h-screen w-screen grid grid-rows-min-full grid-cols-full">
      <Navbar />
      <ProblemView competitionId={query.competition && parseInt(query.competition as string)} id={id} />
    </div>
  );
}

export default ProblemPage;
