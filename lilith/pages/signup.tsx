import type { NextPage } from "next";
import { useRouter } from "next/router";
import { FormEvent, useRef, useState } from "react";
import ErrorBox from "../components/error-box";
import FormRow from "../components/form-row";
import Modal from "../components/modal";
import Navbar from "../components/navbar";
import { api_url } from "../utils/fetcher";
import { useStore } from "../utils/state";

type Error = {
  shown: boolean;
  error: string;
};

const Signup: NextPage = () => {
  const [error, setError] = useState<Error>({ error: "", shown: false });
  const formRef = useRef<HTMLFormElement>(null);
  const router = useRouter();
  const logIn = useStore((state) => state.logIn);

  async function signUp(e: FormEvent) {
    e.preventDefault();

    const form = formRef.current;

    if (!form) return;

    const name: string = form["Name"].value;
    const username: string = form["Username"].value;
    const password: string = form["Password"].value;

    fetch(api_url("/auth/signup"), {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ name, username, password }),
    })
      .then(async (res) => {
        let data = await res.json();
        console.log({ response: data });

        if (data.error) {
          setError({ error: data.error, shown: true });
        } else {
          logIn(data.user, data.token);
          router.push("/");
        }
      })
      .catch(() => {
        setError({ error: "Failed to make request", shown: false });
      });
  }

  return (
    <>
      <Navbar />

      <div className="max-w-md mx-auto px-2 sm:p-0 mt-4 flex flex-col gap-4 text-lg">
        <Modal
          shown={error.shown}
          onClose={() => setError({ error: error.error, shown: false })}
        >
          <ErrorBox>{error.error}</ErrorBox>
        </Modal>

        <h1 className="text-4xl font-bold">{"Join."}</h1>

        <form
          ref={formRef}
          className="flex flex-col gap-4"
          name="signup-form"
          onSubmit={signUp}
          method="POST"
        >
          <FormRow
            title="Name"
            maxLength={16}
            placeholder="'); DROP TABLE STUDENTS;"
          />
          <FormRow
            title="Username"
            pattern="[a-zA-Z0-9]+"
            placeholder="username"
            maxLength={16}
          />
          <FormRow
            title="Password"
            pattern="[a-zA-Z0-9!@#$%^&*()\s]+"
            placeholder="password"
            maxLength={256}
            inputType="password"
          />

          <button
            className="rounded hover:ring transition-all focus:ring-4 border border-green-700 hover:bg-green-500 ring-green-700 text-green-100 p-2 bg-green-600"
            type="submit"
          >
            {"Sign up"}
          </button>
        </form>
      </div>
    </>
  );
};

export default Signup;
