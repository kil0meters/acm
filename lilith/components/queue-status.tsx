type QueueStatusProps = {
  queuePosition: number
  className?: string,
};

export default function QueueStatus({
  queuePosition,
  className
}: QueueStatusProps): JSX.Element {
  return <>
    {queuePosition != 0 && <span className={`animate-pulse ${className}`}>
      Position in queue: {queuePosition}
    </span>}
  </>;
}
