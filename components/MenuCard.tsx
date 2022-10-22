interface MenuCardProps {
  name: string;
  price: number;
}

export default function MenuCard({ name, price }: MenuCardProps) {
  return (
    <div>
      <div class="grid justify-items-center cursor-pointer">
        <div class="w-[64px] h-[64px] grid items-center rounded-lg border opacity-50 lg:w-[96px] lg:h-[96px]">
          <img src="" />
        </div>
        <p class="truncate text-center text-sm">{name}</p>  
        <p class="text-center tetxt-xs">{`₩ ${price}`}</p>
      </div>
    </div>
  )
}