import { useTransition, animated, useSpring } from "@react-spring/web";

type LoadingButtonProps = {
  className?: string;
  loading?: boolean;
  onClick?: () => void;
  children: React.ReactNode;
};

export default function LoadingButton({
  className,
  loading,
  onClick,
  children,
}: LoadingButtonProps): JSX.Element {
  const contentStyles = useSpring({
    marginLeft: loading ? "30px" : "0px",
  });

  const spinnerStyles = useTransition(loading, {
    from: {
      opacity: 0,
      left: "-8px",
    },
    enter: {
      opacity: 1,
      left: "0px",
    },
    leave: {
      opacity: 0,
      left: "-8px",
    },
  });

  return (
    <button onClick={onClick} className={className}>
      <div className="relative">
        {spinnerStyles(
          (styles, item) =>
            item && (
              <animated.svg
                style={styles}
                className="absolute animate-spin h-5 w-5 text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  className="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="4"
                ></circle>
                <path
                  className="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
              </animated.svg>
            )
        )}

        <animated.span
          style={contentStyles}
          className="h-5 flex items-center justify-center"
        >
          {children}
        </animated.span>
      </div>
    </button>
  );
}
