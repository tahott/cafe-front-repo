import { useCart } from '../utils/hooks.tsx';

interface MenuCardProps {
  type: string;
  name: string;
  price: number;
}

export default function MenuCard({ type, name, price }: MenuCardProps) {
  const { setCart } = useCart();

  const handleAddMenu = (type: string, name: string, price: number) => {
    setCart((cart) => [...cart, { type, name, price }])
  }

  return (
    <div>
      <div class="grid justify-items-center cursor-pointer" onClick={() => handleAddMenu(type, name, price)}>
        <div class="w-[64px] h-[64px] grid items-center rounded-lg border opacity-50 lg:w-[96px] lg:h-[96px]">
          <img src="" />
        </div>
        <p class="truncate text-center text-sm">{name}</p>  
        <p class="text-center tetxt-xs">{`₩ ${price}`}</p>
      </div>
    </div>
  )
}