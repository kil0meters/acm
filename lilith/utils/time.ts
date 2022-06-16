export function timeFormat(timeStr: string): string {
  let dateTime = new Date(timeStr);

  let date = dateTime.toLocaleDateString("en-us", {
    weekday: "long",
    month: "short",
    day: "numeric",
  });
  let time = dateTime.toLocaleTimeString("en-us", {
    hour: "numeric",
    minute: "numeric",
  });

  return `${date} @ ${time}`;
}
