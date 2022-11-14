import type { NextPage } from "next";
import Head from "next/head";
import Footer from "../components/footer";
import Navbar from "../components/navbar";
// import ProblemView from "../components/problem";
import { CompetitionGrid } from "./competitions";

const Home: NextPage = () => {
  return (
    <div className="overflow-x-hidden flex flex-col gap-4 min-h-screen justify-center items-center">
      <Head>
        <title>Chico ACM</title>
      </Head>

      <Navbar />

      <h2 className="py-4 text-center text-6xl drop-shadow-md font-extrabold text-transparent bg-clip-text bg-gradient-to-b from-neutral-600 to-neutral-900 dark:from-neutral-50 dark:to-neutral-400">
        {"Chico ACM"}
      </h2>

      <CompetitionGrid />

      <Footer />
    </div>
  );
};

export default Home;
