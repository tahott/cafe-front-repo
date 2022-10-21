import { useEffect, useState } from 'preact/hooks';
import { useInterval } from '../utils/hooks.tsx';

const DELAY = 1000;
const WAITING_TIME = 30; // seconds

export default function Kiosk() {
  const [seconds, setSeconds] = useState(0);
  const [isTouch, setPlaying] = useState(false);

  useInterval(() => {
    setSeconds((second) => second + 1)
  }, isTouch ? DELAY : null);

  useEffect(() => {
    if (seconds > WAITING_TIME) {
      setPlaying(false);
      setSeconds(0);
    } 
  }, [seconds])

  const handleTouch = () => {
    setPlaying(true)
  }

  return (
    <>
      {
        !isTouch ? (
          <div class="h-full flex items-center text-center cursor-pointer text-2xl lg:text-3xl" onClick={handleTouch}>
            주문 하시려면 화면을 터치해 주세요 &#128070;
          </div>
        ) : (
          <div class="m-2">
          </div>
        )
      }
    </>
  )
}