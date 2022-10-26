import { useCart } from "../utils/hooks.tsx";

export function Cart() {
  const { cart } = useCart();

  return (
    <div class="container absolute bottom-0 w-full bg-green-200 p-2">
      <div>
        {
          [...cart.reduce((list, curr) => {
            if (!list.has(curr.name)) {
              list.set(curr.name, 0)
            }

            list.set(curr.name, list.get(curr.name) + 1);

            return list;
          }, new Map())].map(([name, amount]) => (
            <div class="flex justify-between">
              <div>{name}</div>
              <div>{amount}</div>
            </div>
          ))
        }
      </div>
      <div class="text-right">{`₩: ${cart.reduce((total, menu) => total + menu.price, 0)}`}</div>
    </div>
  )
}