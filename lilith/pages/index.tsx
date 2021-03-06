import type { NextPage } from "next";
import Footer from "../components/footer";
import Navbar from "../components/navbar";
import ProblemView from "../components/problem";

const Home: NextPage = () => {
  return (
    <div className="overflow-x-hidden flex flex-col gap-4 min-h-screen justify-center items-center">
      <Navbar />

      <h2 className="py-4 text-center text-6xl drop-shadow-md font-extrabold text-transparent bg-clip-text bg-gradient-to-b from-neutral-600 to-neutral-900 dark:from-neutral-50 dark:to-neutral-400">
        { "Chico ACM" }
      </h2>

      <p className="text-lg text-center">{ "We aren't meeting at the moment, but feel free to try your hand at a programming challenge." }</p>

      <div className="border-y md:border md:rounded-lg border-neutral-300 dark:border-neutral-700 overflow-hidden md:shadow-md w-full max-w-screen-xl md:mx-4 grow md:h-0">
        <ProblemView id={1} />
      </div>

      <Footer />
    </div>
  );
};

export default Home;
