import { animated, useTransition } from "@react-spring/web";

type ModalProps = {
  children: JSX.Element;
  shown: boolean;
  onClose: () => void;
};

export default function Modal({
  shown,
  children,
  onClose,
}: ModalProps): JSX.Element {
  const fadeIn = useTransition(shown, {
    from: {
      opacity: 0,
    },
    enter: {
      opacity: 1,
    },
    leave: {
      opacity: 0,
    },
  });

  const zoomIn = useTransition(shown, {
    from: {
      opacity: 0,
      transform: "translate3d(0, 100px, 0) scale(0.7)",
    },
    enter: {
      opacity: 1,
      transform: "translate3d(0, 0px, 0) scale(1)",
    },
    leave: {
      opacity: 0,
      transform: "translate3d(0, 100px, 0) scale(0.7)",
    },
  });

  let mousePressed = false;

  return fadeIn(
    (styles, item) =>
      item && (
        <animated.div
          ref={(node) =>
            node &&
            (!shown
              ? node.setAttribute("inert", "")
              : node.removeAttribute("inert"))
          }
          style={styles}
          onMouseDown={() => {
            mousePressed = true;
          }}
          onMouseUp={() => {
            if (mousePressed) {
              onClose();
            }
          }}
          className="fixed left-0 right-0 top-0 bottom-0 bg-black/30 z-50 overflow-y-auto"
        >
          {zoomIn(
            (styles, item) =>
              item && (
                <animated.div
                  style={styles}
                  onMouseDown={(e) => e.stopPropagation()}
                  className="mt-16 max-w-lg mx-auto p-2 relative"
                >
                  {children}
                </animated.div>
              )
          )}
        </animated.div>
      )
  );
}
