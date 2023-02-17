import create from "zustand";
import { persist, StateStorage } from "zustand/middleware";
import produce from "immer";
import { FunctionValue, Test, WasmFunctionCall } from "../components/problem/submission/tests";
import { Activity } from "../pages/meetings/new";
import { get, set, del } from 'idb-keyval';

const idb: StateStorage = {
    getItem: async (name: string): Promise<string | null> => {
        return (await get(name)) || null
    },
    setItem: async (name: string, value: string): Promise<void> => {
        await set(name, value)
    },
    removeItem: async (name: string): Promise<void> => {
        await del(name)
    },
}

type EditorThemeType = "light" | "dark" | "system";

type Auth = "ADMIN" | "OFFICER" | "MEMBER";

export interface User {
    id: number;
    name: string;
    username: string;
    discord_id: string;
    auth: Auth;
}

export type AsymptoticComplexity =
    "EXPONENTIAL" |
    "QUADRATIC" |
    "LOG_LINEAR" |
    "LINEAR" |
    "SQRT" |
    "LOG" |
    "CONSTANT";

export type Submission = {
    id: number;
    problem_id: number;
    user_id: number;
    error?: string;
    success: boolean;
    code: string;
    runtime: number;
    time: string;
    complexity?: AsymptoticComplexity;
}


export interface Store {
    vimEnabled: boolean;
    editorTheme: EditorThemeType;

    problemImpls: { [key: number]: string };

    setVimEnabled: (vimEnabled: boolean) => void;
    setEditorTheme: (editorTheme: EditorThemeType) => void;

    setProblemImpl: (id: number, impl: string) => void;
}

export const useStore = create<Store>()(
    persist(
        (set) => ({
            vimEnabled: false,
            editorTheme: "system",
            problemImpls: {},

            setProblemImpl: (id, impl) =>
                set(
                    produce((state: Store) => {
                        state.problemImpls[id] = impl;
                    })
                ),

            setVimEnabled: (vimEnabled) =>
                set(
                    produce((state: Store) => {
                        state.vimEnabled = vimEnabled;
                    })
                ),

            setEditorTheme: (editorTheme) =>
                set(
                    produce((state: Store) => {
                        state.editorTheme = editorTheme;
                    })
                ),
        }),
        {
            name: "data",
            getStorage: () => idb,
        }
    )
);

export interface Session {
    error: string;
    errorShown: boolean;
    submissionShown: boolean;

    setSubmissionShown: (shown: boolean) => void;
    setError: (error: string, shown: boolean) => void;
}

export const useSession = create<Session>()((set) => ({
    error: "",
    errorShown: false,
    submissionShown: true,

    setSubmissionShown: (shown) =>
        set(
            produce((state: Session) => {
                state.submissionShown = shown;
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
    problemTestFormat: WasmFunctionCall;
    problemDescription: string;
    problemReference: string;
    problemTemplate: string;
    problemPublishTime?: string;
    problemTests: Test[];
    problemDateShown: boolean;
    problemRuntimeMultiplier: number;
    problemCompetitionId?: number;

    setProblemTitle: (title: string) => void;
    setProblemTestFormat: (type: WasmFunctionCall) => void;
    setProblemDescription: (description: string) => void;
    setProlbemReference: (reference: string) => void;
    setProblemTemplate: (template: string) => void;
    setProblemDateShown: (shown: boolean) => void;

    updateProblemTest: (index: number, test: Partial<Test>) => void;
    pushProblemTest: (test: Test) => void;
    popProblemTest: () => void;
    setProblemTests: (tests: Test[]) => void;

    clearProblemCreation: () => void,

    meetingTitle: string;
    meetingDescription: string;
    meetingTime: string;
    meetingActivities: Activity[];

    setProblemPublishTime: (time?: string) => void;
    setProblemCompetitionId: (competitionId?: number) => void;
    setProblemRuntimeMultiplier: (multiplier: number) => void;

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
            problemReference: "",
            problemTemplate: "",
            problemTestFormat: {
                name: "",
                arguments: [],
                return_type: { Int: "Single" }
            },
            problemTests: [],
            problemRuntimeMultiplier: 1.1,
            problemDateShown: false,

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


            setProblemTestFormat: (testFormat: WasmFunctionCall) => set(
                produce((state: AdminState) => {
                    state.problemTestFormat = testFormat;
                })
            ),

            setProblemDescription: (description: string) =>
                set(
                    produce((state: AdminState) => {
                        state.problemDescription = description;
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

            setProblemDateShown: (shown: boolean) =>
                set(
                    produce((state: AdminState) => {
                        state.problemDateShown = shown;
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

            pushProblemTest: (test: Test) =>
                set(
                    produce((state: AdminState) => {
                        state.problemTests.push(test);
                    })
                ),

            popProblemTest: () =>
                set(
                    produce((state: AdminState) => {
                        state.problemTests.pop();
                    })
                ),

            setProblemPublishTime: (time?: string) =>
                set(
                    produce((state: AdminState) => {
                        state.problemPublishTime = time;
                    })
                ),

            setProblemCompetitionId: (competitionId?: number) =>
                set(
                    produce((state: AdminState) => {
                        state.problemCompetitionId = competitionId;
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


            setProblemRuntimeMultiplier: (multiplier: number) =>
                set(
                    produce((state: AdminState) => {
                        state.problemRuntimeMultiplier = multiplier;
                    })
                )

        }),
        {
            name: "admin_data",
            getStorage: () => idb,
        }
    )
);
