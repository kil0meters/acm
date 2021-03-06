import create from "zustand";
import { persist } from "zustand/middleware";
import produce from "immer";
import { Test } from "../components/problem/submission/tests";
import { Activity } from "../pages/meetings/new";

export interface User {
  name: string;
  username: string;
  auth: "ADMIN" | "OFFICER" | "MEMBER";
}

export interface Submission {
  id: number;
  problem_id: number;
  error?: string;
  success: boolean;
  code: string;
  runtime: string;
  time: string;
}

export interface Store {
  token?: string;
  user?: User;

  problemImpls: { [key: number]: string };

  setProblemImpl: (id: number, impl: string) => void;

  logIn: (user: User, token: string) => void;
  logOut: () => void;
}

export const useStore = create<Store>()(
  persist(
    (set) => ({
      token: undefined,
      user: undefined,
      problemImpls: {},

      setProblemImpl: (id, impl) =>
        set(
          produce((state: Store) => {
            state.problemImpls[id] = impl
          })
        ),

      logIn: (user, token) =>
        set(
          produce((state: Store) => {
            state.user = user;
            state.token = token;
          })
        ),

      logOut: () =>
        set(
          produce((state: Store) => {
            state.user = undefined;
            state.token = undefined;
          })
        ),
    }),
    {
      name: "data",
      getStorage: () => localStorage,
    }
  )
);

export interface Session {
  error: string;
  errorShown: boolean;

  submissions: { [key: number]: Submission };
  setSubmission: (id: number, submission: Submission) => void;

  setError: (error: string, shown: boolean) => void;
}

export const useSession = create<Session>()((set) => ({
  error: "",
  errorShown: false,

  submissions: {},

  setSubmission: (id, submission) =>
    set(
      produce((state: Session) => {
        state.submissions[id] = submission;
      })
    ),

  setError: (error, shown) =>
    set(
      produce((state: Session) => {
        state.error = error;
        state.errorShown = shown;
      })
    ),
}));

// TODO: restructure this to use nested classes instead.
export interface AdminState {
  problemTitle: string;
  problemDescription: string;
  problemRunner: string;
  problemReference: string;
  problemTemplate: string;
  problemTests: Test[];

  setProblemTitle: (title: string) => void;
  setProblemDescription: (description: string) => void;
  setProblemRunner: (runner: string) => void;
  setProlbemReference: (reference: string) => void;
  setProblemTemplate: (template: string) => void;

  updateProblemTest: (index: number, test: Partial<Test>) => void;
  pushProblemTest: () => void;
  popProblemTest: () => void;
  setProblemTests: (tests: Test[]) => void;

  clearProblemCreation: () => void,

  meetingTitle: string;
  meetingDescription: string;
  meetingTime: string;
  meetingActivities: Activity[];

  setMeetingTitle: (title: string) => void;
  setMeetingDescription: (description: string) => void;
  setMeetingTime: (time: string) => void;

  updateMeetingActivity: (index: number, test: Partial<Activity>) => void;
  pushMeetingActivity: () => void;
  popMeetingActivity: () => void;
}

export const useAdminStore = create<AdminState>()(
  persist(
    (set) => ({
      problemTitle: "",
      problemDescription: "",
      problemRunner: "",
      problemReference: "",
      problemTemplate: "",
      problemTests: [],

      meetingTitle: "",
      meetingDescription: "",
      meetingTime: "",
      meetingActivities: [],

      setProblemTitle: (title: string) =>
        set(
          produce((state: AdminState) => {
            state.problemTitle = title
          })
        ),

      setProblemDescription: (description: string) =>
        set(
          produce((state: AdminState) => {
            state.problemDescription = description;
          })
        ),

      setProblemRunner: (runner: string) =>
        set(
          produce((state: AdminState) => {
            state.problemRunner = runner;
          })
        ),

      setProlbemReference: (reference: string) =>
        set(
          produce((state: AdminState) => {
            state.problemReference = reference;
          })
        ),

      setProblemTemplate: (template: string) =>
        set(
          produce((state: AdminState) => {
            state.problemTemplate = template;
          })
        ),

      // assumes test index is valid. should always be the case.
      updateProblemTest: (index: number, test: Partial<Test>) =>
        set(
          produce((state: AdminState) => {
            state.problemTests[index]! = {
              ...state.problemTests[index]!,
              ...test,
            };
          })
        ),

      pushProblemTest: () =>
        set(
          produce((state: AdminState) => {
            state.problemTests.push({
              id: 0,
              index: state.problemTests.length,
              input: "",
              expected_output: "",
            });
          })
        ),

      popProblemTest: () =>
        set(
          produce((state: AdminState) => {
            state.problemTests.pop();
          })
        ),

      setProblemTests: (tests: Test[]) =>
        set(
          produce((state: AdminState) => {
            state.problemTests = tests;
          })
        ),

      clearProblemCreation: () =>
        set(
          produce((state: AdminState) => {
            state.problemTests = [];
            state.problemTitle = "";
            state.problemDescription = "";
            state.problemReference = "";
            state.problemRunner = "";
            state.problemTemplate = "";
          })
        ),

      setMeetingTitle: (title: string) =>
        set(
          produce((state: AdminState) => {
            state.meetingTitle = title;
          })
        ),

      setMeetingTime: (time: string) =>
        set(
          produce((state: AdminState) => {
            state.meetingTime = time;
          })
        ),

      setMeetingDescription: (description: string) =>
        set(
          produce((state: AdminState) => {
            state.meetingDescription = description;
          })
        ),

      // assumes test index is valid. should always be the case.
      updateMeetingActivity: (index: number, activity: Partial<Activity>) =>
        set(
          produce((state: AdminState) => {
            state.meetingActivities[index]! = {
              ...state.meetingActivities[index]!,
              ...activity,
            };
          })
        ),

      pushMeetingActivity: () =>
        set(
          produce((state: AdminState) => {
            state.meetingActivities.push({
              title: "",
              description: "",
              activity_type: "SOLO"
            });
          })
        ),

      popMeetingActivity: () =>
        set(
          produce((state: AdminState) => {
            state.meetingActivities.pop();
          })
        ),

    }),
    {
      name: "admin_data",
      getStorage: () => localStorage,
    }
  )
);
