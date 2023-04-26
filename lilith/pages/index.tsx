import type { NextPage } from "next";
import Head from "next/head";
import Footer from "../components/footer";
import Navbar from "../components/navbar";
import ProblemView from "../components/problem";
// import ProblemView from "../components/problem";
import { CompetitionGrid } from "./competitions";

const Home: NextPage = () => {
    return (
        <div className="overflow-x-hidden flex flex-col gap-4 min-h-screen">
            <Head>
                <title>Chico ACM</title>
            </Head>

            <Navbar />

            <h1 className="py-4 text-6xl drop-shadow-md text-center font-extrabold text-transparent bg-clip-text bg-gradient-to-b from-neutral-600 to-neutral-900 dark:from-neutral-50 dark:to-neutral-400">
                Chico ACM
            </h1>

            <div className="md:container w-full mx-auto">
                <h2 className="font-bold text-xl mx-2 md:mx-0">Local Competitions</h2>
                <CompetitionGrid />
            </div>

            <div className="md:container md:h-[80vh] md:shadow md:rounded border-neutral-300 dark:border-neutral-700 mx-auto border-y md:border-x w-full overflow-auto">
                <ProblemView id={1} />
            </div>

            <Footer />
        </div>
    );
};

export default Home;
