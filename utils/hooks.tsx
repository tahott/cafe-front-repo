import { useEffect, useRef } from "preact/hooks";

function useInterval (callback: () => void, delay: number | null) {
  const savedCallback = useRef(callback);

  useEffect(() => {
    function tick() {
      savedCallback.current();
    }
    if (delay !== null) {
      const id = setInterval(() => savedCallback.current(), delay);
      return () => clearInterval(id);
    }
  }, [delay]);
}

export { useInterval };