import { useCart } from "../utils/hooks.tsx";

export function Cart() {
  const { cart } = useCart();

  return (
    <div class="container absolute bottom-0 w-full bg-green-200 p-2">
      <div>{`₩: ${cart.reduce((total, menu) => total + menu.price, 0)}`}</div>
    </div>
  )
}