import { createContext } from "react";

export type Meeting = {
  id: number;
  title: string;
  meeting_time: string;
};

export const MeetingContext = createContext<Meeting | undefined>(undefined);
