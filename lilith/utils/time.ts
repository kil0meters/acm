export function timeFormat(timeStr: string): string {
    let dateTime = new Date(timeStr);

    let date = dateTime.toLocaleDateString("en-us", {
        weekday: "short",
        month: "short",
        day: "numeric",
        year: "numeric"
    });
    let time = dateTime.toLocaleTimeString("en-us", {
        hour: "numeric",
        minute: "numeric",
    });

    return `${date} ${time}`;
}
