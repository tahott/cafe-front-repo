import { useEffect, useState } from 'preact/hooks';
import MenuCard from '../components/MenuCard.tsx';
import { useInterval } from '../utils/hooks.tsx';
import { Category, Menu } from "../utils/types.tsx";

const DELAY = 1000;
const WAITING_TIME = 30; // seconds

interface KioskProps {
  categories: Array<Category>;
  menus: Array<Menu>;
}

export default function Kiosk({ categories, menus }: KioskProps) {
  const [seconds, setSeconds] = useState(0);
  const [isTouch, setPlaying] = useState(false);
  const [currentCategoryMenus, setCategoryMenus] = useState(menus);

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
    setPlaying(true);
  }

  const handleChangeCategory = (category: string) => {
    if (category === 'all') {
      setCategoryMenus(menus);
      return;
    }

    const m = menus.filter((menu) => menu.key === category);
    setCategoryMenus(m);
  }

  return (
    <>
      {
        !isTouch ? (
          <div class="h-full grid justify-items-center items-center text-center cursor-pointer text-2xl lg:text-3xl" onClick={handleTouch}>
            주문 하시려면 화면을 터치해 주세요 &#128070;
          </div>
        ) : (
          <div>
            <div class="m-2 text-md text-center font-medium text-gray-500 border-b border-gray-200">
              <ul class="flex flex-wrap -mb-px">
                {
                  categories.map((category) => {
                    return (
                      <li class="mr-2" onClick={() => handleChangeCategory(category.key)}>
                        <a href="#" class="inline-block p-4 border-b-2 border-transparent hover:text-gray-600 hover:border-gray-300">
                          {category.name}
                        </a>
                      </li>
                    )
                  })
                }
              </ul>    
            </div>
            <div class="m-2 grid grid-cols-4 gap-[8px] md:grid-cols-8 md:gap-[16px] lg:grid-cols-4 lg:gap-[16px]">
              {
                currentCategoryMenus.map(({ name, price }) => <MenuCard name={name} price={price} />)    
              }
            </div>
          </div>
        )
      }
    </>
  )
}