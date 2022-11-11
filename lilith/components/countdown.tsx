import { useEffect, useState } from "react";

type CountdownNumberProps = {
  number: number;
  description: string;
};

type CountdownProps = {
  to: Date;
  onFinal?: () => void;
};

export default function Countdown({ to, onFinal }: CountdownProps): JSX.Element {
  function CountdownNumber({
    number,
    description,
  }: CountdownNumberProps): JSX.Element {
    return (
      <div className="flex flex-col items-center w-14">
        <span className="text-3xl font-bold">{number}</span>
        <span className="text-sm font-bold">{description}</span>
      </div>
    );
  }

  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => setTime(new Date()), 1000);

    return () => {
      clearInterval(interval);
    };
  }, []);

  const diff = (to.getTime() - time.getTime()) / 1000;

  if (diff < 0 && onFinal) {
    onFinal();
  }

  const seconds = Math.floor(diff) % 60;
  let minutes = Math.floor(diff / 60) % 60;
  let hours = Math.floor(diff / 3600) % 24;
  let days = Math.floor(diff / 86400);

  return (
    <div className="flex gap-4 justify-center">
      <CountdownNumber number={days} description="days" />
      <CountdownNumber number={hours} description="hours" />
      <CountdownNumber number={minutes} description="minutes" />
      <CountdownNumber number={seconds} description="seconds" />
    </div>
  );
}
