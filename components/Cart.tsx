import { useCart } from "../utils/hooks.tsx";

export function Cart() {
  const { cart, setCart } = useCart();

  return (
    <div class="container absolute bottom-0 w-full bg-green-200 p-2">
      <div>
        {
          [...cart.reduce((list, curr) => {
            if (!list.has(curr.name)) {
              list.set(curr.name, Object.assign(curr, { amount: 0 }))
            }

            const menu = list.get(curr.name)

            list.set(curr.name, Object.assign(menu, { amount: menu.amount + 1 }));

            return list;
          }, new Map())].map(([name, menu]) => (
            <div class="flex justify-between">
              <div>{name}</div>
              <div class="inline-flex">
                <button class="px-3 py-1" onClick={() => setCart((menus) => {
                  const idx = menus.findLastIndex((menu) => menu.name === name)
                  menus.splice(idx, 1)
                  return menus
                })}>-</button>
                <div class="px-3 py-1">{menu.amount}</div>
                <button class="px-3 py-1" onClick={() => setCart((menus) => [...menus, { type: menu.type, name: menu.name, price: menu.price }])}>+</button>
              </div>
            </div>
          ))
        }
      </div>
      <div class="text-right">{`₩: ${cart.reduce((total, menu) => total + menu.price, 0)}`}</div>
    </div>
  )
}